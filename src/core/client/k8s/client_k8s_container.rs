use anyhow::{anyhow, Result};
use reqwest::Client;
use tracing::debug;
use crate::core::client::k8s::client_k8s_container_dto::{ContainerInfo, ContainerList};
use crate::core::client::k8s::client_k8s_pod::{fetch_pod_by_name_and_namespace, fetch_pod_by_uid, fetch_pods};
use crate::core::client::k8s::client_k8s_pod_dto::ContainerStatus;

/// Fetch all containers across all pods
pub async fn fetch_containers(token: &str, client: &Client) -> Result<ContainerList> {
    debug!("Fetching all containers across all pods");
    let pods = fetch_pods(token, client).await?;

    let mut all_containers = Vec::new();
    let empty: [ContainerStatus; 0] = [];

    for p in pods.items.iter() {
        let statuses: &[ContainerStatus] = match &p.status {
            Some(s) => &s.container_statuses,
            None => &empty,
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
                ready: status.and_then(|s| s.ready),
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

/// Fetch **only container names** (across all pods)
pub async fn fetch_container_names(token: &str, client: &Client) -> Result<Vec<String>> {
    let containers = fetch_containers(token, client).await?;
    Ok(containers
        .items
        .into_iter()
        .map(|c| c.container_name)
        .collect())
}

/// Fetch all containers belonging to a specific pod (by UID)
pub async fn fetch_containers_by_pod_uid(
    token: &str,
    client: &Client,
    pod_uid: &str,
) -> Result<ContainerList> {
    debug!("Fetching containers in pod UID '{}'", pod_uid);
    let pod = fetch_pod_by_uid(token, client, pod_uid).await?;
    let empty: [ContainerStatus; 0] = [];

    let statuses: &[ContainerStatus] = match &pod.status {
        Some(s) => &s.container_statuses,
        None => &empty,
    };

    let containers = pod
        .spec
        .containers
        .iter()
        .map(|c| {
            let status = statuses.iter().find(|s| s.name == c.name);
            ContainerInfo {
                container_name: c.name.clone(),
                image: c.image.clone(),
                pod_name: pod.metadata.name.clone(),
                namespace: pod.metadata.namespace.clone(),
                image_id: status.and_then(|s| s.image_id.clone()),
                container_id: status.and_then(|s| s.container_id.clone()),
                ready: status.and_then(|s| s.ready),
                restart_count: status.map(|s| s.restart_count),
                started_at: status
                    .and_then(|s| s.state.as_ref())
                    .and_then(|st| st.running.as_ref())
                    .and_then(|r| r.started_at.clone()),
            }
        })
        .collect::<Vec<_>>();

    Ok(ContainerList { items: containers })
}

/// Fetch **only container names** belonging to a specific pod UID
pub async fn fetch_container_names_by_pod_uid(
    token: &str,
    client: &Client,
    pod_uid: &str,
) -> Result<Vec<String>> {
    let list = fetch_containers_by_pod_uid(token, client, pod_uid).await?;
    Ok(list.items.into_iter().map(|c| c.container_name).collect())
}

/// Fetch a single container by name and pod UID
pub async fn fetch_container_by_name_and_pod_uid(
    token: &str,
    client: &Client,
    pod_uid: &str,
    container_name: &str,
) -> Result<ContainerInfo> {
    debug!("Fetching container '{}' in pod UID '{}'", container_name, pod_uid);
    let pod = fetch_pod_by_uid(token, client, pod_uid).await?;

    let status = pod
        .status
        .as_ref()
        .and_then(|s| s.container_statuses.iter().find(|cs| cs.name == container_name));
    let spec = pod
        .spec
        .containers
        .iter()
        .find(|c| c.name == container_name)
        .ok_or_else(|| anyhow!(
            "Container '{}' not found in pod '{}'",
            container_name,
            pod.metadata.name
        ))?;

    Ok(ContainerInfo {
        container_name: spec.name.clone(),
        image: spec.image.clone(),
        pod_name: pod.metadata.name.clone(),
        namespace: pod.metadata.namespace.clone(),
        image_id: status.and_then(|s| s.image_id.clone()),
        container_id: status.and_then(|s| s.container_id.clone()),
        ready: status.and_then(|s| s.ready),
        restart_count: status.map(|s| s.restart_count),
        started_at: status
            .and_then(|s| s.state.as_ref())
            .and_then(|st| st.running.as_ref())
            .and_then(|r| r.started_at.clone()),
    })
}

/// Fetch **only the container name** (verify existence via UID and name)
pub async fn fetch_container_name_by_pod_uid(
    token: &str,
    client: &Client,
    pod_uid: &str,
    container_name: &str,
) -> Result<String> {
    let c = fetch_container_by_name_and_pod_uid(token, client, pod_uid, container_name).await?;
    Ok(c.container_name)
}

pub async fn fetch_container_by_namespace_and_pod_name(
    token: &str,
    client: &Client,
    namespace: &str,
    pod_name: &str,
    container_name: &str,
) -> Result<ContainerInfo> {
    debug!("Fetching container '{}' in pod '{}' ns '{}'", container_name, pod_name, namespace);
    let pod = fetch_pod_by_name_and_namespace(token, client, namespace, pod_name).await?;

    let status = pod
        .status
        .as_ref()
        .and_then(|s| s.container_statuses.iter().find(|cs| cs.name == container_name));
    let spec = pod
        .spec
        .containers
        .iter()
        .find(|c| c.name == container_name)
        .ok_or_else(|| anyhow!("Container '{}' not found in pod '{}'", container_name, pod_name))?;

    Ok(ContainerInfo {
        container_name: spec.name.clone(),
        image: spec.image.clone(),
        pod_name: pod.metadata.name.clone(),
        namespace: pod.metadata.namespace.clone(),
        image_id: status.and_then(|s| s.image_id.clone()),
        container_id: status.and_then(|s| s.container_id.clone()),
        ready: status.and_then(|s| s.ready),
        restart_count: status.map(|s| s.restart_count),
        started_at: status
            .and_then(|s| s.state.as_ref())
            .and_then(|st| st.running.as_ref())
            .and_then(|r| r.started_at.clone()),
    })
}
