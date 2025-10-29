use anyhow::Result;
use chrono::{Utc, DateTime};
use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Write},
    path::Path,
};
use super::models::Settings;

pub(crate) const PATH: &str = "data/meta/settings.rci";

/// Read settings.rci with key:value pairs and buffered I/O.
/// Missing or unknown keys are ignored, defaults are applied.
pub fn read_settings() -> Result<Settings> {
    if !Path::new(PATH).exists() {
        return Ok(Settings::default());
    }

    let file = File::open(PATH)?;
    let reader = BufReader::new(file);
    let mut s = Settings::default();

    for line in reader.lines() {
        let line = line?;
        if let Some((key, val)) = line.split_once(':') {
            let val = val.trim();
            match key.trim().to_uppercase().as_str() {
                // --- General ---
                "IS_DARK_MODE" => s.is_dark_mode = val.eq_ignore_ascii_case("true"),
                "LANGUAGE" => s.language = val.to_string(),
                "RETENTION_MONTH" => s.retention_month = val.parse().unwrap_or(s.retention_month),
                "RETENTION_POLICY" => s.retention_policy = val.to_string(),

                // --- File-based TSDB ---
                "ENABLE_LINE_NUM_TRACKING" => s.enable_line_num_tracking = val.eq_ignore_ascii_case("true"),
                "ENABLE_INDEX_FILE" => s.enable_index_file = val.eq_ignore_ascii_case("true"),
                "MAX_STORAGE_GB" => s.max_storage_gb = val.parse().unwrap_or(s.max_storage_gb),
                "COMPRESSION_ENABLED" => s.compression_enabled = val.eq_ignore_ascii_case("true"),

                // --- Metrics ---
                "SCRAPE_INTERVAL_SEC" => s.scrape_interval_sec = val.parse().unwrap_or(s.scrape_interval_sec),
                "METRICS_BATCH_SIZE" => s.metrics_batch_size = val.parse().unwrap_or(s.metrics_batch_size),
                "ENABLE_GPU_METRICS" => s.enable_gpu_metrics = val.eq_ignore_ascii_case("true"),
                "ENABLE_NETWORK_METRICS" => s.enable_network_metrics = val.eq_ignore_ascii_case("true"),

                // --- Alerts ---
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

                // --- LLM ---
                "LLM_URL" => s.llm_url = if val.is_empty() { None } else { Some(val.to_string()) },
                "LLM_TOKEN" => s.llm_token = if val.is_empty() { None } else { Some(val.to_string()) },
                "LLM_MODEL" => s.llm_model = if val.is_empty() { None } else { Some(val.to_string()) },

                // --- Metadata ---
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

                _ => {}
            }
        }
    }

    Ok(s)
}

/// Write settings.rci atomically for crash safety.
/// Automatically refreshes `updated_at` timestamp.
pub fn write_settings(mut s: Settings) -> Result<()> {
    if let Some(dir) = Path::new(PATH).parent() {
        fs::create_dir_all(dir)?;
    }

    s.updated_at = Utc::now();
    let tmp_path = format!("{PATH}.tmp");

    {
        let mut f = File::create(&tmp_path)?;
        writeln!(f, "IS_DARK_MODE:{}", s.is_dark_mode)?;
        writeln!(f, "LANGUAGE:{}", s.language)?;
        writeln!(f, "RETENTION_MONTH:{}", s.retention_month)?;
        writeln!(f, "RETENTION_POLICY:{}", s.retention_policy)?;
        writeln!(f, "ENABLE_LINE_NUM_TRACKING:{}", s.enable_line_num_tracking)?;
        writeln!(f, "ENABLE_INDEX_FILE:{}", s.enable_index_file)?;
        writeln!(f, "MAX_STORAGE_GB:{}", s.max_storage_gb)?;
        writeln!(f, "COMPRESSION_ENABLED:{}", s.compression_enabled)?;
        writeln!(f, "SCRAPE_INTERVAL_SEC:{}", s.scrape_interval_sec)?;
        writeln!(f, "METRICS_BATCH_SIZE:{}", s.metrics_batch_size)?;
        writeln!(f, "ENABLE_GPU_METRICS:{}", s.enable_gpu_metrics)?;
        writeln!(f, "ENABLE_NETWORK_METRICS:{}", s.enable_network_metrics)?;
        writeln!(f, "ENABLE_CLUSTER_HEALTH_ALERT:{}", s.enable_cluster_health_alert)?;
        writeln!(f, "ENABLE_RUSTCOST_HEALTH_ALERT:{}", s.enable_rustcost_health_alert)?;
        writeln!(f, "GLOBAL_ALERT_SUBJECT:{}", s.global_alert_subject)?;
        writeln!(f, "LINKBACK_URL:{}", s.linkback_url.clone().unwrap_or_default())?;
        writeln!(f, "EMAIL_RECIPIENTS:{}", s.email_recipients.join(","))?;
        writeln!(f, "SLACK_WEBHOOK_URL:{}", s.slack_webhook_url.clone().unwrap_or_default())?;
        writeln!(f, "TEAMS_WEBHOOK_URL:{}", s.teams_webhook_url.clone().unwrap_or_default())?;
        writeln!(f, "LLM_URL:{}", s.llm_url.clone().unwrap_or_default())?;
        writeln!(f, "LLM_TOKEN:{}", s.llm_token.clone().unwrap_or_default())?;
        writeln!(f, "LLM_MODEL:{}", s.llm_model.clone().unwrap_or_default())?;
        writeln!(f, "CREATED_AT:{}", s.created_at.to_rfc3339())?;
        writeln!(f, "UPDATED_AT:{}", s.updated_at.to_rfc3339())?;
        writeln!(f, "VERSION:{}", s.version)?;
        f.flush()?;
    }

    fs::rename(tmp_path, PATH)?;
    Ok(())
}

