use axum::extract::{Path, Query};
use axum::Json;
use serde_json::Value;
use crate::api::controller::metric::metrics_controller;
use crate::api::dto::ApiResponse;
use crate::api::dto::metrics_dto::RangeQuery;

// ---- Deployments ----
pub async fn deployments_list(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("deployments", "list", None, metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn deployment_get(Path(deployment): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("deployments", "get", Some(deployment), metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn deployments_cost(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("deployments", "cost", None, metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn deployment_cost(Path(deployment): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("deployments", "cost", Some(deployment), metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn deployments_summary(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("deployments", "summary", None, metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn deployment_summary(Path(deployment): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("deployments", "summary", Some(deployment), metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn deployments_trends(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("deployments", "trends", None, metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn deployment_trends(Path(deployment): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("deployments", "trends", Some(deployment), metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn deployments_efficiency(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("deployments", "efficiency", None, metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn deployment_efficiency(Path(deployment): Path<String>, Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("deployments", "efficiency", Some(deployment), metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}