use anyhow::Result;
use chrono::{DateTime, Utc};

/// Unified FS adapter trait for metrics (collector, processor, and API).
/// Each implementation may only use a subset of these methods.
pub trait MetricFsAdapterBase<T>: Send + Sync {
    // === Collector-like ===
    /// Append one raw metric row (e.g. per-minute data)
    #[allow(unused_variables)]
    fn append_row(&self, name: &str, data: &T) -> Result<()> {
        unimplemented!("append_row not used in this adapter")
    }

    // === Processor-like ===
    /// Append aggregated metrics (e.g. hour, day)
    #[allow(unused_variables)]
    fn append_row_aggregated(&self, pod_uid: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<()> {
        unimplemented!("append_row_aggregated not used in this adapter")
    }

    /// Remove old metric files before a given timestamp
    #[allow(unused_variables)]
    fn cleanup_old(&self, name: &str, before: DateTime<Utc>) -> Result<()> {
        unimplemented!("cleanup_old not used in this adapter")
    }

    // === API-like ===
    /// Read a column between timestamps
    #[allow(unused_variables)]
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
    #[allow(unused_variables)]
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
