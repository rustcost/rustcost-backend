use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::node::metric_node_entity::MetricNodeEntity;
use chrono::{DateTime, Utc};
use crate::core::persistence::metrics::node::day::metric_node_day_fs_adapter::MetricNodeDayFsAdapter;
use crate::core::persistence::metrics::node::day::metric_node_day_retention_repository_traits::MetricNodeDayRetentionRepository;

pub struct MetricNodeDayRetentionRepositoryImpl {
    pub adapter: MetricNodeDayFsAdapter,
}

impl MetricNodeDayRetentionRepository for MetricNodeDayRetentionRepositoryImpl  {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricNodeEntity> {
        &self.adapter
    }

    fn cleanup_old(&self, node_key: &str, before: DateTime<Utc>) -> anyhow::Result<()> {
        self.adapter.cleanup_old(node_key, before)
    }
}
