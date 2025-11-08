use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::k8s::container::metric_container_entity::MetricContainerEntity;
use chrono::{DateTime, Utc};
use crate::core::persistence::metrics::k8s::container::day::metric_container_day_fs_adapter::MetricContainerDayFsAdapter;
use crate::core::persistence::metrics::k8s::container::day::metric_container_day_retention_repository_traits::MetricContainerDayRetentionRepository;

pub struct MetricContainerDayRetentionRepositoryImpl {
    pub adapter: MetricContainerDayFsAdapter,
}

impl MetricContainerDayRetentionRepository for MetricContainerDayRetentionRepositoryImpl  {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricContainerEntity> {
        &self.adapter
    }

    fn cleanup_old(&self, container_key: &str, before: DateTime<Utc>) -> anyhow::Result<()> {
        self.adapter.cleanup_old(container_key, before)
    }
}
