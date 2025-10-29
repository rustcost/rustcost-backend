/* Persists metrics to file/TSDB */
use anyhow::Result;
use chrono::Utc;
use std::{fs, io::Write, path::Path};
use std::fs::OpenOptions;
use std::io::BufWriter;
use crate::scheduler::tasks::collectors::k8s::node::entity::{ NodeInfoEntity, NodeMetricsEntity};

/// Ensures all per-node data directories exist.
/// Idempotent and safe for concurrent calls.
pub fn ensure_node_dir(node_name: &str) -> Result<()> {
    let base = format!("data/nodes/{node_name}");
    for sub in ["m", "h", "d"] {
        let path = format!("{base}/{sub}");
        if let Err(e) = fs::create_dir_all(&path) {
            // If another thread created it between the check and here, ignore AlreadyExists
            if e.kind() != std::io::ErrorKind::AlreadyExists {
                return Err(e.into());
            }
        }
    }
    Ok(())
}
/// Writes `info.rci` for a node if it does not already exist.
/// Always writes all 30 fields in fixed order (blank if None).
pub fn write_info_if_missing(node: &str, dto: &NodeInfoEntity) -> Result<()> {
    let path = format!("data/nodes/{node}/info.rci");

    // Skip if already exists
    if Path::new(&path).exists() {
        return Ok(());
    }

    // Ensure directory exists
    if let Some(parent) = Path::new(&path).parent() {
        fs::create_dir_all(parent)?;
    }

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&path)?;
    let mut writer = BufWriter::new(file);

    // Helper macro for concise writing
    macro_rules! write_field {
        ($key:expr, $val:expr) => {
            writeln!(writer, "{}:{}", $key, $val.as_deref().unwrap_or(""))?;
        };
    }

    // --- Write all 30 known fields in canonical order ---
    write_field!("NODE_NAME", dto.node_name);
    write_field!("NODE_UID", dto.node_uid);
    write_field!("CREATION_TIMESTAMP", dto.creation_timestamp);
    write_field!("RESOURCE_VERSION", dto.resource_version);
    write_field!("LAST_UPDATED_INFO_AT", dto.last_updated_info_at);
    write_field!("DELETED", dto.deleted.as_ref().map(|v| if *v { "true" } else { "false" }));
    write_field!("LAST_CHECK_DELETED_COUNT", dto.last_check_deleted_count.as_ref().map(|v| v.to_string()));
    write_field!("HOSTNAME", dto.hostname);
    write_field!("INTERNAL_IP", dto.internal_ip);
    write_field!("ARCHITECTURE", dto.architecture);
    write_field!("OS_IMAGE", dto.os_image);
    write_field!("KERNEL_VERSION", dto.kernel_version);
    write_field!("KUBELET_VERSION", dto.kubelet_version);
    write_field!("CONTAINER_RUNTIME", dto.container_runtime);
    write_field!("OPERATING_SYSTEM", dto.operating_system);
    write_field!("CPU_CAPACITY_CORES", dto.cpu_capacity_cores.as_ref().map(|v| v.to_string()));
    write_field!("MEMORY_CAPACITY_BYTES", dto.memory_capacity_bytes.as_ref().map(|v| v.to_string()));
    write_field!("POD_CAPACITY", dto.pod_capacity.as_ref().map(|v| v.to_string()));
    write_field!("EPHEMERAL_STORAGE_CAPACITY_BYTES", dto.ephemeral_storage_capacity_bytes.as_ref().map(|v| v.to_string()));
    write_field!("CPU_ALLOCATABLE_CORES", dto.cpu_allocatable_cores.as_ref().map(|v| v.to_string()));
    write_field!("MEMORY_ALLOCATABLE_BYTES", dto.memory_allocatable_bytes.as_ref().map(|v| v.to_string()));
    write_field!("EPHEMERAL_STORAGE_ALLOCATABLE_BYTES", dto.ephemeral_storage_allocatable_bytes.as_ref().map(|v| v.to_string()));
    write_field!("POD_ALLOCATABLE", dto.pod_allocatable.as_ref().map(|v| v.to_string()));
    write_field!("READY", dto.ready.as_ref().map(|v| if *v { "true" } else { "false" }));
    write_field!("TAINTS", dto.taints);
    write_field!("LABEL", dto.label);
    write_field!("ANNOTATION", dto.annotation);
    write_field!("IMAGE_COUNT", dto.image_count.as_ref().map(|v| v.to_string()));
    write_field!("IMAGE_NAMES", dto.image_names.as_ref().map(|v| v.join(",")));
    write_field!("IMAGE_TOTAL_SIZE_BYTES", dto.image_total_size_bytes.as_ref().map(|v| v.to_string()));

    writer.flush()?;
    Ok(())
}
pub fn append_metrics(node: &str, dto: &NodeMetricsEntity) -> Result<()> {
    let month = Utc::now().format("%Y-%m").to_string();
    let path = format!("data/nodes/{node}/m/{month}.rcd");

    let new = !Path::new(&path).exists();
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;

    if new {
        let header = "TIME|CPU_USAGE_NANO_CORES|CPU_USAGE_CORE_NANO_SECONDS|MEMORY_USAGE_BYTES|MEMORY_WORKING_SET_BYTES|MEMORY_RSS_BYTES|MEMORY_PAGE_FAULTS|NETWORK_PHYSICAL_RX_BYTES|NETWORK_PHYSICAL_TX_BYTES|NETWORK_PHYSICAL_RX_ERRORS|NETWORK_PHYSICAL_TX_ERRORS|FS_USED_BYTES|FS_CAPACITY_BYTES|FS_INODES_USED|FS_INODES\n";
        file.write_all(header.as_bytes())?;
    }

    // Safely format Option<u64> values
    let row = format!(
        "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
        dto.time,
        opt(dto.cpu_usage_nano_cores),
        opt(dto.cpu_usage_core_nano_seconds),
        opt(dto.memory_usage_bytes),
        opt(dto.memory_working_set_bytes),
        opt(dto.memory_rss_bytes),
        opt(dto.memory_page_faults),
        opt(dto.network_physical_rx_bytes),
        opt(dto.network_physical_tx_bytes),
        opt(dto.network_physical_rx_errors),
        opt(dto.network_physical_tx_errors),
        opt(dto.fs_used_bytes),
        opt(dto.fs_capacity_bytes),
        opt(dto.fs_inodes_used),
        opt(dto.fs_inodes),
    );

    file.write_all(row.as_bytes())?;
    Ok(())
}

/// Helper to convert Option<u64> -> String
fn opt(v: Option<u64>) -> String {
    v.map(|x| x.to_string()).unwrap_or_default()
}