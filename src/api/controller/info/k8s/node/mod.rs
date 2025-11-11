use axum::Json;
use axum::extract::Path;
use crate::api::dto::ApiResponse;
use crate::core::persistence::info::k8s::node::info_node_entity::InfoNodeEntity;
use crate::domain::info::service::info_k8s_node_service;

pub async fn get_info_k8s_node(
    Path(node_name): Path<String>,
) -> Json<ApiResponse<InfoNodeEntity>> {
    match info_k8s_node_service::get_info_k8s_node(node_name).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn list_k8s_nodes() -> Json<ApiResponse<Vec<InfoNodeEntity>>> {
    match info_k8s_node_service::list_k8s_nodes().await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}