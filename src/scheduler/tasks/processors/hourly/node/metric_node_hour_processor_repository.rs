use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::node::hour::metric_node_hour_processor_repository_trait::MetricNodeHourProcessorRepository;
use crate::core::persistence::metrics::node::metric_node_entity::MetricNodeEntity;
use chrono::{DateTime, Utc};
use crate::core::persistence::metrics::node::hour::metric_node_hour_fs_adapter::MetricNodeHourFsAdapter;

pub struct MetricNodeHourProcessorRepositoryImpl {
    pub adapter: MetricNodeHourFsAdapter,
}

impl MetricNodeHourProcessorRepository for MetricNodeHourProcessorRepositoryImpl  {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricNodeEntity> {
        &self.adapter
    }

    fn append_row_aggregated(&self, node_uid: &str, start: DateTime<Utc>, end: DateTime<Utc>) -> anyhow::Result<()> {
        self.adapter.append_row_aggregated(node_uid, start, end)
    }
}
