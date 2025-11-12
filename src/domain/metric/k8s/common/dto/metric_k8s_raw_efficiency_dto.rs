use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::domain::metric::k8s::common::dto::{MetricGranularity, MetricScope};

/// Response wrapper for cluster-level raw efficiency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricRawEfficiencyResponseDto {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub scope: MetricScope,
    pub granularity: MetricGranularity,
    pub efficiency: MetricRawEfficiencyDto,
}

/// Efficiency ratios derived from average usage vs allocatable capacity
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MetricRawEfficiencyDto {
    /// CPU utilization ratio (0.0–1.0)
    pub cpu_efficiency: f64,
    /// Memory utilization ratio (0.0–1.0)
    pub memory_efficiency: f64,
    /// Storage utilization ratio (0.0–1.0)
    pub storage_efficiency: f64,
    /// Overall mean efficiency across all resources
    pub overall_efficiency: f64,

    /// Optional details for reference
    pub total_cpu_allocatable_cores: f64,
    pub total_memory_allocatable_gb: f64,
    pub total_storage_allocatable_gb: f64,
}
