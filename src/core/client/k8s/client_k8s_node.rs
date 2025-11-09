use anyhow::Result;
use reqwest::Client;
use tracing::debug;
use crate::core::client::k8s::util::k8s_api_server;
use crate::core::client::k8s::client_k8s_node_dto::{Node, NodeList};
use crate::scheduler::tasks::collectors::k8s::summary_dto::Summary;

/// Fetch Kubernetes nodes using the in-cluster API server
pub async fn fetch_nodes(token: &str, client: &Client) -> Result<NodeList> {
    let nodes_url = format!("{}/api/v1/nodes", k8s_api_server());
    let node_list: NodeList = client
        .get(&nodes_url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    debug!("Discovered {} node(s)", node_list.items.len());
    Ok(node_list)
}

/// Fetch **only node names**
pub async fn fetch_node_names(token: &str, client: &Client) -> Result<Vec<String>> {
    let node_list = fetch_nodes(token, client).await?;
    let names = node_list
        .items
        .into_iter()
        .map(|n| n.metadata.name)
        .collect::<Vec<_>>();

    debug!("Collected {} node name(s)", names.len());
    Ok(names)
}

/// Fetch a single node by its name
pub async fn fetch_node_by_name(
    token: &str,
    client: &Client,
    node_name: &str,
) -> Result<Node> {
    let url = format!("{}/api/v1/nodes/{}", k8s_api_server(), node_name);
    debug!("Fetching node by name '{}'", node_name);

    let node: Node = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(node)
}

/// Fetch Kubelet /stats/summary for a given node
pub async fn fetch_node_summary(token: &str, client: &Client, node_name: &str) -> Result<Summary> {
    let url = format!("{}/api/v1/nodes/{}/proxy/stats/summary", k8s_api_server(), node_name);
    let resp = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?;

    debug!("Fetching summary for node '{}'", node_name);
    Ok(resp.json().await?)
}

