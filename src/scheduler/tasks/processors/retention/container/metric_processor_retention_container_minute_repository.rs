use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::container::metric_container_entity::MetricContainerEntity;
use chrono::{DateTime, Utc};
use crate::core::persistence::metrics::container::minute::metric_container_minute_fs_adapter::MetricContainerMinuteFsAdapter;
use crate::core::persistence::metrics::container::minute::metric_container_minute_retention_repository_traits::MetricContainerMinuteRetentionRepository;

pub struct MetricContainerMinuteRetentionRepositoryImpl {
    pub adapter: MetricContainerMinuteFsAdapter,
}

impl MetricContainerMinuteRetentionRepository for MetricContainerMinuteRetentionRepositoryImpl  {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricContainerEntity> {
        &self.adapter
    }

    fn cleanup_old(&self, container_key: &str, before: DateTime<Utc>) -> anyhow::Result<()> {
        self.adapter.cleanup_old(container_key, before)
    }
}
