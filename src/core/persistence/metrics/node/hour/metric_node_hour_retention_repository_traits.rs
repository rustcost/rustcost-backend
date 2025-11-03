use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::node::metric_node_entity::MetricNodeEntity;
use anyhow::Result;
use chrono::{DateTime, Utc};

/// Repository trait for reading node minute metrics (API layer).
pub trait MetricNodeHourRetentionRepository: Send + Sync {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricNodeEntity>;

    /// Deletes old metric files for the given node before the cutoff timestamp.
    fn cleanup_old(&self, node_name: &str, before: DateTime<Utc>) -> Result<()> {
        self.fs_adapter().cleanup_old(node_name, before)
    }


}
