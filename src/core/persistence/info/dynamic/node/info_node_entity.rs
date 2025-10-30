use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Represents static and runtime information for a Kubernetes node.
///
/// Combines metadata (from the Node resource) and metrics (from metrics-server or API).
/// Stored at `data/info/node/{node_name}/info.rci`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InfoNodeEntity {
    // --- Identity & Metadata ---
    pub node_name: Option<String>,
    pub node_uid: Option<String>,
    pub creation_timestamp: Option<DateTime<Utc>>,
    pub resource_version: Option<String>,

    // --- Lifecycle ---
    pub last_updated_info_at: Option<DateTime<Utc>>,
    pub deleted: Option<bool>,
    pub last_check_deleted_count: Option<u64>,

    // --- Host Info ---
    pub hostname: Option<String>,
    pub internal_ip: Option<String>,
    pub architecture: Option<String>,
    pub os_image: Option<String>,
    pub kernel_version: Option<String>,
    pub kubelet_version: Option<String>,
    pub container_runtime: Option<String>,
    pub operating_system: Option<String>,

    // --- Capacity ---
    pub cpu_capacity_cores: Option<u32>,
    pub memory_capacity_bytes: Option<u64>,
    pub pod_capacity: Option<u32>,
    pub ephemeral_storage_capacity_bytes: Option<u64>,

    // --- Allocatable ---
    pub cpu_allocatable_cores: Option<u32>,
    pub memory_allocatable_bytes: Option<u64>,
    pub ephemeral_storage_allocatable_bytes: Option<u64>,
    pub pod_allocatable: Option<u32>,

    // --- Status ---
    pub ready: Option<bool>,
    pub taints: Option<String>,
    pub label: Option<String>,
    pub annotation: Option<String>,

    // --- Images ---
    pub image_count: Option<u32>,
    pub image_names: Option<Vec<String>>,
    pub image_total_size_bytes: Option<u64>,

}

