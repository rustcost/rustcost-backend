use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// --- Container types --------------------------------------------------

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerSpec {
    pub name: String,

    #[serde(default)]
    pub image: Option<String>,

    #[serde(default)]
    pub args: Option<Vec<String>>,

    #[serde(default)]
    pub ports: Option<Vec<ContainerPort>>,

    #[serde(default)]
    pub resources: Option<ContainerResources>,

    #[serde(default)]
    pub volume_mounts: Option<Vec<VolumeMount>>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerPort {
    pub container_port: u16,

    #[serde(default)]
    pub name: Option<String>,

    #[serde(default)]
    pub protocol: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerResources {
    #[serde(default)]
    pub requests: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeMount {
    pub mount_path: String,
    pub name: String,

    #[serde(default)]
    pub read_only: Option<bool>,
}

// --- Container status --------------------------------------------------

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerStatus {
    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub image: Option<String>,

    #[serde(default)]
    pub image_id: Option<String>,

    #[serde(default)]
    pub container_id: Option<String>,

    #[serde(default)]
    pub restart_count: i32,

    #[serde(default)]
    pub ready: Option<bool>,

    #[serde(default)]
    pub started: Option<bool>,

    #[serde(default)]
    pub state: Option<ContainerState>,

    #[serde(default)]
    pub last_state: Option<ContainerState>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerState {
    #[serde(default)]
    pub running: Option<ContainerStateRunning>,

    #[serde(default)]
    pub terminated: Option<ContainerStateTerminated>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerStateRunning {
    #[serde(default, rename = "startedAt")]
    pub started_at: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerStateTerminated {
    #[serde(default, rename = "startedAt")]
    pub started_at: Option<String>,

    #[serde(default, rename = "finishedAt")]
    pub finished_at: Option<String>,

    #[serde(default)]
    pub exit_code: Option<i32>,

    #[serde(default)]
    pub reason: Option<String>,
}

// --- Pod-level structures ----------------------------------------------

#[derive(Debug, Deserialize, Clone)]
pub struct PodList {
    pub items: Vec<Pod>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Pod {
    pub metadata: Metadata,
    pub spec: PodSpec,

    #[serde(default)]
    pub status: Option<PodStatus>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Metadata {
    pub name: String,
    pub namespace: String,
    pub uid: String,

    #[serde(default)]
    pub labels: Option<HashMap<String, String>>,

    #[serde(default)]
    pub annotations: Option<HashMap<String, String>>,

    #[serde(default)]
    pub owner_references: Option<Vec<OwnerReference>>,

    #[serde(default)]
    pub creation_timestamp: Option<String>,

    #[serde(default)]
    pub resource_version: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct OwnerReference {
    pub api_version: Option<String>,
    pub kind: Option<String>,
    pub name: Option<String>,
    pub uid: Option<String>,

    #[serde(default)]
    pub controller: Option<bool>,

    #[serde(default)]
    pub block_owner_deletion: Option<bool>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PodSpec {
    #[serde(default)]
    pub containers: Vec<ContainerSpec>,

    #[serde(default)]
    pub node_name: Option<String>,

    #[serde(default)]
    pub restart_policy: Option<String>,

    #[serde(default)]
    pub scheduler_name: Option<String>,

    #[serde(default)]
    pub service_account_name: Option<String>,

    #[serde(default)]
    pub termination_grace_period_seconds: Option<u32>,

    #[serde(default)]
    pub tolerations: Option<Vec<Toleration>>,

    #[serde(default)]
    pub volumes: Option<Vec<Volume>>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Toleration {
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub operator: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub effect: Option<String>,
    #[serde(default, rename = "tolerationSeconds")]
    pub toleration_seconds: Option<u64>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Volume {
    pub name: String,

    #[serde(default)]
    pub empty_dir: Option<HashMap<String, String>>,

    #[serde(default)]
    pub persistent_volume_claim: Option<HashMap<String, String>>,

    #[serde(default)]
    pub config_map: Option<HashMap<String, String>>,

    #[serde(default)]
    pub projected: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PodStatus {
    #[serde(default)]
    pub phase: Option<String>,

    #[serde(default)]
    pub host_ip: Option<String>,

    #[serde(default)]
    pub pod_ip: Option<String>,

    #[serde(default)]
    pub start_time: Option<String>,

    #[serde(default)]
    pub qos_class: Option<String>,

    #[serde(default)]
    pub conditions: Option<Vec<PodCondition>>,

    #[serde(default)]
    pub container_statuses: Vec<ContainerStatus>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PodCondition {
    #[serde(rename = "type")]
    pub type_field: String,

    pub status: String,

    #[serde(default, rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
}
