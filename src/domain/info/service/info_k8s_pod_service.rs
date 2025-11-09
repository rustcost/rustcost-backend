use anyhow::Result;
use chrono::{Duration, Utc};
use serde_json::to_string_pretty;
use tracing::debug;
use crate::api::dto::info_dto::K8sListQuery;
use crate::core::client::k8s::client_k8s_pod::{fetch_pod_by_name_and_namespace, fetch_pod_by_uid, fetch_pods, fetch_pods_by_label, fetch_pods_by_namespace, fetch_pods_by_node};
use crate::core::client::k8s::client_k8s_pod_dto::Pod;
use crate::core::client::k8s::client_k8s_pod_mapper::map_pod_to_info_pod_entity;
use crate::core::client::k8s::util::{build_client, read_token};
use crate::core::persistence::info::k8s::pod::info_pod_api_repository_trait::InfoPodApiRepository;
use crate::core::persistence::info::k8s::pod::info_pod_entity::InfoPodEntity;
use crate::domain::info::repository::info_k8s_pod_api_repository::InfoK8sPodApiRepositoryImpl;

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

    debug!("Listing pods with filter {:?}", filter);

    // 1️⃣ Select appropriate fetcher
    let pod_list = if let Some(ns) = &filter.namespace {
        fetch_pods_by_namespace(&token, &client, ns).await?
    } else if let Some(label) = &filter.label_selector {
        fetch_pods_by_label(&token, &client, label).await?
    } else if let Some(node) = &filter.node_name {
        fetch_pods_by_node(&token, &client, node).await?
    } else {
        fetch_pods(&token, &client).await?
    };

    debug!("Fetched {} pod(s)", pod_list.items.len());

    // 2️⃣ Map all to InfoPodEntity + update repo
    // let mut result_entities = Vec::new();
    // for pod in pod_list.items {
    //     let pod_uid = pod.metadata.uid.clone();



        // --- Refresh or cache
        // let mapped = map_pod_to_info_pod_entity(&pod)?;
        // if let Err(e) = repo.update(&mapped) {
        //     debug!("Failed to update pod '{}': {:?}", pod_uid, e);
        // }
        // result_entities.push(mapped);
    //}
    //TODO!
    Ok(Default::default())
    //Ok(result_entities)
}
