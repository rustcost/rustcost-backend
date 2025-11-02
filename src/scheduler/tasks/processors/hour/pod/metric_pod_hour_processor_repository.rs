use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::pod::hour::metric_pod_hour_processor_repository_trait::MetricPodHourProcessorRepository;
use crate::core::persistence::metrics::pod::metric_pod_entity::MetricPodEntity;
use chrono::{DateTime, Utc};
use crate::core::persistence::metrics::pod::hour::metric_pod_hour_fs_adapter::MetricPodHourFsAdapter;

pub struct MetricPodHourProcessorRepositoryImpl {
    pub adapter: MetricPodHourFsAdapter,
}

impl MetricPodHourProcessorRepository for MetricPodHourProcessorRepositoryImpl  {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricPodEntity> {
        &self.adapter
    }

    fn append_row_aggregated(&self, pod_uid: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> anyhow::Result<()> {
        self.adapter.append_row_aggregated(pod_uid, start, end)
    }
}
