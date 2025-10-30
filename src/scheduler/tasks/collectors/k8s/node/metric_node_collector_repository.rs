/* Persists metrics to file/TSDB */
use anyhow::Result;
use chrono::Utc;
use std::{fs, io::Write, path::Path};
use std::fs::OpenOptions;
use std::io::BufWriter;
use crate::scheduler::tasks::collectors::k8s::node::entity::NodeMetricsEntity;

/// Ensures all per-node data directories exist.
/// Idempotent and safe for concurrent calls.


pub fn append_metrics(node: &str, dto: &NodeMetricsEntity) -> Result<()> {
    let month = Utc::now().format("%Y-%m").to_string();
    let path = format!("data/metrics/nodes/{node}/m/{month}.rcd");

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