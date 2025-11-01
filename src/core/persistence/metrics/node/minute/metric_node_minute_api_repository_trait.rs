use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::node::metric_node_entity::NodeMetricsEntity;
use anyhow::Result;
use chrono::{DateTime, Utc};

/// Repository trait for reading node minute metrics (API layer).
pub trait MetricNodeMinuteApiRepository: Send + Sync {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<NodeMetricsEntity>;

    fn get_column_between(
        &self,
        column_name: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        node_name: &str,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<NodeMetricsEntity>> {
        self.fs_adapter()
            .get_column_between(column_name, start, end, node_name, limit, offset)
    }

    /// Read full rows between timestamps
    fn get_row_between(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        node_name: &str,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<NodeMetricsEntity>> {
        self.fs_adapter()
            .get_row_between(start, end, node_name, limit, offset)
    }

}
