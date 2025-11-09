/* Builds API client (token, cert, base URL) */

use reqwest::{Certificate, Client};
use std::{env, fs};
use crate::core::persistence::storage_path::get_rustcost_base_path;
use std::path::Path;
use anyhow::Result;
use tracing::debug;
use crate::scheduler::tasks::collectors::k8s::node::node_list_dto::NodeList;
use crate::scheduler::tasks::collectors::k8s::summary_dto::Summary;

/// Reads the service account token (mounted in pod)
pub fn read_token() -> anyhow::Result<String> {
    let path = env::var("RUSTCOST_TOKEN_PATH")
        .unwrap_or_else(|_| "/var/run/secrets/kubernetes.io/serviceaccount/token".to_string());
    let token = fs::read_to_string(&path)?;
    Ok(token.trim().to_string())
}

/// Builds a reqwest client with CA cert for in-cluster HTTPS
pub fn build_client() -> anyhow::Result<Client> {
    // default path for in-cluster service account
    let default_ca = "/var/run/secrets/kubernetes.io/serviceaccount/ca.crt".to_string();

    let rustcost_ca_path = env::var("RUSTCOST_CA_PATH").unwrap_or(default_ca.clone());
    let pem = fs::read(&rustcost_ca_path)?;
    let ca = Certificate::from_pem(&pem)?;

    // Determine if we're running locally (Windows / dev)
    let ca_path = get_rustcost_base_path().join("ca.crt");
    let is_local = Path::new(&rustcost_ca_path) == ca_path;

    let builder = Client::builder().add_root_certificate(ca);

    // In dev mode, allow CN mismatch for 127.0.0.1 tunnels
    let client = if is_local {
        builder
            .danger_accept_invalid_certs(false)
            .danger_accept_invalid_hostnames(true)
            .build()?
    } else {
        builder.build()?
    };

    Ok(client)
}

/// Returns API server URL (in-cluster or overridden)
pub fn k8s_api_server() -> String {
    env::var("RUSTCOST_K8S_API_URL").unwrap_or_else(|_| "https://kubernetes.default.svc".to_string())
}

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
