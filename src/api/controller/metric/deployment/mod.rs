use axum::extract::{Path, Query};
use axum::Json;
use serde_json::Value;
use crate::api::controller::metric::metrics_controller;
use crate::api::dto::ApiResponse;
use crate::api::dto::metrics_dto::RangeQuery;
use crate::domain::metrics::service::metric_k8s_deployment_service as svc;

// ---- Deployments ----
pub async fn deployments_list(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::deployments_list(q).await)
}

pub async fn deployment_get(Path(deployment): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::deployment_get(deployment, q).await)
}

pub async fn deployments_cost(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::deployments_cost(q).await)
}

pub async fn deployment_cost(Path(deployment): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::deployment_cost(deployment, q).await)
}

pub async fn deployments_summary(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::deployments_summary(q).await)
}

pub async fn deployment_summary(Path(deployment): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::deployment_summary(deployment, q).await)
}

pub async fn deployments_trends(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::deployments_trends(q).await)
}

pub async fn deployment_trends(Path(deployment): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::deployment_trends(deployment, q).await)
}

pub async fn deployments_efficiency(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::deployments_efficiency(q).await)
}

pub async fn deployment_efficiency(Path(deployment): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::deployment_efficiency(deployment, q).await)
}
