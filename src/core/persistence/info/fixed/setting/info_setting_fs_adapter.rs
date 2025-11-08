use super::info_setting_entity::{InfoSettingEntity, RuntimeType};
use crate::core::persistence::info::fixed::info_fixed_fs_adapter_trait::InfoFixedFsAdapterTrait;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Write},
    path::Path,
};
use crate::core::persistence::storage_path::info_setting_path;

/// File-based FS_ADAPTER implementation for the `Settings` entity.
///
/// Provides lightweight read/write/update/delete operations for
/// the settings file, using key–value text format for simplicity
/// and atomic write for safety.
pub struct InfoSettingFsAdapter;

impl InfoFixedFsAdapterTrait<InfoSettingEntity> for InfoSettingFsAdapter {
    /// Reads the settings file into memory.
    /// Returns default values if the file does not exist.
    fn read(&self) -> Result<InfoSettingEntity> {
        let path = info_setting_path();

        if !path.exists() {
            return Ok(InfoSettingEntity::default());
        }

        let file = File::open(&path).context("Failed to open settings file")?;
        let reader = BufReader::new(file);
        let mut s = InfoSettingEntity::default();

        for line in reader.lines() {
            let line = line?;
            if let Some((key, val)) = line.split_once(':') {
                let key = key.trim().to_uppercase();
                let val = val.trim();

                match key.as_str() {
                    // === General & UI ===
                    "IS_DARK_MODE" => s.is_dark_mode = val.eq_ignore_ascii_case("true"),
                    "LANGUAGE" => s.language = val.to_string(),

                    "MINUTE_RETENTION_DAY" => s.minute_retention_days = val.parse().unwrap_or(s.minute_retention_days),
                    "HOUR_RETENTION_MONTH" => s.hour_retention_months = val.parse().unwrap_or(s.hour_retention_months),
                    "DAY_RETENTION_YEAR" => s.day_retention_years = val.parse().unwrap_or(s.day_retention_years),                    "RETENTION_POLICY" => s.retention_policy = val.to_string(),

                    // === TSDB Options ===
                    "ENABLE_LINE_NUM_TRACKING" => s.enable_line_num_tracking = val.eq_ignore_ascii_case("true"),
                    "ENABLE_INDEX_FILE" => s.enable_index_file = val.eq_ignore_ascii_case("true"),
                    "MAX_STORAGE_GB" => s.max_storage_gb = val.parse().unwrap_or(s.max_storage_gb),
                    "COMPRESSION_ENABLED" => s.compression_enabled = val.eq_ignore_ascii_case("true"),

                    // === Metrics ===
                    "SCRAPE_INTERVAL_SEC" => s.scrape_interval_sec = val.parse().unwrap_or(s.scrape_interval_sec),
                    "METRICS_BATCH_SIZE" => s.metrics_batch_size = val.parse().unwrap_or(s.metrics_batch_size),

                    // === Alerts ===
                    "ENABLE_CLUSTER_HEALTH_ALERT" => s.enable_cluster_health_alert = val.eq_ignore_ascii_case("true"),
                    "ENABLE_RUSTCOST_HEALTH_ALERT" => s.enable_rustcost_health_alert = val.eq_ignore_ascii_case("true"),
                    "GLOBAL_ALERT_SUBJECT" => s.global_alert_subject = val.to_string(),
                    "LINKBACK_URL" => s.linkback_url = if val.is_empty() { None } else { Some(val.to_string()) },
                    "EMAIL_RECIPIENTS" => {
                        s.email_recipients = val
                            .split(',')
                            .map(|v| v.trim().to_string())
                            .filter(|v| !v.is_empty())
                            .collect();
                    }
                    "SLACK_WEBHOOK_URL" => s.slack_webhook_url = if val.is_empty() { None } else { Some(val.to_string()) },
                    "TEAMS_WEBHOOK_URL" => s.teams_webhook_url = if val.is_empty() { None } else { Some(val.to_string()) },

                    // === LLM ===
                    "LLM_URL" => s.llm_url = if val.is_empty() { None } else { Some(val.to_string()) },
                    "LLM_TOKEN" => s.llm_token = if val.is_empty() { None } else { Some(val.to_string()) },
                    "LLM_MODEL" => s.llm_model = if val.is_empty() { None } else { Some(val.to_string()) },

                    // === Metadata ===
                    "CREATED_AT" => {
                        if let Ok(dt) = val.parse::<DateTime<Utc>>() {
                            s.created_at = dt;
                        }
                    }
                    "UPDATED_AT" => {
                        if let Ok(dt) = val.parse::<DateTime<Utc>>() {
                            s.updated_at = dt;
                        }
                    }
                    "VERSION" => s.version = val.to_string(),

                        "RUNTIME_TYPE" => s.runtime_type = match val.to_lowercase().as_str() {
                        "docker" => RuntimeType::Docker,
                        "containerd" => RuntimeType::Containerd,
                        "baremetal" => RuntimeType::BareMetal,
                        _ => RuntimeType::K8s,
                        },
                        "ENABLE_K8S_API" => s.enable_k8s_api = val == "true",
                        "ENABLE_CONTAINER_EXPORTER" => s.enable_container_exporter = val == "true",
                        "ENABLE_GPU_EXPORTER" => s.enable_gpu_exporter = val == "true",

                        "GPU_EXPORTER_URLS" => {
                        s.gpu_exporter_urls = val
                        .split(',')
                        .map(|v| v.trim().to_string())
                        .filter(|v| !v.is_empty())
                        .collect();
                        }
                        "CONTAINER_EXPORTER_URLS" => {
                        s.container_exporter_urls = val
                        .split(',')
                        .map(|v| v.trim().to_string())
                        .filter(|v| !v.is_empty())
                        .collect();
                        }
                        "K8S_API_URL" => {
                        s.k8s_api_url = if val.trim().is_empty() {
                        None
                        } else {
                        Some(val.to_string())
                        };
                        }
                    _ => {}
                }
            }
        }

        Ok(s)
    }

    fn insert(&self, data: &InfoSettingEntity) -> Result<()> {
        self.write(data)
    }

    fn update(&self, data: &InfoSettingEntity) -> Result<()> {
        self.write(data)
    }

    fn delete(&self) -> Result<()> {
        let path = info_setting_path();

        if path.exists() {
            fs::remove_file(&path).context("Failed to delete settings file")?;
        }
        Ok(())
    }
}

impl InfoSettingFsAdapter {
    /// Internal helper to atomically write the settings file.
    fn write(&self, data: &InfoSettingEntity) -> Result<()> {
        let path = info_setting_path();

        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir).context("Failed to create settings directory")?;
        }

        let tmp_path = path.with_extension("rci.tmp");
        let mut f = File::create(&tmp_path).context("Failed to create temp file")?;

        writeln!(f, "IS_DARK_MODE:{}", data.is_dark_mode)?;
        writeln!(f, "LANGUAGE:{}", data.language)?;
        writeln!(f, "MINUTE_RETENTION_DAY:{}", data.minute_retention_days)?;
        writeln!(f, "HOUR_RETENTION_MONTH:{}", data.hour_retention_months)?;
        writeln!(f, "DAY_RETENTION_YEAR:{}", data.day_retention_years)?;
        writeln!(f, "RETENTION_POLICY:{}", data.retention_policy)?;
        writeln!(f, "ENABLE_LINE_NUM_TRACKING:{}", data.enable_line_num_tracking)?;
        writeln!(f, "ENABLE_INDEX_FILE:{}", data.enable_index_file)?;
        writeln!(f, "MAX_STORAGE_GB:{}", data.max_storage_gb)?;
        writeln!(f, "COMPRESSION_ENABLED:{}", data.compression_enabled)?;
        writeln!(f, "SCRAPE_INTERVAL_SEC:{}", data.scrape_interval_sec)?;
        writeln!(f, "METRICS_BATCH_SIZE:{}", data.metrics_batch_size)?;
        writeln!(f, "ENABLE_CLUSTER_HEALTH_ALERT:{}", data.enable_cluster_health_alert)?;
        writeln!(f, "ENABLE_RUSTCOST_HEALTH_ALERT:{}", data.enable_rustcost_health_alert)?;
        writeln!(f, "GLOBAL_ALERT_SUBJECT:{}", data.global_alert_subject)?;
        writeln!(f, "LINKBACK_URL:{}", data.linkback_url.clone().unwrap_or_default())?;
        writeln!(f, "EMAIL_RECIPIENTS:{}", data.email_recipients.join(","))?;
        writeln!(f, "SLACK_WEBHOOK_URL:{}", data.slack_webhook_url.clone().unwrap_or_default())?;
        writeln!(f, "TEAMS_WEBHOOK_URL:{}", data.teams_webhook_url.clone().unwrap_or_default())?;
        writeln!(f, "LLM_URL:{}", data.llm_url.clone().unwrap_or_default())?;
        writeln!(f, "LLM_TOKEN:{}", data.llm_token.clone().unwrap_or_default())?;
        writeln!(f, "LLM_MODEL:{}", data.llm_model.clone().unwrap_or_default())?;
        writeln!(f, "CREATED_AT:{}", data.created_at.to_rfc3339())?;
        writeln!(f, "UPDATED_AT:{}", data.updated_at.to_rfc3339())?;
        writeln!(f, "VERSION:{}", data.version)?;
        writeln!(f, "RUNTIME_TYPE:{:?}", data.runtime_type)?;
        writeln!(f, "ENABLE_K8S_API:{}", data.enable_k8s_api)?;
        writeln!(f, "ENABLE_CONTAINER_EXPORTER:{}", data.enable_container_exporter)?;
        writeln!(f, "ENABLE_GPU_EXPORTER:{}", data.enable_gpu_exporter)?;
        writeln!(f, "GPU_EXPORTER_URLS:{}", data.gpu_exporter_urls.join(", "))?;
        writeln!(f, "CONTAINER_EXPORTER_URLS:{}", data.container_exporter_urls.join(", "))?;
        writeln!(
            f,
            "K8S_API_URL:{}",
            data.k8s_api_url.clone().unwrap_or_default()
        )?;

        f.flush()?;

        fs::rename(&tmp_path, &path).context("Failed to finalize settings file")?;
        Ok(())
    }
}
