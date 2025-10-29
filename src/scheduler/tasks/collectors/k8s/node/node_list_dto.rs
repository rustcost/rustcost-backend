use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeList {
    pub apiVersion: Option<String>,
    pub kind: Option<String>,
    pub items: Vec<Node>,
    pub metadata: Option<ListMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListMetadata {
    pub resourceVersion: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub apiVersion: Option<String>,
    pub kind: Option<String>,
    pub metadata: Metadata,
    pub spec: Option<NodeSpec>,
    pub status: Option<NodeStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub uid: Option<String>,
    pub creationTimestamp: Option<String>,
    pub resourceVersion: Option<String>,
    pub annotations: Option<HashMap<String, String>>,
    pub labels: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeSpec {
    pub taints: Option<Vec<NodeTaint>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeTaint {
    pub key: String,
    pub effect: String,
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeStatus {
    pub addresses: Option<Vec<NodeAddress>>,
    pub allocatable: Option<HashMap<String, String>>,
    pub capacity: Option<HashMap<String, String>>,
    pub conditions: Option<Vec<NodeCondition>>,
    pub daemonEndpoints: Option<DaemonEndpoints>,
    pub images: Option<Vec<ContainerImage>>,
    pub nodeInfo: Option<NodeSystemInfo>,
    pub volumesAttached: Option<Vec<AttachedVolume>>,
    pub volumesInUse: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeAddress {
    #[serde(rename = "type")]
    pub address_type: String,
    pub address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeCondition {
    #[serde(rename = "type")]
    pub condition_type: String,
    pub status: String,
    pub lastHeartbeatTime: Option<String>,
    pub lastTransitionTime: Option<String>,
    pub reason: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DaemonEndpoints {
    pub kubeletEndpoint: Option<KubeletEndpoint>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KubeletEndpoint {
    pub Port: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerImage {
    pub names: Vec<String>,
    pub sizeBytes: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeSystemInfo {
    pub architecture: Option<String>,
    pub bootID: Option<String>,
    pub containerRuntimeVersion: Option<String>,
    pub kernelVersion: Option<String>,
    pub kubeProxyVersion: Option<String>,
    pub kubeletVersion: Option<String>,
    pub machineID: Option<String>,
    pub operatingSystem: Option<String>,
    pub osImage: Option<String>,
    pub systemUUID: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttachedVolume {
    pub name: String,
    pub devicePath: Option<String>,
}
