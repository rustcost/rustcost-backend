use anyhow::Result;
use chrono::{Duration, Utc};
use crate::core::persistence::info::k8s::node::info_node_api_repository_trait::InfoNodeApiRepository;
use crate::core::persistence::info::k8s::node::info_node_entity::InfoNodeEntity;
use crate::domain::info::repository::info_k8s_node_api_repository::InfoK8sNodeApiRepositoryImpl;
use tracing::{debug, error};
use crate::core::client::k8s::client_k8s_node::fetch_node_by_name;
use crate::core::client::k8s::client_k8s_node_mapper::map_node_to_node_info_entity;
use crate::core::client::k8s::util::{build_client, read_token};

pub async fn get_info_k8s_node(node_name: String) -> Result<InfoNodeEntity> {
    let repo = InfoK8sNodeApiRepositoryImpl::default();

    // Load existing entity
    let mut entity = repo.read(&node_name)?;

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
