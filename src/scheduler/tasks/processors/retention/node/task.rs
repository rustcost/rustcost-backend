use std::fs;
use std::path::{PathBuf};

use anyhow::{Result};
use chrono::{Duration, Utc};
use tracing::{debug, error};

use crate::core::persistence::metrics::k8s::node::day::metric_node_day_fs_adapter::MetricNodeDayFsAdapter;
use crate::core::persistence::metrics::k8s::node::day::metric_node_day_retention_repository_traits::MetricNodeDayRetentionRepository;
use crate::core::persistence::metrics::k8s::node::hour::metric_node_hour_fs_adapter::MetricNodeHourFsAdapter;
use crate::core::persistence::metrics::k8s::node::hour::metric_node_hour_retention_repository_traits::MetricNodeHourRetentionRepository;
use crate::core::persistence::metrics::k8s::node::minute::metric_node_minute_fs_adapter::MetricNodeMinuteFsAdapter;
use crate::core::persistence::metrics::k8s::node::minute::metric_node_minute_retention_repository_traits::MetricNodeMinuteRetentionRepository;
use crate::core::persistence::storage_path::metric_node_root_path;
use crate::scheduler::tasks::processors::retention::node::metric_processor_retention_node_day_repository::MetricNodeDayRetentionRepositoryImpl;
use crate::scheduler::tasks::processors::retention::node::metric_processor_retention_node_hour_repository::MetricNodeHourRetentionRepositoryImpl;
use crate::scheduler::tasks::processors::retention::node::metric_processor_retention_node_minute_repository::MetricNodeMinuteRetentionRepositoryImpl;

/// Runs retention cleanup for all nodes across minute/hour/day metrics.
pub async fn run() -> Result<()> {
    let base_dir = metric_node_root_path();

    if !base_dir.exists() {
        debug!("No nodes directory found at {:?}", base_dir);
        return Ok(());
    }

    let node_uids = collect_node_uids(&base_dir)?;
    if node_uids.is_empty() {
        debug!("No node metric directories found under {:?}", base_dir);
        return Ok(());
    }

    // Create adapters (stateless, no constructor needed)
    let day_adapter = MetricNodeDayFsAdapter;
    let hour_adapter = MetricNodeHourFsAdapter;
    let minute_adapter = MetricNodeMinuteFsAdapter;

    // Create repositories
    let day_repo = MetricNodeDayRetentionRepositoryImpl { adapter: day_adapter };
    let hour_repo = MetricNodeHourRetentionRepositoryImpl { adapter: hour_adapter };
    let minute_repo = MetricNodeMinuteRetentionRepositoryImpl { adapter: minute_adapter };

    // Retention thresholds
    let now = Utc::now();
    let minute_before = now - Duration::days(7);
    let hour_before = now - Duration::days(30 * 3);
    let day_before = now - Duration::days(365);

    // Run cleanup for each node
    for node_uid in &node_uids {
        debug!("🧹 Running retention cleanup for node '{}'", node_uid);

        if let Err(err) = minute_repo.cleanup_old(node_uid, minute_before) {
            error!("⚠️ Minute cleanup failed for {}: {}", node_uid, err);
        }
        if let Err(err) = hour_repo.cleanup_old(node_uid, hour_before) {
            error!("⚠️ Hour cleanup failed for {}: {}", node_uid, err);
        }
        if let Err(err) = day_repo.cleanup_old(node_uid, day_before) {
            error!("⚠️ Day cleanup failed for {}: {}", node_uid, err);
        }
    }

    debug!("✅ Retention cleanup complete for all nodes");
    Ok(())
}

/// Collects all node UIDs (directory names) under the given base directory.
fn collect_node_uids(base_dir: &PathBuf) -> Result<Vec<String>> {
    let mut node_uids = Vec::new();

    for entry in fs::read_dir(base_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(node_uid) = entry.file_name().to_str() {
                node_uids.push(node_uid.to_string());
            }
        }
    }

    Ok(node_uids)
}
