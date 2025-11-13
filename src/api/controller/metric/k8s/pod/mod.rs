use axum::{
    extract::{Path, Query},
    Json,
};
use serde_json::Value;

use crate::api::dto::{ApiResponse, metrics_dto::RangeQuery};
use crate::domain::metric::k8s::pod::service as metric_k8s_pod_service;

pub async fn get_metric_k8s_pods_raw(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_pod_service::get_metric_k8s_pods_raw(q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_pods_raw_summary(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_pod_service::get_metric_k8s_pods_raw_summary(q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_pods_raw_efficiency(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_pod_service::get_metric_k8s_pods_raw_efficiency(q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_pod_raw(
    Path(pod_uid): Path<String>,
    Query(q): Query<RangeQuery>,
) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_pod_service::get_metric_k8s_pod_raw(pod_uid, q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_pod_raw_summary(
    Path(pod_uid): Path<String>,
    Query(q): Query<RangeQuery>,
) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_pod_service::get_metric_k8s_pod_raw_summary(pod_uid, q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_pod_raw_efficiency(
    Path(pod_uid): Path<String>,
    Query(q): Query<RangeQuery>,
) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_pod_service::get_metric_k8s_pod_raw_efficiency(pod_uid, q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_pods_cost(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_pod_service::get_metric_k8s_pods_cost(q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_pods_cost_summary(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_pod_service::get_metric_k8s_pods_cost_summary(q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_pods_cost_trend(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_pod_service::get_metric_k8s_pods_cost_trend(q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_pod_cost(
    Path(pod_uid): Path<String>,
    Query(q): Query<RangeQuery>,
) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_pod_service::get_metric_k8s_pod_cost(pod_uid, q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_pod_cost_summary(
    Path(pod_uid): Path<String>,
    Query(q): Query<RangeQuery>,
) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_pod_service::get_metric_k8s_pod_cost_summary(pod_uid, q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_pod_cost_trend(
    Path(pod_uid): Path<String>,
    Query(q): Query<RangeQuery>,
) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_pod_service::get_metric_k8s_pod_cost_trend(pod_uid, q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub use get_metric_k8s_pod_cost as pod_cost;
pub use get_metric_k8s_pod_cost_summary as pod_cost_summary;
pub use get_metric_k8s_pod_cost_trend as pod_cost_trend;
pub use get_metric_k8s_pod_raw as pod_raw;
pub use get_metric_k8s_pod_raw_efficiency as pod_raw_efficiency;
pub use get_metric_k8s_pod_raw_summary as pod_raw_summary;
pub use get_metric_k8s_pods_cost as pods_cost;
pub use get_metric_k8s_pods_cost_summary as pods_cost_summary;
pub use get_metric_k8s_pods_cost_trend as pods_cost_trend;
pub use get_metric_k8s_pods_raw as pods_raw;
pub use get_metric_k8s_pods_raw_efficiency as pods_raw_efficiency;
pub use get_metric_k8s_pods_raw_summary as pods_raw_summary;
