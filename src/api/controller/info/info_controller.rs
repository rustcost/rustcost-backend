//! Info controller: connects routes to info usecases

use axum::Json;
use serde_json::Value;
use crate::api::dto::ApiResponse;
use crate::api::util::validation_ext::ValidateRequestExt;
use crate::core::persistence::info::fixed::unit_price::info_unit_price_entity::InfoUnitPriceEntity;
use crate::core::persistence::info::fixed::version::info_version_entity::InfoVersionEntity;
use crate::domain::info::dto::info_unit_price_upsert_request::InfoUnitPriceUpsertRequest;
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
