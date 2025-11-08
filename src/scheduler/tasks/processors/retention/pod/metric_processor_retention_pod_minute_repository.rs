use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::k8s::pod::metric_pod_entity::MetricPodEntity;
use chrono::{DateTime, Utc};
use crate::core::persistence::metrics::k8s::pod::minute::metric_pod_minute_fs_adapter::MetricPodMinuteFsAdapter;
use crate::core::persistence::metrics::k8s::pod::minute::metric_pod_minute_retention_repository_traits::MetricPodMinuteRetentionRepository;

pub struct MetricPodMinuteRetentionRepositoryImpl {
    pub adapter: MetricPodMinuteFsAdapter,
}

impl MetricPodMinuteRetentionRepository for MetricPodMinuteRetentionRepositoryImpl  {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricPodEntity> {
        &self.adapter
    }

    fn cleanup_old(&self, pod_key: &str, before: DateTime<Utc>) -> anyhow::Result<()> {
        self.adapter.cleanup_old(pod_key, before)
    }
}
