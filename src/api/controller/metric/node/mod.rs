use axum::extract::{Path, Query};
use axum::Json;
use serde_json::Value;
use crate::api::controller::metric::metrics_controller;
use crate::api::dto::ApiResponse;
use crate::api::dto::metrics_dto::RangeQuery;

// ---- Nodes ----
pub async fn nodes_list(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("nodes", "list", None, metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn node_get(Path(node_name): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("nodes", "get", Some(node_name), metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn nodes_cost(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("nodes", "cost", None, metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn node_cost(Path(node_name): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("nodes", "cost", Some(node_name), metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn nodes_summary(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("nodes", "summary", None, metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn node_summary(Path(node_name): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("nodes", "summary", Some(node_name), metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn nodes_trends(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("nodes", "trends", None, metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn node_trends(Path(node_name): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("nodes", "trends", Some(node_name), metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn nodes_efficiency(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("nodes", "efficiency", None, metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn node_efficiency(Path(node_name): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("nodes", "efficiency", Some(node_name), metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}