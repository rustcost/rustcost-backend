use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use chrono::{Duration, Timelike, Utc};

use crate::core::persistence::metrics::k8s::pod::hour::{
    metric_pod_hour_fs_adapter::MetricPodHourFsAdapter,
    metric_pod_hour_processor_repository_trait::MetricPodHourProcessorRepository,
};
use crate::scheduler::tasks::processors::hour::pod::metric_pod_hour_processor_repository::MetricPodHourProcessorRepositoryImpl;
use tracing::{debug, error};
use crate::core::persistence::metrics::k8s::path::metric_k8s_pod_dir_path;

/// Aggregates all pods’ minute-level metrics into hour metrics.
///
/// This scans `data/metric/pod/{pod_uid}/` and calls `append_row_aggregated()`
/// for each pod directory, generating an hour summary.
pub async fn process_pod_minute_to_hour() -> Result<()> {
    let (start, end) = previous_hour_window()?;
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

    let repo = MetricPodHourProcessorRepositoryImpl {
        adapter: MetricPodHourFsAdapter,
    };

    process_all_pods(&repo, &pod_uids, start, end);
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

/// Collects all pod UIDs (directory names) under the given base directory.
fn collect_pod_uids(base_dir: &Path) -> Result<Vec<String>> {
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

/// Aggregates minute-level data into hour data for all given pods.
fn process_all_pods<R: MetricPodHourProcessorRepository>(
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

