use axum::{extract::Query, Json};
use serde_json::Value;
use crate::api::controller::metric::metrics_controller;
use crate::api::dto::{ApiResponse, metrics_dto::RangeQuery};
use crate::domain::info::service::info_k8s_node_service;
use crate::domain::metrics::service::metric_k8s_cluster_service;

// ---- Cluster ----
pub async fn cluster_get(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    let res = async {
        let nodes = info_k8s_node_service::list_k8s_nodes().await?;
        let result = metric_k8s_cluster_service::cluster_get(nodes, q).await?;
        Ok::<Value, _>(result)
    }.await;
    metrics_controller::to_json(res)
}

pub async fn cluster_cost(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(metric_k8s_cluster_service::cluster_cost(q).await)
}

pub async fn cluster_summary(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(metric_k8s_cluster_service::cluster_summary(q).await)
}

pub async fn cluster_trends(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(metric_k8s_cluster_service::cluster_trends(q).await)
}

pub async fn cluster_efficiency(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(metric_k8s_cluster_service::cluster_efficiency(q).await)
}
