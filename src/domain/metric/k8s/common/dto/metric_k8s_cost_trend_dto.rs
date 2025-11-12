use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::domain::metric::k8s::common::dto::{MetricGranularity, MetricScope};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricCostTrendResponseDto {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub scope: MetricScope,
    pub target: Option<String>,
    pub granularity: MetricGranularity,
    pub trend: MetricCostTrendDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MetricCostTrendDto {
    /// Total cost at start and end
    pub start_cost_usd: f64,
    pub end_cost_usd: f64,

    /// Change in absolute and percentage terms
    pub cost_diff_usd: f64,
    pub growth_rate_percent: f64,

    /// Linear regression slope (USD per hour or per granularity)
    pub regression_slope_usd_per_granularity: f64,

    /// Optional next predicted cost point (simple extrapolation)
    pub predicted_next_cost_usd: Option<f64>,
}
