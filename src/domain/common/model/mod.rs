//! Shared domain types (Range, Pagination, MetricUnit, etc.)

use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct RangeParams {
    pub start: Option<NaiveDateTime>,
    pub end: Option<NaiveDateTime>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub sort: Option<String>,
    pub metric: Option<String>,
    pub namespace: Option<String>,
}

