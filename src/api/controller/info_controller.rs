//! Info controller: connects routes to info usecases

use axum::{extract::Path, Json};
use axum::extract::Query;
use serde_json::Value;
use crate::api::dto::ApiResponse;
use crate::api::dto::info_dto::K8sListQuery;
use crate::api::util::validation_ext::ValidateRequestExt;
use crate::core::persistence::info::fixed::setting::info_setting_entity::InfoSettingEntity;
use crate::core::persistence::info::fixed::unit_price::info_unit_price_entity::InfoUnitPriceEntity;
use crate::core::persistence::info::fixed::version::info_version_entity::InfoVersionEntity;
use crate::core::persistence::info::k8s::node::info_node_entity::InfoNodeEntity;
use crate::core::persistence::info::k8s::pod::info_pod_entity::InfoPodEntity;
use crate::core::persistence::info::k8s::container::info_container_entity::InfoContainerEntity;
use crate::domain::info::dto::info_setting_upsert_request::InfoSettingUpsertRequest;
use crate::domain::info::dto::info_unit_price_upsert_request::InfoUnitPriceUpsertRequest;
use crate::domain::info::service::{info_k8s_container_service, info_k8s_node_service, info_k8s_pod_service};

pub async fn get_info_settings() -> Json<ApiResponse<InfoSettingEntity>> {
    match crate::domain::info::service::info_settings_service::get_info_settings().await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}
pub async fn upsert_info_settings(
    Json(payload): Json<InfoSettingUpsertRequest>,
) -> Json<ApiResponse<Value>> {
    // ✅ 1. Validate the payload
    let payload = match payload.validate_or_err() {
        Ok(v) => v,
        Err(err_json) => return err_json,
    };

    // ✅ 2. Call your service
    match crate::domain::info::service::info_settings_service::upsert_info_settings(payload).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_info_unit_prices() -> Json<ApiResponse<InfoUnitPriceEntity>> {
    match crate::domain::info::service::info_unit_price_service::get_info_unit_prices().await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}
pub async fn upsert_info_unit_prices(
    Json(payload): Json<InfoUnitPriceUpsertRequest>,
) -> Json<ApiResponse<Value>> {
    // ✅ 1. Validate the payload
    let payload = match payload.validate_or_err() {
        Ok(v) => v,
        Err(err_json) => return err_json,
    };

    // ✅ 2. Call your service
    match crate::domain::info::service::info_unit_price_service::upsert_info_unit_prices(payload).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_info_versions() -> Json<ApiResponse<InfoVersionEntity>> {
    match crate::domain::info::service::info_version_service::get_info_versions().await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}




/* -------------------- NODES -------------------- */

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

/* -------------------- PODS -------------------- */

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

/* -------------------- CONTAINERS -------------------- */

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
