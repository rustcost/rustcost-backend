use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use chrono::{Duration, Timelike, Utc};

use crate::core::persistence::metrics::node::hour::{
    metric_node_hour_fs_adapter::MetricNodeHourFsAdapter,
    metric_node_hour_processor_repository_trait::MetricNodeHourProcessorRepository,
};
use crate::scheduler::tasks::processors::hourly::node::metric_node_hour_processor_repository::MetricNodeHourProcessorRepositoryImpl;
use tracing::{debug, error};

/// Aggregates all nodes’ minute-level metrics into hourly metrics.
///
/// This scans `data/metrics/nodes/{node_name}/` and calls `append_row_aggregated()`
/// for each node directory, generating an hourly summary.
pub async fn process_node_minute_to_hour() -> Result<()> {
    let (start, end) = previous_hour_window()?;
    let base_dir = Path::new("data/metrics/nodes/");

    if !base_dir.exists() {
        debug!("No nodes directory found at {:?}", base_dir);
        return Ok(());
    }

    let node_names = collect_node_names(base_dir)?;
    if node_names.is_empty() {
        debug!("No node metric directories found under {:?}", base_dir);
        return Ok(());
    }

    let repo = MetricNodeHourProcessorRepositoryImpl {
        adapter: MetricNodeHourFsAdapter,
    };

    process_all_nodes(&repo, &node_names, start, end);
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

/// Collects all node UIDs (directory names) under the given base directory.
fn collect_node_names(base_dir: &Path) -> Result<Vec<String>> {
    let mut node_names = Vec::new();

    for entry in fs::read_dir(base_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(node_name) = entry.file_name().to_str() {
                node_names.push(node_name.to_string());
            }
        }
    }

    Ok(node_names)
}

/// Aggregates minute-level data into hourly data for all given nodes.
fn process_all_nodes<R: MetricNodeHourProcessorRepository>(
    repo: &R,
    node_names: &[String],
    start: chrono::DateTime<Utc>,
    end: chrono::DateTime<Utc>,
) {
    for node_name in node_names {
        match repo.append_row_aggregated(node_name, start, end) {
            Ok(_) => debug!(
                "✅ Aggregated node '{}' minute metrics from {} → {}",
                node_name, start, end
            ),
            Err(err) => error!(
                "⚠️ Failed to aggregate node '{}' metrics: {}",
                node_name, err
            ),
        }
    }
}
