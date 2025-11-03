use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents static and runtime information for a Kubernetes **Container**.
///
/// Derived from Pod/Container metadata and Kubelet `/stats/summary`.
/// Stored at: `data/info/container/{pod_uid}-{container_name}/info.rci`
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InfoContainerEntity {
    // --- Identity ---
    /// UID of the parent Pod
    pub pod_uid: Option<String>,
    /// Container name (unique within the Pod)
    pub container_name: Option<String>,
    /// Namespace of the Pod
    pub namespace: Option<String>,

    // --- Lifecycle ---
    /// When the container spec was first created
    pub creation_timestamp: Option<DateTime<Utc>>,
    /// When the container actually started
    pub start_time: Option<DateTime<Utc>>,
    /// Container runtime ID (e.g. "docker://...", "containerd://...")
    pub container_id: Option<String>,
    /// Image name used
    pub image: Option<String>,
    /// Image ID hash (from runtime)
    pub image_id: Option<String>,

    // --- Status ---
    /// Current container state: "Running", "Waiting", "Terminated"
    pub state: Option<String>,
    /// Reason if Waiting/Terminated
    pub reason: Option<String>,
    /// Message if Waiting/Terminated
    pub message: Option<String>,
    /// Exit code if Terminated
    pub exit_code: Option<i32>,
    /// Last restart count
    pub restart_count: Option<u32>,
    /// Whether container is currently ready
    pub ready: Option<bool>,

    // --- Node association ---
    pub node_name: Option<String>,
    pub host_ip: Option<String>,
    pub pod_ip: Option<String>,

    // --- Resources ---
    /// Requested CPU (cores)
    pub cpu_request_millicores: Option<u64>,
    /// Requested Memory (bytes)
    pub memory_request_bytes: Option<u64>,
    /// CPU limit (cores)
    pub cpu_limit_millicores: Option<u64>,
    /// Memory limit (bytes)
    pub memory_limit_bytes: Option<u64>,

    // --- Volumes and mounts ---
    pub volume_mounts: Option<Vec<String>>,
    pub volume_devices: Option<Vec<String>>,

    // --- Metadata ---
    pub labels: Option<String>,       // "key=value,..."
    pub annotations: Option<String>,  // "key=value,..."

    // --- Bookkeeping ---
    pub last_updated_info_at: Option<DateTime<Utc>>,
    pub deleted: Option<bool>,
    pub last_check_deleted_count: Option<u64>,
}
