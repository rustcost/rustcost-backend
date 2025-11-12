use axum::{extract::Query, Json};
use serde_json::Value;
use crate::api::controller::metric::metrics_controller;
use crate::api::dto::{ApiResponse, metrics_dto::RangeQuery};
use crate::domain::info::service::info_k8s_node_service;
use crate::domain::metric::k8s::cluster::service as metric_k8s_cluster_service;

// ---- Cluster ---- (new naming)
pub async fn get_metric_k8s_cluster_raw(
    Query(q): Query<RangeQuery>,
) -> Json<ApiResponse<Value>> {
    match async {
        let nodes = info_k8s_node_service::list_k8s_nodes().await?;
        let result = metric_k8s_cluster_service::get_metric_k8s_cluster_raw(nodes, q).await?;
        Ok::<Value, anyhow::Error>(result)
    }.await {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_cluster_summary(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(metric_k8s_cluster_service::get_metric_k8s_cluster_summary(q).await)
}

pub async fn get_metric_k8s_cluster_cost(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(metric_k8s_cluster_service::get_metric_k8s_cluster_cost(q).await)
}

pub async fn get_metric_k8s_cluster_trend(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(metric_k8s_cluster_service::get_metric_k8s_cluster_trend(q).await)
}

pub async fn get_metric_k8s_cluster_efficiency(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    metrics_controller::to_json(metric_k8s_cluster_service::get_metric_k8s_cluster_efficiency(q).await)
}
