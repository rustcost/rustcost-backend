use crate::core::persistence::info::fixed::version::info_version_entity::InfoVersionEntity;
use anyhow::{Context, Result};
use chrono::Utc;
use reqwest::Client;
use std::env;

/// Fetches Kubernetes version info from API_SERVER in .env
pub async fn fetch_version() -> Result<InfoVersionEntity> {
    // Load from environment (dotenv or process env)
    dotenvy::dotenv().ok();
    let api_server =
        env::var("API_SERVER").unwrap_or_else(|_| "https://127.0.0.1:6443".to_string());
    let url = format!("{}/version", api_server.trim_end_matches('/'));

    let client = Client::builder()
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
