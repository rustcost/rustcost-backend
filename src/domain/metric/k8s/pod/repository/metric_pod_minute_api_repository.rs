use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::k8s::pod::minute::metric_pod_minute_api_repository_trait::MetricPodMinuteApiRepository;
use crate::core::persistence::metrics::k8s::pod::minute::metric_pod_minute_fs_adapter::MetricPodMinuteFsAdapter;
use crate::core::persistence::metrics::k8s::pod::metric_pod_entity::MetricPodEntity;

#[derive(Debug)]
pub struct MetricPodMinuteApiRepositoryImpl { pub adapter: MetricPodMinuteFsAdapter }

impl Default for MetricPodMinuteApiRepositoryImpl { fn default() -> Self { Self { adapter: MetricPodMinuteFsAdapter } } }

impl MetricPodMinuteApiRepository for MetricPodMinuteApiRepositoryImpl {
    fn fs_adapter(&self) -> &dyn MetricFsAdapterBase<MetricPodEntity> { &self.adapter }
}

