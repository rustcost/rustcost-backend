use anyhow::Result;
use serde_json::{json, Value};
use crate::api::dto::metrics_dto::RangeQuery;

fn not_implemented_payload(endpoint: &str) -> Value {
    json!({
        "status": "not_implemented",
        "endpoint": endpoint,
    })
}

pub async fn get_metric_k8s_pods_raw(_q: RangeQuery) -> Result<Value> {
    Ok(not_implemented_payload("get_metric_k8s_pods_raw"))
}

pub async fn get_metric_k8s_pods_raw_summary(_q: RangeQuery) -> Result<Value> {
    Ok(not_implemented_payload("get_metric_k8s_pods_raw_summary"))
}

pub async fn get_metric_k8s_pods_raw_efficiency(_q: RangeQuery) -> Result<Value> {
    Ok(not_implemented_payload("get_metric_k8s_pods_raw_efficiency"))
}

pub async fn get_metric_k8s_pod_raw(pod_uid: String, _q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "not_implemented",
        "endpoint": "get_metric_k8s_pod_raw",
        "pod_uid": pod_uid,
    }))
}

pub async fn get_metric_k8s_pod_raw_summary(pod_uid: String, _q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "not_implemented",
        "endpoint": "get_metric_k8s_pod_raw_summary",
        "pod_uid": pod_uid,
    }))
}

pub async fn get_metric_k8s_pod_raw_efficiency(pod_uid: String, _q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "not_implemented",
        "endpoint": "get_metric_k8s_pod_raw_efficiency",
        "pod_uid": pod_uid,
    }))
}

pub async fn get_metric_k8s_pods_cost(_q: RangeQuery) -> Result<Value> {
    Ok(not_implemented_payload("get_metric_k8s_pods_cost"))
}

pub async fn get_metric_k8s_pods_cost_summary(_q: RangeQuery) -> Result<Value> {
    Ok(not_implemented_payload("get_metric_k8s_pods_cost_summary"))
}

pub async fn get_metric_k8s_pods_cost_trend(_q: RangeQuery) -> Result<Value> {
    Ok(not_implemented_payload("get_metric_k8s_pods_cost_trend"))
}

pub async fn get_metric_k8s_pod_cost(pod_uid: String, _q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "not_implemented",
        "endpoint": "get_metric_k8s_pod_cost",
        "pod_uid": pod_uid,
    }))
}

pub async fn get_metric_k8s_pod_cost_summary(pod_uid: String, _q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "not_implemented",
        "endpoint": "get_metric_k8s_pod_cost_summary",
        "pod_uid": pod_uid,
    }))
}

pub async fn get_metric_k8s_pod_cost_trend(pod_uid: String, _q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "not_implemented",
        "endpoint": "get_metric_k8s_pod_cost_trend",
        "pod_uid": pod_uid,
    }))
}
