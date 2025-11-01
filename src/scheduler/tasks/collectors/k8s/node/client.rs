use std::process::Command;
use reqwest::Client;
use crate::core::kube_client::api_server;
use anyhow::{anyhow, Context, Result};
use crate::scheduler::tasks::collectors::k8s::summary_dto::Summary;
use tracing::{debug, error, info};
use crate::scheduler::tasks::collectors::k8s::node::node_list_dto::NodeList;
use serde_json::Value;

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

/// Fetches full node JSON (only called when changes detected)
pub async fn fetch_nodes_json() -> anyhow::Result<String> {
    let output = Command::new("kubectl")
        .args(["get", "nodes", "-o", "json"])
        .output()
        .context("Failed to execute kubectl get nodes -o json")?;

    if !output.status.success() {
        return Err(anyhow!(
            "kubectl command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}
pub(crate) async fn get_latest_resource_version(token: &str, client: &Client) -> Result<String> {
    let url = format!("{}/api/v1/nodes?limit=1", api_server());
    let resp: Value = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(resp["metadata"]["resourceVersion"]
        .as_str()
        .unwrap_or_default()
        .to_string())
}