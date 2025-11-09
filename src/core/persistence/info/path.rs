use std::path::PathBuf;

use crate::core::persistence::storage_path::get_rustcost_base_path;

fn info_path<S: AsRef<str>>(sub_path: S) -> PathBuf {
    get_rustcost_base_path().join("info").join(sub_path.as_ref())
}

fn info_k8s_path<S: AsRef<str>>(sub_path: S) -> PathBuf {
    get_rustcost_base_path().join("info").join("k8s").join(sub_path.as_ref())
}

// Fixed info files
pub fn info_version_path() -> PathBuf {
    info_path("version.rci")
}

pub fn info_unit_price_path() -> PathBuf {
    info_path("unit_price.rci")
}

pub fn info_setting_path() -> PathBuf {
    info_path("settings.rci")
}

// Dynamic info: container
pub fn info_container_dir_path(container_key: &str) -> PathBuf {
    info_k8s_path(format!("container/{}", container_key))
}

pub fn info_container_file_path(container_key: &str) -> PathBuf {
    info_k8s_path(format!("container/{}/info.rci", container_key))
}

// Dynamic info: pod
pub fn info_pod_dir_path(pod_key: &str) -> PathBuf {
    info_k8s_path(format!("pod/{}", pod_key))
}

pub fn info_pod_file_path(pod_key: &str) -> PathBuf {
    info_k8s_path(format!("pod/{}/info.rci", pod_key))
}

// Dynamic info: node
pub fn info_node_dir_path(node_key: &str) -> PathBuf {
    info_k8s_path(format!("node/{}", node_key))
}

pub fn info_node_file_path(node_key: &str) -> PathBuf {
    info_k8s_path(format!("node/{}/info.rci", node_key))
}

