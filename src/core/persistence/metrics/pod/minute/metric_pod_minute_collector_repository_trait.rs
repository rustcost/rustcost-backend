use anyhow::Result;
use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::pod::metric_pod_entity::MetricPodEntity;

/// Repository trait for reading pod minute metrics (API layer).
pub trait MetricPodMinuteCollectorRepository: Send + Sync {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricPodEntity>;

    /// Inserts one metric sample for a given pod.
    fn append_row(&self, pod_uid: &str, data: &MetricPodEntity) -> Result<()> {
        self.fs_adapter().append_row(pod_uid, data)
    }

}
