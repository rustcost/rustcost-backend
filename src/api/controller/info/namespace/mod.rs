use axum::Json;
use crate::api::dto::ApiResponse;
use crate::domain::info::service::info_namespace_service;

pub async fn get_k8s_namespaces() -> Json<ApiResponse<serde_json::Value>> {
    match info_namespace_service::get_k8s_namespaces().await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}