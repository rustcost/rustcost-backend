//! Metrics controller helpers: common response mapping utilities.

use anyhow::Result;
use axum::Json;
use serde_json::Value;

use crate::api::dto::ApiResponse;

/// Map a domain Result<Value> into Json<ApiResponse<Value>>
pub fn to_json(result: Result<Value>) -> Json<ApiResponse<Value>> {
    match result {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}
