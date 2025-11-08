use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::k8s::node::day::metric_node_day_processor_repository_trait::MetricNodeDayProcessorRepository;
use crate::core::persistence::metrics::k8s::node::metric_node_entity::MetricNodeEntity;
use chrono::{DateTime, Utc};
use crate::core::persistence::metrics::k8s::node::day::metric_node_day_fs_adapter::MetricNodeDayFsAdapter;

pub struct MetricNodeDayProcessorRepositoryImpl {
    pub adapter: MetricNodeDayFsAdapter,
}

impl MetricNodeDayProcessorRepository for MetricNodeDayProcessorRepositoryImpl  {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricNodeEntity> {
        &self.adapter
    }

    fn append_row_aggregated(&self, node_uid: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> anyhow::Result<()> {
        self.adapter.append_row_aggregated(node_uid, start, end)
    }
}
