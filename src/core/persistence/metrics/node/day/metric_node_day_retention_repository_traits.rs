use crate::core::persistence::metrics::node::metric_node_entity::MetricNodeEntity;
use anyhow::Result;
use chrono::{DateTime, Utc};
use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;

/// Repository trait for reading node minute metrics (API layer).
pub trait MetricNodeDayRetentionRepository: Send + Sync {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricNodeEntity>;

    /// Deletes old metric files for the given node before the cutoff timestamp.
    fn cleanup_old(&self, node_key: &str, before: DateTime<Utc>) -> Result<()> {
        self.fs_adapter().cleanup_old(node_key, before)
    }


}
