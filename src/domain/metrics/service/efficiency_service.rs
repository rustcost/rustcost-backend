use anyhow::Result;
use serde_json::{json, Value};
use crate::domain::common::model::RangeParams;

pub async fn efficiency_nodes(_range: &RangeParams) -> Result<Value> {
    Ok(json!({"score": 0.0}))
}

pub async fn efficiency_pods(_range: &RangeParams) -> Result<Value> {
    Ok(json!({"score": 0.0}))
}

