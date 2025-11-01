use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MetricPodEntity {
    pub time: DateTime<Utc>,

    // CPU
    pub cpu_usage_nano_cores: Option<u64>,
    pub cpu_usage_core_nano_seconds: Option<u64>,

    // Memory
    pub memory_usage_bytes: Option<u64>,
    pub memory_working_set_bytes: Option<u64>,
    pub memory_rss_bytes: Option<u64>,
    pub memory_page_faults: Option<u64>,

    // Network (physical)
    pub network_physical_rx_bytes: Option<u64>,
    pub network_physical_tx_bytes: Option<u64>,
    pub network_physical_rx_errors: Option<u64>,
    pub network_physical_tx_errors: Option<u64>,

    // ephemeral storage usage
    pub es_used_bytes: Option<u64>,
    pub es_capacity_bytes: Option<u64>,
    pub es_inodes_used: Option<u64>,
    pub es_inodes: Option<u64>,

    // persistent storage usage
    pub pv_used_bytes: Option<u64>,
    pub pv_capacity_bytes: Option<u64>,
    pub pv_inodes_used: Option<u64>,
    pub pv_inodes: Option<u64>,
}