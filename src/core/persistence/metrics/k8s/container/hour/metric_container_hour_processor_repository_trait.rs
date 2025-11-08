use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::k8s::container::metric_container_entity::MetricContainerEntity;
use anyhow::Result;
use chrono::{DateTime, Utc};

/// Repository trait for reading container minute metrics (API layer).
pub trait MetricContainerHourProcessorRepository: Send + Sync {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricContainerEntity>;


    fn append_row_aggregated(&self, container_key: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> Result<()>;

}
