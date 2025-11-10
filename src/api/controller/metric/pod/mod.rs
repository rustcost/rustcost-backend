use axum::extract::{Path, Query};
use axum::Json;
use serde_json::Value;
use crate::api::controller::metric::metrics_controller;
use crate::api::dto::ApiResponse;
use crate::api::dto::metrics_dto::RangeQuery;

// ---- Pods ----
pub async fn pods_list(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("pods", "list", None, metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn pod_get(Path(pod_uid): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("pods", "get", Some(pod_uid), metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn pods_cost(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("pods", "cost", None, metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn pod_cost(Path(pod_uid): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("pods", "cost", Some(pod_uid), metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn pods_summary(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("pods", "summary", None, metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn pod_summary(Path(pod_uid): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("pods", "summary", Some(pod_uid), metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn pods_trends(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("pods", "trends", None, metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn pod_trends(Path(pod_uid): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("pods", "trends", Some(pod_uid), metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn pods_efficiency(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("pods", "efficiency", None, metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn pod_efficiency(Path(pod_uid): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("pods", "efficiency", Some(pod_uid), metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}