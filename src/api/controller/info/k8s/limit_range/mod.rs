use axum::Json;
use crate::api::dto::ApiResponse;
use crate::domain::info::service::info_k8s_limit_range_service;

pub async fn get_k8s_limit_ranges() -> Json<ApiResponse<serde_json::Value>> {
    match info_k8s_limit_range_service::get_k8s_limit_ranges().await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

