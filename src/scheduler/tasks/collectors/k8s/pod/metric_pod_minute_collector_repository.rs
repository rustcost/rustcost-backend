use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::pod::metric_pod_entity::MetricPodEntity;
use crate::core::persistence::metrics::pod::minute::metric_pod_minute_collector_repository_trait::MetricPodMinuteCollectorRepository;
use crate::core::persistence::metrics::pod::minute::metric_pod_minute_fs_adapter::MetricPodMinuteFsAdapter;

pub struct MetricPodMinuteCollectorRepositoryImpl {
    pub adapter: MetricPodMinuteFsAdapter,
}

impl MetricPodMinuteCollectorRepository for MetricPodMinuteCollectorRepositoryImpl {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricPodEntity> {
        &self.adapter
    }
}
