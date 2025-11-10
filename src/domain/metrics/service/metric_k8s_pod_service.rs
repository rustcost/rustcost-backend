use anyhow::Result;
use serde_json::{json, Value};

use crate::api::dto::metrics_dto::RangeQuery;

// ---- Pods Service ----
pub async fn pods_list(q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "ok",
        "message": "pods_list not implemented yet",
        "query": q
    }))
}

pub async fn pod_get(pod_uid: String, q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "ok",
        "message": "pod_get not implemented yet",
        "pod_uid": pod_uid,
        "query": q
    }))
}

pub async fn pods_cost(q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "ok",
        "message": "pods_cost not implemented yet",
        "query": q
    }))
}

pub async fn pod_cost(pod_uid: String, q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "ok",
        "message": "pod_cost not implemented yet",
        "pod_uid": pod_uid,
        "query": q
    }))
}

pub async fn pods_summary(q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "ok",
        "message": "pods_summary not implemented yet",
        "query": q
    }))
}

pub async fn pod_summary(pod_uid: String, q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "ok",
        "message": "pod_summary not implemented yet",
        "pod_uid": pod_uid,
        "query": q
    }))
}

pub async fn pods_trends(q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "ok",
        "message": "pods_trends not implemented yet",
        "query": q
    }))
}

pub async fn pod_trends(pod_uid: String, q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "ok",
        "message": "pod_trends not implemented yet",
        "pod_uid": pod_uid,
        "query": q
    }))
}

pub async fn pods_efficiency(q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "ok",
        "message": "pods_efficiency not implemented yet",
        "query": q
    }))
}

pub async fn pod_efficiency(pod_uid: String, q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "ok",
        "message": "pod_efficiency not implemented yet",
        "pod_uid": pod_uid,
        "query": q
    }))
}

