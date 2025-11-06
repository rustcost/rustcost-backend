use anyhow::Result;
use serde_json::{json, Value};
use crate::domain::common::model::RangeParams;

pub async fn trends_nodes(_range: &RangeParams) -> Result<Value> {
    Ok(json!({"points": []}))
}

pub async fn trends_pods(_range: &RangeParams) -> Result<Value> {
    Ok(json!({"points": []}))
}

