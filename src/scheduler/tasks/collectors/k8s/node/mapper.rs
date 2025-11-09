/* Maps K8s API objects → internal models */

use crate::core::persistence::info::k8s::node::info_node_entity::InfoNodeEntity;
use crate::core::persistence::metrics::k8s::node::metric_node_entity::MetricNodeEntity;
use crate::core::client::k8s::client_k8s_node_dto::Node;
use crate::scheduler::tasks::collectors::k8s::summary_dto::{NetworkStats, Summary};
use anyhow::Result;
use chrono::{DateTime, Utc};
use std::str::FromStr;

pub fn map_summary_to_node_info(summary: &Summary) -> InfoNodeEntity {
    InfoNodeEntity {
        node_name: Some(summary.node.node_name.clone()),
        last_updated_info_at: Some(Utc::now().to_rfc3339().parse().unwrap()),
        ready: Some(true),
        ..Default::default() // leaves all other fields as None
    }
}

pub fn map_summary_to_metrics(summary: &Summary) -> MetricNodeEntity {
    let n = &summary.node;

    // --- Compute summed physical network stats ---
    let (rx, tx, rx_err, tx_err) = n.network.as_ref()
        .and_then(|net| sum_network_interfaces(net))
        .unwrap_or((None, None, None, None));

    MetricNodeEntity {
        time: Utc::now(),

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




/// Converts a Kubernetes `Node` object into an `InfoNodeEntity`.
pub fn map_node_to_node_info_entity(node: &Node) -> Result<InfoNodeEntity> {
    let metadata = &node.metadata;
    let status = node.status.as_ref();
    let spec = node.spec.as_ref();

    // Parse creation timestamp (if exists)
    let creation_timestamp = metadata
        .creation_timestamp
        .as_ref()
        .and_then(|ts| DateTime::from_str(ts).ok());

    // Extract addresses (hostname, internal IP)
    let (hostname, internal_ip) = status
        .and_then(|s| s.addresses.as_ref())
        .map(|addresses| {
            let mut hostname = None;
            let mut internal_ip = None;
            for addr in addresses {
                match addr.address_type.as_str() {
                    "Hostname" => hostname = Some(addr.address.clone()),
                    "InternalIP" => internal_ip = Some(addr.address.clone()),
                    _ => {}
                }
            }
            (hostname, internal_ip)
        })
        .unwrap_or_default();

    // Extract NodeSystemInfo
    let sys_info = status.and_then(|s| s.node_info.as_ref());
    let (architecture, os_image, kernel_version, kubelet_version, container_runtime, operating_system) =
        sys_info
            .map(|info| {
                (
                    info.architecture.clone(),
                    info.os_image.clone(),
                    info.kernel_version.clone(),
                    info.kubelet_version.clone(),
                    info.container_runtime_version.clone(),
                    info.operating_system.clone(),
                )
            })
            .unwrap_or_default();

    // Parse capacities and allocatables
    let capacity = status.and_then(|s| s.capacity.as_ref());
    let allocatable = status.and_then(|s| s.allocatable.as_ref());

    let parse_cpu = |v: Option<&String>| {
        v.and_then(|s| s.trim_end_matches('m').parse::<u32>().ok())
            .map(|millicores| millicores / 1000)
    };
    let parse_mem = |v: Option<&String>| {
        v.and_then(|s| {
            let s = s.to_lowercase();
            if s.ends_with("ki") {
                s.trim_end_matches("ki").parse::<u64>().ok().map(|v| v * 1024)
            } else if s.ends_with('k') {
                s.trim_end_matches('k').parse::<u64>().ok().map(|v| v * 1000)
            } else if s.ends_with("mi") {
                s.trim_end_matches("mi").parse::<u64>().ok().map(|v| v * 1024 * 1024)
            } else if s.ends_with('m') {
                s.trim_end_matches('m').parse::<u64>().ok().map(|v| v * 1000 * 1000)
            } else if s.ends_with("gi") {
                s.trim_end_matches("gi").parse::<u64>().ok().map(|v| v * 1024 * 1024 * 1024)
            } else {
                s.parse::<u64>().ok()
            }
        })
    };

    let cpu_capacity_cores = parse_cpu(capacity.and_then(|c| c.get("cpu")));
    let memory_capacity_bytes = parse_mem(capacity.and_then(|c| c.get("memory")));
    let pod_capacity = capacity
        .and_then(|c| c.get("pods"))
        .and_then(|v| v.parse::<u32>().ok());
    let ephemeral_storage_capacity_bytes =
        parse_mem(capacity.and_then(|c| c.get("ephemeral-storage")));

    let cpu_allocatable_cores = parse_cpu(allocatable.and_then(|a| a.get("cpu")));
    let memory_allocatable_bytes = parse_mem(allocatable.and_then(|a| a.get("memory")));
    let pod_allocatable = allocatable
        .and_then(|a| a.get("pods"))
        .and_then(|v| v.parse::<u32>().ok());
    let ephemeral_storage_allocatable_bytes =
        parse_mem(allocatable.and_then(|a| a.get("ephemeral-storage")));

    // Determine readiness
    let ready = status
        .and_then(|s| s.conditions.as_ref())
        .and_then(|conds| {
            conds.iter()
                .find(|c| c.condition_type == "Ready")
                .map(|c| c.status == "True")
        });

    // Serialize taints, labels, annotations
    let taints = spec
        .and_then(|s| s.taints.as_ref())
        .map(|t| {
            t.iter()
                .map(|t| format!("{}={} ({})", t.key, t.value.clone().unwrap_or_default(), t.effect))
                .collect::<Vec<_>>()
                .join(", ")
        });

    let label = metadata
        .labels
        .as_ref()
        .map(|l| serde_json::to_string(l).unwrap_or_default());
    let annotation = metadata
        .annotations
        .as_ref()
        .map(|a| serde_json::to_string(a).unwrap_or_default());

    // Images
    let (image_count, image_names, image_total_size_bytes) = status
        .and_then(|s| s.images.as_ref())
        .map(|imgs| {
            let count = imgs.len() as u32;
            let names = imgs
                .iter()
                .flat_map(|i| i.names.clone())
                .collect::<Vec<_>>();
            let total_size = imgs
                .iter()
                .filter_map(|i| i.size_bytes)
                .sum::<u64>();
            (Some(count), Some(names), Some(total_size))
        })
        .unwrap_or((None, None, None));

    Ok(InfoNodeEntity {
        node_name: Some(metadata.name.clone()),
        node_uid: metadata.uid.clone(),
        creation_timestamp,
        resource_version: metadata.resource_version.clone(),
        hostname,
        internal_ip,
        architecture,
        os_image,
        kernel_version,
        kubelet_version,
        container_runtime,
        operating_system,
        cpu_capacity_cores,
        memory_capacity_bytes,
        pod_capacity,
        ephemeral_storage_capacity_bytes,
        cpu_allocatable_cores,
        memory_allocatable_bytes,
        ephemeral_storage_allocatable_bytes,
        pod_allocatable,
        ready,
        taints,
        label,
        annotation,
        image_count,
        image_names,
        image_total_size_bytes,
        ..Default::default()
    })
}