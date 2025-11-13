use anyhow::Result;
use serde_json::{json, Value};

use crate::api::dto::metrics_dto::RangeQuery;

fn not_implemented_payload(endpoint: &str) -> Value {
    json!({
        "status": "not_implemented",
        "endpoint": endpoint,
    })
}

pub async fn get_metric_k8s_deployments_raw(_q: RangeQuery) -> Result<Value> {
    Ok(not_implemented_payload("get_metric_k8s_deployments_raw"))
}

pub async fn get_metric_k8s_deployments_raw_summary(_q: RangeQuery) -> Result<Value> {
    Ok(not_implemented_payload("get_metric_k8s_deployments_raw_summary"))
}

pub async fn get_metric_k8s_deployments_raw_efficiency(_q: RangeQuery) -> Result<Value> {
    Ok(not_implemented_payload("get_metric_k8s_deployments_raw_efficiency"))
}

pub async fn get_metric_k8s_deployment_raw(deployment: String, _q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "not_implemented",
        "endpoint": "get_metric_k8s_deployment_raw",
        "deployment": deployment,
    }))
}

pub async fn get_metric_k8s_deployment_raw_summary(deployment: String, _q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "not_implemented",
        "endpoint": "get_metric_k8s_deployment_raw_summary",
        "deployment": deployment,
    }))
}

pub async fn get_metric_k8s_deployment_raw_efficiency(deployment: String, _q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "not_implemented",
        "endpoint": "get_metric_k8s_deployment_raw_efficiency",
        "deployment": deployment,
    }))
}

pub async fn get_metric_k8s_deployments_cost(_q: RangeQuery) -> Result<Value> {
    Ok(not_implemented_payload("get_metric_k8s_deployments_cost"))
}

pub async fn get_metric_k8s_deployments_cost_summary(_q: RangeQuery) -> Result<Value> {
    Ok(not_implemented_payload("get_metric_k8s_deployments_cost_summary"))
}

pub async fn get_metric_k8s_deployments_cost_trend(_q: RangeQuery) -> Result<Value> {
    Ok(not_implemented_payload("get_metric_k8s_deployments_cost_trend"))
}

pub async fn get_metric_k8s_deployment_cost(deployment: String, _q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "not_implemented",
        "endpoint": "get_metric_k8s_deployment_cost",
        "deployment": deployment,
    }))
}

pub async fn get_metric_k8s_deployment_cost_summary(deployment: String, _q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "not_implemented",
        "endpoint": "get_metric_k8s_deployment_cost_summary",
        "deployment": deployment,
    }))
}

pub async fn get_metric_k8s_deployment_cost_trend(deployment: String, _q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "not_implemented",
        "endpoint": "get_metric_k8s_deployment_cost_trend",
        "deployment": deployment,
    }))
}
