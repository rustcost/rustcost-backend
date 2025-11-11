use anyhow::Result;
use serde_json::Value;
use crate::core::client::k8s::client_k8s_persistent_volume_claim;
use crate::core::client::k8s::util::{build_client, read_token};

pub async fn get_k8s_persistent_volume_claims() -> Result<Value> {
    let token = read_token()?;
    let client = build_client()?;

    let v = client_k8s_persistent_volume_claim::fetch_persistent_volume_claims(&token, &client).await?;
    Ok(v)
}

