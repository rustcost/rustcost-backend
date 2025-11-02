use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::pod::metric_pod_entity::MetricPodEntity;
use anyhow::Result;
use chrono::{DateTime, Utc};

/// Repository trait for reading pod minute metrics (API layer).
pub trait MetricPodHourProcessorRepository: Send + Sync {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricPodEntity>;

    /// Deletes old metric files for the given pod before the cutoff timestamp.
    fn cleanup_old(&self, pod_uid: &str, before: DateTime<Utc>) -> Result<()> {
        self.fs_adapter().cleanup_old(pod_uid, before)
    }

    fn append_row_aggregated(&self, pod_uid: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<()>;

}
