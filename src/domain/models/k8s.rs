use serde::Deserialize;
use crate::domain::models::node::NewNodeMetric;
use crate::domain::models::pod::NewPodMetric;

use chrono::Utc;

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub namespace: Option<String>,
    pub labels: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct Quantity {
    #[serde(flatten)]
    pub values: std::collections::HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub cpu: String,
    pub memory: String,
}

#[derive(Debug, Deserialize)]
pub struct NodeMetricsItem {
    pub metadata: Metadata,
    pub usage: Usage,
}

#[derive(Debug, Deserialize)]
pub struct NodeMetricsList {
    pub items: Vec<NodeMetricsItem>,
}

#[derive(Debug, Deserialize)]
pub struct PodMetricsItem {
    pub metadata: Metadata,
    pub usage: Usage,
}

#[derive(Debug, Deserialize)]
pub struct PodMetricsList {
    pub items: Vec<PodMetricsItem>,
}

#[derive(Debug, Deserialize)]
pub struct NodeList {
    pub items: Vec<K8sNode>,
}

#[derive(Debug, Deserialize)]
pub struct K8sNode {
    pub metadata: Metadata,
    pub status: Option<NodeStatus>,
}

#[derive(Debug, Deserialize)]
pub struct K8sPod {
    pub metadata: Metadata,
    // spec, status etc…
}


#[derive(Debug, Deserialize)]
pub struct NodeStatus {
    pub capacity: Option<std::collections::HashMap<String, String>>,
    pub allocatable: Option<std::collections::HashMap<String, String>>,
}

fn node_metrics_to_new(item: &NodeMetricsItem, node_id: Option<i32>) -> NewNodeMetric {
    NewNodeMetric {
        node_id,
        cpu_mcores: parse_cpu(&item.usage.cpu),
        memory_bytes: parse_memory(&item.usage.memory),
        timestamp: Utc::now().naive_utc(),
    }
}

fn pod_metrics_to_new(item: &PodMetricsItem, pod_id: Option<i32>) -> NewPodMetric {
    NewPodMetric {
        pod_id,
        namespace: item.metadata.namespace.clone().unwrap_or("default".to_string()),
        cpu_mcores: parse_cpu(&item.usage.cpu),
        memory_bytes: parse_memory(&item.usage.memory),
        timestamp: Utc::now().naive_utc(),
    }
}

/// --- CPU/Memory Parsers (simplified) ---
fn parse_cpu(cpu_str: &str) -> i64 {
    if cpu_str.ends_with("m") {
        cpu_str.trim_end_matches("m").parse::<i64>().unwrap_or(0)
    } else {
        cpu_str.parse::<i64>().unwrap_or(0) * 1000
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
