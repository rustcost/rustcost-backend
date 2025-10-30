/* Maps K8s API objects → internal models */

use chrono::{DateTime, Utc};
use crate::scheduler::tasks::collectors::k8s::node::node_list_dto::Node;
use crate::scheduler::tasks::collectors::k8s::summary_dto::{NetworkStats, Summary};
use anyhow::Result;
use crate::core::persistence::info::dynamic::node::info_node_entity::InfoNodeEntity;
use crate::scheduler::tasks::collectors::k8s::node::entity::NodeMetricsEntity;

pub fn map_summary_to_node_info(summary: &Summary) -> InfoNodeEntity {
    InfoNodeEntity {
        node_name: Some(summary.node.node_name.clone()),
        last_updated_info_at: Some(Utc::now().to_rfc3339().parse().unwrap()),
        ready: Some(true),
        ..Default::default() // leaves all other fields as None
    }
}

pub fn map_summary_to_metrics(summary: &Summary) -> NodeMetricsEntity {
    let n = &summary.node;

    // --- Compute summed physical network stats ---
    let (rx, tx, rx_err, tx_err) = n.network.as_ref()
        .and_then(|net| sum_network_interfaces(net))
        .unwrap_or((None, None, None, None));

    NodeMetricsEntity {
        time: Utc::now().to_rfc3339(),

        // CPU
        cpu_usage_nano_cores: n.cpu.usage_nano_cores,
        cpu_usage_core_nano_seconds: n.cpu.usage_core_nano_seconds,

        // Memory
        memory_usage_bytes: n.memory.usage_bytes,
        memory_working_set_bytes: n.memory.working_set_bytes,
        memory_rss_bytes: n.memory.rss_bytes,
        memory_page_faults: n.memory.page_faults,

        // Network (physical)
        network_physical_rx_bytes: rx,
        network_physical_tx_bytes: tx,
        network_physical_rx_errors: rx_err,
        network_physical_tx_errors: tx_err,

        // Filesystem
        fs_used_bytes: n.fs.as_ref().and_then(|x| x.used_bytes),
        fs_capacity_bytes: n.fs.as_ref().and_then(|x| x.capacity_bytes),
        fs_inodes_used: n.fs.as_ref().and_then(|x| x.inodes_used),
        fs_inodes: n.fs.as_ref().and_then(|x| x.inodes),
    }
}

fn sum_network_interfaces(net: &NetworkStats) -> Option<(Option<u64>, Option<u64>, Option<u64>, Option<u64>)> {
    net.interfaces.as_ref().map(|interfaces| {
        let (rx, tx, rx_err, tx_err) = interfaces.iter().fold((0, 0, 0, 0), |acc, iface| {
            (
                acc.0 + iface.rx_bytes.unwrap_or(0),
                acc.1 + iface.tx_bytes.unwrap_or(0),
                acc.2 + iface.rx_errors.unwrap_or(0),
                acc.3 + iface.tx_errors.unwrap_or(0),
            )
        });
        (Some(rx), Some(tx), Some(rx_err), Some(tx_err))
    })
}



/// Maps a Kubernetes Node (from /api/v1/nodes) into our DTO for persistence.
pub fn map_node_to_info_dto(node: &Node) -> Result<InfoNodeEntity> {
    let metadata = &node.metadata;
    let status = node.status.as_ref();
    let spec = node.spec.as_ref();

    let now = Utc::now().to_rfc3339();

    // Collect labels and annotations into comma-joined strings
    let label_str = metadata.labels.as_ref().map(|map| {
        map.iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join(",")
    });

    let annotation_str = metadata.annotations.as_ref().map(|map| {
        map.iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join(",")
    });

    let taints_str = spec.and_then(|s| {
        s.taints.as_ref().map(|taints| {
            taints
                .iter()
                .map(|t| format!("{}={} ({})", t.key, t.value.as_deref().unwrap_or(""), t.effect))
                .collect::<Vec<_>>()
                .join(",")
        })
    });

    // Extract addresses
    let (internal_ip, hostname) = if let Some(status) = status {
        let internal_ip = status
            .addresses
            .as_ref()
            .and_then(|a| a.iter().find(|x| x.address_type == "InternalIP"))
            .map(|x| x.address.clone());
        let hostname = status
            .addresses
            .as_ref()
            .and_then(|a| a.iter().find(|x| x.address_type == "Hostname"))
            .map(|x| x.address.clone());
        (internal_ip, hostname)
    } else {
        (None, None)
    };

    // Build DTO
    Ok(InfoNodeEntity {
        node_name: Some(metadata.name.clone()),
        node_uid: metadata.uid.clone(),
        creation_timestamp: metadata
            .creationTimestamp
            .as_ref()
            .and_then(|ts| DateTime::parse_from_rfc3339(ts).ok())
            .map(|dt| dt.with_timezone(&Utc)),
        resource_version: metadata.resourceVersion.clone(),
        last_updated_info_at: Some(now.parse()?),

        deleted: Some(false),
        last_check_deleted_count: Some(0),

        hostname,
        internal_ip,
        architecture: status.and_then(|s| s.nodeInfo.as_ref().and_then(|ni| ni.architecture.clone())),
        os_image: status.and_then(|s| s.nodeInfo.as_ref().and_then(|ni| ni.osImage.clone())),
        kernel_version: status.and_then(|s| s.nodeInfo.as_ref().and_then(|ni| ni.kernelVersion.clone())),
        kubelet_version: status.and_then(|s| s.nodeInfo.as_ref().and_then(|ni| ni.kubeletVersion.clone())),
        container_runtime: status.and_then(|s| s.nodeInfo.as_ref().and_then(|ni| ni.containerRuntimeVersion.clone())),
        operating_system: status.and_then(|s| s.nodeInfo.as_ref().and_then(|ni| ni.operatingSystem.clone())),

        cpu_capacity_cores: status
            .and_then(|s| s.capacity.as_ref().and_then(|c| c.get("cpu")))
            .and_then(|s| s.parse::<u32>().ok()),
        memory_capacity_bytes: status
            .and_then(|s| s.capacity.as_ref().and_then(|c| c.get("memory")))
            .and_then(|s| parse_quantity_to_bytes(s)),
        pod_capacity: status
            .and_then(|s| s.capacity.as_ref().and_then(|c| c.get("pods")))
            .and_then(|s| s.parse::<u32>().ok()),
        ephemeral_storage_capacity_bytes: status
            .and_then(|s| s.capacity.as_ref().and_then(|c| c.get("ephemeral-storage")))
            .and_then(|s| parse_quantity_to_bytes(s)),

        cpu_allocatable_cores: status
            .and_then(|s| s.allocatable.as_ref().and_then(|a| a.get("cpu")))
            .and_then(|s| s.parse::<u32>().ok()),
        memory_allocatable_bytes: status
            .and_then(|s| s.allocatable.as_ref().and_then(|a| a.get("memory")))
            .and_then(|s| parse_quantity_to_bytes(s)),
        ephemeral_storage_allocatable_bytes: status
            .and_then(|s| s.allocatable.as_ref().and_then(|a| a.get("ephemeral-storage")))
            .and_then(|s| parse_quantity_to_bytes(s)),
        pod_allocatable: status
            .and_then(|s| s.allocatable.as_ref().and_then(|a| a.get("pods")))
            .and_then(|s| s.parse::<u32>().ok()),

        ready: status
            .and_then(|s| s.conditions.as_ref())
            .and_then(|conds| conds.iter().find(|c| c.condition_type == "Ready"))
            .map(|c| c.status == "True"),

        taints: taints_str,
        label: label_str,
        annotation: annotation_str,

        image_count: status.and_then(|s| s.images.as_ref().map(|imgs| imgs.len() as u32)),
        image_names: status.and_then(|s| {
            s.images.as_ref().map(|imgs| {
                imgs.iter()
                    .flat_map(|img| img.names.clone())
                    .collect::<Vec<_>>()
            })
        }),
        image_total_size_bytes: status.and_then(|s| {
            s.images.as_ref().map(|imgs| {
                imgs.iter()
                    .map(|i| i.sizeBytes.unwrap_or(0))
                    .sum::<u64>()
            })
        }),
    })
}


/// Parses K8s-style memory/storage quantity strings (e.g., "8131852Ki") → bytes.
fn parse_quantity_to_bytes(input: &str) -> Option<u64> {
    let input = input.trim().to_ascii_lowercase();
    if let Some(v) = input.strip_suffix("ki") {
        v.parse::<f64>().ok().map(|n| (n * 1024.0) as u64)
    } else if let Some(v) = input.strip_suffix("mi") {
        v.parse::<f64>().ok().map(|n| (n * 1024.0 * 1024.0) as u64)
    } else if let Some(v) = input.strip_suffix("gi") {
        v.parse::<f64>().ok().map(|n| (n * 1024.0 * 1024.0 * 1024.0) as u64)
    } else if let Some(v) = input.strip_suffix("ti") {
        v.parse::<f64>().ok().map(|n| (n * 1024.0 * 1024.0 * 1024.0 * 1024.0) as u64)
    } else {
        input.parse::<u64>().ok()
    }
}
