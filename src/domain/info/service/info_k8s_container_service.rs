use anyhow::{anyhow, Result};
use chrono::{Duration, Utc};
use serde_json::to_string_pretty;
use tracing::debug;

use crate::api::dto::info_dto::K8sListQuery;
use crate::core::client::k8s::util::{build_client, read_token};
use crate::core::persistence::info::k8s::container::info_container_api_repository_trait::InfoContainerApiRepository;
use crate::core::persistence::info::k8s::container::info_container_entity::InfoContainerEntity;
use crate::core::persistence::info::path::info_k8s_container_dir_path;
use crate::domain::info::repository::info_k8s_container_api_repository::InfoK8sContainerApiRepositoryImpl;
use std::fs;
use crate::core::client::k8s::client_k8s_container_mapper::map_container_status_to_info_container_entity;
use crate::core::client::k8s::client_k8s_pod::{fetch_pod_by_name_and_namespace, fetch_pods, fetch_pods_by_namespace, fetch_pods_by_node};

/// Fetch one container info by its unique ID, with cache + refresh if stale.
pub async fn get_info_k8s_container(container_id: String) -> Result<InfoContainerEntity> {
    let repo = InfoK8sContainerApiRepositoryImpl::default();

    // 1️⃣ Try reading existing entity from repo
    if let Ok(existing) = repo.read(&container_id) {
        if let Some(ts) = existing.last_updated_info_at {
            if Utc::now().signed_duration_since(ts) <= Duration::hours(1) {
                debug!("✅ Using cached container info for '{}'", container_id);
                return Ok(existing);
            }
        }

        // Cached but expired — refresh via API
        if let (Some(ns), Some(pod_name), Some(container_name)) = (
            existing.namespace.clone(),
            existing.pod_uid.clone(),
            existing.container_name.clone(),
        ) {
            debug!("🔄 Cache expired; fetching fresh container info for '{}'", container_id);

            let token = read_token()?;
            let client = build_client()?;

            let pod = fetch_pod_by_name_and_namespace(&token, &client, &ns, &pod_name).await?;
            let status = pod.status
                .as_ref()
                .and_then(|s| s.container_statuses.iter().find(|cs| cs.name == container_name));

            let spec = pod.spec
                .containers
                .iter()
                .find(|c| c.name == container_name)
                .ok_or_else(|| anyhow!("Container '{}' not found", container_name))?;

            let mut updated_entity =
                map_container_status_to_info_container_entity(&pod, spec, status)?;

            updated_entity.last_updated_info_at = Some(Utc::now());
            updated_entity.container_id = Some(container_id.clone());

            debug!("🧩 Updated InfoContainerEntity for '{}': {}", container_id, to_string_pretty(&updated_entity)?);

            repo.update(&updated_entity)?;
            return Ok(updated_entity);
        } else {
            debug!(
                "⚠️ Missing namespace/pod/container name for '{}', cannot refresh.",
                container_id
            );
            return Ok(existing);
        }
    }

    // 2️⃣ No cache found → fetch directly (requires identifiers)
    debug!(
        "🔍 No cache found; cannot fetch container '{}' without namespace/pod/container name",
        container_id
    );

    Err(anyhow!(
        "Missing namespace, pod name, or container name to fetch container '{}'",
        container_id
    ))
}


/// List containers — supports optional filters: namespace, pod_name, node_name.
/// List containers — supports optional filters: namespace, pod_name, node_name.
pub async fn list_k8s_containers(filter: K8sListQuery) -> Result<Vec<InfoContainerEntity>> {
    let token = read_token()?;
    let client = build_client()?;
    let repo = InfoK8sContainerApiRepositoryImpl::default();

    let mut fresh_entities = Vec::new();
    let mut expired_entities = Vec::new();

    // 1️⃣ Load cache
    let container_dir = info_k8s_container_dir_path();
    if container_dir.exists() {
        if let Ok(entries) = fs::read_dir(&container_dir) {
            for entry in entries.flatten() {
                let id = entry.file_name().to_string_lossy().to_string();
                if let Ok(existing) = repo.read(&id) {
                    if let Some(ts) = existing.last_updated_info_at {
                        if Utc::now().signed_duration_since(ts) <= Duration::hours(1) {
                            debug!("✅ Using cached container info for '{}'", id);
                            fresh_entities.push(existing);
                            continue;
                        }
                    }
                    debug!("⚠️ Cache expired for '{}'", id);
                    expired_entities.push(existing);
                }
            }
        }
    }

    // 2️⃣ If all cache entries are still valid
    if expired_entities.is_empty() {
        debug!("📦 All cached container info fresh, skipping API fetch.");
        return Ok(fresh_entities);
    }

    debug!(
        "🌐 Fetching {} expired/missing container(s) from K8s API",
        expired_entities.len()
    );

    // 3️⃣ Select appropriate fetcher
    let pods = if let Some(ns) = &filter.namespace {
        fetch_pods_by_namespace(&token, &client, ns).await?
    } else if let Some(node) = &filter.node_name {
        fetch_pods_by_node(&token, &client, node).await?
    } else {
        fetch_pods(&token, &client).await?
    };

    debug!("Fetched {} pod(s) from API", pods.items.len());

    // 4️⃣ Map pod containers → InfoContainerEntity
    for pod in pods.items {
        let ns = pod.metadata.namespace.clone();
        let pod_uid = pod.metadata.uid.clone();

        // `spec` is not Option, `status` is Option
        if let Some(status) = pod.status.as_ref() {
            for container in &pod.spec.containers {
                let container_name = &container.name;

                let cs = status
                    .container_statuses
                    .iter()
                    .find(|s| s.name == *container_name);

                let mut entity =
                    map_container_status_to_info_container_entity(&pod, container, cs).unwrap_or_else(|e| {
                        panic!("Failed to map container '{}': {:?}", container_name, e)
                    });


                entity.namespace = Some(ns.clone());
                entity.pod_uid = Some(pod_uid.clone());
                entity.container_name = Some(container_name.clone());
                entity.container_id = Some(format!("{}:{}:{}", ns, pod_uid, container_name));
                entity.last_updated_info_at = Some(Utc::now());

                if let Err(e) = repo.update(&entity) {
                    debug!(
                        "⚠️ Failed to update container '{:?}': {:?}",
                        entity.container_id, e
                    );
                }

                fresh_entities.push(entity);
            }
        }
    }

    Ok(fresh_entities)
}
