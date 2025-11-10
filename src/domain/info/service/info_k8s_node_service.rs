use anyhow::Result;
use chrono::{Duration, Utc};
use crate::core::persistence::info::k8s::node::info_node_api_repository_trait::InfoNodeApiRepository;
use crate::core::persistence::info::k8s::node::info_node_entity::InfoNodeEntity;
use crate::domain::info::repository::info_k8s_node_api_repository::InfoK8sNodeApiRepositoryImpl;
use tracing::{debug};
use crate::core::client::k8s::client_k8s_node::{fetch_node_by_name, fetch_nodes};
use crate::core::client::k8s::client_k8s_node_mapper::map_node_to_node_info_entity;
use crate::core::client::k8s::util::{build_client, read_token};

pub async fn get_info_k8s_node(node_name: String) -> Result<InfoNodeEntity> {
    let repo = InfoK8sNodeApiRepositoryImpl::default();

    // Load existing entity
    let entity = repo.read(&node_name)?;

    let now = Utc::now();
    let needs_refresh = match entity.last_updated_info_at {
        None => true,
        Some(last) => now.signed_duration_since(last) > Duration::hours(1),
    };

    if needs_refresh {
        debug!("Node '{}' info is missing or stale — refreshing from K8s API", node_name);

        // Build K8s client
        let token = read_token()?;
        let client = build_client()?;

        // Fetch from K8s API
        let node = fetch_node_by_name(&token, &client, &node_name).await?;
        let updated_entity = map_node_to_node_info_entity(&node)?;

        // Save refreshed info
        repo.update(&updated_entity)?;

        debug!(
            "Updated node '{}' info successfully (last_updated_info_at = {})",
            node_name, now
        );

        Ok(updated_entity)
    } else {
        debug!(
            "Node '{}' info is up-to-date (last_updated_info_at = {:?})",
            node_name, entity.last_updated_info_at
        );
        Ok(entity)
    }
}


/// Lists all Kubernetes nodes, refreshing local cache if older than 1h
pub async fn list_k8s_nodes() -> Result<Vec<InfoNodeEntity>> {
    debug!("Listing all Kubernetes nodes");

    // 1️⃣ Build client & token
    let token = read_token()?;
    let client = build_client()?;

    // 2️⃣ Fetch nodes from K8s API
    let node_list = fetch_nodes(&token, &client).await?;
    debug!("Fetched {} node(s) from API", node_list.items.len());

    // 3️⃣ Repository to persist/update
    let repo = InfoK8sNodeApiRepositoryImpl::default();
    let mut result_entities = Vec::new();

    // 4️⃣ Process each node
    for node in node_list.items.iter() {
        let node_name = node.metadata.name.clone();

        // --- Try to load existing record
        let existing = repo.read(&node_name).ok();

        // --- Decide if refresh is needed
        let needs_refresh = match existing.as_ref() {
            None => true,
            Some(entity) => entity
                .last_updated_info_at
                .map(|t| Utc::now().signed_duration_since(t) > Duration::hours(1))
                .unwrap_or(true),
        };

        let entity = if needs_refresh {
            debug!("Refreshing node info for '{}'", node_name);
            let mapped = map_node_to_node_info_entity(node)?;
            repo.update(&mapped)?;
            mapped
        } else {
            debug!("Using cached node info for '{}'", node_name);
            existing.unwrap()
        };

        result_entities.push(entity);
    }

    Ok(result_entities)
}