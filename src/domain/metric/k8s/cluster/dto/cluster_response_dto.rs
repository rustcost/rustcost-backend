use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::core::persistence::info::k8s::node::info_node_entity::InfoNodeEntity;
use crate::domain::metric::k8s::node::dto::metric_node_dto::MetricNodeDto;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterMetricsResponseDto {
    pub cluster: String,
    pub nodes: Vec<String>,
    pub node_metrics: Vec<MetricNodeDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClusterMetricDto {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub total_nodes: usize,
    pub total_cpu_usage_nano_cores: u64,
    pub total_memory_usage_bytes: u64,
    pub total_fs_used_bytes: u64,
    pub total_network_rx_bytes: u64,
    pub total_network_tx_bytes: u64,
    pub nodes: Vec<NodeMetricDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodeMetricDto {
    pub node_name: String,
    pub info: InfoNodeEntity,
    pub avg_cpu_usage_nano_cores: Option<f64>,
    pub avg_memory_usage_bytes: Option<f64>,
    pub avg_fs_used_bytes: Option<f64>,
    pub total_network_rx_bytes: Option<u64>,
    pub total_network_tx_bytes: Option<u64>,
}