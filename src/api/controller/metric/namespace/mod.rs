use axum::extract::{Path, Query};
use axum::Json;
use serde_json::Value;
use crate::api::controller::metric::metrics_controller;
use crate::api::dto::ApiResponse;
use crate::api::dto::metrics_dto::RangeQuery;
use crate::domain::metrics::service::metric_k8s_namespace_service as svc;

// ---- Namespaces ----
pub async fn namespaces_list(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::namespaces_list(q).await)
}

pub async fn namespace_get(Path(namespace): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::namespace_get(namespace, q).await)
}

pub async fn namespaces_cost(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::namespaces_cost(q).await)
}

pub async fn namespace_cost(Path(namespace): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::namespace_cost(namespace, q).await)
}

pub async fn namespaces_summary(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::namespaces_summary(q).await)
}

pub async fn namespace_summary(Path(namespace): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::namespace_summary(namespace, q).await)
}

pub async fn namespaces_trends(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::namespaces_trends(q).await)
}

pub async fn namespace_trends(Path(namespace): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::namespace_trends(namespace, q).await)
}

pub async fn namespaces_efficiency(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::namespaces_efficiency(q).await)
}

pub async fn namespace_efficiency(Path(namespace): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(svc::namespace_efficiency(namespace, q).await)
}
