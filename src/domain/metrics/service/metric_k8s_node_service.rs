use anyhow::Result;
use serde_json::{json, Value};

use crate::api::dto::metrics_dto::RangeQuery;

// ---- Nodes Service ----
pub async fn nodes_list(q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "ok",
        "message": "nodes_list not implemented yet",
        "query": q
    }))
}

pub async fn node_get(node_name: String, q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "ok",
        "message": "node_get not implemented yet",
        "node_name": node_name,
        "query": q
    }))
}

pub async fn nodes_cost(q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "ok",
        "message": "nodes_cost not implemented yet",
        "query": q
    }))
}

pub async fn node_cost(node_name: String, q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "ok",
        "message": "node_cost not implemented yet",
        "node_name": node_name,
        "query": q
    }))
}

pub async fn nodes_summary(q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "ok",
        "message": "nodes_summary not implemented yet",
        "query": q
    }))
}

pub async fn node_summary(node_name: String, q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "ok",
        "message": "node_summary not implemented yet",
        "node_name": node_name,
        "query": q
    }))
}

pub async fn nodes_trends(q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "ok",
        "message": "nodes_trends not implemented yet",
        "query": q
    }))
}

pub async fn node_trends(node_name: String, q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "ok",
        "message": "node_trends not implemented yet",
        "node_name": node_name,
        "query": q
    }))
}

pub async fn nodes_efficiency(q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "ok",
        "message": "nodes_efficiency not implemented yet",
        "query": q
    }))
}

pub async fn node_efficiency(node_name: String, q: RangeQuery) -> Result<Value> {
    Ok(json!({
        "status": "ok",
        "message": "node_efficiency not implemented yet",
        "node_name": node_name,
        "query": q
    }))
}

