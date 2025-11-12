use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::core::persistence::metrics::k8s::container::metric_container_entity::MetricContainerEntity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricContainerDto {
    pub time: DateTime<Utc>,
    pub cpu_usage_nano_cores: Option<u64>,
    pub cpu_usage_core_nano_seconds: Option<u64>,
    pub memory_usage_bytes: Option<u64>,
    pub memory_working_set_bytes: Option<u64>,
    pub memory_rss_bytes: Option<u64>,
    pub memory_page_faults: Option<u64>,
    pub fs_used_bytes: Option<u64>,
    pub fs_capacity_bytes: Option<u64>,
    pub fs_inodes_used: Option<u64>,
    pub fs_inodes: Option<u64>,
}

impl From<MetricContainerEntity> for MetricContainerDto {
    fn from(e: MetricContainerEntity) -> Self {
        Self {
            time: e.time,
            cpu_usage_nano_cores: e.cpu_usage_nano_cores,
            cpu_usage_core_nano_seconds: e.cpu_usage_core_nano_seconds,
            memory_usage_bytes: e.memory_usage_bytes,
            memory_working_set_bytes: e.memory_working_set_bytes,
            memory_rss_bytes: e.memory_rss_bytes,
            memory_page_faults: e.memory_page_faults,
            fs_used_bytes: e.fs_used_bytes,
            fs_capacity_bytes: e.fs_capacity_bytes,
            fs_inodes_used: e.fs_inodes_used,
            fs_inodes: e.fs_inodes,
        }
    }
}

