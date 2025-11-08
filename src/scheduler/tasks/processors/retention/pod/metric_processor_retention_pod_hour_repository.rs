use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::k8s::pod::metric_pod_entity::MetricPodEntity;
use chrono::{DateTime, Utc};
use crate::core::persistence::metrics::k8s::pod::hour::metric_pod_hour_fs_adapter::MetricPodHourFsAdapter;
use crate::core::persistence::metrics::k8s::pod::hour::metric_pod_hour_retention_repository_traits::MetricPodHourRetentionRepository;

pub struct MetricPodHourRetentionRepositoryImpl {
    pub adapter: MetricPodHourFsAdapter,
}

impl MetricPodHourRetentionRepository for MetricPodHourRetentionRepositoryImpl  {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricPodEntity> {
        &self.adapter
    }

    fn cleanup_old(&self, pod_key: &str, before: DateTime<Utc>) -> anyhow::Result<()> {
        self.adapter.cleanup_old(pod_key, before)
    }
}
