use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::k8s::pod::day::metric_pod_day_processor_repository_trait::MetricPodDayProcessorRepository;
use crate::core::persistence::metrics::k8s::pod::metric_pod_entity::MetricPodEntity;
use chrono::{DateTime, Utc};
use crate::core::persistence::metrics::k8s::pod::day::metric_pod_day_fs_adapter::MetricPodDayFsAdapter;

pub struct MetricPodDayProcessorRepositoryImpl {
    pub adapter: MetricPodDayFsAdapter,
}

impl MetricPodDayProcessorRepository for MetricPodDayProcessorRepositoryImpl  {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricPodEntity> {
        &self.adapter
    }

    fn append_row_aggregated(&self, pod_uid: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> anyhow::Result<()> {
        self.adapter.append_row_aggregated(pod_uid, start, end)
    }
}
