use crate::domain::metric::k8s::common::dto::{MetricGranularity, MetricScope};
use super::k8s_metric_repository_variant::K8sMetricRepositoryVariant;

/// Resolve metric repository variant from metric scope and granularity.
pub fn resolve_k8s_metric_repository(
    scope: &MetricScope,
    granularity: &MetricGranularity,
) -> K8sMetricRepositoryVariant {
    use crate::domain::metric::k8s::common::dto::MetricGranularity::*;
    use K8sMetricRepositoryVariant::*;

    match scope {
        MetricScope::Node => match granularity {
            Minute => NodeMinute(Default::default()),
            Hour => NodeHour(Default::default()),
            Day => NodeDay(Default::default()),
        },
        MetricScope::Pod => match granularity {
            Minute => PodMinute(Default::default()),
            Hour => PodHour(Default::default()),
            Day => PodDay(Default::default()),
        },
        MetricScope::Container => match granularity {
            Minute => ContainerMinute(Default::default()),
            Hour => ContainerHour(Default::default()),
            Day => ContainerDay(Default::default()),
        },
        MetricScope::Cluster => match granularity {
            // For cluster, reuse node-level repos
            Minute => NodeMinute(Default::default()),
            Hour => NodeHour(Default::default()),
            Day => NodeDay(Default::default()),
        },
        MetricScope::Namespace | MetricScope::Deployment => match granularity {
            Minute => PodMinute(Default::default()),
            Hour => PodHour(Default::default()),
            Day => PodDay(Default::default()),
        },
    }
}
