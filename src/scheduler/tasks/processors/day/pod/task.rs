use std::fs;
use std::path::{PathBuf};

use anyhow::{Context, Result};
use chrono::{Duration, Timelike, Utc};

use crate::core::persistence::metrics::k8s::pod::day::{
    metric_pod_day_fs_adapter::MetricPodDayFsAdapter,
    metric_pod_day_processor_repository_trait::MetricPodDayProcessorRepository,
};
use tracing::{debug, error};
use crate::core::persistence::metrics::k8s::path::metric_k8s_pod_dir_path;
use crate::scheduler::tasks::processors::day::pod::metric_pod_day_processor_repository::MetricPodDayProcessorRepositoryImpl;

/// Aggregates all pods’ minute-level metrics into dayly metrics.
///
/// This scans `data/metric/pod/{pod_uid}/` and calls `append_row_aggregated()`
/// for each pod directory, generating an dayly summary.
pub async fn process_pod_hour_to_day() -> Result<()> {
    let (start, end) = previous_day_window()?;
    let base_dir = metric_k8s_pod_dir_path();

    if !base_dir.exists() {
        debug!("No pods directory found at {:?}", base_dir);
        return Ok(());
    }

    let pod_uids = collect_pod_uids(&base_dir)?;
    if pod_uids.is_empty() {
        debug!("No pod metric directories found under {:?}", base_dir);
        return Ok(());
    }

    let repo = MetricPodDayProcessorRepositoryImpl {
        adapter: MetricPodDayFsAdapter,
    };

    process_all_pods(&repo, &pod_uids, start, end);
    Ok(())
}

/// Returns the start and end of the previous full day.
fn previous_day_window() -> Result<(chrono::DateTime<Utc>, chrono::DateTime<Utc>)> {
    let now = Utc::now();
    let end = now
        .with_minute(0)
        .and_then(|d| d.with_second(0))
        .and_then(|d| d.with_nanosecond(0))
        .context("failed to round current time to day")?;
    let start = end - Duration::days(1);
    Ok((start, end))
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

/// Aggregates minute-level data into dayly data for all given pods.
fn process_all_pods<R: MetricPodDayProcessorRepository>(
    repo: &R,
    pod_uids: &[String],
    start: chrono::DateTime<Utc>,
    end: chrono::DateTime<Utc>,
) {
    for pod_uid in pod_uids {
        match repo.append_row_aggregated(pod_uid, start, end) {
            Ok(_) => debug!(
                "✅ Aggregated pod '{}' minute metrics from {} → {}",
                pod_uid, start, end
            ),
            Err(err) => error!(
                "⚠️ Failed to aggregate pod '{}' metrics: {}",
                pod_uid, err
            ),
        }
    }
}

