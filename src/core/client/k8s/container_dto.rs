use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ContainerSpec {
    pub name: String,
    pub image: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ContainerState {
    pub running: Option<ContainerRunning>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ContainerRunning {
    pub started_at: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ContainerStatus {
    pub name: String,
    pub ready: bool,
    pub restart_count: i32,
    pub image: String,
    pub image_id: Option<String>,
    pub container_id: Option<String>,
    pub state: Option<ContainerState>,
}

#[derive(Debug, Clone)]
pub struct ContainerInfo {
    pub container_name: String,
    pub image: String,
    pub pod_name: String,
    pub namespace: String,
    pub image_id: Option<String>,
    pub container_id: Option<String>,
    pub ready: Option<bool>,
    pub restart_count: Option<i32>,
    pub started_at: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ContainerList {
    pub items: Vec<ContainerInfo>,
}
