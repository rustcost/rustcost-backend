use anyhow::Result;
use chrono::{DateTime, Utc};
use crate::core::client::k8s::client_k8s_container_dto::ContainerInfo;
use crate::core::client::k8s::client_k8s_pod_dto::{ContainerStatus, ContainerSpec, Pod};
use crate::core::persistence::info::k8s::container::info_container_entity::InfoContainerEntity;

pub fn map_container_info_to_info_container_entity(info: &ContainerInfo) -> InfoContainerEntity {
    InfoContainerEntity {
        container_name: Some(info.container_name.clone()),
        image: info.image.clone(),
        pod_uid: None, // not known from ContainerInfo alone
        namespace: Some(info.namespace.clone()),
        start_time: info
            .started_at
            .as_ref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc)),
        image_id: info.image_id.clone(),
        container_id: info.container_id.clone(),
        ready: info.ready,
        restart_count: info.restart_count.map(|x| x as u32),
        last_updated_info_at: Some(Utc::now()),
        ..Default::default()
    }
}

/// Map a `ContainerStatus` (plus related pod/spec info) to `InfoContainerEntity`.
pub fn map_container_status_to_info_container_entity(
    pod: &Pod,
    spec: &ContainerSpec,
    status: Option<&ContainerStatus>,
) -> Result<InfoContainerEntity> {
    let metadata = &pod.metadata;
    let pod_status = pod.status.as_ref();
    let spec_resources = spec.resources.as_ref();

    // --- Parse timestamps ---
    let creation_timestamp = metadata
        .creation_timestamp
        .as_ref()
        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&Utc));

    let start_time = status
        .and_then(|cs| {
            cs.state
                .as_ref()
                .and_then(|s| s.running.as_ref())
                .and_then(|r| r.started_at.as_ref())
        })
        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&Utc));

    // --- Lifecycle / identity ---
    let container_name = Some(spec.name.clone());
    let pod_uid = Some(metadata.uid.clone());
    let namespace = Some(metadata.namespace.clone());

    // --- Runtime ---
    let (state, reason, message, exit_code) = status
        .and_then(|cs| cs.state.as_ref())
        .map(|s| {
            if let Some(_) = &s.running {
                ("Running".to_string(), None, None, None)
            } else if let Some(t) = &s.terminated {
                (
                    "Terminated".to_string(),
                    t.reason.clone(),
                    None,
                    t.exit_code,
                )
            } else {
                ("Waiting".to_string(), None, None, None)
            }
        })
        .unwrap_or(("Unknown".to_string(), None, None, None));

    // --- Resource requests & limits (if available) ---
    let (cpu_request_millicores, memory_request_bytes) = spec_resources
        .and_then(|r| r.requests.as_ref())
        .map(|reqs| {
            let cpu = reqs
                .get("cpu")
                .and_then(|s| parse_cpu_millicores(s))
                .unwrap_or(0);
            let mem = reqs
                .get("memory")
                .and_then(|s| parse_memory_bytes(s))
                .unwrap_or(0);
            (Some(cpu), Some(mem))
        })
        .unwrap_or((None, None));

    // --- Volume mounts ---
    let volume_mounts = spec
        .volume_mounts
        .as_ref()
        .map(|v| v.iter().map(|m| m.mount_path.clone()).collect());

    // --- Metadata labels/annotations (flattened) ---
    let labels = metadata
        .labels
        .as_ref()
        .map(|l| serde_json::to_string(l).unwrap_or_default());
    let annotations = metadata
        .annotations
        .as_ref()
        .map(|a| serde_json::to_string(a).unwrap_or_default());

    // --- Build entity ---
    Ok(InfoContainerEntity {
        pod_uid,
        container_name,
        namespace,
        creation_timestamp,
        start_time,
        container_id: status.and_then(|cs| cs.container_id.clone()),
        image: status
            .and_then(|cs| cs.image.clone())
            .or_else(|| spec.image.clone()),
        image_id: status.and_then(|cs| cs.image_id.clone()),
        state: Some(state),
        reason,
        message,
        exit_code,
        restart_count: status.map(|cs| cs.restart_count as u32),
        ready: status.and_then(|cs| cs.ready),
        node_name: pod.spec.node_name.clone(),
        host_ip: pod_status.and_then(|s| s.host_ip.clone()),
        pod_ip: pod_status.and_then(|s| s.pod_ip.clone()),
        cpu_request_millicores,
        memory_request_bytes,
        cpu_limit_millicores: None, // Add when limits parsed
        memory_limit_bytes: None,   // Add when limits parsed
        volume_mounts,
        volume_devices: None,
        labels,
        annotations,
        last_updated_info_at: Some(Utc::now()),
        deleted: Some(false),
        last_check_deleted_count: None,
    })
}
/// Convert CPU quantity (e.g., "100m", "1", "0.5") → millicores
fn parse_cpu_millicores(s: &str) -> Option<u64> {
    if s.ends_with("m") {
        s.trim_end_matches('m').parse::<u64>().ok()
    } else if let Ok(v) = s.parse::<f64>() {
        Some((v * 1000.0) as u64)
    } else {
        None
    }
}

/// Convert memory quantity (e.g., "128Mi", "2Gi") → bytes
fn parse_memory_bytes(s: &str) -> Option<u64> {
    let s = s.to_lowercase();
    let units = [
        ("ki", 1024_u64),
        ("mi", 1024_u64.pow(2)),
        ("gi", 1024_u64.pow(3)),
        ("ti", 1024_u64.pow(4)),
        ("k", 1000_u64),
        ("m", 1000_u64.pow(2)),
        ("g", 1000_u64.pow(3)),
    ];

    for (suffix, mult) in units {
        if s.ends_with(suffix) {
            let num = s.trim_end_matches(suffix).trim();
            return num.parse::<f64>().ok().map(|v| (v * mult as f64) as u64);
        }
    }

    s.trim().parse::<u64>().ok()
}
