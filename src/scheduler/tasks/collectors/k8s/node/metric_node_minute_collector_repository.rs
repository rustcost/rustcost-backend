use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::node::metric_node_entity::NodeMetricsEntity;
use crate::core::persistence::metrics::node::minute::metric_node_minute_collector_repository_trait::MetricNodeMinuteCollectorRepository;
use crate::core::persistence::metrics::node::minute::metric_node_minute_fs_adapter::MetricNodeMinuteFsAdapter;

pub struct MetricNodeMinuteCollectorRepositoryImpl {
    pub adapter: MetricNodeMinuteFsAdapter,
}

impl MetricNodeMinuteCollectorRepository for MetricNodeMinuteCollectorRepositoryImpl {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<NodeMetricsEntity> {
        &self.adapter
    }
}
