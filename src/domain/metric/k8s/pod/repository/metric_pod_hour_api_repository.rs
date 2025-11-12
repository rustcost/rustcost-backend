use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::k8s::pod::hour::metric_pod_hour_api_repository_trait::MetricPodHourApiRepository;
use crate::core::persistence::metrics::k8s::pod::hour::metric_pod_hour_fs_adapter::MetricPodHourFsAdapter;
use crate::core::persistence::metrics::k8s::pod::metric_pod_entity::MetricPodEntity;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct MetricPodHourApiRepositoryImpl { pub adapter: MetricPodHourFsAdapter }

impl Default for MetricPodHourApiRepositoryImpl { fn default() -> Self { Self { adapter: MetricPodHourFsAdapter } } }

impl MetricPodHourApiRepository for MetricPodHourApiRepositoryImpl {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricPodEntity> { &self.adapter }
    fn get_row_between(&self, pod_uid: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> anyhow::Result<Vec<MetricPodEntity>> {
        self.adapter.get_row_between(start, end, pod_uid, None, None)
    }
}

