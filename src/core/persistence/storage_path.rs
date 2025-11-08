//! Core path resolution utilities for persistence layer.

use std::{env, path::PathBuf};

/// Returns the base data path, using `RUSTCOST_BASE_PATH` env var if set.
/// Defaults to `data/` if not configured.
pub fn get_rustcost_base_path() -> PathBuf {
    env::var("RUSTCOST_BASE_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("data"))
}

/// Type of entity used in persistence paths.
#[derive(Debug, Clone, Copy)]
pub enum EntityType {
    Container,
    Pod,
    Node,
}

impl EntityType {
    fn as_dir(&self) -> &'static str {
        match self {
            EntityType::Container => "container",
            EntityType::Pod => "pod",
            EntityType::Node => "node",
        }
    }
}

/// Metric granularity (time resolution).
#[derive(Debug, Clone, Copy)]
pub enum MetricGranularity {
    Day,   // yyyy
    Hour,  // yyyy-mm
    Minute // yyyy-mm-dd
}

impl MetricGranularity {
    fn as_dir(&self) -> &'static str {
        match self {
            MetricGranularity::Day => "d",
            MetricGranularity::Hour => "h",
            MetricGranularity::Minute => "m",
        }
    }
}

// Re-export info path builders from the new module
pub use crate::core::persistence::info::path::{
    info_container_dir_path,
    info_container_file_path,
    info_node_dir_path,
    info_node_file_path,
    info_pod_dir_path,
    info_pod_file_path,
    info_setting_path,
    info_unit_price_path,
    info_version_path,
};

// Backward-compatible aliases for existing call-sites
pub use crate::core::persistence::metrics::k8s::path as k8s_path_aliases;
pub use k8s_path_aliases::metric_k8s_container_dir_path as metric_container_root_path;
pub use k8s_path_aliases::metric_k8s_container_key_dir_path as metric_container_rustcost_base_path;
pub use k8s_path_aliases::metric_k8s_container_key_day_file_path as metric_container_day_path;
pub use k8s_path_aliases::metric_k8s_container_key_day_dir_path as metric_container_day_dir;
pub use k8s_path_aliases::metric_k8s_container_key_hour_file_path as metric_container_hour_path;
pub use k8s_path_aliases::metric_k8s_container_key_hour_dir_path as metric_container_hour_dir;
pub use k8s_path_aliases::metric_k8s_container_key_minute_file_path as metric_container_minute_path;
pub use k8s_path_aliases::metric_k8s_container_key_minute_dir_path as metric_container_minute_dir;

pub use k8s_path_aliases::metric_k8s_pod_dir_path as metric_pod_root_path;
pub use k8s_path_aliases::metric_k8s_pod_key_dir_path as metric_pod_rustcost_base_path;
pub use k8s_path_aliases::metric_k8s_pod_key_day_file_path as metric_pod_day_path;
pub use k8s_path_aliases::metric_k8s_pod_key_day_dir_path as metric_pod_day_dir;
pub use k8s_path_aliases::metric_k8s_pod_key_hour_file_path as metric_pod_hour_path;
pub use k8s_path_aliases::metric_k8s_pod_key_hour_dir_path as metric_pod_hour_dir;
pub use k8s_path_aliases::metric_k8s_pod_key_minute_file_path as metric_pod_minute_path;
pub use k8s_path_aliases::metric_k8s_pod_key_minute_dir_path as metric_pod_minute_dir;

pub use k8s_path_aliases::metric_k8s_node_dir_path as metric_node_root_path;
pub use k8s_path_aliases::metric_k8s_node_key_dir_path as metric_node_rustcost_base_path;
pub use k8s_path_aliases::metric_k8s_node_key_day_file_path as metric_node_day_path;
pub use k8s_path_aliases::metric_k8s_node_key_day_dir_path as metric_node_day_dir;
pub use k8s_path_aliases::metric_k8s_node_key_hour_file_path as metric_node_hour_path;
pub use k8s_path_aliases::metric_k8s_node_key_hour_dir_path as metric_node_hour_dir;
pub use k8s_path_aliases::metric_k8s_node_key_minute_file_path as metric_node_minute_path;
pub use k8s_path_aliases::metric_k8s_node_key_minute_dir_path as metric_node_minute_dir;
