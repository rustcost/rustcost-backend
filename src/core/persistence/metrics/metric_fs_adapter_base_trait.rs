use anyhow::Result;
use chrono::{DateTime, Utc};

/// Unified FS adapter trait for metrics (collector, processor, and API).
/// Each implementation may only use a subset of these methods.
pub trait MetricFsAdapterBase<T>: Send + Sync {
    // === Collector-like ===
    /// Append one raw metric row (e.g. per-minute data)
    fn append_row(&self, name: &str, data: &T) -> Result<()> {
        unimplemented!("append_row not used in this adapter")
    }

    // === Processor-like ===
    /// Append aggregated metrics (e.g. hourly, daily)
    fn append_row_aggregated(&self, name: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<()> {
        unimplemented!("append_row_aggregated not used in this adapter")
    }

    /// Remove old metric files before a given timestamp
    fn cleanup_old(&self, name: &str, before: DateTime<Utc>) -> Result<()> {
        Ok(()) // default: do nothing
    }

    // === API-like ===
    /// Read a column between timestamps
    fn get_column_between(
        &self,
        column_name: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        object_name: &str,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<T>> {
        unimplemented!("get_column_between not used in this adapter")
    }

    /// Read full rows between timestamps
    fn get_row_between(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        object_name: &str,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<T>> {
        unimplemented!("get_row_between not used in this adapter")
    }
}
