use anyhow::{anyhow, Result};
use chrono::{Duration, Utc};
use tracing::debug;
use crate::api::dto::info_dto::K8sListQuery;
use crate::core::client::k8s::client_k8s_pod::{fetch_pod_by_name_and_namespace, fetch_pod_by_uid, fetch_pods, fetch_pods_by_label, fetch_pods_by_namespace, fetch_pods_by_node};
use crate::core::client::k8s::client_k8s_pod_mapper::map_pod_to_info_pod_entity;
use crate::core::client::k8s::util::{build_client, read_token};
use crate::core::persistence::info::k8s::pod::info_pod_api_repository_trait::InfoPodApiRepository;
use crate::core::persistence::info::k8s::pod::info_pod_entity::InfoPodEntity;
use crate::core::persistence::info::path::info_k8s_pod_dir_path;
use crate::domain::info::repository::info_k8s_pod_api_repository::InfoK8sPodApiRepositoryImpl;
use std::fs;
use crate::domain::info::dto::info_k8s_node_patch_request::InfoK8sNodePatchRequest;
use crate::domain::info::dto::info_k8s_pod_patch_request::InfoK8sPodPatchRequest;
use crate::domain::info::repository::info_k8s_node_api_repository::InfoK8sNodeApiRepositoryImpl;

pub async fn get_info_k8s_pod(pod_uid: String) -> Result<InfoPodEntity> {
    let repo = InfoK8sPodApiRepositoryImpl::default();

    // 1️⃣ Try read existing entity from repo
    if let Ok(existing) = repo.read(&pod_uid) {
        if let Some(ts) = existing.last_updated_info_at {
            if Utc::now().signed_duration_since(ts) <= Duration::hours(1) {
                debug!("✅ Using cached pod info for '{}'", pod_uid);
                return Ok(existing);
            }
        }

        // We have the metadata, use it for fetching fresh info
        if let (Some(ns), Some(name)) = (existing.namespace.clone(), existing.pod_name.clone()) {
            debug!("🔄 Cached data expired; fetching fresh pod info for '{}'", pod_uid);

            let token = read_token()?;
            let client = build_client()?;

            // 2️⃣ Fetch from K8s API
            let pod = fetch_pod_by_name_and_namespace(&token, &client, &ns, &name).await?;
            debug!("📦 Retrieved Pod '{}' in ns '{}'", name, ns);


            // 3️⃣ Map to InfoPodEntity
            let mut updated_entity = map_pod_to_info_pod_entity(&pod)?;
            updated_entity.last_updated_info_at = Some(Utc::now());
            updated_entity.pod_uid = Some(pod_uid.clone());
            debug!("🧩 Updated InfoPodEntity for '{}':", pod_uid);
            debug!("{}", serde_json::to_string_pretty(&updated_entity)?);

            // 4️⃣ Persist
            repo.update(&updated_entity)?;

            return Ok(updated_entity);
        } else {
            debug!("⚠️ Missing namespace or pod_name for '{}', cannot refresh.", pod_uid);
            return Ok(existing);
        }
    }

    // 5️⃣ No existing record: fetch fresh by UID (requires cluster-level list)
    debug!("🔍 No cache found; fetching pod '{}' by UID directly", pod_uid);
    let token = read_token()?;
    let client = build_client()?;


    // Fallback: if only UID known, use list+filter (requires RBAC permissions)
    let pod = fetch_pod_by_uid(&token, &client, &pod_uid).await?;
    let mut entity = map_pod_to_info_pod_entity(&pod)?;
    entity.last_updated_info_at = Some(Utc::now());
    entity.pod_uid = Some(pod_uid.clone());
    repo.insert(&entity)?;

    Ok(entity)
}


/// List Pods — supports optional filters: namespace, labelSelector, nodeName.
pub async fn list_k8s_pods(filter: K8sListQuery) -> Result<Vec<InfoPodEntity>> {
    let token = read_token()?;
    let client = build_client()?;
    let repo = InfoK8sPodApiRepositoryImpl::default();

    let mut cached_entities = Vec::new();
    let mut expired_or_missing = false;

    // 1️⃣ Try loading from filesystem cache via repo
    let pod_dir = info_k8s_pod_dir_path();
    if pod_dir.exists() {
        if let Ok(entries) = fs::read_dir(&pod_dir) {
            for entry in entries.flatten() {
                let pod_uid = entry.file_name().to_string_lossy().to_string();
                if let Ok(existing) = repo.read(&pod_uid) {
                    if let Some(ts) = existing.last_updated_info_at {
                        if Utc::now().signed_duration_since(ts) <= Duration::hours(1) {
                            debug!("✅ Using cached pod info for '{}'", pod_uid);
                            cached_entities.push(existing);
                            continue;
                        }
                    }
                }
                debug!("⚠️ Cache expired or missing for '{}'", pod_uid);
                expired_or_missing = true;
            }
        }
    }

    // 2️⃣ If all pods were fresh, return cached data only
    if !expired_or_missing && !cached_entities.is_empty() {
        debug!("📦 All cached pod info is fresh, skipping API call.");
        return Ok(cached_entities);
    }

    debug!("🌐 Fetching pods from K8s API (some cache expired or missing)");

    // 3️⃣ Select appropriate fetcher
    let pod_list = if let Some(ns) = &filter.namespace {
        fetch_pods_by_namespace(&token, &client, ns).await?
    } else if let Some(label) = &filter.label_selector {
        fetch_pods_by_label(&token, &client, label).await?
    } else if let Some(node) = &filter.node_name {
        fetch_pods_by_node(&token, &client, node).await?
    } else {
        fetch_pods(&token, &client).await?
    };

    debug!("Fetched {} pod(s) from API", pod_list.items.len());

    // 4️⃣ Map API pods → entities, update repo, and merge results
    let mut result_entities = cached_entities;
    for pod in pod_list.items {
        let pod_uid = pod.metadata.uid.clone();
        let mapped = map_pod_to_info_pod_entity(&pod)?;

        if let Err(e) = repo.update(&mapped) {
            debug!("⚠️ Failed to update pod '{}': {:?}", pod_uid, e);
        }

        result_entities.push(mapped);
    }

    Ok(result_entities)
}

pub async fn patch_info_k8s_pod(
    id: String,
    patch: InfoK8sPodPatchRequest,
) -> Result<serde_json::Value> {
    let repo = InfoK8sPodApiRepositoryImpl::default();

    // 1️⃣ Load existing record
    let mut entity = repo
        .read(&id)
        .map_err(|_| anyhow!("Node '{}' not found", id))?;

    // 2️⃣ Apply patch — only update fields that are Some()
    if let Some(team) = patch.team {
        entity.team = Some(team);
    }

    if let Some(service) = patch.service {
        entity.service = Some(service);
    }

    if let Some(env) = patch.env {
        entity.env = Some(env);
    }

    // 3️⃣ Update timestamp
    entity.last_updated_info_at = Some(Utc::now());

    // 4️⃣ Store back
    repo.update(&entity)?;

    // 5️⃣ Return updated JSON
    Ok(serde_json::to_value(&entity)?)
}