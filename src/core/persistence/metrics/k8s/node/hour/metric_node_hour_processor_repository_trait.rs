use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::k8s::node::metric_node_entity::MetricNodeEntity;
use anyhow::Result;
use chrono::{DateTime, Utc};

/// Repository trait for reading node minute metrics (API layer).
pub trait MetricNodeHourProcessorRepository: Send + Sync {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricNodeEntity>;

    fn append_row_aggregated(&self, node_name: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<()>;

}
