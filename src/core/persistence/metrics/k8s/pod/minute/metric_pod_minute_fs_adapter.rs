use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::k8s::pod::metric_pod_entity::MetricPodEntity;
use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, NaiveDate, Utc};
use std::io::BufWriter;
use std::{
    fs::File,
    fs::{self, OpenOptions},
    io::Write,
    io::{BufRead, BufReader},
    path::Path,
};
use std::path::PathBuf;
use crate::core::persistence::metrics::k8s::path::{
    metric_k8s_pod_key_minute_file_path,
    metric_k8s_pod_key_minute_dir_path,
};

/// Adapter for pod minute-level metrics.
/// Responsible for appending minute samples to the filesystem and cleaning up old data.
pub struct MetricPodMinuteFsAdapter;

impl MetricPodMinuteFsAdapter {
    fn build_path(&self, pod_uid: &str) -> PathBuf {
        let date = Utc::now().format("%Y-%m-%d").to_string();
        metric_k8s_pod_key_minute_file_path(pod_uid, &date)
    }

    fn parse_line(header: &[&str], line: &str) -> Option<MetricPodEntity> {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() != header.len() {
            return None;
        }

        // TIME|CPU_USAGE_NANO_CORES|CPU_USAGE_CORE_NANO_SECONDS|... etc.
        let time = parts[0].parse::<DateTime<Utc>>().ok()?;
        Some(MetricPodEntity {
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
            es_used_bytes: parts[11].parse().ok(),
            es_capacity_bytes: parts[12].parse().ok(),
            es_inodes_used: parts[13].parse().ok(),
            es_inodes: parts[14].parse().ok(),
            pv_used_bytes: parts[15].parse().ok(),
            pv_capacity_bytes: parts[16].parse().ok(),
            pv_inodes_used: parts[17].parse().ok(),
            pv_inodes: parts[18].parse().ok(),
        })
    }

    // fn ensure_header(&self, path: &Path, file: &mut std::fs::File) -> Result<()> {
    //     if file.metadata()?.len() == 0 {
    //         let header = "TIME|CPU_USAGE_NANO_CORES|CPU_USAGE_CORE_NANO_SECONDS|MEMORY_USAGE_BYTES|MEMORY_WORKING_SET_BYTES|MEMORY_RSS_BYTES|MEMORY_PAGE_FAULTS|NETWORK_PHYSICAL_RX_BYTES|NETWORK_PHYSICAL_TX_BYTES|NETWORK_PHYSICAL_RX_ERRORS|NETWORK_PHYSICAL_TX_ERRORS|ES_USED_BYTES|ES_CAPACITY_BYTES|ES_INODES_USED|ES_INODES|PV_USED_BYTES|PV_CAPACITY_BYTES|PV_INODES_USED|PV_INODES\n";
    //         file.write_all(header.as_bytes())?;
    //     }
    //     Ok(())
    // }


    fn opt(v: Option<u64>) -> String {
        v.map(|x| x.to_string()).unwrap_or_default()
    }
}

impl MetricFsAdapterBase<MetricPodEntity> for MetricPodMinuteFsAdapter {
    fn append_row(&self, pod: &str, dto: &MetricPodEntity) -> Result<()> {
        let path_str = self.build_path(pod);
        let path = Path::new(&path_str);

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        // let new = !path.exists();

        // ✅ open file and wrap in BufWriter
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)?;
        let mut writer = BufWriter::new(file);

        // Write header if file newly created
        // if new {
        //     self.ensure_header(path, &mut writer)?;
        // }

        // Format the row
        let row = format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
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
            Self::opt(dto.es_used_bytes),
            Self::opt(dto.es_capacity_bytes),
            Self::opt(dto.es_inodes_used),
            Self::opt(dto.es_inodes),
            Self::opt(dto.pv_used_bytes),
            Self::opt(dto.pv_capacity_bytes),
            Self::opt(dto.pv_inodes_used),
            Self::opt(dto.pv_inodes),
        );


        // ✅ write to buffer
        writer.write_all(row.as_bytes())?;

        // ✅ ensure everything flushed to disk
        writer.flush()?;
        Ok(())
    }

    fn cleanup_old(&self, pod_uid: &str, before: DateTime<Utc>) -> Result<()> {
        let dir = metric_k8s_pod_key_minute_dir_path(pod_uid);
        if !dir.exists() { return Ok(()); }

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
    ) -> Result<Vec<MetricPodEntity>> {
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

        let mut data: Vec<MetricPodEntity> = vec![];
        let header: Vec<&str>;
        // If the first line looks like a timestamp -> treat it as data
        if first_line.starts_with("20") {
            header = vec![
                "TIME", "CPU_USAGE_NANO_CORES", "CPU_USAGE_CORE_NANO_SECONDS",
                "MEMORY_USAGE_BYTES", "MEMORY_WORKING_SET_BYTES", "MEMORY_RSS_BYTES",
                "MEMORY_PAGE_FAULTS", "NETWORK_PHYSICAL_RX_BYTES", "NETWORK_PHYSICAL_TX_BYTES",
                "NETWORK_PHYSICAL_RX_ERRORS", "NETWORK_PHYSICAL_TX_ERRORS",
                "ES_USED_BYTES", "ES_CAPACITY_BYTES", "ES_INODES_USED", "ES_INODES",
                "PV_USED_BYTES", "PV_CAPACITY_BYTES", "PV_INODES_USED", "PV_INODES"
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
    ) -> Result<Vec<MetricPodEntity>> {
        let rows = self.get_row_between(start, end, object_name, limit, offset)?;
        let filtered: Vec<MetricPodEntity> = rows
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
