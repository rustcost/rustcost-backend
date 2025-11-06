use anyhow::Result;
use serde_json::{json, Value};
use crate::domain::common::model::RangeParams;

pub async fn cost_nodes(_range: &RangeParams) -> Result<Value> {
    Ok(json!({"total_cost": 0.0, "currency": "USD"}))
}

pub async fn cost_pods(_range: &RangeParams) -> Result<Value> {
    Ok(json!({"total_cost": 0.0, "currency": "USD"}))
}

