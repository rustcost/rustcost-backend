use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::k8s::container::metric_container_entity::MetricContainerEntity;
use crate::core::persistence::metrics::k8s::container::minute::metric_container_minute_collector_repository_trait::MetricContainerMinuteCollectorRepository;
use crate::core::persistence::metrics::k8s::container::minute::metric_container_minute_fs_adapter::MetricContainerMinuteFsAdapter;

pub struct MetricContainerMinuteCollectorRepositoryImpl {
    pub adapter: MetricContainerMinuteFsAdapter,
}

impl MetricContainerMinuteCollectorRepository for MetricContainerMinuteCollectorRepositoryImpl {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricContainerEntity> {
        &self.adapter
    }
}
