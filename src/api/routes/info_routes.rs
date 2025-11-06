//! Info routes (e.g., /api/v1/info/*)

use axum::{routing::{get}, Router};
use crate::api::controller::info_controller as ic;

pub fn info_routes() -> Router {
    Router::new()
        .route("/settings", get(ic::get_settings).put(ic::upsert_settings))
        .route("/unit-prices", get(ic::get_unit_prices).put(ic::upsert_unit_prices))
        .route("/versions", get(ic::get_versions))
        // Examples for entity reads
        .route("/nodes/{node_name}", get(ic::get_node_info))
        .route("/pods/{pod_uid}", get(ic::get_pod_info))
        .route("/containers/{id}", get(ic::get_container_info))
}

