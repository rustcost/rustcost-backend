use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::k8s::container::hour::metric_container_hour_api_repository_trait::MetricContainerHourApiRepository;
use crate::core::persistence::metrics::k8s::container::hour::metric_container_hour_fs_adapter::MetricContainerHourFsAdapter;
use crate::core::persistence::metrics::k8s::container::metric_container_entity::MetricContainerEntity;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct MetricContainerHourApiRepositoryImpl { pub adapter: MetricContainerHourFsAdapter }

impl Default for MetricContainerHourApiRepositoryImpl { fn default() -> Self { Self { adapter: MetricContainerHourFsAdapter } } }

impl MetricContainerHourApiRepository for MetricContainerHourApiRepositoryImpl {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricContainerEntity> { &self.adapter }
    fn get_row_between(&self, container_key: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> anyhow::Result<Vec<MetricContainerEntity>> {
        self.adapter.get_row_between(start, end, container_key, None, None)
    }
}

