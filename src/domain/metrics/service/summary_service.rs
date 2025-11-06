use anyhow::Result;
use serde_json::{json, Value};
use crate::domain::common::model::RangeParams;

pub async fn summarize_nodes(_range: &RangeParams) -> Result<Value> {
    Ok(json!({"avg_cpu": 0.0, "avg_mem": 0.0, "samples": 0}))
}

pub async fn summarize_pods(_range: &RangeParams) -> Result<Value> {
    Ok(json!({"avg_cpu": 0.0, "avg_mem": 0.0, "samples": 0}))
}

