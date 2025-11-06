//! System controller: connects routes to system usecases

use axum::Json;
use serde_json::Value;

use crate::api::dto::ApiResponse;

pub async fn status() -> Json<ApiResponse<Value>> {
    match crate::domain::system::usecase::status().await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn health() -> Json<ApiResponse<Value>> {
    match crate::domain::system::usecase::health().await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn backup() -> Json<ApiResponse<Value>> {
    match crate::domain::system::usecase::backup().await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn resync() -> Json<ApiResponse<Value>> {
    match crate::domain::system::usecase::resync().await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

