use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::k8s::container::minute::metric_container_minute_api_repository_trait::MetricContainerMinuteApiRepository;
use crate::core::persistence::metrics::k8s::container::minute::metric_container_minute_fs_adapter::MetricContainerMinuteFsAdapter;
use crate::core::persistence::metrics::k8s::container::metric_container_entity::MetricContainerEntity;

pub struct MetricContainerMinuteApiRepositoryImpl { pub adapter: MetricContainerMinuteFsAdapter }

impl Default for MetricContainerMinuteApiRepositoryImpl { fn default() -> Self { Self { adapter: MetricContainerMinuteFsAdapter } } }

impl MetricContainerMinuteApiRepository for MetricContainerMinuteApiRepositoryImpl {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricContainerEntity> { &self.adapter }
}

