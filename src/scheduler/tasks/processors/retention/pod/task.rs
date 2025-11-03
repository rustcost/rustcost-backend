use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{ Result};
use chrono::{Duration, Utc};
use tracing::{debug, error};

use crate::core::persistence::metrics::pod::day::metric_pod_day_fs_adapter::MetricPodDayFsAdapter;
use crate::core::persistence::metrics::pod::day::metric_pod_day_retention_repository_traits::MetricPodDayRetentionRepository;
use crate::core::persistence::metrics::pod::hour::metric_pod_hour_fs_adapter::MetricPodHourFsAdapter;
use crate::core::persistence::metrics::pod::hour::metric_pod_hour_retention_repository_traits::MetricPodHourRetentionRepository;
use crate::core::persistence::metrics::pod::minute::metric_pod_minute_fs_adapter::MetricPodMinuteFsAdapter;
use crate::core::persistence::metrics::pod::minute::metric_pod_minute_retention_repository_traits::MetricPodMinuteRetentionRepository;
use crate::core::persistence::storage_path::{metric_pod_base_path, metric_pod_minute_path, metric_pod_root_path};
use crate::scheduler::tasks::processors::retention::pod::metric_processor_retention_pod_day_repository::MetricPodDayRetentionRepositoryImpl;
use crate::scheduler::tasks::processors::retention::pod::metric_processor_retention_pod_hour_repository::MetricPodHourRetentionRepositoryImpl;
use crate::scheduler::tasks::processors::retention::pod::metric_processor_retention_pod_minute_repository::MetricPodMinuteRetentionRepositoryImpl;

/// Runs retention cleanup for all pods across minute/hour/day metrics.
pub async fn run() -> Result<()> {

    let base_dir = metric_pod_root_path();

    if !base_dir.exists() {
        debug!("No pods directory found at {:?}", base_dir);
        return Ok(());
    }

    let pod_uids = collect_pod_uids(base_dir)?;
    if pod_uids.is_empty() {
        debug!("No pod metric directories found under {:?}", base_dir);
        return Ok(());
    }

    // Create adapters (stateless, no constructor needed)
    let day_adapter = MetricPodDayFsAdapter;
    let hour_adapter = MetricPodHourFsAdapter;
    let minute_adapter = MetricPodMinuteFsAdapter;

    // Create repositories
    let day_repo = MetricPodDayRetentionRepositoryImpl { adapter: day_adapter };
    let hour_repo = MetricPodHourRetentionRepositoryImpl { adapter: hour_adapter };
    let minute_repo = MetricPodMinuteRetentionRepositoryImpl { adapter: minute_adapter };

    // Retention thresholds
    let now = Utc::now();
    let minute_before = now - Duration::days(7);
    let hour_before = now - Duration::days(30 * 3);
    let day_before = now - Duration::days(365);

    // Run cleanup for each pod
    for pod_uid in &pod_uids {
        debug!("🧹 Running retention cleanup for pod '{}'", pod_uid);

        if let Err(err) = minute_repo.cleanup_old(pod_uid, minute_before) {
            error!("⚠️ Minute cleanup failed for {}: {}", pod_uid, err);
        }
        if let Err(err) = hour_repo.cleanup_old(pod_uid, hour_before) {
            error!("⚠️ Hour cleanup failed for {}: {}", pod_uid, err);
        }
        if let Err(err) = day_repo.cleanup_old(pod_uid, day_before) {
            error!("⚠️ Day cleanup failed for {}: {}", pod_uid, err);
        }
    }

    debug!("✅ Retention cleanup complete for all pods");
    Ok(())
}

/// Collects all pod UIDs (directory names) under the given base directory.
fn collect_pod_uids(base_dir: &PathBuf) -> Result<Vec<String>> {
    let mut pod_uids = Vec::new();

    for entry in fs::read_dir(base_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(pod_uid) = entry.file_name().to_str() {
                pod_uids.push(pod_uid.to_string());
            }
        }
    }

    Ok(pod_uids)
}
