use std::fs;
use std::path::{PathBuf};

use anyhow::{ Result};
use chrono::{Duration, Utc};
use tracing::{debug, error};

use crate::core::persistence::metrics::container::day::metric_container_day_fs_adapter::MetricContainerDayFsAdapter;
use crate::core::persistence::metrics::container::day::metric_container_day_retention_repository_traits::MetricContainerDayRetentionRepository;
use crate::core::persistence::metrics::container::hour::metric_container_hour_fs_adapter::MetricContainerHourFsAdapter;
use crate::core::persistence::metrics::container::hour::metric_container_hour_retention_repository_traits::MetricContainerHourRetentionRepository;
use crate::core::persistence::metrics::container::minute::metric_container_minute_fs_adapter::MetricContainerMinuteFsAdapter;
use crate::core::persistence::metrics::container::minute::metric_container_minute_retention_repository_traits::MetricContainerMinuteRetentionRepository;
use crate::core::persistence::storage_path::metric_container_root_path;
use crate::scheduler::tasks::processors::retention::container::metric_processor_retention_container_day_repository::MetricContainerDayRetentionRepositoryImpl;
use crate::scheduler::tasks::processors::retention::container::metric_processor_retention_container_hour_repository::MetricContainerHourRetentionRepositoryImpl;
use crate::scheduler::tasks::processors::retention::container::metric_processor_retention_container_minute_repository::MetricContainerMinuteRetentionRepositoryImpl;

/// Runs retention cleanup for all containers across minute/hour/day metrics.
pub async fn run() -> Result<()> {
    let base_dir = metric_container_root_path();

    if !base_dir.exists() {
        debug!("No containers directory found at {:?}", base_dir);
        return Ok(());
    }

    let container_uids = collect_container_uids(&base_dir)?;
    if container_uids.is_empty() {
        debug!("No container metric directories found under {:?}", base_dir);
        return Ok(());
    }

    // Create adapters (stateless, no constructor needed)
    let day_adapter = MetricContainerDayFsAdapter;
    let hour_adapter = MetricContainerHourFsAdapter;
    let minute_adapter = MetricContainerMinuteFsAdapter;

    // Create repositories
    let day_repo = MetricContainerDayRetentionRepositoryImpl { adapter: day_adapter };
    let hour_repo = MetricContainerHourRetentionRepositoryImpl { adapter: hour_adapter };
    let minute_repo = MetricContainerMinuteRetentionRepositoryImpl { adapter: minute_adapter };

    // Retention thresholds
    let now = Utc::now();
    let minute_before = now - Duration::days(7);
    let hour_before = now - Duration::days(30 * 3);
    let day_before = now - Duration::days(365);

    // Run cleanup for each container
    for container_uid in &container_uids {
        debug!("🧹 Running retention cleanup for container '{}'", container_uid);

        if let Err(err) = minute_repo.cleanup_old(container_uid, minute_before) {
            error!("⚠️ Minute cleanup failed for {}: {}", container_uid, err);
        }
        if let Err(err) = hour_repo.cleanup_old(container_uid, hour_before) {
            error!("⚠️ Hour cleanup failed for {}: {}", container_uid, err);
        }
        if let Err(err) = day_repo.cleanup_old(container_uid, day_before) {
            error!("⚠️ Day cleanup failed for {}: {}", container_uid, err);
        }
    }

    debug!("✅ Retention cleanup complete for all containers");
    Ok(())
}

/// Collects all container UIDs (directory names) under the given base directory.
fn collect_container_uids(base_dir: &PathBuf) -> Result<Vec<String>> {
    let mut container_uids = Vec::new();

    for entry in fs::read_dir(base_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(container_uid) = entry.file_name().to_str() {
                container_uids.push(container_uid.to_string());
            }
        }
    }

    Ok(container_uids)
}
