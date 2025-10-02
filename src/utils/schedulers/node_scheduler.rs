use std::fs;
use std::time::Duration;
use std::env;

use chrono::Utc;
use reqwest::{Client, Certificate};
use tokio::time::{interval};
use tracing::{debug, error, info, warn};

use crate::domain::models::node::{NewNode, NewNodeMetric};
use crate::domain::models::pod::{NewPod, NewPodMetric};
use crate::domain::models::k8s::{
    NodeMetricsList, PodMetricsList, K8sNode, NodeList, PodMetricsItem,
};

use crate::infra::repositories::node_repository::{insert_node, insert_node_metric};
use crate::infra::repositories::pod_repository::{insert_pod, insert_pod_metric};

/// --- Helpers for token and client ---
fn read_token() -> anyhow::Result<String> {
    let path = env::var("TOKEN_PATH")
        .unwrap_or("/var/run/secrets/kubernetes.io/serviceaccount/token".to_string());
    let token = fs::read_to_string(&path)
        .map_err(|e| anyhow::anyhow!("Failed to read token from {}: {}", path, e))?;
    Ok(token.trim().to_string())
}

fn build_client() -> anyhow::Result<Client> {
    let ca_path = env::var("CA_PATH")
        .unwrap_or("/var/run/secrets/kubernetes.io/serviceaccount/ca.crt".to_string());
    let pem = fs::read(&ca_path)?;
    let ca = Certificate::from_pem(&pem)?;
    Ok(Client::builder().add_root_certificate(ca).build()?)
}

fn get_api_server() -> String {
    env::var("API_SERVER").unwrap_or("https://kubernetes.default.svc".to_string())
}

/// --- API calls ---
pub async fn get_nodes(client: &Client, token: &str) -> anyhow::Result<NodeList> {
    let url = format!("{}/api/v1/nodes", get_api_server());
    let resp = client.get(&url).bearer_auth(token).send().await?;
    Ok(resp.error_for_status()?.json::<NodeList>().await?)
}

pub async fn get_node_metrics(client: &Client, token: &str) -> anyhow::Result<NodeMetricsList> {
    let url = format!("{}/apis/metrics.k8s.io/v1beta1/nodes", get_api_server());
    let resp = client.get(&url).bearer_auth(token).send().await?;
    Ok(resp.error_for_status()?.json::<NodeMetricsList>().await?)
}

pub async fn get_pod_metrics(client: &Client, token: &str) -> anyhow::Result<PodMetricsList> {
    let url = format!("{}/apis/metrics.k8s.io/v1beta1/pods", get_api_server());
    let resp = client.get(&url).bearer_auth(token).send().await?;

    let status = resp.status();

    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(anyhow::anyhow!(
            "Pod metrics fetch failed: {} - {}",
            status,
            body
        ));
    }

    Ok(resp.json::<PodMetricsList>().await?)
}



/// --- Mapping functions ---
fn node_to_new_node(node: &K8sNode) -> NewNode {
    NewNode {
        name: node.metadata.name.clone(),
        labels: node
            .metadata
            .labels
            .clone()
            .map(|m| serde_json::to_value(m).unwrap_or_default()),
    }
}

fn pod_to_new_pod(item: &PodMetricsItem) -> NewPod {
    NewPod {
        name: item.metadata.name.clone(),
        namespace: item.metadata.namespace.clone().unwrap_or_else(|| "default".to_string()),
        node_id: None,
        labels: item.metadata.labels.clone().map(|m| serde_json::to_value(m).unwrap_or_default()),
    }
}

fn node_metric_to_new(cpu: i64, mem: i64, node_id: Option<i32>) -> NewNodeMetric {
    NewNodeMetric {
        node_id,
        cpu_mcores: cpu,
        memory_bytes: mem,
        timestamp: Utc::now().naive_utc(),
    }
}

fn pod_metric_to_new(namespace: &str, cpu: i64, mem: i64, pod_id: Option<i32>) -> NewPodMetric {
    NewPodMetric {
        pod_id,
        namespace: namespace.to_string(),
        cpu_mcores: cpu,
        memory_bytes: mem,
        timestamp: Utc::now().naive_utc(),
    }
}

/// --- Collector ---
pub async fn start_collector() -> anyhow::Result<()> {
    info!("Starting K8s collector…");
    let token = read_token()?;
    let client = build_client()?;
    let mut ticker = interval(Duration::from_secs(60)); // 60s (prod: 300s)
    ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

    loop {
        ticker.tick().await;

        // Fetch all in parallel
        let (nodes_res, node_metrics_res, pod_metrics_res) = tokio::join!(
            get_nodes(&client, &token),
            get_node_metrics(&client, &token),
            get_pod_metrics(&client, &token),
        );

        // --- Nodes ---
        match nodes_res {
            Ok(node_list) => {
                info!("Processing {} nodes", node_list.items.len());
                for node in node_list.items {
                    let new_node = node_to_new_node(&node);
                    match insert_node(new_node) {
                        Ok(inserted_node) => {
                            debug!("✅ Node upserted: {}", inserted_node.name);

                            // node metrics
                            if let Ok(metrics) = &node_metrics_res {
                                for item in &metrics.items {
                                    let cpu_mcores = parse_cpu(&item.usage.cpu);
                                    let mem_bytes = parse_memory(&item.usage.memory);
                                    let new_metric =
                                        node_metric_to_new(cpu_mcores, mem_bytes, Some(inserted_node.node_id));
                                    if let Err(e) = insert_node_metric(new_metric) {
                                        error!("❌ Failed to insert node metric: {:?}", e);
                                    }
                                }
                            }
                        }
                        Err(e) => error!("❌ Failed to upsert node: {:?}", e),
                    }
                }
            }
            Err(e) => error!("❌ Error fetching nodes: {:?}", e),
        }

        // --- Pods ---
        match pod_metrics_res {
            Ok(metrics) => {
                if metrics.items.is_empty() {
                    warn!("⚠️ No pod metrics returned by metrics-server");
                } else {
                    info!("Processing {} pods", metrics.items.len());
                }

                for item in metrics.items {
                    let ns = item.metadata.namespace.clone().unwrap_or_else(|| "default".to_string());

                    let new_pod = pod_to_new_pod(&item);
                    match insert_pod(new_pod) {
                        Ok(inserted_pod) => {
                            debug!("✅ Pod upserted: {} in ns {}", inserted_pod.name, inserted_pod.namespace);

                            let mut cpu_mcores: i64 = 0;
                            let mut mem_bytes: i64 = 0;

                            for c in &item.containers {
                                cpu_mcores += parse_cpu(&c.usage.cpu);
                                mem_bytes += parse_memory(&c.usage.memory);
                            }

                            let new_metric =
                                pod_metric_to_new(&ns, cpu_mcores, mem_bytes, Some(inserted_pod.pod_id));

                            if let Err(e) = insert_pod_metric(new_metric) {
                                error!("❌ Failed to insert pod metric: {:?}", e);
                            }
                        }
                        Err(e) => error!("❌ Failed to upsert pod: {:?}", e),
                    }
                }
            }
            Err(e) => error!("❌ Error fetching pod metrics: {:?}", e),
        }
    }
}

/// --- CPU/Memory Parsers ---
fn parse_cpu(cpu_str: &str) -> i64 {
    if cpu_str.ends_with('n') {
        let nanos = cpu_str.trim_end_matches('n').parse::<i64>().unwrap_or(0);
        nanos / 1_000_000 // nanocores → millicores
    } else if cpu_str.ends_with('m') {
        cpu_str.trim_end_matches('m').parse::<i64>().unwrap_or(0)
    } else {
        cpu_str.parse::<i64>().unwrap_or(0) * 1000 // cores → millicores
    }
}

fn parse_memory(mem_str: &str) -> i64 {
    if mem_str.ends_with("Ki") {
        mem_str.trim_end_matches("Ki").parse::<i64>().unwrap_or(0) * 1024
    } else if mem_str.ends_with("Mi") {
        mem_str.trim_end_matches("Mi").parse::<i64>().unwrap_or(0) * 1024 * 1024
    } else if mem_str.ends_with("Gi") {
        mem_str.trim_end_matches("Gi").parse::<i64>().unwrap_or(0) * 1024 * 1024 * 1024
    } else {
        mem_str.parse::<i64>().unwrap_or(0)
    }
}
