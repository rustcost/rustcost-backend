use crate::core::persistence::info::fixed::version::info_version_entity::InfoVersionEntity;
use anyhow::{Context, Result};
use chrono::Utc;
use reqwest::Client;
use std::env;
use crate::core::client::k8s::util::k8s_api_server;

/// Fetches Kubernetes version info from RUSTCOST_K8S_API_URL in .env
pub async fn fetch_version() -> Result<InfoVersionEntity> {

    let url = format!("{}/version", k8s_api_server());

    let client = Client::builder()
        .danger_accept_invalid_hostnames(true)
        .danger_accept_invalid_certs(true) // for dev self-signed certs
        .build()
        .context("Failed to build HTTP client")?;

    let resp = client
        .get(&url)
        .send()
        .await
        .context("Failed to request Kubernetes version endpoint")?
        .json::<serde_json::Value>()
        .await
        .context("Failed to parse version JSON")?;

    Ok(InfoVersionEntity {
        date: Utc::now().to_rfc3339(),
        major: resp["major"].as_str().unwrap_or_default().to_string(),
        minor: resp["minor"].as_str().unwrap_or_default().to_string(),
        git_version: resp["gitVersion"].as_str().unwrap_or_default().to_string(),
        git_commit: resp["gitCommit"].as_str().unwrap_or_default().to_string(),
        build_date: resp["buildDate"].as_str().unwrap_or_default().to_string(),
        go_version: resp["goVersion"].as_str().unwrap_or_default().to_string(),
        compiler: resp["compiler"].as_str().unwrap_or_default().to_string(),
        platform: resp["platform"].as_str().unwrap_or_default().to_string(),
        updated_at: Default::default(),
    })
}
