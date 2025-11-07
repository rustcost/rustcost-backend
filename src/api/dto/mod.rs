//! API Data Transfer Objects

use serde::Serialize;

pub mod metrics_dto;
pub mod info_dto;
pub mod system_dto;

/// Standard API response wrapper used by all endpoints
#[derive(Serialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub is_successful: bool,
    pub data: Option<T>,
    pub error_code: Option<String>,
    pub error_msg: Option<String>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    /// Creates a successful API response with data
    pub fn ok(data: T) -> Self {
        Self {
            is_successful: true,
            data: Some(data),
            error_code: None,
            error_msg: None,
        }
    }

    /// Creates an error API response with a message and optional code
    pub fn err(msg: impl Into<String>) -> Self {
        Self {
            is_successful: false,
            data: None,
            error_code: None,
            error_msg: Some(msg.into()),
        }
    }

    /// Creates an error response with both code and message
    pub fn err_with_code(code: impl Into<String>, msg: impl Into<String>) -> Self {
        Self {
            is_successful: false,
            data: None,
            error_code: Some(code.into()),
            error_msg: Some(msg.into()),
        }
    }
}
