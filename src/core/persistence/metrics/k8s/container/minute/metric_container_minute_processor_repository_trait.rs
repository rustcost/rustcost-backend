use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::k8s::container::metric_container_entity::MetricContainerEntity;

/// Repository trait for reading container minute metrics (API layer).
pub trait MetricContainerMinuteProcessorRepository: Send + Sync {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricContainerEntity>;

}
