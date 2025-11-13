use axum::{
    extract::{Path, Query},
    Json,
};
use serde_json::Value;

use crate::api::dto::{ApiResponse, metrics_dto::RangeQuery};
use crate::domain::metric::k8s::node::service as metric_k8s_node_service;

pub async fn get_metric_k8s_nodes_raw(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_node_service::get_metric_k8s_nodes_raw(q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_nodes_raw_summary(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_node_service::get_metric_k8s_nodes_raw_summary(q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_nodes_raw_efficiency(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_node_service::get_metric_k8s_nodes_raw_efficiency(q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_node_raw(
    Path(node_name): Path<String>,
    Query(q): Query<RangeQuery>,
) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_node_service::get_metric_k8s_node_raw(node_name, q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_node_raw_summary(
    Path(node_name): Path<String>,
    Query(q): Query<RangeQuery>,
) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_node_service::get_metric_k8s_node_raw_summary(node_name, q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_node_raw_efficiency(
    Path(node_name): Path<String>,
    Query(q): Query<RangeQuery>,
) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_node_service::get_metric_k8s_node_raw_efficiency(node_name, q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_nodes_cost(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_node_service::get_metric_k8s_nodes_cost(q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_nodes_cost_summary(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_node_service::get_metric_k8s_nodes_cost_summary(q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_nodes_cost_trend(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_node_service::get_metric_k8s_nodes_cost_trend(q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_node_cost(
    Path(node_name): Path<String>,
    Query(q): Query<RangeQuery>,
) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_node_service::get_metric_k8s_node_cost(node_name, q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_node_cost_summary(
    Path(node_name): Path<String>,
    Query(q): Query<RangeQuery>,
) -> Json<ApiResponse<Value>> {
    match async {
        let result =
            metric_k8s_node_service::get_metric_k8s_node_cost_summary(node_name, q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_node_cost_trend(
    Path(node_name): Path<String>,
    Query(q): Query<RangeQuery>,
) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_node_service::get_metric_k8s_node_cost_trend(node_name, q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub use get_metric_k8s_node_cost as node_cost;
pub use get_metric_k8s_node_cost_summary as node_cost_summary;
pub use get_metric_k8s_node_cost_trend as node_cost_trend;
pub use get_metric_k8s_node_raw as node_raw;
pub use get_metric_k8s_node_raw_efficiency as node_raw_efficiency;
pub use get_metric_k8s_node_raw_summary as node_raw_summary;
pub use get_metric_k8s_nodes_cost as nodes_cost;
pub use get_metric_k8s_nodes_cost_summary as nodes_cost_summary;
pub use get_metric_k8s_nodes_cost_trend as nodes_cost_trend;
pub use get_metric_k8s_nodes_raw as nodes_raw;
pub use get_metric_k8s_nodes_raw_efficiency as nodes_raw_efficiency;
pub use get_metric_k8s_nodes_raw_summary as nodes_raw_summary;
