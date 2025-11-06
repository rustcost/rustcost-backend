//! System routes (e.g., /api/v1/system/*)

use axum::{routing::{get, post}, Router};
use crate::api::controller::system_controller as sc;

pub fn system_routes() -> Router {
    Router::new()
        .route("/status", get(sc::status))
        .route("/health", get(sc::health))
        .route("/backup", post(sc::backup))
        .route("/resync", post(sc::resync))
}
