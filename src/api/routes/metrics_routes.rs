//! Metrics routes (e.g., /api/v1/metrics/*)

use axum::{routing::get, Router};

use crate::api::controller::metrics_controller as mc;

/// Build the router for metrics endpoints under /api/v1/metrics
pub fn metrics_routes() -> Router {
    Router::new()
        // Nodes
        .route("/nodes", get(mc::nodes_list))
        .route("/nodes/{node_name}", get(mc::node_get))
        .route("/nodes/cost", get(mc::nodes_cost))
        .route("/nodes/{node_name}/cost", get(mc::node_cost))
        .route("/nodes/summary", get(mc::nodes_summary))
        .route("/nodes/{node_name}/summary", get(mc::node_summary))
        .route("/nodes/trends", get(mc::nodes_trends))
        .route("/nodes/{node_name}/trends", get(mc::node_trends))
        .route("/nodes/efficiency", get(mc::nodes_efficiency))
        .route("/nodes/{node_name}/efficiency", get(mc::node_efficiency))

        // Pods
        .route("/pods", get(mc::pods_list))
        .route("/pods/{pod_uid}", get(mc::pod_get))
        .route("/pods/cost", get(mc::pods_cost))
        .route("/pods/{pod_uid}/cost", get(mc::pod_cost))
        .route("/pods/summary", get(mc::pods_summary))
        .route("/pods/{pod_uid}/summary", get(mc::pod_summary))
        .route("/pods/trends", get(mc::pods_trends))
        .route("/pods/{pod_uid}/trends", get(mc::pod_trends))
        .route("/pods/efficiency", get(mc::pods_efficiency))
        .route("/pods/{pod_uid}/efficiency", get(mc::pod_efficiency))

        // Containers
        .route("/containers", get(mc::containers_list))
        .route("/containers/{id}", get(mc::container_get))
        .route("/containers/cost", get(mc::containers_cost))
        .route("/containers/{id}/cost", get(mc::container_cost))
        .route("/containers/summary", get(mc::containers_summary))
        .route("/containers/{id}/summary", get(mc::container_summary))
        .route("/containers/trends", get(mc::containers_trends))
        .route("/containers/{id}/trends", get(mc::container_trends))
        .route("/containers/efficiency", get(mc::containers_efficiency))
        .route("/containers/{id}/efficiency", get(mc::container_efficiency))

        // Namespaces
        .route("/namespaces", get(mc::namespaces_list))
        .route("/namespaces/{namespace}", get(mc::namespace_get))
        .route("/namespaces/cost", get(mc::namespaces_cost))
        .route("/namespaces/{namespace}/cost", get(mc::namespace_cost))
        .route("/namespaces/summary", get(mc::namespaces_summary))
        .route("/namespaces/{namespace}/summary", get(mc::namespace_summary))
        .route("/namespaces/trends", get(mc::namespaces_trends))
        .route("/namespaces/{namespace}/trends", get(mc::namespaces_trends))
        .route("/namespaces/efficiency", get(mc::namespaces_efficiency))
        .route("/namespaces/{namespace}/efficiency", get(mc::namespace_efficiency))

        // Deployments
        .route("/deployments", get(mc::deployments_list))
        .route("/deployments/{deployment}", get(mc::deployment_get))
        .route("/deployments/cost", get(mc::deployments_cost))
        .route("/deployments/{deployment}/cost", get(mc::deployment_cost))
        .route("/deployments/summary", get(mc::deployments_summary))
        .route("/deployments/{deployment}/summary", get(mc::deployment_summary))
        .route("/deployments/trends", get(mc::deployments_trends))
        .route("/deployments/{deployment}/trends", get(mc::deployment_trends))
        .route("/deployments/efficiency", get(mc::deployments_efficiency))
        .route("/deployments/{deployment}/efficiency", get(mc::deployment_efficiency))

        // Cluster
        .route("/cluster", get(mc::cluster_get))
        .route("/cluster/cost", get(mc::cluster_cost))
        .route("/cluster/summary", get(mc::cluster_summary))
        .route("/cluster/trends", get(mc::cluster_trends))
        .route("/cluster/efficiency", get(mc::cluster_efficiency))
}

