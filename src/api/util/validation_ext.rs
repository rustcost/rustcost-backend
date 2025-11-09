use axum::extract::Json;
use serde_json::Value;
use validator::Validate;

use crate::api::dto::ApiResponse;

/// A small trait to unify `.validate()` + `ApiResponse` error handling.
pub trait ValidateRequestExt {
    fn validate_or_err(self) -> Result<Self, Json<ApiResponse<Value>>>
    where
        Self: Sized + Validate;
}

impl<T: Validate> ValidateRequestExt for T {
    fn validate_or_err(self) -> Result<Self, Json<ApiResponse<Value>>> {
        if let Err(errors) = self.validate() {
            // Format validation errors nicely as JSON
            let json_err = serde_json::to_string_pretty(&errors).unwrap_or_default();
            Err(Json(ApiResponse::err(format!(
                "Validation failed: {}",
                json_err
            ))))
        } else {
            Ok(self)
        }
    }
}
