use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::k8s::node::metric_node_entity::MetricNodeEntity;
use chrono::{DateTime, Utc};
use crate::core::persistence::metrics::k8s::node::minute::metric_node_minute_fs_adapter::MetricNodeMinuteFsAdapter;
use crate::core::persistence::metrics::k8s::node::minute::metric_node_minute_retention_repository_traits::MetricNodeMinuteRetentionRepository;

pub struct MetricNodeMinuteRetentionRepositoryImpl {
    pub adapter: MetricNodeMinuteFsAdapter,
}

impl MetricNodeMinuteRetentionRepository for MetricNodeMinuteRetentionRepositoryImpl  {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricNodeEntity> {
        &self.adapter
    }

    fn cleanup_old(&self, node_key: &str, before: DateTime<Utc>) -> anyhow::Result<()> {
        self.adapter.cleanup_old(node_key, before)
    }
}
