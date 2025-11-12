use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::k8s::container::day::metric_container_day_api_repository_trait::MetricContainerDayApiRepository;
use crate::core::persistence::metrics::k8s::container::day::metric_container_day_fs_adapter::MetricContainerDayFsAdapter;
use crate::core::persistence::metrics::k8s::container::metric_container_entity::MetricContainerEntity;
use chrono::{DateTime, Utc};

pub struct MetricContainerDayApiRepositoryImpl { pub adapter: MetricContainerDayFsAdapter }

impl Default for MetricContainerDayApiRepositoryImpl { fn default() -> Self { Self { adapter: MetricContainerDayFsAdapter } } }

impl MetricContainerDayApiRepository for MetricContainerDayApiRepositoryImpl {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricContainerEntity> { &self.adapter }
    fn get_row_between(&self, container_key: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> anyhow::Result<Vec<MetricContainerEntity>> {
        self.adapter.get_row_between(start, end, container_key, None, None)
    }
}

