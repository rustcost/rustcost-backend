use anyhow::Result;
use serde_json::{json, Value};

use crate::api::dto::metrics_dto::RangeQuery;

fn not_implemented_payload(endpoint: &str) -> Value {
    json!({
        "status": "not_implemented",
        "endpoint": endpoint,
    })
}

pub async fn get_metric_k8s_namespaces_raw(_q: RangeQuery) -> Result<Value> {
    Ok(not_implemented_payload("get_metric_k8s_namespaces_raw"))
}

pub async fn get_metric_k8s_namespaces_raw_summary(_q: RangeQuery) -> Result<Value> {
    Ok(not_implemented_payload("get_metric_k8s_namespaces_raw_summary"))
}

pub async fn get_metric_k8s_namespaces_raw_efficiency(_q: RangeQuery) -> Result<Value> {
    Ok(not_implemented_payload("get_metric_k8s_namespaces_raw_efficiency"))
}

pub async fn get_metric_k8s_namespace_raw(namespace: String, _q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "not_implemented",
        "endpoint": "get_metric_k8s_namespace_raw",
        "namespace": namespace,
    }))
}

pub async fn get_metric_k8s_namespace_raw_summary(namespace: String, _q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "not_implemented",
        "endpoint": "get_metric_k8s_namespace_raw_summary",
        "namespace": namespace,
    }))
}

pub async fn get_metric_k8s_namespace_raw_efficiency(namespace: String, _q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "not_implemented",
        "endpoint": "get_metric_k8s_namespace_raw_efficiency",
        "namespace": namespace,
    }))
}

pub async fn get_metric_k8s_namespaces_cost(_q: RangeQuery) -> Result<Value> {
    Ok(not_implemented_payload("get_metric_k8s_namespaces_cost"))
}

pub async fn get_metric_k8s_namespaces_cost_summary(_q: RangeQuery) -> Result<Value> {
    Ok(not_implemented_payload("get_metric_k8s_namespaces_cost_summary"))
}

pub async fn get_metric_k8s_namespaces_cost_trend(_q: RangeQuery) -> Result<Value> {
    Ok(not_implemented_payload("get_metric_k8s_namespaces_cost_trend"))
}

pub async fn get_metric_k8s_namespace_cost(namespace: String, _q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "not_implemented",
        "endpoint": "get_metric_k8s_namespace_cost",
        "namespace": namespace,
    }))
}

pub async fn get_metric_k8s_namespace_cost_summary(namespace: String, _q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "not_implemented",
        "endpoint": "get_metric_k8s_namespace_cost_summary",
        "namespace": namespace,
    }))
}

pub async fn get_metric_k8s_namespace_cost_trend(namespace: String, _q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "not_implemented",
        "endpoint": "get_metric_k8s_namespace_cost_trend",
        "namespace": namespace,
    }))
}
