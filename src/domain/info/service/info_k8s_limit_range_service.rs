use anyhow::Result;
use serde_json::Value;
use crate::core::client::k8s::client_k8s_limit_range;
use crate::core::client::k8s::util::{build_client, read_token};

pub async fn get_k8s_limit_ranges() -> Result<Value> {
    let token = read_token()?;
    let client = build_client()?;

    let v = client_k8s_limit_range::fetch_limit_ranges(&token, &client).await?;
    Ok(v)
}

