use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::k8s::node::minute::metric_node_minute_api_repository_trait::MetricNodeMinuteApiRepository;
use crate::core::persistence::metrics::k8s::node::minute::metric_node_minute_fs_adapter::MetricNodeMinuteFsAdapter;
use crate::core::persistence::metrics::k8s::node::metric_node_entity::MetricNodeEntity;

pub struct MetricNodeMinuteApiRepositoryImpl {
    pub adapter: MetricNodeMinuteFsAdapter,
}

impl Default for MetricNodeMinuteApiRepositoryImpl {
    fn default() -> Self { Self { adapter: MetricNodeMinuteFsAdapter } }
}

impl MetricNodeMinuteApiRepository for MetricNodeMinuteApiRepositoryImpl {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricNodeEntity> { &self.adapter }
}

