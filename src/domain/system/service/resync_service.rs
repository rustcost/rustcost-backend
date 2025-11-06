use anyhow::Result;
use serde_json::{json, Value};

pub async fn resync() -> Result<Value> {
    Ok(json!({"resync": "started"}))
}

