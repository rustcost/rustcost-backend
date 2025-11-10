use axum::extract::{Path, Query};
use axum::Json;
use serde_json::Value;
use crate::api::controller::metric::metrics_controller;
use crate::api::dto::ApiResponse;
use crate::api::dto::metrics_dto::RangeQuery;
use crate::domain::metrics::service::metric_k8s_node_service as svc;

// ---- Nodes ----
pub async fn nodes_list(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::nodes_list(q).await)
}

pub async fn node_get(Path(node_name): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::node_get(node_name, q).await)
}

pub async fn nodes_cost(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::nodes_cost(q).await)
}

pub async fn node_cost(Path(node_name): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::node_cost(node_name, q).await)
}

pub async fn nodes_summary(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::nodes_summary(q).await)
}

pub async fn node_summary(Path(node_name): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::node_summary(node_name, q).await)
}

pub async fn nodes_trends(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::nodes_trends(q).await)
}

pub async fn node_trends(Path(node_name): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::node_trends(node_name, q).await)
}

pub async fn nodes_efficiency(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::nodes_efficiency(q).await)
}

pub async fn node_efficiency(Path(node_name): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::node_efficiency(node_name, q).await)
}
