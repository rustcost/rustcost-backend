use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::k8s::container::metric_container_entity::MetricContainerEntity;
use anyhow::Result;
use chrono::{DateTime, Utc};

/// Repository trait for reading container day metrics (API layer).
pub trait MetricContainerDayApiRepository: Send + Sync {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricContainerEntity>;

    fn get_column_between(
        &self,
        column_name: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        container_key: &str,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<MetricContainerEntity>> {
        self.fs_adapter()
            .get_column_between(column_name, start, end, container_key, limit, offset)
    }

    fn get_row_between(
        &self,
        container_key: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<MetricContainerEntity>>;
}

