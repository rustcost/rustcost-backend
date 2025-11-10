use axum::extract::{Path, Query};
use axum::Json;
use serde_json::Value;
use crate::api::controller::metric::metrics_controller;
use crate::api::dto::ApiResponse;
use crate::api::dto::metrics_dto::RangeQuery;

// ---- Containers ----
pub async fn containers_list(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("containers", "list", None, metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn container_get(Path(id): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("containers", "get", Some(id), metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn containers_cost(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("containers", "cost", None, metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn container_cost(Path(id): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("containers", "cost", Some(id), metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn containers_summary(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("containers", "summary", None, metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn container_summary(Path(id): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("containers", "summary", Some(id), metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn containers_trends(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("containers", "trends", None, metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn container_trends(Path(id): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("containers", "trends", Some(id), metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn containers_efficiency(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("containers", "efficiency", None, metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn container_efficiency(Path(id): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("containers", "efficiency", Some(id), metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}