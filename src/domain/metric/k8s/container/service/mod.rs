use anyhow::Result;
use serde_json::{json, Value};
use crate::api::dto::metrics_dto::RangeQuery;

fn not_implemented_payload(endpoint: &str) -> Value {
    json!({
        "status": "not_implemented",
        "endpoint": endpoint,
    })
}

pub async fn get_metric_k8s_containers_raw(_q: RangeQuery) -> Result<Value> {
    Ok(not_implemented_payload("get_metric_k8s_containers_raw"))
}

pub async fn get_metric_k8s_containers_raw_summary(_q: RangeQuery) -> Result<Value> {
    Ok(not_implemented_payload("get_metric_k8s_containers_raw_summary"))
}

pub async fn get_metric_k8s_containers_raw_efficiency(_q: RangeQuery) -> Result<Value> {
    Ok(not_implemented_payload("get_metric_k8s_containers_raw_efficiency"))
}

pub async fn get_metric_k8s_container_raw(id: String, _q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "not_implemented",
        "endpoint": "get_metric_k8s_container_raw",
        "container_id": id,
    }))
}

pub async fn get_metric_k8s_container_raw_summary(id: String, _q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "not_implemented",
        "endpoint": "get_metric_k8s_container_raw_summary",
        "container_id": id,
    }))
}

pub async fn get_metric_k8s_container_raw_efficiency(id: String, _q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "not_implemented",
        "endpoint": "get_metric_k8s_container_raw_efficiency",
        "container_id": id,
    }))
}

pub async fn get_metric_k8s_containers_cost(_q: RangeQuery) -> Result<Value> {
    Ok(not_implemented_payload("get_metric_k8s_containers_cost"))
}

pub async fn get_metric_k8s_containers_cost_summary(_q: RangeQuery) -> Result<Value> {
    Ok(not_implemented_payload("get_metric_k8s_containers_cost_summary"))
}

pub async fn get_metric_k8s_containers_cost_trend(_q: RangeQuery) -> Result<Value> {
    Ok(not_implemented_payload("get_metric_k8s_containers_cost_trend"))
}

pub async fn get_metric_k8s_container_cost(id: String, _q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "not_implemented",
        "endpoint": "get_metric_k8s_container_cost",
        "container_id": id,
    }))
}

pub async fn get_metric_k8s_container_cost_summary(id: String, _q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "not_implemented",
        "endpoint": "get_metric_k8s_container_cost_summary",
        "container_id": id,
    }))
}

pub async fn get_metric_k8s_container_cost_trend(id: String, _q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "not_implemented",
        "endpoint": "get_metric_k8s_container_cost_trend",
        "container_id": id,
    }))
}
