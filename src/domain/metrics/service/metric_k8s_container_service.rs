use anyhow::Result;
use serde_json::{json, Value};

use crate::api::dto::metrics_dto::RangeQuery;

// ---- Containers Service ----
pub async fn containers_list(q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "ok",
        "message": "containers_list not implemented yet",
        "query": q
    }))
}

pub async fn container_get(id: String, q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "ok",
        "message": "container_get not implemented yet",
        "id": id,
        "query": q
    }))
}

pub async fn containers_cost(q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "ok",
        "message": "containers_cost not implemented yet",
        "query": q
    }))
}

pub async fn container_cost(id: String, q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "ok",
        "message": "container_cost not implemented yet",
        "id": id,
        "query": q
    }))
}

pub async fn containers_summary(q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "ok",
        "message": "containers_summary not implemented yet",
        "query": q
    }))
}

pub async fn container_summary(id: String, q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "ok",
        "message": "container_summary not implemented yet",
        "id": id,
        "query": q
    }))
}

pub async fn containers_trends(q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "ok",
        "message": "containers_trends not implemented yet",
        "query": q
    }))
}

pub async fn container_trends(id: String, q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "ok",
        "message": "container_trends not implemented yet",
        "id": id,
        "query": q
    }))
}

pub async fn containers_efficiency(q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "ok",
        "message": "containers_efficiency not implemented yet",
        "query": q
    }))
}

pub async fn container_efficiency(id: String, q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "ok",
        "message": "container_efficiency not implemented yet",
        "id": id,
        "query": q
    }))
}

