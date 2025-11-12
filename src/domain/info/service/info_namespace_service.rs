use anyhow::Result;
use serde_json::Value;
use crate::core::client::k8s::client_k8s_namespace;
use crate::core::client::k8s::util::{build_client, read_token};

pub async fn get_k8s_namespaces() -> Result<Value> {
    let token = read_token()?;
    let client = build_client()?;

    let namespaces = client_k8s_namespace::fetch_namespaces(&token, &client).await?;
    Ok(namespaces)
}
