use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MetricContainerEntity {
    pub time: DateTime<Utc>,

    // CPU
    pub cpu_usage_nano_cores: Option<u64>,
    pub cpu_usage_core_nano_seconds: Option<u64>,

    // Memory
    pub memory_usage_bytes: Option<u64>,
    pub memory_working_set_bytes: Option<u64>,
    pub memory_rss_bytes: Option<u64>,
    pub memory_page_faults: Option<u64>,

    // Ephemeral filesystem (rootfs + logs)
    pub fs_used_bytes: Option<u64>,
    pub fs_capacity_bytes: Option<u64>,
    pub fs_inodes_used: Option<u64>,
    pub fs_inodes: Option<u64>,

    // Swap (optional)
    // pub swap_used_bytes: Option<u64>,
    // pub swap_available_bytes: Option<u64>,
}
