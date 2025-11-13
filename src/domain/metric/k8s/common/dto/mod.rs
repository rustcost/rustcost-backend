pub mod metric_k8s_cost_summary_dto;
pub mod metric_k8s_cost_trend_dto;
pub mod metric_k8s_raw_summary_dto;
pub mod metric_k8s_raw_efficiency_dto;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricGetResponseDto {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub scope: String,
    pub target: Option<String>,
    pub granularity: MetricGranularity,
    pub series: Vec<MetricSeriesDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricSeriesDto {
    pub name: String,
    pub scope: MetricScope,
    pub points: Vec<UniversalMetricPointDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MetricGranularity {
    Minute,
    Hour,
    Day,
}

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UniversalMetricPointDto {
    pub time: DateTime<Utc>,

    pub cpu_memory: CommonMetricValuesDto,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub filesystem: Option<FilesystemMetricDto>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<NetworkMetricDto>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<StorageMetricDto>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost: Option<CostMetricDto>, // <-- add this
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageMetricDto {
    pub ephemeral: Option<FilesystemMetricDto>,
    pub persistent: Option<FilesystemMetricDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkMetricDto {
    pub rx_bytes: Option<f64>,
    pub tx_bytes: Option<f64>,
    pub rx_errors: Option<f64>,
    pub tx_errors: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FilesystemMetricDto {
    pub used_bytes: Option<f64>,
    pub capacity_bytes: Option<f64>,
    pub inodes_used: Option<f64>,
    pub inodes: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommonMetricValuesDto {
    // CPU
    pub cpu_usage_nano_cores: Option<f64>,
    pub cpu_usage_core_nano_seconds: Option<f64>,

    // Memory
    pub memory_usage_bytes: Option<f64>,
    pub memory_working_set_bytes: Option<f64>,
    pub memory_rss_bytes: Option<f64>,
    pub memory_page_faults: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MetricScope {
    Cluster,
    Node,
    Pod,
    Container,
    Namespace,
    Deployment,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CostMetricDto {
    pub total_cost_usd: Option<f64>,
    pub cpu_cost_usd: Option<f64>,
    pub memory_cost_usd: Option<f64>,
    pub storage_cost_usd: Option<f64>,
}

