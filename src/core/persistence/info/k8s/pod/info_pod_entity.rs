use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents static and runtime information for a Kubernetes Pod.
///
/// Derived from Pod metadata (`.metadata`, `.spec`, `.status`) and runtime summary.
/// Stored at: `data/info/pod/{pod_uid}/info.rci`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InfoPodEntity {
    // --- Identity ---
    pub pod_name: Option<String>,
    pub namespace: Option<String>,
    pub pod_uid: Option<String>,

    // --- Lifecycle ---
    pub creation_timestamp: Option<DateTime<Utc>>,
    pub start_time: Option<DateTime<Utc>>,
    pub resource_version: Option<String>,

    pub last_updated_info_at: Option<DateTime<Utc>>,
    pub deleted: Option<bool>,
    pub last_check_deleted_count: Option<u64>,

    // --- Node association ---
    pub node_name: Option<String>,
    pub host_ip: Option<String>,
    pub pod_ip: Option<String>,

    // --- Status ---
    pub qos_class: Option<String>,
    pub phase: Option<String>,
    pub ready: Option<bool>,
    pub restart_count: Option<u32>,

    // --- Owner ---
    pub owner_kind: Option<String>,
    pub owner_name: Option<String>,
    pub owner_uid: Option<String>,

    // --- Containers ---
    pub container_count: Option<u32>,
    pub container_names: Option<Vec<String>>,
    pub container_images: Option<Vec<String>>,
    pub container_ids: Option<Vec<String>>,
    pub container_started_at: Option<Vec<DateTime<Utc>>>,
    pub image_ids: Option<Vec<String>>,
    pub container_ports: Option<Vec<u16>>,
    pub restart_policy: Option<String>,
    pub scheduler_name: Option<String>,
    pub service_account: Option<String>,

    // --- Volumes ---
    pub volume_count: Option<u32>,
    pub volume_names: Option<Vec<String>>,
    pub pvc_names: Option<Vec<String>>,
    pub mount_paths: Option<Vec<String>>,
    pub termination_grace_period_seconds: Option<u32>,
    pub tolerations: Option<Vec<String>>,

    // --- Metadata ---
    pub label: Option<String>,        // flattened "key=value,..."
    pub annotation: Option<String>,   // flattened "key=value,..."

    pub team: Option<String>,
    pub service: Option<String>,
    pub env: Option<String>, // "dev", "stage", "prod"
}
