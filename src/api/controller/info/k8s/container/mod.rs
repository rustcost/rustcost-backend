use axum::extract::{Path, Query};
use axum::Json;
use serde_json::Value;
use crate::api::dto::ApiResponse;
use crate::api::dto::info_dto::K8sListQuery;
use crate::api::util::validation_ext::ValidateRequestExt;
use crate::core::persistence::info::k8s::container::info_container_entity::InfoContainerEntity;
use crate::domain::info::dto::info_k8s_container_patch_request::InfoK8sContainerPatchRequest;
use crate::domain::info::dto::info_setting_upsert_request::InfoSettingUpsertRequest;
use crate::domain::info::service::info_k8s_container_service;

pub async fn get_info_k8s_container(
    Path(id): Path<String>,
) -> Json<ApiResponse<InfoContainerEntity>> {
    match info_k8s_container_service::get_info_k8s_container(id).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn list_k8s_containers(
    Query(filter): Query<K8sListQuery>,
) -> Json<ApiResponse<Vec<InfoContainerEntity>>> {
    match info_k8s_container_service::list_k8s_containers(filter).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn patch_info_k8s_container(
    Path(id): Path<String>,
    Json(payload): Json<InfoK8sContainerPatchRequest>,
) -> Json<ApiResponse<Value>> {
    // ✅ 1. Validate the payload
    let payload = match payload.validate_or_err() {
        Ok(v) => v,
        Err(err_json) => return err_json,
    };

    // ✅ 2. Call your service
    match info_k8s_container_service::patch_info_k8s_container(id, payload).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}