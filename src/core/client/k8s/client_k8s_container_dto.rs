
// --- Your custom structs for container summary results ---
#[derive(Debug, Clone)]
pub struct ContainerInfo {
    pub container_name: String,
    pub image: Option<String>,
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