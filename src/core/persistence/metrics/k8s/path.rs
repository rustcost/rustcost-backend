use std::path::PathBuf;

use crate::core::persistence::storage_path::get_rustcost_base_path;

fn k8s_root() -> PathBuf {
    get_rustcost_base_path().join("metric").join("k8s")
}

// --- Node ---
pub fn metric_k8s_node_dir_path() -> PathBuf {
    k8s_root().join("node")
}

pub fn metric_k8s_node_key_dir_path(key: &str) -> PathBuf {
    metric_k8s_node_dir_path().join(key)
}

pub fn metric_k8s_node_key_day_dir_path(key: &str) -> PathBuf {
    metric_k8s_node_key_dir_path(key).join("d")
}

pub fn metric_k8s_node_key_hour_dir_path(key: &str) -> PathBuf {
    metric_k8s_node_key_dir_path(key).join("h")
}

pub fn metric_k8s_node_key_minute_dir_path(key: &str) -> PathBuf {
    metric_k8s_node_key_dir_path(key).join("m")
}

pub fn metric_k8s_node_key_day_file_path(key: &str, yyyy: &str) -> PathBuf {
    metric_k8s_node_key_day_dir_path(key).join(format!("{}.rcd", yyyy))
}

pub fn metric_k8s_node_key_hour_file_path(key: &str, yyyy_mm: &str) -> PathBuf {
    metric_k8s_node_key_hour_dir_path(key).join(format!("{}.rcd", yyyy_mm))
}

pub fn metric_k8s_node_key_minute_file_path(key: &str, yyyy_mm_dd: &str) -> PathBuf {
    metric_k8s_node_key_minute_dir_path(key).join(format!("{}.rcd", yyyy_mm_dd))
}

// --- Pod ---
pub fn metric_k8s_pod_dir_path() -> PathBuf {
    k8s_root().join("pod")
}

pub fn metric_k8s_pod_key_dir_path(key: &str) -> PathBuf {
    metric_k8s_pod_dir_path().join(key)
}

pub fn metric_k8s_pod_key_day_dir_path(key: &str) -> PathBuf {
    metric_k8s_pod_key_dir_path(key).join("d")
}

pub fn metric_k8s_pod_key_hour_dir_path(key: &str) -> PathBuf {
    metric_k8s_pod_key_dir_path(key).join("h")
}

pub fn metric_k8s_pod_key_minute_dir_path(key: &str) -> PathBuf {
    metric_k8s_pod_key_dir_path(key).join("m")
}

pub fn metric_k8s_pod_key_day_file_path(key: &str, yyyy: &str) -> PathBuf {
    metric_k8s_pod_key_day_dir_path(key).join(format!("{}.rcd", yyyy))
}

pub fn metric_k8s_pod_key_hour_file_path(key: &str, yyyy_mm: &str) -> PathBuf {
    metric_k8s_pod_key_hour_dir_path(key).join(format!("{}.rcd", yyyy_mm))
}

pub fn metric_k8s_pod_key_minute_file_path(key: &str, yyyy_mm_dd: &str) -> PathBuf {
    metric_k8s_pod_key_minute_dir_path(key).join(format!("{}.rcd", yyyy_mm_dd))
}

// --- Container ---
pub fn metric_k8s_container_dir_path() -> PathBuf {
    k8s_root().join("container")
}

pub fn metric_k8s_container_key_dir_path(key: &str) -> PathBuf {
    metric_k8s_container_dir_path().join(key)
}

pub fn metric_k8s_container_key_day_dir_path(key: &str) -> PathBuf {
    metric_k8s_container_key_dir_path(key).join("d")
}

pub fn metric_k8s_container_key_hour_dir_path(key: &str) -> PathBuf {
    metric_k8s_container_key_dir_path(key).join("h")
}

pub fn metric_k8s_container_key_minute_dir_path(key: &str) -> PathBuf {
    metric_k8s_container_key_dir_path(key).join("m")
}

pub fn metric_k8s_container_key_day_file_path(key: &str, yyyy: &str) -> PathBuf {
    metric_k8s_container_key_day_dir_path(key).join(format!("{}.rcd", yyyy))
}

pub fn metric_k8s_container_key_hour_file_path(key: &str, yyyy_mm: &str) -> PathBuf {
    metric_k8s_container_key_hour_dir_path(key).join(format!("{}.rcd", yyyy_mm))
}

pub fn metric_k8s_container_key_minute_file_path(key: &str, yyyy_mm_dd: &str) -> PathBuf {
    metric_k8s_container_key_minute_dir_path(key).join(format!("{}.rcd", yyyy_mm_dd))
}

