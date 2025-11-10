//! Metrics API DTOs

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// Common time range + pagination query parameters
#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct RangeQuery {
    pub start: Option<NaiveDateTime>,
    pub end: Option<NaiveDateTime>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub sort: Option<String>,
    pub metric: Option<String>,
    pub namespace: Option<String>,
}

