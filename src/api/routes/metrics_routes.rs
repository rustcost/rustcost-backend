//! Metrics routes (e.g., /api/v1/metrics/*)

use axum::{routing::get, Router};

use crate::api::controller::metric::k8s::namespace as ns_ctr;
use crate::api::controller::metric::k8s::node as node_ctr;
use crate::api::controller::metric::k8s::container as con_ctr;
use crate::api::controller::metric::k8s::deployment as deploy_ctr;
use crate::api::controller::metric::k8s::pod as pod_ctr;
use crate::api::controller::metric::k8s::cluster as cluster_ctr;

/// Build the router for metrics endpoints under /api/v1/metrics
pub fn metrics_routes() -> Router {
    Router::new()
        // Nodes
        .route("/nodes/raw", get(node_ctr::nodes_raw))
        .route("/nodes/raw/summary", get(node_ctr::nodes_raw_summary))
        .route("/nodes/raw/efficiency", get(node_ctr::nodes_raw_efficiency))
        .route("/nodes/{node_name}/raw", get(node_ctr::node_raw))
        .route("/nodes/{node_name}/raw/summary", get(node_ctr::node_raw_summary))
        .route("/nodes/{node_name}/raw/efficiency", get(node_ctr::node_raw_efficiency))
        .route("/nodes/cost", get(node_ctr::nodes_cost))
        .route("/nodes/cost/summary", get(node_ctr::nodes_cost_summary))
        .route("/nodes/cost/trend", get(node_ctr::nodes_cost_trend))
        .route("/nodes/{node_name}/cost", get(node_ctr::node_cost))
        .route("/nodes/{node_name}/cost/summary", get(node_ctr::node_cost_summary))
        .route("/nodes/{node_name}/cost/trend", get(node_ctr::node_cost_trend))

        // Pods
        .route("/pods/raw", get(pod_ctr::pods_raw))
        .route("/pods/raw/summary", get(pod_ctr::pods_raw_summary))
        .route("/pods/raw/efficiency", get(pod_ctr::pods_raw_efficiency))
        .route("/pods/{pod_uid}/raw", get(pod_ctr::pod_raw))
        .route("/pods/{pod_uid}/raw/summary", get(pod_ctr::pod_raw_summary))
        .route("/pods/{pod_uid}/raw/efficiency", get(pod_ctr::pod_raw_efficiency))
        .route("/pods/cost", get(pod_ctr::pods_cost))
        .route("/pods/cost/summary", get(pod_ctr::pods_cost_summary))
        .route("/pods/cost/trend", get(pod_ctr::pods_cost_trend))
        .route("/pods/{pod_uid}/cost", get(pod_ctr::pod_cost))
        .route("/pods/{pod_uid}/cost/summary", get(pod_ctr::pod_cost_summary))
        .route("/pods/{pod_uid}/cost/trend", get(pod_ctr::pod_cost_trend))

        // Containers
        .route("/containers/raw", get(con_ctr::containers_raw))
        .route("/containers/raw/summary", get(con_ctr::containers_raw_summary))
        .route("/containers/raw/efficiency", get(con_ctr::containers_raw_efficiency))
        .route("/containers/{id}/raw", get(con_ctr::container_raw))
        .route("/containers/{id}/raw/summary", get(con_ctr::container_raw_summary))
        .route("/containers/{id}/raw/efficiency", get(con_ctr::container_raw_efficiency))
        .route("/containers/cost", get(con_ctr::containers_cost))
        .route("/containers/cost/summary", get(con_ctr::containers_cost_summary))
        .route("/containers/cost/trend", get(con_ctr::containers_cost_trend))
        .route("/containers/{id}/cost", get(con_ctr::container_cost))
        .route("/containers/{id}/cost/summary", get(con_ctr::container_cost_summary))
        .route("/containers/{id}/cost/trend", get(con_ctr::container_cost_trend))

        // Namespaces
        .route("/namespaces/raw", get(ns_ctr::namespaces_raw))
        .route("/namespaces/raw/summary", get(ns_ctr::namespaces_raw_summary))
        .route("/namespaces/raw/efficiency", get(ns_ctr::namespaces_raw_efficiency))
        .route("/namespaces/{namespace}/raw", get(ns_ctr::namespace_raw))
        .route("/namespaces/{namespace}/raw/summary", get(ns_ctr::namespace_raw_summary))
        .route("/namespaces/{namespace}/raw/efficiency", get(ns_ctr::namespace_raw_efficiency))
        .route("/namespaces/cost", get(ns_ctr::namespaces_cost))
        .route("/namespaces/cost/summary", get(ns_ctr::namespaces_cost_summary))
        .route("/namespaces/cost/trend", get(ns_ctr::namespaces_cost_trend))
        .route("/namespaces/{namespace}/cost", get(ns_ctr::namespace_cost))
        .route("/namespaces/{namespace}/cost/summary", get(ns_ctr::namespace_cost_summary))
        .route("/namespaces/{namespace}/cost/trend", get(ns_ctr::namespace_cost_trend))

        // Deployments
        .route("/deployments/raw", get(deploy_ctr::deployments_raw))
        .route("/deployments/raw/summary", get(deploy_ctr::deployments_raw_summary))
        .route("/deployments/raw/efficiency", get(deploy_ctr::deployments_raw_efficiency))
        .route("/deployments/{deployment}/raw", get(deploy_ctr::deployment_raw))
        .route("/deployments/{deployment}/raw/summary", get(deploy_ctr::deployment_raw_summary))
        .route("/deployments/{deployment}/raw/efficiency", get(deploy_ctr::deployment_raw_efficiency))
        .route("/deployments/cost", get(deploy_ctr::deployments_cost))
        .route("/deployments/cost/summary", get(deploy_ctr::deployments_cost_summary))
        .route("/deployments/cost/trend", get(deploy_ctr::deployments_cost_trend))
        .route("/deployments/{deployment}/cost", get(deploy_ctr::deployment_cost))
        .route("/deployments/{deployment}/cost/summary", get(deploy_ctr::deployment_cost_summary))
        .route("/deployments/{deployment}/cost/trend", get(deploy_ctr::deployment_cost_trend))

        // Cluster
        .route("/cluster/raw", get(cluster_ctr::get_metric_k8s_cluster_raw))
        .route("/cluster/raw/summary", get(cluster_ctr::get_metric_k8s_cluster_raw_summary))
        .route("/cluster/raw/efficiency", get(cluster_ctr::get_metric_k8s_cluster_raw_efficiency))
        .route("/cluster/cost", get(cluster_ctr::get_metric_k8s_cluster_cost))
        .route("/cluster/cost/summary", get(cluster_ctr::get_metric_k8s_cluster_cost_summary))
        .route("/cluster/cost/trend", get(cluster_ctr::get_metric_k8s_cluster_cost_trend))
}
