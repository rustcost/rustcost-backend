use anyhow::Result;
use serde_json::{json, Value};

pub async fn health() -> Result<Value> {
    Ok(json!({"healthy": true}))
}

