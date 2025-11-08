use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::k8s::container::metric_container_entity::MetricContainerEntity;
use chrono::{DateTime, Utc};
use crate::core::persistence::metrics::k8s::container::hour::metric_container_hour_fs_adapter::MetricContainerHourFsAdapter;
use crate::core::persistence::metrics::k8s::container::hour::metric_container_hour_retention_repository_traits::MetricContainerHourRetentionRepository;

pub struct MetricContainerHourRetentionRepositoryImpl {
    pub adapter: MetricContainerHourFsAdapter,
}

impl MetricContainerHourRetentionRepository for MetricContainerHourRetentionRepositoryImpl  {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricContainerEntity> {
        &self.adapter
    }

    fn cleanup_old(&self, container_key: &str, before: DateTime<Utc>) -> anyhow::Result<()> {
        self.adapter.cleanup_old(container_key, before)
    }
}
