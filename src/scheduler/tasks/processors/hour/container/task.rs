use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use chrono::{Duration, Timelike, Utc};

use crate::core::persistence::metrics::container::hour::{
    metric_container_hour_fs_adapter::MetricContainerHourFsAdapter,
    metric_container_hour_processor_repository_trait::MetricContainerHourProcessorRepository,
};
use crate::scheduler::tasks::processors::hour::container::metric_container_hour_processor_repository::MetricContainerHourProcessorRepositoryImpl;
use tracing::{debug, error};

/// Aggregates all containers’ minute-level metrics into hour metrics.
///
/// This scans `data/metrics/containers/{container_key}/` and calls `append_row_aggregated()`
/// for each container directory, generating an hour summary.
pub async fn process_container_minute_to_hour() -> Result<()> {
    let (start, end) = previous_hour_window()?;
    let base_dir = Path::new("data/metrics/containers/");

    if !base_dir.exists() {
        debug!("No containers directory found at {:?}", base_dir);
        return Ok(());
    }

    let container_keys = collect_container_keys(base_dir)?;
    if container_keys.is_empty() {
        debug!("No container metric directories found under {:?}", base_dir);
        return Ok(());
    }

    let repo = MetricContainerHourProcessorRepositoryImpl {
        adapter: MetricContainerHourFsAdapter,
    };

    process_all_containers(&repo, &container_keys, start, end);
    Ok(())
}

/// Returns the start and end of the previous full hour.
fn previous_hour_window() -> Result<(chrono::DateTime<Utc>, chrono::DateTime<Utc>)> {
    let now = Utc::now();
    let end = now
        .with_minute(0)
        .and_then(|d| d.with_second(0))
        .and_then(|d| d.with_nanosecond(0))
        .context("failed to round current time to hour")?;
    let start = end - Duration::hours(1);
    Ok((start, end))
}

/// Collects all container UIDs (directory names) under the given base directory.
fn collect_container_keys(base_dir: &Path) -> Result<Vec<String>> {
    let mut container_keys = Vec::new();

    for entry in fs::read_dir(base_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(container_key) = entry.file_name().to_str() {
                container_keys.push(container_key.to_string());
            }
        }
    }

    Ok(container_keys)
}

/// Aggregates minute-level data into hour data for all given containers.
fn process_all_containers<R: MetricContainerHourProcessorRepository>(
    repo: &R,
    container_keys: &[String],
    start: chrono::DateTime<Utc>,
    end: chrono::DateTime<Utc>,
) {
    for container_key in container_keys {
        match repo.append_row_aggregated(container_key, start, end) {
            Ok(_) => debug!(
                "✅ Aggregated container '{}' minute metrics from {} → {}",
                container_key, start, end
            ),
            Err(err) => error!(
                "⚠️ Failed to aggregate container '{}' metrics: {}",
                container_key, err
            ),
        }
    }
}
