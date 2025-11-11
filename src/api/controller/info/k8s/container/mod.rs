use axum::extract::{Path, Query};
use axum::Json;
use crate::api::dto::ApiResponse;
use crate::api::dto::info_dto::K8sListQuery;
use crate::core::persistence::info::k8s::container::info_container_entity::InfoContainerEntity;
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