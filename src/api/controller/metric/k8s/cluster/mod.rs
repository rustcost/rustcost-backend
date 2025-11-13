use axum::{extract::Query, Json};
use serde_json::Value;
use crate::api::dto::{ApiResponse, metrics_dto::RangeQuery};
use crate::domain::info::service::{info_k8s_node_service, info_unit_price_service};
use crate::domain::metric::k8s::cluster::service as metric_k8s_cluster_service;


/// ---- Cluster Metric Endpoints ----
///
/// - `raw` → time-series metrics for charts
/// - `summary` → aggregated snapshot (avg/sum)
/// - `cost` → derived cost over time
/// - `cost_summary` → aggregated cost snapshot
/// - `cost_trend` → cost trend / prediction
/// - `cost_efficiency` → ratios like cost per CPU, cost per pod, etc.


// Time-series for charts
pub async fn get_metric_k8s_cluster_raw(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match async {
        let nodes = info_k8s_node_service::list_k8s_nodes().await?;
        let result = metric_k8s_cluster_service::get_metric_k8s_cluster_raw(nodes, q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
        .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

// Aggregated snapshot (avg/sum for time range)
pub async fn get_metric_k8s_cluster_raw_summary(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match async {
        let nodes = info_k8s_node_service::list_k8s_nodes().await?;
        let result = metric_k8s_cluster_service::get_metric_k8s_cluster_raw_summary(nodes, q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
        .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

// Derived cost over time for charts
pub async fn get_metric_k8s_cluster_cost(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match async {
        let nodes = info_k8s_node_service::list_k8s_nodes().await?;
        let costs = info_unit_price_service::get_info_unit_prices().await?;
        let result = metric_k8s_cluster_service::get_metric_k8s_cluster_cost(nodes, costs, q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
        .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

// Summarized cost (total/avg for time range)
pub async fn get_metric_k8s_cluster_cost_summary(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match async {
        let nodes = info_k8s_node_service::list_k8s_nodes().await?;
        let costs = info_unit_price_service::get_info_unit_prices().await?;
        let result = metric_k8s_cluster_service::get_metric_k8s_cluster_cost_summary(nodes, costs, q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
        .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

// Trendline (growth, regression, prediction)
pub async fn get_metric_k8s_cluster_cost_trend(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match async {
        let nodes = info_k8s_node_service::list_k8s_nodes().await?;
        let costs = info_unit_price_service::get_info_unit_prices().await?;
        let result = metric_k8s_cluster_service::get_metric_k8s_cluster_cost_trend(nodes, costs, q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
        .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

// Ratios (cost per CPU, cost per pod, etc.)

pub async fn get_metric_k8s_cluster_raw_efficiency(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match async {
        let nodes = info_k8s_node_service::list_k8s_nodes().await?;
        let result = metric_k8s_cluster_service::get_metric_k8s_cluster_raw_efficiency(nodes, q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
        .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}