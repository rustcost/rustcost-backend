use axum::Json;
use crate::api::dto::ApiResponse;
use crate::domain::info::service::info_k8s_hpa_service;

pub async fn get_k8s_hpas() -> Json<ApiResponse<serde_json::Value>> {
    match info_k8s_hpa_service::get_k8s_hpas().await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

