use crate::core::kube_client::api_server;
use crate::scheduler::tasks::collectors::k8s::node::node_list_dto::NodeList;
use crate::scheduler::tasks::collectors::k8s::summary_dto::Summary;
use anyhow::{Result};
use reqwest::Client;
use tracing::debug;

pub async fn fetch_nodes(token: &str, client: &Client) -> Result<NodeList> {
    let nodes_url = format!("{}/api/v1/nodes", api_server());
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

pub async fn fetch_node_summary(token: &str, client: &Client, node_name: &str) -> Result<Summary> {
    let url = format!("{}/api/v1/nodes/{}/proxy/stats/summary", api_server(), node_name);
    let resp = client.get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?;

    debug!("Fetching summary for node '{}'", node_name);
    Ok(resp.json().await?)
}

// pub(crate) async fn get_latest_resource_version(token: &str, client: &Client) -> Result<String> {
//     let url = format!("{}/api/v1/nodes?limit=1", api_server());
//     let resp: Value = client
//         .get(&url)
//         .bearer_auth(token)
//         .send()
//         .await?
//         .error_for_status()?
//         .json()
//         .await?;
//
//     Ok(resp["metadata"]["resourceVersion"]
//         .as_str()
//         .unwrap_or_default()
//         .to_string())
// }