use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::k8s::node::metric_node_entity::MetricNodeEntity;
use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, NaiveDate, Utc};
use std::{
    fs::File,
    fs::{self, OpenOptions},
    io::Write,
    io::{BufRead, BufReader},
    path::Path,
};
use std::path::PathBuf;
use crate::core::persistence::metrics::k8s::path::{
    metric_k8s_node_key_minute_dir_path,
    metric_k8s_node_key_minute_file_path,
};

/// Adapter for node minute-level metrics.
/// Responsible for appending minute samples to the filesystem and cleaning up old data.
pub struct MetricNodeMinuteFsAdapter;

impl MetricNodeMinuteFsAdapter {
    fn build_path(&self, node_name: &str) -> PathBuf {
        let date = Utc::now().format("%Y-%m-%d").to_string();
        metric_k8s_node_key_minute_file_path(node_name, &date)
    }

    fn parse_line(header: &[&str], line: &str) -> Option<MetricNodeEntity> {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() != header.len() {
            return None;
        }

        // TIME|CPU_USAGE_NANO_CORES|CPU_USAGE_CORE_NANO_SECONDS|... etc.
        let time = parts[0].parse::<DateTime<Utc>>().ok()?;
        Some(MetricNodeEntity {
            time,
            cpu_usage_nano_cores: parts[1].parse().ok(),
            cpu_usage_core_nano_seconds: parts[2].parse().ok(),
            memory_usage_bytes: parts[3].parse().ok(),
            memory_working_set_bytes: parts[4].parse().ok(),
            memory_rss_bytes: parts[5].parse().ok(),
            memory_page_faults: parts[6].parse().ok(),
            network_physical_rx_bytes: parts[7].parse().ok(),
            network_physical_tx_bytes: parts[8].parse().ok(),
            network_physical_rx_errors: parts[9].parse().ok(),
            network_physical_tx_errors: parts[10].parse().ok(),
            fs_used_bytes: parts[11].parse().ok(),
            fs_capacity_bytes: parts[12].parse().ok(),
            fs_inodes_used: parts[13].parse().ok(),
            fs_inodes: parts[14].parse().ok(),
        })
    }

    // fn ensure_header(&self, path: &Path, file: &mut std::fs::File) -> Result<()> {
    //     if !path.exists() {
    //         let header = "TIME|CPU_USAGE_NANO_CORES|CPU_USAGE_CORE_NANO_SECONDS|MEMORY_USAGE_BYTES|MEMORY_WORKING_SET_BYTES|MEMORY_RSS_BYTES|MEMORY_PAGE_FAULTS|NETWORK_PHYSICAL_RX_BYTES|NETWORK_PHYSICAL_TX_BYTES|NETWORK_PHYSICAL_RX_ERRORS|NETWORK_PHYSICAL_TX_ERRORS|FS_USED_BYTES|FS_CAPACITY_BYTES|FS_INODES_USED|FS_INODES\n";
    //         file.write_all(header.as_bytes())?;
    //     }
    //     Ok(())
    // }

    fn opt(v: Option<u64>) -> String {
        v.map(|x| x.to_string()).unwrap_or_default()
    }
}

impl MetricFsAdapterBase<MetricNodeEntity> for MetricNodeMinuteFsAdapter {
    fn append_row(&self, node: &str, dto: &MetricNodeEntity) -> Result<()> {
        let path_str = self.build_path(node);
        let path = Path::new(&path_str);

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        // let new = !path.exists();
        let mut file = OpenOptions::new().create(true).append(true).open(&path)?;
        // if new {
        //     self.ensure_header(path, &mut file)?;
        // }

        let row = format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
            dto.time.to_rfc3339_opts(chrono::SecondsFormat::Secs, false),
            Self::opt(dto.cpu_usage_nano_cores),
            Self::opt(dto.cpu_usage_core_nano_seconds),
            Self::opt(dto.memory_usage_bytes),
            Self::opt(dto.memory_working_set_bytes),
            Self::opt(dto.memory_rss_bytes),
            Self::opt(dto.memory_page_faults),
            Self::opt(dto.network_physical_rx_bytes),
            Self::opt(dto.network_physical_tx_bytes),
            Self::opt(dto.network_physical_rx_errors),
            Self::opt(dto.network_physical_tx_errors),
            Self::opt(dto.fs_used_bytes),
            Self::opt(dto.fs_capacity_bytes),
            Self::opt(dto.fs_inodes_used),
            Self::opt(dto.fs_inodes),
        );

        file.write_all(row.as_bytes())?;
        Ok(())
    }
    fn cleanup_old(&self, node: &str, before: DateTime<Utc>) -> Result<()> {
        let dir = metric_k8s_node_key_minute_dir_path(node);
        if !dir.exists() {
            return Ok(());
        }

        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("rcd") { continue; }

            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                if let Ok(file_date) = NaiveDate::parse_from_str(stem, "%Y-%m-%d") {
                    if file_date < before.date_naive() {
                        fs::remove_file(&path)
                            .with_context(|| format!("Failed to delete old metric file {:?}", path))?;
                    }
                }
            }
        }

        Ok(())
    }

    fn get_row_between(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        object_name: &str,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<MetricNodeEntity>> {
        let path = self.build_path(object_name);
        let path_obj = Path::new(&path);
        if !path_obj.exists() {
            return Ok(vec![]);
        }

        let file = File::open(&path_obj)?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        // Try to read the header line
        let first_line = lines.next().ok_or_else(|| anyhow!("empty metric file"))??;

        let mut data: Vec<MetricNodeEntity> = vec![];
        let header: Vec<&str>;
        // If the first line looks like a timestamp -> treat it as data
        if first_line.starts_with("20") {
            header = vec![
                "TIME", "CPU_USAGE_NANO_CORES", "CPU_USAGE_CORE_NANO_SECONDS",
                "MEMORY_USAGE_BYTES", "MEMORY_WORKING_SET_BYTES", "MEMORY_RSS_BYTES",
                "MEMORY_PAGE_FAULTS", "NETWORK_PHYSICAL_RX_BYTES", "NETWORK_PHYSICAL_TX_BYTES",
                "NETWORK_PHYSICAL_RX_ERRORS", "NETWORK_PHYSICAL_TX_ERRORS",
                "FS_USED_BYTES", "FS_CAPACITY_BYTES", "FS_INODES_USED", "FS_INODES"
            ];

            // process that first line as data
            if let Some(row) = Self::parse_line(&header, &first_line) {
                if row.time >= start && row.time <= end {
                    data.push(row);
                }
            }
        } else {
            // otherwise treat as a header
            header = first_line.split('|').collect();
        }

        // Now process the rest
        for line in lines.flatten() {
            if let Some(row) = Self::parse_line(&header, &line) {
                if row.time < start { continue; }
                if row.time > end { break; }
                data.push(row);
            }
        }

        // Apply pagination
        let start_idx = offset.unwrap_or(0);
        let limit = limit.unwrap_or(data.len());
        let slice: Vec<_> = data.into_iter().skip(start_idx).take(limit).collect();

        Ok(slice)
    }

    fn get_column_between(
        &self,
        column_name: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        object_name: &str,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> Result<Vec<MetricNodeEntity>> {
        let rows = self.get_row_between(start, end, object_name, limit, offset)?;
        let filtered: Vec<MetricNodeEntity> = rows
            .into_iter()
            .map(|mut row| {
                match column_name {
                    "CPU_USAGE_NANO_CORES" => {
                        let keep = row.cpu_usage_nano_cores;
                        row.cpu_usage_core_nano_seconds = None;
                        row.memory_usage_bytes = None;
                        row.memory_working_set_bytes = None;
                        row.memory_rss_bytes = None;
                        row.memory_page_faults = None;
                        row.network_physical_rx_bytes = None;
                        row.network_physical_tx_bytes = None;
                        row.network_physical_rx_errors = None;
                        row.network_physical_tx_errors = None;
                        row.fs_used_bytes = None;
                        row.fs_capacity_bytes = None;
                        row.fs_inodes_used = None;
                        row.fs_inodes = None;
                        row.cpu_usage_nano_cores = keep;
                    }
                    "CPU_USAGE_CORE_NANO_SECONDS" => {
                        let keep = row.cpu_usage_core_nano_seconds;
                        row.cpu_usage_nano_cores = None;
                        row.memory_usage_bytes = None;
                        row.memory_working_set_bytes = None;
                        row.memory_rss_bytes = None;
                        row.memory_page_faults = None;
                        row.network_physical_rx_bytes = None;
                        row.network_physical_tx_bytes = None;
                        row.network_physical_rx_errors = None;
                        row.network_physical_tx_errors = None;
                        row.fs_used_bytes = None;
                        row.fs_capacity_bytes = None;
                        row.fs_inodes_used = None;
                        row.fs_inodes = None;
                        row.cpu_usage_core_nano_seconds = keep;
                    }
                    "MEMORY_USAGE_BYTES" => {
                        let keep = row.memory_usage_bytes;
                        row.cpu_usage_nano_cores = None;
                        row.cpu_usage_core_nano_seconds = None;
                        row.memory_working_set_bytes = None;
                        row.memory_rss_bytes = None;
                        row.memory_page_faults = None;
                        row.network_physical_rx_bytes = None;
                        row.network_physical_tx_bytes = None;
                        row.network_physical_rx_errors = None;
                        row.network_physical_tx_errors = None;
                        row.fs_used_bytes = None;
                        row.fs_capacity_bytes = None;
                        row.fs_inodes_used = None;
                        row.fs_inodes = None;
                        row.memory_usage_bytes = keep;
                    }
                    "MEMORY_WORKING_SET_BYTES" => {
                        let keep = row.memory_working_set_bytes;
                        row.cpu_usage_nano_cores = None;
                        row.cpu_usage_core_nano_seconds = None;
                        row.memory_usage_bytes = None;
                        row.memory_rss_bytes = None;
                        row.memory_page_faults = None;
                        row.network_physical_rx_bytes = None;
                        row.network_physical_tx_bytes = None;
                        row.network_physical_rx_errors = None;
                        row.network_physical_tx_errors = None;
                        row.fs_used_bytes = None;
                        row.fs_capacity_bytes = None;
                        row.fs_inodes_used = None;
                        row.fs_inodes = None;
                        row.memory_working_set_bytes = keep;
                    }
                    "MEMORY_RSS_BYTES" => {
                        let keep = row.memory_rss_bytes;
                        row.cpu_usage_nano_cores = None;
                        row.cpu_usage_core_nano_seconds = None;
                        row.memory_usage_bytes = None;
                        row.memory_working_set_bytes = None;
                        row.memory_page_faults = None;
                        row.network_physical_rx_bytes = None;
                        row.network_physical_tx_bytes = None;
                        row.network_physical_rx_errors = None;
                        row.network_physical_tx_errors = None;
                        row.fs_used_bytes = None;
                        row.fs_capacity_bytes = None;
                        row.fs_inodes_used = None;
                        row.fs_inodes = None;
                        row.memory_rss_bytes = keep;
                    }
                    "MEMORY_PAGE_FAULTS" => {
                        let keep = row.memory_page_faults;
                        row.cpu_usage_nano_cores = None;
                        row.cpu_usage_core_nano_seconds = None;
                        row.memory_usage_bytes = None;
                        row.memory_working_set_bytes = None;
                        row.memory_rss_bytes = None;
                        row.network_physical_rx_bytes = None;
                        row.network_physical_tx_bytes = None;
                        row.network_physical_rx_errors = None;
                        row.network_physical_tx_errors = None;
                        row.fs_used_bytes = None;
                        row.fs_capacity_bytes = None;
                        row.fs_inodes_used = None;
                        row.fs_inodes = None;
                        row.memory_page_faults = keep;
                    }
                    "NETWORK_PHYSICAL_RX_BYTES" => {
                        let keep = row.network_physical_rx_bytes;
                        row.cpu_usage_nano_cores = None;
                        row.cpu_usage_core_nano_seconds = None;
                        row.memory_usage_bytes = None;
                        row.memory_working_set_bytes = None;
                        row.memory_rss_bytes = None;
                        row.memory_page_faults = None;
                        row.network_physical_tx_bytes = None;
                        row.network_physical_rx_errors = None;
                        row.network_physical_tx_errors = None;
                        row.fs_used_bytes = None;
                        row.fs_capacity_bytes = None;
                        row.fs_inodes_used = None;
                        row.fs_inodes = None;
                        row.network_physical_rx_bytes = keep;
                    }
                    "NETWORK_PHYSICAL_TX_BYTES" => {
                        let keep = row.network_physical_tx_bytes;
                        row.cpu_usage_nano_cores = None;
                        row.cpu_usage_core_nano_seconds = None;
                        row.memory_usage_bytes = None;
                        row.memory_working_set_bytes = None;
                        row.memory_rss_bytes = None;
                        row.memory_page_faults = None;
                        row.network_physical_rx_bytes = None;
                        row.network_physical_rx_errors = None;
                        row.network_physical_tx_errors = None;
                        row.fs_used_bytes = None;
                        row.fs_capacity_bytes = None;
                        row.fs_inodes_used = None;
                        row.fs_inodes = None;
                        row.network_physical_tx_bytes = keep;
                    }
                    "NETWORK_PHYSICAL_RX_ERRORS" => {
                        let keep = row.network_physical_rx_errors;
                        row.cpu_usage_nano_cores = None;
                        row.cpu_usage_core_nano_seconds = None;
                        row.memory_usage_bytes = None;
                        row.memory_working_set_bytes = None;
                        row.memory_rss_bytes = None;
                        row.memory_page_faults = None;
                        row.network_physical_rx_bytes = None;
                        row.network_physical_tx_bytes = None;
                        row.network_physical_tx_errors = None;
                        row.fs_used_bytes = None;
                        row.fs_capacity_bytes = None;
                        row.fs_inodes_used = None;
                        row.fs_inodes = None;
                        row.network_physical_rx_errors = keep;
                    }
                    "NETWORK_PHYSICAL_TX_ERRORS" => {
                        let keep = row.network_physical_tx_errors;
                        row.cpu_usage_nano_cores = None;
                        row.cpu_usage_core_nano_seconds = None;
                        row.memory_usage_bytes = None;
                        row.memory_working_set_bytes = None;
                        row.memory_rss_bytes = None;
                        row.memory_page_faults = None;
                        row.network_physical_rx_bytes = None;
                        row.network_physical_tx_bytes = None;
                        row.network_physical_rx_errors = None;
                        row.fs_used_bytes = None;
                        row.fs_capacity_bytes = None;
                        row.fs_inodes_used = None;
                        row.fs_inodes = None;
                        row.network_physical_tx_errors = keep;
                    }
                    "FS_USED_BYTES" => {
                        let keep = row.fs_used_bytes;
                        row.cpu_usage_nano_cores = None;
                        row.cpu_usage_core_nano_seconds = None;
                        row.memory_usage_bytes = None;
                        row.memory_working_set_bytes = None;
                        row.memory_rss_bytes = None;
                        row.memory_page_faults = None;
                        row.network_physical_rx_bytes = None;
                        row.network_physical_tx_bytes = None;
                        row.network_physical_rx_errors = None;
                        row.network_physical_tx_errors = None;
                        row.fs_capacity_bytes = None;
                        row.fs_inodes_used = None;
                        row.fs_inodes = None;
                        row.fs_used_bytes = keep;
                    }
                    "FS_CAPACITY_BYTES" => {
                        let keep = row.fs_capacity_bytes;
                        row.cpu_usage_nano_cores = None;
                        row.cpu_usage_core_nano_seconds = None;
                        row.memory_usage_bytes = None;
                        row.memory_working_set_bytes = None;
                        row.memory_rss_bytes = None;
                        row.memory_page_faults = None;
                        row.network_physical_rx_bytes = None;
                        row.network_physical_tx_bytes = None;
                        row.network_physical_rx_errors = None;
                        row.network_physical_tx_errors = None;
                        row.fs_used_bytes = None;
                        row.fs_inodes_used = None;
                        row.fs_inodes = None;
                        row.fs_capacity_bytes = keep;
                    }
                    "FS_INODES_USED" => {
                        let keep = row.fs_inodes_used;
                        row.cpu_usage_nano_cores = None;
                        row.cpu_usage_core_nano_seconds = None;
                        row.memory_usage_bytes = None;
                        row.memory_working_set_bytes = None;
                        row.memory_rss_bytes = None;
                        row.memory_page_faults = None;
                        row.network_physical_rx_bytes = None;
                        row.network_physical_tx_bytes = None;
                        row.network_physical_rx_errors = None;
                        row.network_physical_tx_errors = None;
                        row.fs_used_bytes = None;
                        row.fs_capacity_bytes = None;
                        row.fs_inodes = None;
                        row.fs_inodes_used = keep;
                    }
                    "FS_INODES" => {
                        let keep = row.fs_inodes;
                        row.cpu_usage_nano_cores = None;
                        row.cpu_usage_core_nano_seconds = None;
                        row.memory_usage_bytes = None;
                        row.memory_working_set_bytes = None;
                        row.memory_rss_bytes = None;
                        row.memory_page_faults = None;
                        row.network_physical_rx_bytes = None;
                        row.network_physical_tx_bytes = None;
                        row.network_physical_rx_errors = None;
                        row.network_physical_tx_errors = None;
                        row.fs_used_bytes = None;
                        row.fs_capacity_bytes = None;
                        row.fs_inodes_used = None;
                        row.fs_inodes = keep;
                    }
                    _ => {}
                }
                row
            })
            .collect();

        Ok(filtered)
    }
}
