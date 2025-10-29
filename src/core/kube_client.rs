/* Builds API client (token, cert, base URL) */

use reqwest::{Certificate, Client};
use std::{env, fs};

/// Reads the service account token (mounted in pod)
pub fn read_token() -> anyhow::Result<String> {
    let path = env::var("TOKEN_PATH")
        .unwrap_or_else(|_| "/var/run/secrets/kubernetes.io/serviceaccount/token".to_string());
    let token = fs::read_to_string(&path)?;
    Ok(token.trim().to_string())
}

/// Builds a reqwest client with CA cert for in-cluster HTTPS
pub fn build_client() -> anyhow::Result<Client> {
    // default path for in-cluster service account
    let default_ca = "/var/run/secrets/kubernetes.io/serviceaccount/ca.crt".to_string();

    let ca_path = env::var("CA_PATH").unwrap_or(default_ca.clone());
    let pem = fs::read(&ca_path)?;
    let ca = Certificate::from_pem(&pem)?;

    // Determine if we're running locally (Windows / dev)
    let is_local = ca_path.ends_with("data/ca.crt");

    let builder = Client::builder().add_root_certificate(ca);

    // In dev mode, allow CN mismatch for 127.0.0.1 tunnels
    let client = if is_local {
        builder
            .danger_accept_invalid_certs(false)
            .danger_accept_invalid_hostnames(true)
            .build()?
    } else {
        builder.build()?
    };

    Ok(client)
}


/// Returns API server URL (in-cluster or overridden)
pub fn api_server() -> String {
    env::var("API_SERVER").unwrap_or_else(|_| "https://kubernetes.default.svc".to_string())
}
