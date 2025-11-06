use anyhow::Result;
use serde_json::{json, Value};

pub async fn status() -> Result<Value> {
    Ok(json!({"status": "ok"}))
}

