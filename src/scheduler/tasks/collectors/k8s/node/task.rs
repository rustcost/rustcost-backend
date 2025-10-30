use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::Command;
use anyhow::{anyhow, Context};
use crate::scheduler::tasks::collectors::k8s::node::api::{fetch_nodes, fetch_nodes_json, get_latest_resource_version};
use crate::scheduler::tasks::collectors::k8s::summary_dto::Summary;
use crate::scheduler::tasks::collectors::k8s::node::mapper::{map_node_to_info_dto, map_summary_to_metrics, map_summary_to_node_info};
use crate::scheduler::tasks::collectors::k8s::node::node_list_dto::NodeList;
use crate::scheduler::tasks::collectors::k8s::node::metric_node_collector_repository::{append_metrics};
use std::io::Write;
use reqwest::Client;
use tracing::log::debug;
use crate::core::persistence::info::dynamic::info_dynamic_fs_adapter_trait::InfoDynamicFsAdapterTrait;
use crate::core::persistence::info::dynamic::node::info_node_collector_repository_trait::InfoNodeCollectorRepository;
use crate::core::persistence::info::dynamic::node::info_node_entity::InfoNodeEntity;
use crate::scheduler::tasks::collectors::k8s::node::info_node_collector_repository::InfoNodeCollectorRepositoryImpl;

pub async fn handle_node(summary: &Summary) -> Result<bool, anyhow::Error> {
    let node_name = &summary.node.node_name;
    let repo = InfoNodeCollectorRepositoryImpl::default();

    // Step 1: Write info.rci if missing
    let node_info = map_summary_to_node_info(summary);
    let created = repo.create_if_missing(node_name, &node_info)?;

    // Step 2: Append metrics
    let metrics_dto = map_summary_to_metrics(summary);
    append_metrics(node_name, &metrics_dto)?;

    Ok(created)
}
/// Keeps track of last known cluster resourceVersion in-memory.
/// You can later persist it to DB if needed.
static mut LAST_RESOURCE_VERSION: Option<String> = None;

/// Checks cluster nodes and updates node info files if any node is new or changed.
pub async fn update_node_infos(token: &str, client: &Client, updated_nodes:&Vec<String>) -> Result<NodeList, anyhow::Error> {
    let latest_version = get_latest_resource_version(token, client).await?;

    // SAFETY: this is a single-threaded mutable global (ok in async context if used carefully)
    let should_fetch = unsafe {
        match &LAST_RESOURCE_VERSION {
            None => {
                debug!("No stored resourceVersion — fetching full NodeList");
                LAST_RESOURCE_VERSION = Some(latest_version.clone());
                true
            }
            Some(prev) if *prev != latest_version => {
                debug!(
                    "ResourceVersion changed (old={}, new={}) → refreshing NodeList",
                    prev, latest_version
                );
                LAST_RESOURCE_VERSION = Some(latest_version.clone());
                true
            }
            Some(prev) => {
                debug!("ResourceVersion unchanged ({}) → skipping fetch", prev);
                false
            }
        }
    };

    if should_fetch {
        let node_list = fetch_nodes(token, client).await?;
        Ok(node_list)
    } else {
        Ok(NodeList {
            apiVersion: None,
            kind: Some("NodeList".to_string()),
            items: vec![],
            metadata: None,
        })
    }
}

/// Loads cached versions from data/nodes/.versions file
fn load_cached_versions(path: &str) -> anyhow::Result<HashMap<String, String>> {
    if !Path::new(path).exists() {
        return Ok(HashMap::new());
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut map = HashMap::new();

    for line in reader.lines().flatten() {
        let parts: Vec<_> = line.split_whitespace().collect();
        if parts.len() == 2 {
            map.insert(parts[0].to_string(), parts[1].to_string());
        }
    }
    Ok(map)
}

/// Saves the latest resourceVersions back to .versions
fn save_versions(path: &str, versions: &HashMap<String, String>) -> anyhow::Result<()> {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = File::create(path)?;
    for (name, version) in versions {
        writeln!(file, "{} {}", name, version)?;
    }
    Ok(())
}

/// Gets node name + resourceVersion from cluster using kubectl (lightweight)
fn get_current_resource_versions() -> anyhow::Result<HashMap<String, String>> {
    let output = Command::new("kubectl")
        .args([
            "get",
            "nodes",
            "-o",
            "jsonpath={range .items[*]}{.metadata.name} {.metadata.resourceVersion}{\"\\n\"}{end}",
        ])
        .output()
        .context("Failed to execute kubectl get nodes")?;

    if !output.status.success() {
        return Err(anyhow!(
            "kubectl command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut map = HashMap::new();

    for line in stdout.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        if parts.len() == 2 {
            map.insert(parts[0].to_string(), parts[1].to_string());
        }
    }

    Ok(map)
}


