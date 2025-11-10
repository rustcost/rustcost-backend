use crate::core::persistence::metrics::k8s::node::metric_node_entity::MetricNodeEntity;
use anyhow::Result;
use chrono::{DateTime, Utc};
use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;

/// Repository trait for reading node minute metrics (API layer).
pub trait MetricNodeDayApiRepository: Send + Sync {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricNodeEntity>;
    fn get_column_between(
        &self,
        column_name: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        node_name: &str,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<MetricNodeEntity>> {
        self.fs_adapter()
            .get_column_between(column_name, start, end, node_name, limit, offset)
    }
    fn get_row_between(&self, node_key: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Vec<MetricNodeEntity>>;

}
