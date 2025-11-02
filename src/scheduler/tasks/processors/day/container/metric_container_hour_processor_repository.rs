use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::container::day::metric_container_day_processor_repository_trait::MetricContainerDayProcessorRepository;
use crate::core::persistence::metrics::container::metric_container_entity::MetricContainerEntity;
use chrono::{DateTime, Utc};
use crate::core::persistence::metrics::container::day::metric_container_day_fs_adapter::MetricContainerDayFsAdapter;

pub struct MetricContainerDayProcessorRepositoryImpl {
    pub adapter: MetricContainerDayFsAdapter,
}

impl MetricContainerDayProcessorRepository for MetricContainerDayProcessorRepositoryImpl  {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricContainerEntity> {
        &self.adapter
    }

    fn append_row_aggregated(&self, container_uid: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> anyhow::Result<()> {
        self.adapter.append_row_aggregated(container_uid, start, end)
    }
}
