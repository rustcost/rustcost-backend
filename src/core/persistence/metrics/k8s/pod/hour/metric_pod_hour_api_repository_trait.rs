use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::k8s::pod::metric_pod_entity::MetricPodEntity;
use anyhow::Result;
use chrono::{DateTime, Utc};

/// Repository trait for reading pod hour metrics (API layer).
pub trait MetricPodHourApiRepository: Send + Sync {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricPodEntity>;

    fn get_column_between(
        &self,
        column_name: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        pod_key: &str,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<MetricPodEntity>> {
        self.fs_adapter()
            .get_column_between(column_name, start, end, pod_key, limit, offset)
    }

    fn get_row_between(
        &self,
        pod_key: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<MetricPodEntity>>;
}

