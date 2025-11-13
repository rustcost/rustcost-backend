use axum::{
    extract::{Path, Query},
    Json,
};
use serde_json::Value;

use crate::api::dto::{ApiResponse, metrics_dto::RangeQuery};
use crate::domain::metric::k8s::deployment::service as metric_k8s_deployment_service;

pub async fn get_metric_k8s_deployments_raw(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_deployment_service::get_metric_k8s_deployments_raw(q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_deployments_raw_summary(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_deployment_service::get_metric_k8s_deployments_raw_summary(q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_deployments_raw_efficiency(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_deployment_service::get_metric_k8s_deployments_raw_efficiency(q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_deployment_raw(
    Path(deployment): Path<String>,
    Query(q): Query<RangeQuery>,
) -> Json<ApiResponse<Value>> {
    match async {
        let result =
            metric_k8s_deployment_service::get_metric_k8s_deployment_raw(deployment, q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_deployment_raw_summary(
    Path(deployment): Path<String>,
    Query(q): Query<RangeQuery>,
) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_deployment_service::get_metric_k8s_deployment_raw_summary(deployment, q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_deployment_raw_efficiency(
    Path(deployment): Path<String>,
    Query(q): Query<RangeQuery>,
) -> Json<ApiResponse<Value>> {
    match async {
        let result =
            metric_k8s_deployment_service::get_metric_k8s_deployment_raw_efficiency(deployment, q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_deployments_cost(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_deployment_service::get_metric_k8s_deployments_cost(q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_deployments_cost_summary(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_deployment_service::get_metric_k8s_deployments_cost_summary(q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_deployments_cost_trend(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_deployment_service::get_metric_k8s_deployments_cost_trend(q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_deployment_cost(
    Path(deployment): Path<String>,
    Query(q): Query<RangeQuery>,
) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_deployment_service::get_metric_k8s_deployment_cost(deployment, q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_deployment_cost_summary(
    Path(deployment): Path<String>,
    Query(q): Query<RangeQuery>,
) -> Json<ApiResponse<Value>> {
    match async {
        let result =
            metric_k8s_deployment_service::get_metric_k8s_deployment_cost_summary(deployment, q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_deployment_cost_trend(
    Path(deployment): Path<String>,
    Query(q): Query<RangeQuery>,
) -> Json<ApiResponse<Value>> {
    match async {
        let result =
            metric_k8s_deployment_service::get_metric_k8s_deployment_cost_trend(deployment, q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub use get_metric_k8s_deployment_cost as deployment_cost;
pub use get_metric_k8s_deployment_cost_summary as deployment_cost_summary;
pub use get_metric_k8s_deployment_cost_trend as deployment_cost_trend;
pub use get_metric_k8s_deployment_raw as deployment_raw;
pub use get_metric_k8s_deployment_raw_efficiency as deployment_raw_efficiency;
pub use get_metric_k8s_deployment_raw_summary as deployment_raw_summary;
pub use get_metric_k8s_deployments_cost as deployments_cost;
pub use get_metric_k8s_deployments_cost_summary as deployments_cost_summary;
pub use get_metric_k8s_deployments_cost_trend as deployments_cost_trend;
pub use get_metric_k8s_deployments_raw as deployments_raw;
pub use get_metric_k8s_deployments_raw_efficiency as deployments_raw_efficiency;
pub use get_metric_k8s_deployments_raw_summary as deployments_raw_summary;
