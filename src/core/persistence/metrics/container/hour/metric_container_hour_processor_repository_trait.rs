use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::container::metric_container_entity::MetricContainerEntity;
use anyhow::Result;
use chrono::{DateTime, Utc};

/// Repository trait for reading container minute metrics (API layer).
pub trait MetricContainerHourProcessorRepository: Send + Sync {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricContainerEntity>;

    /// Deletes old metric files for the given container before the cutoff timestamp.
    fn cleanup_old(&self, container_key: &str, before: DateTime<Utc>) -> Result<()> {
        self.fs_adapter().cleanup_old(container_key, before)
    }

    fn append_row_aggregated(&self, container_key: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<()>;

}
