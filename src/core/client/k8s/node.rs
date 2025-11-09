use anyhow::Result;
use reqwest::Client;
use tracing::debug;

use crate::core::client::k8s::util::k8s_api_server;
use crate::scheduler::tasks::collectors::k8s::node::node_list_dto::NodeList;
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

