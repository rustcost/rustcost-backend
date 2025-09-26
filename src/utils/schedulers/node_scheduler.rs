use std::fs;
use std::time::Duration;
use chrono::Utc;
use reqwest::{Client, Certificate};
use tokio::time::interval;
use std::env;
use tracing::debug;
use tracing::error;
use tracing::info;

use crate::domain::models::metrics_dto::{NodeList, NodeMetricsList, PodMetricsList, Node};
use crate::domain::models::node::NewNodeEntity;
use crate::infra::db::connection::establish_connection;
use crate::infra::repositories::node_repository;

/// Path to mounted SA token and CA cert in Kubernetes


/// Load the Bearer token from the service account file
fn read_token() -> anyhow::Result<String> {
    let path = env::var("TOKEN_PATH")
        .unwrap_or("/var/run/secrets/kubernetes.io/serviceaccount/token".to_string());
    debug!("Current working directory: {:?}", env::current_dir()?);
    debug!("Attempting to read token from: {}", path);
    let token = fs::read_to_string(&path)
        .map_err(|e| anyhow::anyhow!("Failed to read token from {}: {}", path, e))?;
    debug!("Token read successfully");
    Ok(token.trim().to_string())
}

fn build_client() -> anyhow::Result<Client> {
    let ca_path = env::var("CA_PATH")
        .unwrap_or("/var/run/secrets/kubernetes.io/serviceaccount/ca.crt".to_string());
    debug!("Attempting to read CA certificate from: {}", ca_path);
    let pem = fs::read(&ca_path)
        .map_err(|e| anyhow::anyhow!("Failed to read CA certificate from {}: {}", ca_path, e))?;
    let ca = Certificate::from_pem(&pem)
        .map_err(|e| anyhow::anyhow!("Failed to parse CA certificate: {}", e))?;
    let client = Client::builder()
        .add_root_certificate(ca)
        .build()
        .map_err(|e| anyhow::anyhow!("Failed to build client: {}", e))?;
    debug!("Client built successfully");
    Ok(client)
}

fn get_api_server() -> String {
    env::var("API_SERVER")
        .unwrap_or("https://kubernetes.default.svc".to_string())
}
pub async fn get_nodes(client: &Client, token: &str) -> anyhow::Result<NodeList> {
    let url = format!("{}/api/v1/nodes", get_api_server());
    debug!("Fetching nodes from: {}", url);
    let resp = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to send request to {}: {}", url, e))?
        .error_for_status()
        .map_err(|e| anyhow::anyhow!("Request to {} failed with status: {}", url, e))?
        .json::<NodeList>()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to parse nodes response: {}", e))?;
    debug!("Successfully fetched nodes");
    Ok(resp)
}

/// Method 2: get Node metrics (metrics.k8s.io/v1beta1)
pub async fn get_node_metrics(client: &Client, token: &str) -> anyhow::Result<NodeMetricsList> {
    let url = format!("{}/apis/metrics.k8s.io/v1beta1/nodes", get_api_server());
    debug!("Fetching node metrics from: {}", url);
    let resp = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to send request to {}: {}", url, e))?
        .error_for_status()
        .map_err(|e| anyhow::anyhow!("Request to {} failed with status: {}", url, e))?
        .json::<NodeMetricsList>()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to parse node metrics response: {}", e))?;
    debug!("Successfully fetched node metrics");
    Ok(resp)
}

/// Method 3: get Pod metrics in namespace rustcost
pub async fn get_pod_metrics(client: &Client, token: &str) -> anyhow::Result<PodMetricsList> {
    let url = format!("{}/apis/metrics.k8s.io/v1beta1/namespaces/rustcost/pods", get_api_server());
    debug!("Fetching pod metrics from: {}", url);
    let resp = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to send request to {}: {}", url, e))?
        .error_for_status()
        .map_err(|e| anyhow::anyhow!("Request to {} failed with status: {}", url, e))?
        .json::<PodMetricsList>()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to parse pod metrics response: {}", e))?;
    debug!("Successfully fetched pod metrics");
    Ok(resp)
}

/// Collector task – runs every 5 minutes, inserts node data into repo

pub async fn start_node_collector() -> anyhow::Result<()> {
    info!("Starting node collector...");
    let token = read_token().map_err(|e| {
        error!("Failed to read token: {:?}", e);
        e
    })?;
    info!("Token read successfully");
    let client = build_client()?;
    info!("Client built successfully");
    let mut ticker = tokio::time::interval(Duration::from_secs(300));

    loop {
        info!("Hello, {}!", "HI");
        ticker.tick().await;

        match get_nodes(&client, &token).await {
            Ok(node_list) => {
                for node in node_list.items {
                    let new_node = node_to_new_node_entity(node);
                    let result = tokio::task::spawn_blocking({
                        let new_node = new_node.clone();
                        move || {
                            let mut conn = establish_connection();
                            node_repository::insert(&mut conn, &new_node)
                        }
                    })
                        .await
                        .map_err(|e| anyhow::anyhow!(e))?;

                    if let Err(e) = result {
                        error!("Error inserting node into repo: {:?}", e);
                    }
                }
            }
            Err(e) => error!("Error fetching nodes: {:?}", e),
        }

        if let Ok(metrics) = get_node_metrics(&client, &token).await {
            info!("Got {} node metrics entries", metrics.items.len());
        }
        if let Ok(metrics) = get_pod_metrics(&client, &token).await {
            info!("Got {} pod metrics entries", metrics.items.len());
        }
    }
}

fn node_to_new_node_entity(node: Node) -> NewNodeEntity {
    NewNodeEntity {
        name: node
            .metadata
            .as_ref()
            .and_then(|m| m.name.clone())
            .unwrap_or_default(),

        cpu_capacity: node
            .status
            .as_ref()
            .and_then(|s| s.capacity.get("cpu").cloned()),

        memory_capacity: node
            .status
            .as_ref()
            .and_then(|s| s.capacity.get("memory").cloned()),

        kubelet_version: None, // NodeStatus has no kubelet info in your DTO
        os_image: None,        // NodeStatus has no os_image info
        architecture: None,    // NodeStatus has no architecture info

        created_at: Utc::now().naive_utc(),
    }
}