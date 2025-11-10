use axum::extract::Query;
use axum::Json;
use serde_json::Value;
use crate::api::controller::metric::metrics_controller;
use crate::api::dto::ApiResponse;
use crate::api::dto::metrics_dto::RangeQuery;

// ---- Cluster ----
pub async fn cluster_get(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("cluster", "get", None, metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn cluster_cost(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("cluster", "cost", None, metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn cluster_summary(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("cluster", "summary", None, metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn cluster_trends(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("cluster", "trends", None, metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn cluster_efficiency(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match crate::domain::metrics::usecase::handle_request("cluster", "efficiency", None, metrics_controller::to_params(q)).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}