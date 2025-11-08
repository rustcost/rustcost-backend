use crate::core::persistence::metrics::k8s::pod::metric_pod_entity::MetricPodEntity;
use anyhow::Result;
use chrono::{DateTime, Utc};
use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;

/// Repository trait for reading pod minute metrics (API layer).
pub trait MetricPodDayRetentionRepository: Send + Sync {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricPodEntity>;

    /// Deletes old metric files for the given pod before the cutoff timestamp.
    fn cleanup_old(&self, pod_key: &str, before: DateTime<Utc>) -> Result<()> {
        self.fs_adapter().cleanup_old(pod_key, before)
    }
}
