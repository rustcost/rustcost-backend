use anyhow::Result;
use serde_json::Value;
use crate::core::client::k8s::client_k8s_deployment;
use crate::core::client::k8s::util::{build_client, read_token};

pub async fn get_k8s_deployments() -> Result<Value> {
    let token = read_token()?;
    let client = build_client()?;

    let deployments = client_k8s_deployment::fetch_deployments(&token, &client).await?;
    Ok(deployments)
}

