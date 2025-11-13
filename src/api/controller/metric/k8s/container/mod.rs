use axum::{
    extract::{Path, Query},
    Json,
};
use serde_json::Value;

use crate::api::dto::{ApiResponse, metrics_dto::RangeQuery};
use crate::domain::metric::k8s::container::service as metric_k8s_container_service;

pub async fn get_metric_k8s_containers_raw(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_container_service::get_metric_k8s_containers_raw(q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_containers_raw_summary(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_container_service::get_metric_k8s_containers_raw_summary(q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_containers_raw_efficiency(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_container_service::get_metric_k8s_containers_raw_efficiency(q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_container_raw(
    Path(id): Path<String>,
    Query(q): Query<RangeQuery>,
) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_container_service::get_metric_k8s_container_raw(id, q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_container_raw_summary(
    Path(id): Path<String>,
    Query(q): Query<RangeQuery>,
) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_container_service::get_metric_k8s_container_raw_summary(id, q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_container_raw_efficiency(
    Path(id): Path<String>,
    Query(q): Query<RangeQuery>,
) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_container_service::get_metric_k8s_container_raw_efficiency(id, q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_containers_cost(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_container_service::get_metric_k8s_containers_cost(q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_containers_cost_summary(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_container_service::get_metric_k8s_containers_cost_summary(q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_containers_cost_trend(Query(q): Query<RangeQuery>) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_container_service::get_metric_k8s_containers_cost_trend(q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_container_cost(
    Path(id): Path<String>,
    Query(q): Query<RangeQuery>,
) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_container_service::get_metric_k8s_container_cost(id, q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_container_cost_summary(
    Path(id): Path<String>,
    Query(q): Query<RangeQuery>,
) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_container_service::get_metric_k8s_container_cost_summary(id, q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub async fn get_metric_k8s_container_cost_trend(
    Path(id): Path<String>,
    Query(q): Query<RangeQuery>,
) -> Json<ApiResponse<Value>> {
    match async {
        let result = metric_k8s_container_service::get_metric_k8s_container_cost_trend(id, q).await?;
        Ok::<Value, anyhow::Error>(result)
    }
    .await
    {
        Ok(v) => Json(ApiResponse::ok(v)),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

pub use get_metric_k8s_container_cost as container_cost;
pub use get_metric_k8s_container_cost_summary as container_cost_summary;
pub use get_metric_k8s_container_cost_trend as container_cost_trend;
pub use get_metric_k8s_container_raw as container_raw;
pub use get_metric_k8s_container_raw_efficiency as container_raw_efficiency;
pub use get_metric_k8s_container_raw_summary as container_raw_summary;
pub use get_metric_k8s_containers_cost as containers_cost;
pub use get_metric_k8s_containers_cost_summary as containers_cost_summary;
pub use get_metric_k8s_containers_cost_trend as containers_cost_trend;
pub use get_metric_k8s_containers_raw as containers_raw;
pub use get_metric_k8s_containers_raw_efficiency as containers_raw_efficiency;
pub use get_metric_k8s_containers_raw_summary as containers_raw_summary;
