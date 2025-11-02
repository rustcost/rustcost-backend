use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::node::metric_node_entity::MetricNodeEntity;
use anyhow::{anyhow, Result};
use chrono::{DateTime, NaiveDate, Utc};
use std::{
    fs::File,
    fs::{self, OpenOptions},
    io::Write,
    io::{BufRead, BufReader},
    path::Path,
};

/// Adapter for node minute-level metrics.
/// Responsible for appending minute samples to the filesystem and cleaning up old data.
pub struct MetricNodeMinuteFsAdapter;

impl MetricNodeMinuteFsAdapter {
    fn build_path(&self, node_name: &str) -> String {
        let date = Utc::now().format("%Y-%m-%d").to_string();
        format!("data/metrics/nodes/{node_name}/m/{date}.rcd")
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
        let metrics_dir = Path::new("data/metrics/nodes").join(node).join("m");

        if !metrics_dir.exists() {
            return Ok(());
        }

        for entry in fs::read_dir(&metrics_dir)? {
            let entry = entry?;
            let path = entry.path();

            // Only process .rcd files
            if path.extension().and_then(|e| e.to_str()) != Some("rcd") {
                continue;
            }

            if let Some(file_name) = path.file_stem().and_then(|s| s.to_str()) {
                // Expect filenames like "2025-11-01"
                if let Ok(file_date) = NaiveDate::parse_from_str(file_name, "%Y-%m-%d") {
                    if let Some(naive_dt) = file_date.and_hms_opt(0, 0, 0) {
                        let file_dt: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive_dt, Utc);
                        if file_dt < before {
                            fs::remove_file(&path)?;
                        }
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

        // Read header first
        let header_line = lines.next().ok_or_else(|| anyhow!("empty metric file"))??;
        let header: Vec<&str> = header_line.split('|').collect();

        let mut data: Vec<MetricNodeEntity> = vec![];

        for line in lines.flatten() {
            if let Some(row) = Self::parse_line(&header, &line) {
                if row.time >= start && row.time <= end {
                    data.push(row);
                }
            }
        }

        // Apply pagination
        let start_idx = offset.unwrap_or(0);
        let end_idx = limit.map(|l| start_idx + l).unwrap_or(data.len());
        let slice = data.into_iter().skip(start_idx).take(end_idx - start_idx).collect();

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
                // Zero out all other columns except the one requested
                match column_name {
                    "CPU_USAGE_NANO_CORES" => {
                        row.cpu_usage_core_nano_seconds = None;
                        row.memory_usage_bytes = None;
                        // ... set others to None as needed
                    }
                    "MEMORY_USAGE_BYTES" => {
                        row.cpu_usage_nano_cores = None;
                        row.cpu_usage_core_nano_seconds = None;
                        // ... etc.
                    }
                    _ => {}
                }
                row
            })
            .collect();

        Ok(filtered)
    }
}
