use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodeInfoEntity {
    pub node_name: Option<String>,
    pub node_uid: Option<String>,
    pub creation_timestamp: Option<String>,
    pub resource_version: Option<String>,
    pub last_updated_info_at: Option<String>,
    pub deleted: Option<bool>,
    pub last_check_deleted_count: Option<u64>,
    pub hostname: Option<String>,
    pub internal_ip: Option<String>,
    pub architecture: Option<String>,
    pub os_image: Option<String>,
    pub kernel_version: Option<String>,
    pub kubelet_version: Option<String>,
    pub container_runtime: Option<String>,
    pub operating_system: Option<String>,
    pub cpu_capacity_cores: Option<u32>,
    pub memory_capacity_bytes: Option<u64>,
    pub pod_capacity: Option<u32>,
    pub ephemeral_storage_capacity_bytes: Option<u64>,
    pub cpu_allocatable_cores: Option<u32>,
    pub memory_allocatable_bytes: Option<u64>,
    pub ephemeral_storage_allocatable_bytes: Option<u64>,
    pub pod_allocatable: Option<u32>,
    pub ready: Option<bool>,
    pub taints: Option<String>,
    pub label: Option<String>,
    pub annotation: Option<String>,
    pub image_count: Option<u32>,
    pub image_names: Option<Vec<String>>,
    pub image_total_size_bytes: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NodeMetricsEntity {
    pub time: String,

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

    // Filesystem
    pub fs_used_bytes: Option<u64>,
    pub fs_capacity_bytes: Option<u64>,
    pub fs_inodes_used: Option<u64>,
    pub fs_inodes: Option<u64>,
}