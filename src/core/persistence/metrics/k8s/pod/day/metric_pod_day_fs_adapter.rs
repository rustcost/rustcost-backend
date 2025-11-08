use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::k8s::pod::metric_pod_entity::MetricPodEntity;
use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use std::io::BufWriter;
use std::{
    fs::File,
    fs::{self, OpenOptions},
    io::Write,
    io::{BufRead, BufReader},
    path::Path,
};
use std::path::PathBuf;
use crate::core::persistence::metrics::k8s::pod::hour::metric_pod_hour_fs_adapter::MetricPodHourFsAdapter;
use crate::core::persistence::storage_path::metric_pod_day_path;

/// Adapter for pod hour-level metrics.
/// Responsible for appending hour samples to the filesystem and cleaning up old data.
pub struct MetricPodDayFsAdapter;

impl MetricPodDayFsAdapter {
    fn build_path(&self, pod_key: &str) -> PathBuf {
        let year = Utc::now().format("%Y").to_string();
        metric_pod_day_path(pod_key, &year)
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

    // fn ensure_header(file: &mut File) -> Result<()> {
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

impl MetricFsAdapterBase<MetricPodEntity> for MetricPodDayFsAdapter {
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

    /// Aggregate hour-level metrics into an dayly sample and append to day file.
    fn append_row_aggregated(
        &self,
        pod_uid: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<()> {
        // --- 1️⃣ Load hour data
        let hour_adapter = MetricPodHourFsAdapter;
        let rows = hour_adapter.get_row_between(start, end, pod_uid, None, None)?;

        if rows.is_empty() {
            return Err(anyhow!("no hour data found for aggregation"));
        }

        // --- 2️⃣ Compute aggregates
        let first = rows.first().unwrap();
        let last = rows.last().unwrap();

        let avg = |f: fn(&MetricPodEntity) -> Option<u64>| -> Option<u64> {
            let (sum, count): (u64, u64) =
                rows.iter().filter_map(f).fold((0, 0), |(s, c), v| (s + v, c + 1));
            if count > 0 {
                Some(sum / count)
            } else {
                None
            }
        };

        let delta = |f: fn(&MetricPodEntity) -> Option<u64>| -> Option<u64> {
            match (f(first), f(last)) {
                (Some(a), Some(b)) if b >= a => Some(b - a),
                _ => None,
            }
        };

        let aggregated = MetricPodEntity {
            time: end, // time marker = end of the aggregation window

            // CPU
            cpu_usage_nano_cores: avg(|r| r.cpu_usage_nano_cores),
            cpu_usage_core_nano_seconds: delta(|r| r.cpu_usage_core_nano_seconds),

            // Memory
            memory_usage_bytes: avg(|r| r.memory_usage_bytes),
            memory_working_set_bytes: avg(|r| r.memory_working_set_bytes),
            memory_rss_bytes: avg(|r| r.memory_rss_bytes),
            memory_page_faults: delta(|r| r.memory_page_faults),

            // Network
            network_physical_rx_bytes: delta(|r| r.network_physical_rx_bytes),
            network_physical_tx_bytes: delta(|r| r.network_physical_tx_bytes),
            network_physical_rx_errors: delta(|r| r.network_physical_rx_errors),
            network_physical_tx_errors: delta(|r| r.network_physical_tx_errors),

            // Ephemeral storage
            es_used_bytes: avg(|r| r.es_used_bytes),
            es_capacity_bytes: last.es_capacity_bytes,
            es_inodes_used: avg(|r| r.es_inodes_used),
            es_inodes: last.es_inodes,

            // Persistent storage
            pv_used_bytes: avg(|r| r.pv_used_bytes),
            pv_capacity_bytes: last.pv_capacity_bytes,
            pv_inodes_used: avg(|r| r.pv_inodes_used),
            pv_inodes: last.pv_inodes,
        };

        // --- 3️⃣ Append the aggregated row into the day-level file
        self.append_row(pod_uid, &aggregated)?;

        Ok(())
    }



    fn cleanup_old(&self, pod_uid: &str, before: DateTime<Utc>) -> Result<()> {
        let cutoff_month = before.format("%Y-%m").to_string();

        let paths = [
            //TODO
            format!("data/metric/pod/{pod_uid}/m/{cutoff_month}.rcd"),
        ];

        for path in &paths {
            if Path::new(path).exists() {
                let _ = fs::remove_file(path);
            }
        }

        Ok(())
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

        // Read header first
        let header_line = lines.next().ok_or_else(|| anyhow!("empty metric file"))??;
        let header: Vec<&str> = header_line.split('|').collect();

        let mut data: Vec<MetricPodEntity> = vec![];

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
}
