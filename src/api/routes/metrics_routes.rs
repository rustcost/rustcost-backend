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
        .route("/nodes", get(node_ctr::nodes_list))
        .route("/nodes/{node_name}", get(node_ctr::node_get))
        .route("/nodes/cost", get(node_ctr::nodes_cost))
        .route("/nodes/{node_name}/cost", get(node_ctr::node_cost))
        .route("/nodes/summary", get(node_ctr::nodes_summary))
        .route("/nodes/{node_name}/summary", get(node_ctr::node_summary))
        .route("/nodes/trends", get(node_ctr::nodes_trends))
        .route("/nodes/{node_name}/trends", get(node_ctr::node_trends))
        .route("/nodes/efficiency", get(node_ctr::nodes_efficiency))
        .route("/nodes/{node_name}/efficiency", get(node_ctr::node_efficiency))

        // Pods
        .route("/pods", get(pod_ctr::pods_list))
        .route("/pods/{pod_uid}", get(pod_ctr::pod_get))
        .route("/pods/cost", get(pod_ctr::pods_cost))
        .route("/pods/{pod_uid}/cost", get(pod_ctr::pod_cost))
        .route("/pods/summary", get(pod_ctr::pods_summary))
        .route("/pods/{pod_uid}/summary", get(pod_ctr::pod_summary))
        .route("/pods/trends", get(pod_ctr::pods_trends))
        .route("/pods/{pod_uid}/trends", get(pod_ctr::pod_trends))
        .route("/pods/efficiency", get(pod_ctr::pods_efficiency))
        .route("/pods/{pod_uid}/efficiency", get(pod_ctr::pod_efficiency))

        // Containers
        .route("/containers", get(con_ctr::containers_list))
        .route("/containers/{id}", get(con_ctr::container_get))
        .route("/containers/cost", get(con_ctr::containers_cost))
        .route("/containers/{id}/cost", get(con_ctr::container_cost))
        .route("/containers/summary", get(con_ctr::containers_summary))
        .route("/containers/{id}/summary", get(con_ctr::container_summary))
        .route("/containers/trends", get(con_ctr::containers_trends))
        .route("/containers/{id}/trends", get(con_ctr::container_trends))
        .route("/containers/efficiency", get(con_ctr::containers_efficiency))
        .route("/containers/{id}/efficiency", get(con_ctr::container_efficiency))

        // Namespaces
        .route("/namespaces", get(ns_ctr::namespaces_list))
        .route("/namespaces/{namespace}", get(ns_ctr::namespace_get))
        .route("/namespaces/cost", get(ns_ctr::namespaces_cost))
        .route("/namespaces/{namespace}/cost", get(ns_ctr::namespace_cost))
        .route("/namespaces/summary", get(ns_ctr::namespaces_summary))
        .route("/namespaces/{namespace}/summary", get(ns_ctr::namespace_summary))
        .route("/namespaces/trends", get(ns_ctr::namespaces_trends))
        .route("/namespaces/{namespace}/trends", get(ns_ctr::namespace_trends))
        .route("/namespaces/efficiency", get(ns_ctr::namespaces_efficiency))
        .route("/namespaces/{namespace}/efficiency", get(ns_ctr::namespace_efficiency))

        // Deployments
        .route("/deployments", get(deploy_ctr::deployments_list))
        .route("/deployments/{deployment}", get(deploy_ctr::deployment_get))
        .route("/deployments/cost", get(deploy_ctr::deployments_cost))
        .route("/deployments/{deployment}/cost", get(deploy_ctr::deployment_cost))
        .route("/deployments/summary", get(deploy_ctr::deployments_summary))
        .route("/deployments/{deployment}/summary", get(deploy_ctr::deployment_summary))
        .route("/deployments/trends", get(deploy_ctr::deployments_trends))
        .route("/deployments/{deployment}/trends", get(deploy_ctr::deployment_trends))
        .route("/deployments/efficiency", get(deploy_ctr::deployments_efficiency))
        .route("/deployments/{deployment}/efficiency", get(deploy_ctr::deployment_efficiency))

        // Cluster
        .route("/cluster", get(cluster_ctr::cluster_get))
        .route("/cluster/cost", get(cluster_ctr::cluster_cost))
        .route("/cluster/summary", get(cluster_ctr::cluster_summary))
        .route("/cluster/trends", get(cluster_ctr::cluster_trends))
        .route("/cluster/efficiency", get(cluster_ctr::cluster_efficiency))
}
