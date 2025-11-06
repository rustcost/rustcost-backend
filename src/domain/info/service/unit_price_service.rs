use anyhow::Result;
use serde_json::{json, Value};

pub async fn get_unit_prices() -> Result<Value> {
    Ok(json!({"unit_prices": []}))
}

pub async fn upsert_unit_prices() -> Result<Value> {
    Ok(json!({"updated": true}))
}

