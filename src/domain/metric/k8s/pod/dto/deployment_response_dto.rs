use serde::{Serialize, Deserialize};
use crate::domain::metric::k8s::pod::dto::metric_pod_dto::MetricPodDto;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentMetricsResponseDto {
    pub namespace: String,
    pub deployment: String,
    pub pod_uids: Vec<String>,
    pub pod_metrics: Vec<MetricPodDto>,
}

