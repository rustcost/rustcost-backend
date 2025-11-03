use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::pod::metric_pod_entity::MetricPodEntity;

/// Repository trait for reading pod minute metrics (API layer).
pub trait MetricPodMinuteProcessorRepository: Send + Sync {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricPodEntity>;
}
