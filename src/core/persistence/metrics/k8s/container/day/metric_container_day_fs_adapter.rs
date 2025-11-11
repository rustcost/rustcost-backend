use crate::core::persistence::metrics::metric_fs_adapter_base_trait::MetricFsAdapterBase;
use crate::core::persistence::metrics::k8s::container::metric_container_entity::MetricContainerEntity;
use anyhow::{anyhow, Context, Result};
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
use crate::core::persistence::metrics::k8s::container::hour::metric_container_hour_fs_adapter::MetricContainerHourFsAdapter;
use crate::core::persistence::metrics::k8s::path::{
    metric_k8s_container_key_day_dir_path,
    metric_k8s_container_key_day_file_path,
};

/// Adapter for container hour-level metrics.
/// Responsible for appending hour samples to the filesystem and cleaning up old data.
pub struct MetricContainerDayFsAdapter;

impl MetricContainerDayFsAdapter {
    fn build_path(&self, container_key: &str) -> PathBuf {
        let year = Utc::now().format("%Y").to_string();
        metric_k8s_container_key_day_file_path(container_key, &year)
    }

    fn parse_line(header: &[&str], line: &str) -> Option<MetricContainerEntity> {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() != header.len() {
            return None;
        }

        // TIME|CPU_USAGE_NANO_CORES|CPU_USAGE_CORE_NANO_SECONDS|... etc.
        let time = parts[0].parse::<DateTime<Utc>>().ok()?;
        Some(MetricContainerEntity {
            time,
            cpu_usage_nano_cores: parts[1].parse().ok(),
            cpu_usage_core_nano_seconds: parts[2].parse().ok(),
            memory_usage_bytes: parts[3].parse().ok(),
            memory_working_set_bytes: parts[4].parse().ok(),
            memory_rss_bytes: parts[5].parse().ok(),
            memory_page_faults: parts[6].parse().ok(),
            fs_used_bytes: parts[7].parse().ok(),
            fs_capacity_bytes: parts[8].parse().ok(),
            fs_inodes_used: parts[9].parse().ok(),
            fs_inodes: parts[10].parse().ok(),
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

impl MetricFsAdapterBase<MetricContainerEntity> for MetricContainerDayFsAdapter {
    fn append_row(&self, container: &str, dto: &MetricContainerEntity) -> Result<()> {
        let path_str = self.build_path(container);
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
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
            dto.time.to_rfc3339_opts(chrono::SecondsFormat::Secs, false),
            Self::opt(dto.cpu_usage_nano_cores),
            Self::opt(dto.cpu_usage_core_nano_seconds),
            Self::opt(dto.memory_usage_bytes),
            Self::opt(dto.memory_working_set_bytes),
            Self::opt(dto.memory_rss_bytes),
            Self::opt(dto.memory_page_faults),
            // --- FS fields (rootfs + logs) ---
            Self::opt(dto.fs_used_bytes),
            Self::opt(dto.fs_capacity_bytes),
            Self::opt(dto.fs_inodes_used),
            Self::opt(dto.fs_inodes),
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
        container_uid: &str,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<()> {
        // --- 1️⃣ Load hour data
        let hour_adapter = MetricContainerHourFsAdapter;
        let rows = hour_adapter.get_row_between(start, end, container_uid, None, None)?;

        if rows.is_empty() {
            return Err(anyhow!("no hour data found for aggregation"));
        }

        // --- 2️⃣ Compute aggregates
        let first = rows.first().unwrap();
        let last = rows.last().unwrap();

        let avg = |f: fn(&MetricContainerEntity) -> Option<u64>| -> Option<u64> {
            let (sum, count): (u64, u64) =
                rows.iter().filter_map(f).fold((0, 0), |(s, c), v| (s + v, c + 1));
            if count > 0 {
                Some(sum / count)
            } else {
                None
            }
        };

        let delta = |f: fn(&MetricContainerEntity) -> Option<u64>| -> Option<u64> {
            match (f(first), f(last)) {
                (Some(a), Some(b)) if b >= a => Some(b - a),
                _ => None,
            }
        };

        let aggregated = MetricContainerEntity {
            time: end, // time marker = end of the aggregation window

            // CPU
            cpu_usage_nano_cores: avg(|r| r.cpu_usage_nano_cores),
            cpu_usage_core_nano_seconds: delta(|r| r.cpu_usage_core_nano_seconds),

            // Memory
            memory_usage_bytes: avg(|r| r.memory_usage_bytes),
            memory_working_set_bytes: avg(|r| r.memory_working_set_bytes),
            memory_rss_bytes: avg(|r| r.memory_rss_bytes),
            memory_page_faults: delta(|r| r.memory_page_faults),

            // Ephemeral filesystem
            fs_used_bytes: avg(|r| r.fs_used_bytes),
            fs_capacity_bytes: last.fs_capacity_bytes,
            fs_inodes_used: avg(|r| r.fs_inodes_used),
            fs_inodes: last.fs_inodes,
        };

        // --- 3️⃣ Append the aggregated row into the day-level file
        self.append_row(container_uid, &aggregated)?;

        Ok(())
    }



    fn cleanup_old(&self, container_key: &str, before: DateTime<Utc>) -> Result<()> {
        let cutoff_year: i32 = before.format("%Y").to_string().parse().unwrap_or(0);
        let dir = metric_k8s_container_key_day_dir_path(container_key);

        if !dir.exists() {
            return Ok(());
        }

        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();

            if let Some(filename) = path.file_stem().and_then(|s| s.to_str()) {
                if let Ok(year) = filename.parse::<i32>() {
                    if year < cutoff_year {
                        fs::remove_file(&path)
                            .with_context(|| format!("Failed to delete old metric file {:?}", path))?;
                    }
                }
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
    ) -> Result<Vec<MetricContainerEntity>> {
        let rows = self.get_row_between(start, end, object_name, limit, offset)?;
        let filtered: Vec<MetricContainerEntity> = rows
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
    ) -> Result<Vec<MetricContainerEntity>> {
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

        let mut data: Vec<MetricContainerEntity> = vec![];
        let header: Vec<&str>;
        // If the first line looks like a timestamp -> treat it as data
        if first_line.starts_with("20") {
            header = vec![
                "TIME", "CPU_USAGE_NANO_CORES", "CPU_USAGE_CORE_NANO_SECONDS",
                "MEMORY_USAGE_BYTES", "MEMORY_WORKING_SET_BYTES", "MEMORY_RSS_BYTES",
                "MEMORY_PAGE_FAULTS", "FS_USED_BYTES", "FS_CAPACITY_BYTES",
                "FS_INODES_USED", "FS_INODES"
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
}
