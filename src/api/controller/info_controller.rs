//! Info controller: connects routes to info usecases

use axum::{extract::Path, Json};
use serde_json::Value;
use crate::api::dto::ApiResponse;
use crate::api::util::validation_ext::ValidateRequestExt;
use crate::core::persistence::info::fixed::setting::info_setting_entity::InfoSettingEntity;
use crate::domain::info::dto::info_setting_upsert_request::InfoSettingUpsertRequest;

pub async fn get_settings() -> Json<ApiResponse<InfoSettingEntity>> {
    match crate::domain::info::service::settings_service::get_settings().await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}
pub async fn upsert_settings(
    Json(payload): Json<InfoSettingUpsertRequest>,
) -> Json<ApiResponse<Value>> {
    // ✅ 1. Validate the payload
    let payload = match payload.validate_or_err() {
        Ok(v) => v,
        Err(err_json) => return err_json,
    };

    // ✅ 2. Call your service
    match crate::domain::info::service::settings_service::upsert_settings(payload).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_unit_prices() -> Json<ApiResponse<Value>> {
    match crate::domain::info::usecase::get_unit_prices().await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn upsert_unit_prices() -> Json<ApiResponse<Value>> {
    match crate::domain::info::usecase::upsert_unit_prices().await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_versions() -> Json<ApiResponse<Value>> {
    match crate::domain::info::usecase::get_versions().await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_node_info(Path(node_name): Path<String>) -> Json<ApiResponse<Value>> {
    match crate::domain::info::usecase::get_node_info(node_name).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_pod_info(Path(pod_uid): Path<String>) -> Json<ApiResponse<Value>> {
    match crate::domain::info::usecase::get_pod_info(pod_uid).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_container_info(Path(id): Path<String>) -> Json<ApiResponse<Value>> {
    match crate::domain::info::usecase::get_container_info(id).await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

