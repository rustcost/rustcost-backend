use axum::extract::{Path, Query};
use axum::Json;
use serde_json::Value;
use crate::api::controller::metric::metrics_controller;
use crate::api::dto::ApiResponse;
use crate::api::dto::metrics_dto::RangeQuery;
use crate::domain::metric::k8s::container::service as svc;

// ---- Containers ----
pub async fn containers_list(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::containers_list(q).await)
}

pub async fn container_get(Path(id): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::container_get(id, q).await)
}

pub async fn containers_cost(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::containers_cost(q).await)
}

pub async fn container_cost(Path(id): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::container_cost(id, q).await)
}

pub async fn containers_summary(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::containers_summary(q).await)
}

pub async fn container_summary(Path(id): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::container_summary(id, q).await)
}

pub async fn containers_trends(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::containers_trends(q).await)
}

pub async fn container_trends(Path(id): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::container_trends(id, q).await)
}

pub async fn containers_efficiency(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::containers_efficiency(q).await)
}

pub async fn container_efficiency(Path(id): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::container_efficiency(id, q).await)
}
