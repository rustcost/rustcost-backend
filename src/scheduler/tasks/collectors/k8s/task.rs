use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::Command;
use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use reqwest::Client;
use tracing::{debug, error, info};
use serde_json::to_string_pretty;
use crate::core::kube_client::{api_server, build_client, read_token};
use crate::scheduler::tasks::collectors::k8s::node::mapper::{ map_summary_to_metrics, map_summary_to_node_info};
use crate::scheduler::tasks::collectors::k8s::summary_dto::Summary;
use crate::scheduler::tasks::collectors::k8s::node::client::{fetch_node_summary, fetch_nodes};
use crate::scheduler::tasks::collectors::k8s::node::task::{handle_node, update_node_info};
use crate::scheduler::tasks::collectors::k8s::pod::task::handle_pod;

/// Collects node-level stats from the Kubelet `/stats/summary` endpoint.
pub async fn run() -> Result<()> {
    debug!("Starting K8s node stats task...");

    // --- Build client & token ---
    let token = read_token()?;
    let client = build_client()?;

    // --- Step 1: Fetch all nodes ---
    let node_list = fetch_nodes(&token, &client).await?;

    // --- Step 2: For each node, call /proxy/stats/summary ---
    for node in node_list.items {
        let node_name = node.metadata.name.clone();

        match fetch_node_summary(&token, &client, &node_name).await {
            Ok(summary) => {
                match handle_summary(&summary).await {
                    Ok(result) => {

                        // if new node
                        if let Some(name) = result.node_name {
                            update_node_info(node).await?;
                        }
                        // new_pods.extend(result.updated_pods);
                        // new_containers.extend(result.updated_containers);
                    }
                    Err(e) => error!("❌ Failed to handle summary for {}: {:?}", node_name, e),
                }
            }
            Err(e) => {
                error!("❌ Failed to fetch summary for {}: {:?}", node_name, e);
            }
        }
    }
    Ok(())
}

#[derive(Debug, Default)]
pub struct SummaryHandleResultDto {
    pub node_name: Option<String>,
    pub updated_pods: Vec<String>,
    pub updated_containers: Vec<String>,
}


/// Handle and persist one `/stats/summary` response
pub async fn handle_summary(summary: &Summary) -> Result<SummaryHandleResultDto> {
    let mut result = SummaryHandleResultDto::default();

    if handle_node(summary).await? {
        result.node_name = Some(summary.node.node_name.clone());
    }

    handle_pod(summary).await?;

    // result.updated_pods = pod::handle_pod(summary).await?;
    // result.updated_containers = container::handle_container(summary).await?;

    Ok(result)
}

/* ---------------- Tests ---------------- */

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_subscriber::{fmt, EnvFilter};

    #[test]
    fn test_run_does_not_panic() {
        // Initialize full tracing (only once)
        let _ = fmt()
            .with_env_filter(EnvFilter::new("debug")) // show debug/info/warn/error
            .with_target(true)
            .with_level(true)
            .with_test_writer()
            .try_init();

        // Build a single-threaded Tokio runtime
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed to build tokio runtime");

        // Run async code inside the same thread (so debugger can attach)
        rt.block_on(async {
            let result = run().await;
            // Allow both Ok and Err but ensure no panic
            assert!(result.is_ok() || result.is_err());
        });
    }
}
