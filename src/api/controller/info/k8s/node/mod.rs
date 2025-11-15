use axum::Json;
use axum::extract::Path;
use serde_json::Value;
use crate::api::dto::ApiResponse;
use crate::api::util::validation_ext::ValidateRequestExt;
use crate::core::persistence::info::k8s::node::info_node_entity::InfoNodeEntity;
use crate::domain::info::dto::info_k8s_container_patch_request::InfoK8sContainerPatchRequest;
use crate::domain::info::dto::info_k8s_node_patch_request::InfoK8sNodePatchRequest;
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

pub async fn patch_info_k8s_node(
    Path(id): Path<String>,
    Json(payload): Json<InfoK8sNodePatchRequest>,
) -> Json<ApiResponse<Value>> {
    // ✅ 1. Validate the payload
    let payload = match payload.validate_or_err() {
        Ok(v) => v,
        Err(err_json) => return err_json,
    };

    // ✅ 2. Call your service
    match info_k8s_node_service::patch_info_k8s_node(id, payload).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}