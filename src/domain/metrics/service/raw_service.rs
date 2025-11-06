use anyhow::Result;
use serde_json::{json, Value};
use crate::domain::common::model::RangeParams;

pub async fn list_nodes(_range: &RangeParams) -> Result<Value> {
    Ok(json!([]))
}

pub async fn get_node(_node_name: &str, _range: &RangeParams) -> Result<Value> {
    Ok(json!({}))
}

