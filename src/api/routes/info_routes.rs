//! Info routes (e.g., /api/v1/info/*)

use axum::{routing::{get}, Router};
use crate::api::controller::info_controller as ic;

pub fn info_routes() -> Router {
    Router::new()
        .route("/settings", get(ic::get_info_settings).put(ic::upsert_info_settings))
        .route("/unit-prices", get(ic::get_info_unit_prices).put(ic::upsert_info_unit_prices))
        .route("/versions", get(ic::get_info_versions))
        // Examples for entity reads
        .route("/k8s/nodes/{node_name}", get(ic::get_info_k8s_node))
        .route("/k8s/pods/{pod_uid}", get(ic::get_info_k8s_pod))
        .route("/k8s/containers/{id}", get(ic::get_info_k8s_container))
}

