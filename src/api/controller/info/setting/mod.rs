use axum::Json;
use crate::api::dto::ApiResponse;
use crate::core::persistence::info::fixed::setting::info_setting_entity::InfoSettingEntity;
use crate::domain::info::dto::info_setting_upsert_request::InfoSettingUpsertRequest;
use crate::api::util::validation_ext::ValidateRequestExt;
use serde_json::Value;
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