use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::k8s::container::hour::metric_container_hour_processor_repository_trait::MetricContainerHourProcessorRepository;
use crate::core::persistence::metrics::k8s::container::metric_container_entity::MetricContainerEntity;
use chrono::{DateTime, Utc};
use crate::core::persistence::metrics::k8s::container::hour::metric_container_hour_fs_adapter::MetricContainerHourFsAdapter;

pub struct MetricContainerHourProcessorRepositoryImpl {
    pub adapter: MetricContainerHourFsAdapter,
}

impl MetricContainerHourProcessorRepository for MetricContainerHourProcessorRepositoryImpl  {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricContainerEntity> {
        &self.adapter
    }

    fn append_row_aggregated(&self, container_uid: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> anyhow::Result<()> {
        self.adapter.append_row_aggregated(container_uid, start, end)
    }
}
