use axum::extract::{Path, Query};
use axum::Json;
use serde_json::Value;
use crate::api::controller::metric::metrics_controller;
use crate::api::dto::ApiResponse;
use crate::api::dto::metrics_dto::RangeQuery;
use crate::domain::metric::k8s::pod::service as svc;

// ---- Pods ----
pub async fn pods_list(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::pods_list(q).await)
}

pub async fn pod_get(Path(pod_uid): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::pod_get(pod_uid, q).await)
}

pub async fn pods_cost(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::pods_cost(q).await)
}

pub async fn pod_cost(Path(pod_uid): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::pod_cost(pod_uid, q).await)
}

pub async fn pods_summary(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::pods_summary(q).await)
}

pub async fn pod_summary(Path(pod_uid): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::pod_summary(pod_uid, q).await)
}

pub async fn pods_trends(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::pods_trends(q).await)
}

pub async fn pod_trends(Path(pod_uid): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::pod_trends(pod_uid, q).await)
}

pub async fn pods_efficiency(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::pods_efficiency(q).await)
}

pub async fn pod_efficiency(Path(pod_uid): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::pod_efficiency(pod_uid, q).await)
}
