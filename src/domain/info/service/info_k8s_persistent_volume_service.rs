use anyhow::Result;
use serde_json::Value;
use crate::core::client::k8s::client_k8s_persistent_volume;
use crate::core::client::k8s::util::{build_client, read_token};

pub async fn get_k8s_persistent_volumes() -> Result<Value> {
    let token = read_token()?;
    let client = build_client()?;

    let v = client_k8s_persistent_volume::fetch_persistent_volumes(&token, &client).await?;
    Ok(v)
}

