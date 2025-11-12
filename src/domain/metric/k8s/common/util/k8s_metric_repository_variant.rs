
use crate::domain::metric::k8s::container::repository::metric_container_day_api_repository::MetricContainerDayApiRepositoryImpl;
use crate::domain::metric::k8s::container::repository::metric_container_hour_api_repository::MetricContainerHourApiRepositoryImpl;
use crate::domain::metric::k8s::container::repository::metric_container_minute_api_repository::MetricContainerMinuteApiRepositoryImpl;
use crate::domain::metric::k8s::node::repository::metric_node_day_api_repository::MetricNodeDayApiRepositoryImpl;
use crate::domain::metric::k8s::node::repository::metric_node_hour_api_repository::MetricNodeHourApiRepositoryImpl;
use crate::domain::metric::k8s::node::repository::metric_node_minute_api_repository::MetricNodeMinuteApiRepositoryImpl;
use crate::domain::metric::k8s::pod::repository::metric_pod_day_api_repository::MetricPodDayApiRepositoryImpl;
use crate::domain::metric::k8s::pod::repository::metric_pod_hour_api_repository::MetricPodHourApiRepositoryImpl;
use crate::domain::metric::k8s::pod::repository::metric_pod_minute_api_repository::MetricPodMinuteApiRepositoryImpl;

#[derive(Debug)]
pub enum K8sMetricRepositoryVariant {
    // Node
    NodeMinute(MetricNodeMinuteApiRepositoryImpl),
    NodeHour(MetricNodeHourApiRepositoryImpl),
    NodeDay(MetricNodeDayApiRepositoryImpl),

    // Pod
    PodMinute(MetricPodMinuteApiRepositoryImpl),
    PodHour(MetricPodHourApiRepositoryImpl),
    PodDay(MetricPodDayApiRepositoryImpl),

    // Container
    ContainerMinute(MetricContainerMinuteApiRepositoryImpl),
    ContainerHour(MetricContainerHourApiRepositoryImpl),
    ContainerDay(MetricContainerDayApiRepositoryImpl),
}
