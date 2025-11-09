use anyhow::{anyhow, Result};
use reqwest::Client;
use tracing::debug;
use crate::core::client::k8s::container_dto::{ContainerInfo, ContainerList, ContainerStatus};
use crate::core::client::k8s::pod::{fetch_pod_by_uid, fetch_pods};

/// Fetch all containers across all pods
pub async fn fetch_containers(token: &str, client: &Client) -> Result<ContainerList> {
    debug!("Fetching all containers across all pods");
    let pods = fetch_pods(token, client).await?;

    let mut all_containers = Vec::new();

    for p in pods.items.iter() {
        // Convert both match arms to the same type: &[ContainerStatus]
        let statuses: &[ContainerStatus] = match &p.status {
            Some(s) => &s.container_statuses,
            None => &[],
        };

        for c in &p.spec.containers {
            let status = statuses.iter().find(|s| s.name == c.name);

            all_containers.push(ContainerInfo {
                container_name: c.name.clone(),
                image: c.image.clone(),
                pod_name: p.metadata.name.clone(),
                namespace: p.metadata.namespace.clone(),
                image_id: status.and_then(|s| s.image_id.clone()),
                container_id: status.and_then(|s| s.container_id.clone()),
                ready: status.map(|s| s.ready),
                restart_count: status.map(|s| s.restart_count),
                started_at: status
                    .and_then(|s| s.state.as_ref())
                    .and_then(|st| st.running.as_ref())
                    .and_then(|r| r.started_at.clone()),
            });
        }
    }

    debug!("Discovered {} container(s)", all_containers.len());
    Ok(ContainerList { items: all_containers })
}
