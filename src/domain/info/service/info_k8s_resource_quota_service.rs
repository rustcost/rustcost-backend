use anyhow::Result;
use serde_json::Value;
use crate::core::client::k8s::client_k8s_resource_quota;
use crate::core::client::k8s::util::{build_client, read_token};

pub async fn get_k8s_resource_quotas() -> Result<Value> {
    let token = read_token()?;
    let client = build_client()?;

    let v = client_k8s_resource_quota::fetch_resource_quotas(&token, &client).await?;
    Ok(v)
}

