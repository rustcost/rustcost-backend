use reqwest::Client;
use serde_json::Value;
use crate::core::client::k8s::util::k8s_api_server;

pub async fn fetch_horizontal_pod_autoscalers(token: &str, client: &Client) -> anyhow::Result<Value> {
    let url = format!("{}/apis/autoscaling/v2/horizontalpodautoscalers", k8s_api_server());
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

