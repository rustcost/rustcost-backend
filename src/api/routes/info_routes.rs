//! Info routes (e.g., /api/v1/info/*)

use axum::{routing::get, Router};
use axum::routing::patch;
use crate::api::controller::info::info_controller as ic;
use crate::api::controller::info::setting::get_info_settings;
use crate::api::controller::info::setting::upsert_info_settings;
use crate::api::controller::info::k8s::namespace::get_k8s_namespaces;
use crate::api::controller::info::k8s::deployment::get_k8s_deployments;
use crate::api::controller::info::k8s::persistent_volume::get_k8s_persistent_volumes;
use crate::api::controller::info::k8s::pvc::get_k8s_persistent_volume_claims;
use crate::api::controller::info::k8s::resource_quota::get_k8s_resource_quotas;
use crate::api::controller::info::k8s::limit_range::get_k8s_limit_ranges;
use crate::api::controller::info::k8s::hpa::get_k8s_hpas;
use crate::api::controller::info::k8s::{container, node, pod};

pub fn info_routes() -> Router {
    Router::new()
        .route("/settings", get(get_info_settings).put(upsert_info_settings))
        .route("/unit-prices", get(ic::get_info_unit_prices).put(ic::upsert_info_unit_prices))
        .route("/versions", get(ic::get_info_versions))

        .route("/k8s/namespaces", get(get_k8s_namespaces))
        .route("/k8s/deployments", get(get_k8s_deployments))
        .route("/k8s/persistentvolumes", get(get_k8s_persistent_volumes))
        .route("/k8s/persistentvolumeclaims", get(get_k8s_persistent_volume_claims))
        .route("/k8s/resourcequotas", get(get_k8s_resource_quotas))
        .route("/k8s/limitranges", get(get_k8s_limit_ranges))
        .route("/k8s/horizontalpodautoscalers", get(get_k8s_hpas))
        .route("/k8s/nodes", get(node::list_k8s_nodes))
        .route("/k8s/pods", get(pod::list_k8s_pods))
        .route("/k8s/containers", get(container::list_k8s_containers))
        .route("/k8s/nodes/{node_name}", get(node::get_info_k8s_node))
        .route("/k8s/pods/{pod_uid}", get(pod::get_info_k8s_pod))
        .route("/k8s/containers/{id}", get(container::get_info_k8s_container))
        .route("/k8s/nodes/{node_name}", patch(node::patch_info_k8s_node))
        .route("/k8s/pods/{pod_uid}", patch(pod::patch_info_k8s_pod))
        .route("/k8s/containers/{id}", patch(container::patch_info_k8s_container))

}
