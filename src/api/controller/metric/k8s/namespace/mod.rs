use axum::{
    extract::{Path, Query},
    Json,
};
use serde_json::Value;

use crate::api::dto::{ApiResponse, metrics_dto::RangeQuery};
use crate::domain::metric::k8s::namespace::service as metric_k8s_namespace_service;

pub async fn get_metric_k8s_namespaces_raw(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_namespace_service::get_metric_k8s_namespaces_raw(q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_namespaces_raw_summary(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_namespace_service::get_metric_k8s_namespaces_raw_summary(q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_namespaces_raw_efficiency(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_namespace_service::get_metric_k8s_namespaces_raw_efficiency(q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_namespace_raw(
    Path(namespace): Path<String>,
    Query(q): Query<RangeQuery>,
) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_namespace_service::get_metric_k8s_namespace_raw(namespace, q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_namespace_raw_summary(
    Path(namespace): Path<String>,
    Query(q): Query<RangeQuery>,
) -> Json<ApiResponse<Value>> {
    match async {
        let result =
            metric_k8s_namespace_service::get_metric_k8s_namespace_raw_summary(namespace, q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_namespace_raw_efficiency(
    Path(namespace): Path<String>,
    Query(q): Query<RangeQuery>,
) -> Json<ApiResponse<Value>> {
    match async {
        let result =
            metric_k8s_namespace_service::get_metric_k8s_namespace_raw_efficiency(namespace, q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_namespaces_cost(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_namespace_service::get_metric_k8s_namespaces_cost(q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_namespaces_cost_summary(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_namespace_service::get_metric_k8s_namespaces_cost_summary(q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_namespaces_cost_trend(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_namespace_service::get_metric_k8s_namespaces_cost_trend(q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_namespace_cost(
    Path(namespace): Path<String>,
    Query(q): Query<RangeQuery>,
) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_namespace_service::get_metric_k8s_namespace_cost(namespace, q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_namespace_cost_summary(
    Path(namespace): Path<String>,
    Query(q): Query<RangeQuery>,
) -> Json<ApiResponse<Value>> {
    match async {
        let result =
            metric_k8s_namespace_service::get_metric_k8s_namespace_cost_summary(namespace, q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_namespace_cost_trend(
    Path(namespace): Path<String>,
    Query(q): Query<RangeQuery>,
) -> Json<ApiResponse<Value>> {
    match async {
        let result =
            metric_k8s_namespace_service::get_metric_k8s_namespace_cost_trend(namespace, q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub use get_metric_k8s_namespace_cost as namespace_cost;
pub use get_metric_k8s_namespace_cost_summary as namespace_cost_summary;
pub use get_metric_k8s_namespace_cost_trend as namespace_cost_trend;
pub use get_metric_k8s_namespace_raw as namespace_raw;
pub use get_metric_k8s_namespace_raw_efficiency as namespace_raw_efficiency;
pub use get_metric_k8s_namespace_raw_summary as namespace_raw_summary;
pub use get_metric_k8s_namespaces_cost as namespaces_cost;
pub use get_metric_k8s_namespaces_cost_summary as namespaces_cost_summary;
pub use get_metric_k8s_namespaces_cost_trend as namespaces_cost_trend;
pub use get_metric_k8s_namespaces_raw as namespaces_raw;
pub use get_metric_k8s_namespaces_raw_efficiency as namespaces_raw_efficiency;
pub use get_metric_k8s_namespaces_raw_summary as namespaces_raw_summary;
