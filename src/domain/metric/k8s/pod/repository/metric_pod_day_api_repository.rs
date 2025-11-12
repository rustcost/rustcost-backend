use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::k8s::pod::day::metric_pod_day_api_repository_trait::MetricPodDayApiRepository;
use crate::core::persistence::metrics::k8s::pod::day::metric_pod_day_fs_adapter::MetricPodDayFsAdapter;
use crate::core::persistence::metrics::k8s::pod::metric_pod_entity::MetricPodEntity;
use chrono::{DateTime, Utc};

pub struct MetricPodDayApiRepositoryImpl { pub adapter: MetricPodDayFsAdapter }

impl Default for MetricPodDayApiRepositoryImpl { fn default() -> Self { Self { adapter: MetricPodDayFsAdapter } } }

impl MetricPodDayApiRepository for MetricPodDayApiRepositoryImpl {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricPodEntity> { &self.adapter }
    fn get_row_between(&self, pod_uid: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> anyhow::Result<Vec<MetricPodEntity>> {
        self.adapter.get_row_between(start, end, pod_uid, None, None)
    }
}

