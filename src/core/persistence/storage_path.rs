//! Core path resolution utilities for persistence layer.

use std::{env, path::PathBuf};

/// Returns the base data path, using `RUSTCOST_BASE_PATH` env var if set.
/// Defaults to `data/` if not configured.
pub fn get_rustcost_base_path() -> PathBuf {
    env::var("RUSTCOST_BASE_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("data"))
}

/// Base helper for paths under `info/`.
fn info_path<S: AsRef<str>>(sub_path: S) -> PathBuf {
    get_rustcost_base_path().join("info").join(sub_path.as_ref())
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

/// Info file paths
pub fn info_version_path() -> PathBuf {
    info_path("version.rci")
}

pub fn info_unit_price_path() -> PathBuf {
    info_path("unit_price.rci")
}

pub fn info_setting_path() -> PathBuf {
    info_path("settings.rci")
}

pub fn info_container_dir_path(container_key: &str) -> PathBuf {
    info_path(format!("container/{}", container_key))
}
pub fn info_container_file_path(container_key: &str) -> PathBuf {
    info_path(format!("container/{}/info.rci", container_key))
}

pub fn info_pod_dir_path(pod_key: &str) -> PathBuf {
    info_path(format!("pod/{}", pod_key))
}
pub fn info_pod_file_path(pod_key: &str) -> PathBuf {
    info_path(format!("pod/{}/info.rci", pod_key))
}

pub fn info_node_dir_path(node_key: &str) -> PathBuf {
    info_path(format!("node/{}", node_key))
}
pub fn info_node_file_path(node_key: &str) -> PathBuf {
    info_path(format!("node/{}/info.rci", node_key))
}

/// Metric file paths
pub fn metric_path(entity: EntityType, key: &str, granularity: MetricGranularity, time_key: &str) -> PathBuf {
    get_rustcost_base_path().join("metric").join(format!(
        "{}/{}/{}/{}.rcd",
        entity.as_dir(),
        key,
        granularity.as_dir(),
        time_key
    ))
}
pub fn metric_rustcost_base_path(entity: EntityType, key: &str) -> PathBuf {
    get_rustcost_base_path().join("metric").join(format!("{}/{}", entity.as_dir(), key))
}

// Convenience wrappers for containers
pub fn metric_container_root_path() -> PathBuf {
    get_rustcost_base_path().join("metric").join( EntityType::Container.as_dir())
}
pub fn metric_container_rustcost_base_path(container_key: &str) -> PathBuf {
    metric_rustcost_base_path(EntityType::Container, container_key)
}
pub fn metric_container_day_path(container_key: &str, period_year: &str) -> PathBuf {
    metric_path(EntityType::Container, container_key, MetricGranularity::Day, period_year)
}

pub fn metric_container_day_dir(container_key: &str) -> PathBuf {
    get_rustcost_base_path()
        .join("metric")
        .join("container")
        .join(container_key)
        .join("d")
}

pub fn metric_container_hour_path(container_key: &str, period_month: &str) -> PathBuf {
    metric_path(EntityType::Container, container_key, MetricGranularity::Hour, period_month)
}

pub fn metric_container_hour_dir(container_key: &str) -> PathBuf {
    get_rustcost_base_path()
        .join("metric")
        .join("container")
        .join(container_key)
        .join("h")
}

pub fn metric_container_minute_path(container_key: &str, period_day: &str) -> PathBuf {
    metric_path(EntityType::Container, container_key, MetricGranularity::Minute, period_day)
}

pub fn metric_container_minute_dir(container_key: &str) -> PathBuf {
    get_rustcost_base_path()
        .join("metric")
        .join("container")
        .join(container_key)
        .join("m")
}

// Convenience wrappers for pods
pub fn metric_pod_root_path() -> PathBuf {
    get_rustcost_base_path().join("metric").join( EntityType::Pod.as_dir())
}
pub fn metric_pod_rustcost_base_path(pod_key: &str) -> PathBuf {
    metric_rustcost_base_path(EntityType::Pod, pod_key)
}
pub fn metric_pod_day_path(pod_key: &str, period_year: &str) -> PathBuf {
    metric_path(EntityType::Pod, pod_key, MetricGranularity::Day, period_year)
}
pub fn metric_pod_day_dir(pod_key: &str) -> PathBuf {
    get_rustcost_base_path()
        .join("metric")
        .join("pod")
        .join(pod_key)
        .join("d")
}
pub fn metric_pod_hour_path(pod_key: &str, period_month: &str) -> PathBuf {
    metric_path(EntityType::Pod, pod_key, MetricGranularity::Hour, period_month)
}
pub fn metric_pod_hour_dir(pod_key: &str) -> PathBuf {
    get_rustcost_base_path()
        .join("metric")
        .join("pod")
        .join(pod_key)
        .join("h")
}
pub fn metric_pod_minute_path(pod_key: &str, period_day: &str) -> PathBuf {
    metric_path(EntityType::Pod, pod_key, MetricGranularity::Minute, period_day)
}
pub fn metric_pod_minute_dir(pod_key: &str) -> PathBuf {
    get_rustcost_base_path()
        .join("metric")
        .join("pod")
        .join(pod_key)
        .join("m")
}
// Convenience wrappers for nodes

pub fn metric_node_root_path() -> PathBuf {
    get_rustcost_base_path().join("metric").join( EntityType::Node.as_dir())
}
pub fn metric_node_rustcost_base_path(node_key: &str) -> PathBuf {
    metric_rustcost_base_path(EntityType::Node, node_key)
}

pub fn metric_node_day_path(node_key: &str, period_year: &str) -> PathBuf {
    metric_path(EntityType::Node, node_key, MetricGranularity::Day, period_year)
}
pub fn metric_node_day_dir(pod_key: &str) -> PathBuf {
    get_rustcost_base_path()
        .join("metric")
        .join("node")
        .join(pod_key)
        .join("d")
}

pub fn metric_node_hour_path(node_key: &str, period_month: &str) -> PathBuf {
    metric_path(EntityType::Node, node_key, MetricGranularity::Hour, period_month)
}
pub fn metric_pod_node_hour_dir(pod_key: &str) -> PathBuf {
    get_rustcost_base_path()
        .join("metric")
        .join("node")
        .join(pod_key)
        .join("h")
}

pub fn metric_node_minute_path(node_key: &str, period_day: &str) -> PathBuf {
    metric_path(EntityType::Node, node_key, MetricGranularity::Minute, period_day)
}
pub fn metric_pod_node_minute_dir(pod_key: &str) -> PathBuf {
    get_rustcost_base_path()
        .join("metric")
        .join("node")
        .join(pod_key)
        .join("m")
}