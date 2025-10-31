use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeList {
    pub api_version: Option<String>,
    pub kind: Option<String>,
    pub items: Vec<Node>,
    pub metadata: Option<ListMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListMetadata {
    pub resource_version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub api_version: Option<String>,
    pub kind: Option<String>,
    pub metadata: Metadata,
    pub spec: Option<NodeSpec>,
    pub status: Option<NodeStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub name: String,
    pub uid: Option<String>,
    pub creation_timestamp: Option<String>,
    pub resource_version: Option<String>,
    pub annotations: Option<HashMap<String, String>>,
    pub labels: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeSpec {
    pub taints: Option<Vec<NodeTaint>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeTaint {
    pub key: String,
    pub effect: String,
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeStatus {
    pub addresses: Option<Vec<NodeAddress>>,
    pub allocatable: Option<HashMap<String, String>>,
    pub capacity: Option<HashMap<String, String>>,
    pub conditions: Option<Vec<NodeCondition>>,
    pub daemon_endpoints: Option<DaemonEndpoints>,
    pub images: Option<Vec<ContainerImage>>,
    pub node_info: Option<NodeSystemInfo>,
    pub volumes_attached: Option<Vec<AttachedVolume>>,
    pub volumes_in_use: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeAddress {
    #[serde(rename = "type")]
    pub address_type: String,
    pub address: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeCondition {
    #[serde(rename = "type")]
    pub condition_type: String,
    pub status: String,
    pub last_heartbeat_time: Option<String>,
    pub last_transition_time: Option<String>,
    pub reason: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DaemonEndpoints {
    pub kubelet_endpoint: Option<KubeletEndpoint>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KubeletEndpoint {
    pub port: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerImage {
    pub names: Vec<String>,
    pub size_bytes: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeSystemInfo {
    pub architecture: Option<String>,
    pub boot_id: Option<String>,
    pub container_runtime_version: Option<String>,
    pub kernel_version: Option<String>,
    pub kube_proxy_version: Option<String>,
    pub kubelet_version: Option<String>,
    pub machine_id: Option<String>,
    pub operating_system: Option<String>,
    pub os_image: Option<String>,
    pub system_uuid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachedVolume {
    pub name: String,
    pub device_path: Option<String>,
}
