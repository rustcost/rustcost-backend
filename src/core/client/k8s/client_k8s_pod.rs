use anyhow::{anyhow, Result};
use reqwest::Client;
use tracing::debug;
use crate::core::client::k8s::client_k8s_pod_dto::{PodList, Pod};
use crate::core::client::k8s::util::k8s_api_server;

/// Fetch all pods in the cluster
pub async fn fetch_pods(token: &str, client: &Client) -> Result<PodList> {
    let url = format!("{}/api/v1/pods", k8s_api_server());
    debug!("Fetching all pods from '{}'", url);

    let pods: PodList = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    debug!("Discovered {} pod(s)", pods.items.len());
    Ok(pods)
}

/// Fetch **only pod names** (all namespaces)
pub async fn fetch_pod_names(token: &str, client: &Client) -> Result<Vec<String>> {
    let pods = fetch_pods(token, client).await?;
    let names = pods
        .items
        .into_iter()
        .map(|p| p.metadata.name)
        .collect::<Vec<_>>();
    Ok(names)
}

/// Fetch pods filtered by label selector (e.g. "app=myservice")
pub async fn fetch_pods_by_label(
    token: &str,
    client: &Client,
    label_selector: &str,
) -> Result<PodList> {
    let url = format!(
        "{}/api/v1/pods?labelSelector={}",
        k8s_api_server(),
        label_selector
    );
    debug!("Fetching pods with labelSelector='{}'", label_selector);

    let pods: PodList = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(pods)
}

/// Fetch **only pod names** matching label selector
pub async fn fetch_pod_names_by_label(
    token: &str,
    client: &Client,
    label_selector: &str,
) -> Result<Vec<String>> {
    let pods = fetch_pods_by_label(token, client, label_selector).await?;
    Ok(pods.items.into_iter().map(|p| p.metadata.name).collect())
}

/// Fetch pods scheduled on a given node
pub async fn fetch_pods_by_node(token: &str, client: &Client, node_name: &str) -> Result<PodList> {
    let url = format!(
        "{}/api/v1/pods?fieldSelector=spec.nodeName={}",
        k8s_api_server(),
        node_name
    );
    debug!("Fetching pods on node '{}'", node_name);

    let pods: PodList = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(pods)
}

/// Fetch **only pod names** scheduled on a given node
pub async fn fetch_pod_names_by_node(
    token: &str,
    client: &Client,
    node_name: &str,
) -> Result<Vec<String>> {
    let pods = fetch_pods_by_node(token, client, node_name).await?;
    Ok(pods.items.into_iter().map(|p| p.metadata.name).collect())
}

/// Fetch pods within a specific namespace
pub async fn fetch_pods_by_namespace(
    token: &str,
    client: &Client,
    namespace: &str,
) -> Result<PodList> {
    let url = format!("{}/api/v1/namespaces/{}/pods", k8s_api_server(), namespace);
    debug!("Fetching pods in namespace '{}'", namespace);

    let pods: PodList = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(pods)
}

/// Fetch **only pod names** within a specific namespace
pub async fn fetch_pod_names_by_namespace(
    token: &str,
    client: &Client,
    namespace: &str,
) -> Result<Vec<String>> {
    let pods = fetch_pods_by_namespace(token, client, namespace).await?;
    Ok(pods.items.into_iter().map(|p| p.metadata.name).collect())
}

/// Fetch a single pod by its unique UID
pub async fn fetch_pod_by_uid(token: &str, client: &Client, pod_uid: &str) -> Result<Pod> {
    let url = format!(
        "{}/api/v1/pods?fieldSelector=metadata.uid={}",
        k8s_api_server(),
        pod_uid
    );
    debug!("Fetching pod with UID '{}'", pod_uid);

    let list: PodList = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    list.items
        .into_iter()
        .next()
        .ok_or_else(|| anyhow!("Pod with UID '{}' not found", pod_uid))
}

/// Fetch **only the pod name** by UID
pub async fn fetch_pod_name_by_uid(
    token: &str,
    client: &Client,
    pod_uid: &str,
) -> Result<String> {
    let pod = fetch_pod_by_uid(token, client, pod_uid).await?;
    Ok(pod.metadata.name)
}
