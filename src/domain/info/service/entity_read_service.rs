use anyhow::Result;
use serde_json::{json, Value};

pub async fn get_node_info(node_name: String) -> Result<Value> {
    Ok(json!({"node": node_name}))
}

pub async fn get_pod_info(pod_uid: String) -> Result<Value> {
    Ok(json!({"pod": pod_uid}))
}

pub async fn get_container_info(id: String) -> Result<Value> {
    Ok(json!({"container": id}))
}

