use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::core::persistence::metrics::k8s::node::metric_node_entity::MetricNodeEntity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricNodeDto {
    pub time: DateTime<Utc>,
    pub cpu_usage_nano_cores: Option<u64>,
    pub cpu_usage_core_nano_seconds: Option<u64>,
    pub memory_usage_bytes: Option<u64>,
    pub memory_working_set_bytes: Option<u64>,
    pub memory_rss_bytes: Option<u64>,
    pub memory_page_faults: Option<u64>,
    pub network_physical_rx_bytes: Option<u64>,
    pub network_physical_tx_bytes: Option<u64>,
    pub network_physical_rx_errors: Option<u64>,
    pub network_physical_tx_errors: Option<u64>,
    pub fs_used_bytes: Option<u64>,
    pub fs_capacity_bytes: Option<u64>,
    pub fs_inodes_used: Option<u64>,
    pub fs_inodes: Option<u64>,
}

impl From<MetricNodeEntity> for MetricNodeDto {
    fn from(e: MetricNodeEntity) -> Self {
        Self {
            time: e.time,
            cpu_usage_nano_cores: e.cpu_usage_nano_cores,
            cpu_usage_core_nano_seconds: e.cpu_usage_core_nano_seconds,
            memory_usage_bytes: e.memory_usage_bytes,
            memory_working_set_bytes: e.memory_working_set_bytes,
            memory_rss_bytes: e.memory_rss_bytes,
            memory_page_faults: e.memory_page_faults,
            network_physical_rx_bytes: e.network_physical_rx_bytes,
            network_physical_tx_bytes: e.network_physical_tx_bytes,
            network_physical_rx_errors: e.network_physical_rx_errors,
            network_physical_tx_errors: e.network_physical_tx_errors,
            fs_used_bytes: e.fs_used_bytes,
            fs_capacity_bytes: e.fs_capacity_bytes,
            fs_inodes_used: e.fs_inodes_used,
            fs_inodes: e.fs_inodes,
        }
    }
}

