use serde::Deserialize;
use crate::core::client::k8s::client_k8s_container_dto::{ContainerSpec, ContainerStatus};

/// Represents a list of Pods returned by /api/v1/pods
#[derive(Deserialize, Debug)]
pub struct PodList {
    pub items: Vec<Pod>,
}

/// A single Pod resource, combining metadata, spec, and status
#[derive(Deserialize, Debug, Clone)]
pub struct Pod {
    pub metadata: Metadata,
    pub spec: PodSpec,

    /// `status` is optional when a Pod is Pending or incomplete
    #[serde(default)]
    pub status: Option<PodStatus>,
}

/// Core metadata of a Pod
#[derive(Deserialize, Debug, Clone)]
pub struct Metadata {
    pub name: String,
    pub namespace: String,
    pub uid: String,
}

/// Declarative Pod spec (containers, nodeName, etc.)
#[derive(Deserialize, Debug, Clone)]
pub struct PodSpec {
    pub containers: Vec<ContainerSpec>,
}

/// Observed runtime status of a Pod
#[derive(Deserialize, Debug, Clone)]
pub struct PodStatus {
    #[serde(default)]
    pub container_statuses: Vec<ContainerStatus>,
}
