use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::k8s::node::metric_node_entity::MetricNodeEntity;
use chrono::{DateTime, Utc};
use crate::core::persistence::metrics::k8s::node::hour::metric_node_hour_fs_adapter::MetricNodeHourFsAdapter;
use crate::core::persistence::metrics::k8s::node::hour::metric_node_hour_retention_repository_traits::MetricNodeHourRetentionRepository;

pub struct MetricNodeHourRetentionRepositoryImpl {
    pub adapter: MetricNodeHourFsAdapter,
}

impl MetricNodeHourRetentionRepository for MetricNodeHourRetentionRepositoryImpl  {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricNodeEntity> {
        &self.adapter
    }

    fn cleanup_old(&self, node_key: &str, before: DateTime<Utc>) -> anyhow::Result<()> {
        self.adapter.cleanup_old(node_key, before)
    }
}
