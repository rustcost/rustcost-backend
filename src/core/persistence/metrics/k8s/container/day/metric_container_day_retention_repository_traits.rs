use crate::core::persistence::metrics::k8s::container::metric_container_entity::MetricContainerEntity;
use anyhow::Result;
use chrono::{DateTime, Utc};
use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;

/// Deletes old metric files for the given container before the cutoff timestamp.
pub trait MetricContainerDayRetentionRepository: Send + Sync {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricContainerEntity>;


    fn cleanup_old(&self, container_key: &str, before: DateTime<Utc>) -> Result<()> {
        self.fs_adapter().cleanup_old(container_key, before)
    }

}
