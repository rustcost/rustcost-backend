use anyhow::Result;
use chrono::{DateTime, Utc};
use crate::core::persistence::metrics::k8s::node::day::metric_node_day_api_repository_trait::MetricNodeDayApiRepository;
use crate::core::persistence::metrics::k8s::node::hour::metric_node_hour_api_repository_trait::MetricNodeHourApiRepository;
use crate::core::persistence::metrics::k8s::node::metric_node_entity::MetricNodeEntity;
use crate::core::persistence::metrics::k8s::node::minute::metric_node_minute_api_repository_trait::MetricNodeMinuteApiRepository;
use crate::domain::metric::k8s::node::repository::{
    metric_node_day_api_repository::MetricNodeDayApiRepositoryImpl,
    metric_node_hour_api_repository::MetricNodeHourApiRepositoryImpl,
    metric_node_minute_api_repository::MetricNodeMinuteApiRepositoryImpl,
};

/// Service for fetching node metrics with automatic time-resolution routing
pub struct MetricK8sNodeService;

impl MetricK8sNodeService {
    /// Select repository level (minute/hour/day) based on time range
    fn select_repo(
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> &'static str {
        let hours = (end - start).num_hours();
        if hours > 48 {
            "day"
        } else if hours > 3 {
            "hour"
        } else {
            "minute"
        }
    }

    /// Get rows for a node between start/end automatically choosing repo
    pub fn get_row_between(
        node_name: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<MetricNodeEntity>> {
        match Self::select_repo(start, end) {
            "day" => {
                let repo = MetricNodeDayApiRepositoryImpl::default();
                repo.get_row_between(node_name, start, end)
            }
            "hour" => {
                let repo = MetricNodeHourApiRepositoryImpl::default();
                repo.get_row_between(node_name, start, end)
            }
            _ => {
                let repo = MetricNodeMinuteApiRepositoryImpl::default();
                repo.get_row_between(node_name, start, end)
            }
        }
    }

    /// Get one metric column for a node between start/end
    pub fn get_column_between(
        column_name: &str,
        node_name: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<MetricNodeEntity>> {
        match Self::select_repo(start, end) {
            "day" => {
                let repo = MetricNodeDayApiRepositoryImpl::default();
                repo.get_column_between(column_name, start, end, node_name, limit, offset)
            }
            "hour" => {
                let repo = MetricNodeHourApiRepositoryImpl::default();
                repo.get_column_between(column_name, start, end, node_name, limit, offset)
            }
            _ => {
                let repo = MetricNodeMinuteApiRepositoryImpl::default();
                repo.get_column_between(column_name, start, end, node_name, limit, offset)
            }
        }
    }
}

// Also re-export legacy node service functions to maintain API surface
use serde_json::{json, Value};
use crate::api::dto::metrics_dto::RangeQuery;

fn not_implemented_payload(endpoint: &str) -> Value {
    json!({
        "status": "not_implemented",
        "endpoint": endpoint,
    })
}

pub async fn get_metric_k8s_nodes_raw(_q: RangeQuery) -> Result<Value> {
    Ok(not_implemented_payload("get_metric_k8s_nodes_raw"))
}

pub async fn get_metric_k8s_nodes_raw_summary(_q: RangeQuery) -> Result<Value> {
    Ok(not_implemented_payload("get_metric_k8s_nodes_raw_summary"))
}

pub async fn get_metric_k8s_nodes_raw_efficiency(_q: RangeQuery) -> Result<Value> {
    Ok(not_implemented_payload("get_metric_k8s_nodes_raw_efficiency"))
}

pub async fn get_metric_k8s_node_raw(node_name: String, _q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "not_implemented",
        "endpoint": "get_metric_k8s_node_raw",
        "node_name": node_name,
    }))
}

pub async fn get_metric_k8s_node_raw_summary(node_name: String, _q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "not_implemented",
        "endpoint": "get_metric_k8s_node_raw_summary",
        "node_name": node_name,
    }))
}

pub async fn get_metric_k8s_node_raw_efficiency(node_name: String, _q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "not_implemented",
        "endpoint": "get_metric_k8s_node_raw_efficiency",
        "node_name": node_name,
    }))
}

pub async fn get_metric_k8s_nodes_cost(_q: RangeQuery) -> Result<Value> {
    Ok(not_implemented_payload("get_metric_k8s_nodes_cost"))
}

pub async fn get_metric_k8s_nodes_cost_summary(_q: RangeQuery) -> Result<Value> {
    Ok(not_implemented_payload("get_metric_k8s_nodes_cost_summary"))
}

pub async fn get_metric_k8s_nodes_cost_trend(_q: RangeQuery) -> Result<Value> {
    Ok(not_implemented_payload("get_metric_k8s_nodes_cost_trend"))
}

pub async fn get_metric_k8s_node_cost(node_name: String, _q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "not_implemented",
        "endpoint": "get_metric_k8s_node_cost",
        "node_name": node_name,
    }))
}

pub async fn get_metric_k8s_node_cost_summary(node_name: String, _q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "not_implemented",
        "endpoint": "get_metric_k8s_node_cost_summary",
        "node_name": node_name,
    }))
}

pub async fn get_metric_k8s_node_cost_trend(node_name: String, _q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "not_implemented",
        "endpoint": "get_metric_k8s_node_cost_trend",
        "node_name": node_name,
    }))
}
