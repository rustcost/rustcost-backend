use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::domain::metric::k8s::common::dto::{MetricGranularity, MetricScope};

/// High-level summary of raw cluster metrics over a time range
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricRawSummaryResponseDto {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub scope: MetricScope,
    pub granularity: MetricGranularity,
    pub summary: MetricRawSummaryDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MetricRawSummaryDto {
    pub avg_cpu_cores: f64,
    pub max_cpu_cores: f64,
    pub avg_memory_gb: f64,
    pub max_memory_gb: f64,
    pub avg_storage_gb: f64,
    pub max_storage_gb: f64,
    pub avg_network_gb: f64,
    pub max_network_gb: f64,
    pub node_count: usize,
}
