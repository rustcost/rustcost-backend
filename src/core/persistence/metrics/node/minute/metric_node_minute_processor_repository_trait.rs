use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::node::metric_node_entity::MetricNodeEntity;

/// Repository trait for reading node minute metrics (API layer).
pub trait MetricNodeMinuteProcessorRepository: Send + Sync {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricNodeEntity>;

}
