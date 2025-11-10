use reqwest::Client;
use serde_json::Value;
use crate::core::client::k8s::util::k8s_api_server;

pub async fn fetch_namespaces(token: &str, client: &Client) -> anyhow::Result<Value> {
    let url = format!("{}/api/v1/namespaces", k8s_api_server());
    let resp = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .error_for_status()?
        .json::<Value>()
        .await?;
    Ok(resp)
}
