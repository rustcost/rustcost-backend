use anyhow::Result;
use serde_json::{json, Value};

pub async fn backup() -> Result<Value> {
    Ok(json!({"backup": "scheduled"}))
}

