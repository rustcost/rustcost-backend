use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::core::persistence::metrics::k8s::pod::metric_pod_entity::MetricPodEntity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricPodDto {
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
    pub es_used_bytes: Option<u64>,
    pub es_capacity_bytes: Option<u64>,
    pub es_inodes_used: Option<u64>,
    pub es_inodes: Option<u64>,
    pub pv_used_bytes: Option<u64>,
    pub pv_capacity_bytes: Option<u64>,
    pub pv_inodes_used: Option<u64>,
    pub pv_inodes: Option<u64>,
}

impl From<MetricPodEntity> for MetricPodDto {
    fn from(e: MetricPodEntity) -> Self {
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
            es_used_bytes: e.es_used_bytes,
            es_capacity_bytes: e.es_capacity_bytes,
            es_inodes_used: e.es_inodes_used,
            es_inodes: e.es_inodes,
            pv_used_bytes: e.pv_used_bytes,
            pv_capacity_bytes: e.pv_capacity_bytes,
            pv_inodes_used: e.pv_inodes_used,
            pv_inodes: e.pv_inodes,
        }
    }
}

