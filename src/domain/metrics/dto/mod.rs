//! Metrics domain DTOs (pure domain-facing, transport-agnostic)

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricPoint {
    pub bucket: NaiveDateTime,
    pub cpu: f64,
    pub mem: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryDto {
    pub avg_cpu: f64,
    pub avg_mem: f64,
    pub samples: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostSummaryDto {
    pub total_cost: f64,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendPoint {
    pub bucket: NaiveDateTime,
    pub delta_cpu: f64,
    pub delta_mem: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EfficiencyDto {
    pub score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeSeriesDto {
    pub node_name: String,
    pub points: Vec<MetricPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodSeriesDto {
    pub pod_uid: String,
    pub points: Vec<MetricPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerSeriesDto {
    pub id: String, // pod_uid + "_" + container_name
    pub points: Vec<MetricPoint>,
}

