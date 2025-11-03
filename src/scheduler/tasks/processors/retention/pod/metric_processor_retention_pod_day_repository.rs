use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::pod::metric_pod_entity::MetricPodEntity;
use chrono::{DateTime, Utc};
use crate::core::persistence::metrics::pod::day::metric_pod_day_fs_adapter::MetricPodDayFsAdapter;
use crate::core::persistence::metrics::pod::day::metric_pod_day_retention_repository_traits::MetricPodDayRetentionRepository;

pub struct MetricPodDayRetentionRepositoryImpl {
    pub adapter: MetricPodDayFsAdapter,
}

impl MetricPodDayRetentionRepository for MetricPodDayRetentionRepositoryImpl  {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricPodEntity> {
        &self.adapter
    }

    fn cleanup_old(&self, pod_key: &str, before: DateTime<Utc>) -> anyhow::Result<()> {
        self.adapter.cleanup_old(pod_key, before)
    }
}
