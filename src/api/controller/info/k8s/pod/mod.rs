use axum::extract::{Path, Query};
use axum::Json;
use crate::api::dto::ApiResponse;
use crate::api::dto::info_dto::K8sListQuery;
use crate::core::persistence::info::k8s::pod::info_pod_entity::InfoPodEntity;
use crate::domain::info::service::info_k8s_pod_service;

pub async fn get_info_k8s_pod(
    Path(pod_uid): Path<String>,
) -> Json<ApiResponse<InfoPodEntity>> {
    match info_k8s_pod_service::get_info_k8s_pod(pod_uid).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

/// List pods — optionally filter by `namespace`, `labelSelector`, or `nodeName`
pub async fn list_k8s_pods(
    Query(filter): Query<K8sListQuery>,
) -> Json<ApiResponse<Vec<InfoPodEntity>>> {
    match info_k8s_pod_service::list_k8s_pods(filter).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}