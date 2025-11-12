use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::domain::metric::k8s::common::dto::{MetricGranularity, MetricScope};

/// Summarized cost view for any Kubernetes metric scope (Cluster, Node, Pod, Container)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricCostSummaryResponseDto {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub scope: MetricScope,
    pub target: Option<String>,             // Node / Pod / Container name
    pub granularity: MetricGranularity,
    pub summary: MetricCostSummaryDto,
}

/// Aggregated cost breakdown (includes PV and network)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MetricCostSummaryDto {
    /// Total combined cost in USD
    pub total_cost_usd: f64,

    /// CPU resource cost in USD
    pub cpu_cost_usd: f64,

    /// Memory resource cost in USD
    pub memory_cost_usd: f64,

    /// Ephemeral storage cost (e.g. rootfs, node FS)
    pub ephemeral_storage_cost_usd: f64,

    /// Persistent volume (PV) storage cost
    pub persistent_storage_cost_usd: f64,

    /// Network transfer cost in USD
    pub network_cost_usd: f64,
}
