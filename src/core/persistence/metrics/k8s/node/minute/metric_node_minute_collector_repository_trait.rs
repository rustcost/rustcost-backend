use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::k8s::node::metric_node_entity::MetricNodeEntity;
use anyhow::Result;

/// Repository trait for reading node minute metrics (API layer).
pub trait MetricNodeMinuteCollectorRepository: Send + Sync {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricNodeEntity>;

    /// Inserts one metric sample for a given node.
    fn append_row(&self, node_name: &str, data: &MetricNodeEntity) -> Result<()> {
        self.fs_adapter().append_row(node_name, data)
    }

}
