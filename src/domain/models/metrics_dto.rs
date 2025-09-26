// src/k8s_dto.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Common metadata used across many K8s objects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectMeta {
    pub name: Option<String>,
    pub namespace: Option<String>,
    #[serde(default)]
    pub labels: HashMap<String, String>,
    #[serde(default)]
    pub annotations: HashMap<String, String>,
    // Add other fields you need (uid, resource_version, creationTimestamp, ...)
}

/// Generic List metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListMeta {
    pub resource_version: Option<String>,
    pub continue_token: Option<String>,
    // kube uses "continue" but Rust field can't be named `continue`.
    // If you need it, add #[serde(rename = "continue")]
}

/// --- Core resource basics (Node, Pod minimal) ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeList {
    pub api_version: Option<String>,
    pub kind: Option<String>,
    pub metadata: Option<ListMeta>,
    pub items: Vec<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub metadata: Option<ObjectMeta>,
    pub status: Option<NodeStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeStatus {
    #[serde(default)]
    pub capacity: HashMap<String, String>,
    #[serde(default)]
    pub allocatable: HashMap<String, String>,
    // addresses, conditions, etc. can be added if needed
}

/// Minimal Pod list / Pod
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodList {
    pub api_version: Option<String>,
    pub kind: Option<String>,
    pub metadata: Option<ListMeta>,
    pub items: Vec<Pod>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pod {
    pub metadata: Option<ObjectMeta>,
    pub spec: Option<PodSpec>,
    pub status: Option<PodStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodSpec {
    #[serde(default)]
    pub containers: Vec<ContainerSpec>,
    pub service_account_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerSpec {
    pub name: Option<String>,
    pub image: Option<String>,
    #[serde(default)]
    pub env: Vec<EnvVar>,
    pub resources: Option<ResourceRequirements>,
    // add ports/volumeMounts if needed
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvVar {
    pub name: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    #[serde(default)]
    pub limits: HashMap<String, String>,
    #[serde(default)]
    pub requests: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodStatus {
    pub phase: Option<String>,
    // container statuses etc.
}

/// --- Service DTO ---
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub api_version: Option<String>,
    pub kind: Option<String>,
    pub metadata: Option<ObjectMeta>,
    pub spec: Option<ServiceSpec>,
    pub status: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceSpec {
    pub selector: Option<HashMap<String, String>>,
    pub ports: Option<Vec<ServicePort>>,
    #[serde(rename = "type")]
    pub svc_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicePort {
    pub name: Option<String>,
    pub port: Option<u16>,
    pub target_port: Option<serde_json::Value>, // number or string
}

/// --- RBAC (ClusterRole, ClusterRoleBinding) minimal DTOs ---
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterRole {
    pub api_version: Option<String>,
    pub kind: Option<String>,
    pub metadata: Option<ObjectMeta>,
    pub rules: Option<Vec<PolicyRule>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRule {
    #[serde(default)]
    pub api_groups: Vec<String>,
    #[serde(default)]
    pub resources: Vec<String>,
    #[serde(default)]
    pub verbs: Vec<String>,
    // resourceNames, nonResourceURLs, etc. omitted but can be added
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterRoleBinding {
    pub api_version: Option<String>,
    pub kind: Option<String>,
    pub metadata: Option<ObjectMeta>,
    pub role_ref: Option<RoleRef>,
    pub subjects: Option<Vec<Subject>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleRef {
    pub api_group: Option<String>,
    pub kind: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subject {
    pub kind: Option<String>,
    pub name: Option<String>,
    pub namespace: Option<String>,
}

/// --- metrics.k8s.io DTOs ---
/// Node metrics list and single node metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeMetricsList {
    pub api_version: Option<String>,
    pub kind: Option<String>,
    pub items: Vec<NodeMetrics>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeMetrics {
    pub metadata: Option<ObjectMeta>,
    // timestamp/window usually present
    pub timestamp: Option<String>,
    pub window: Option<String>,
    /// usage: cpu, memory, ... — values typically "123m" or "456Ki"
    #[serde(default)]
    pub usage: HashMap<String, String>,
}

/// Pod metrics list and pod metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodMetricsList {
    pub api_version: Option<String>,
    pub kind: Option<String>,
    pub items: Vec<PodMetrics>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodMetrics {
    pub metadata: Option<ObjectMeta>,
    pub timestamp: Option<String>,
    pub window: Option<String>,
    pub containers: Vec<ContainerMetrics>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerMetrics {
    pub name: Option<String>,
    #[serde(default)]
    pub usage: HashMap<String, String>,
}
