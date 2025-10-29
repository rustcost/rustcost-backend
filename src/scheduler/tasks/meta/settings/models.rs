use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

/// Global configuration for RustCost.
/// Persisted as YAML: `data/meta/settings.yaml`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    // ===== General & UI =====
    pub is_dark_mode: bool,
    pub language: String,              // "en", "ko", etc.
    pub retention_month: u32,          // how long to keep metrics
    pub retention_policy: String,      // "delete" | "archive"

    // ===== File-based TSDB Options =====
    pub enable_line_num_tracking: bool, // include line number in records
    pub enable_index_file: bool,        // generate .idx sidecar
    pub max_storage_gb: u32,            // optional cap for local data
    pub compression_enabled: bool,      // enable gzip or zstd compression

    // ===== Metrics Collection =====
    pub scrape_interval_sec: u32,       // e.g. 60 seconds
    pub metrics_batch_size: u32,        // e.g. 500
    pub enable_gpu_metrics: bool,
    pub enable_network_metrics: bool,

    // ===== Alert & Notification =====
    pub enable_cluster_health_alert: bool,
    pub enable_rustcost_health_alert: bool,
    pub global_alert_subject: String,
    pub linkback_url: Option<String>,          // for alert messages
    pub email_recipients: Vec<String>,         // list of global recipients
    pub slack_webhook_url: Option<String>,
    pub teams_webhook_url: Option<String>,

    // ===== LLM Integration =====
    pub llm_url: Option<String>,        // e.g. https://api.openai.com/v1/chat/completions
    pub llm_token: Option<String>,      // API key (mask when displayed)
    pub llm_model: Option<String>,      // e.g. gpt-4-turbo

    // ===== Metadata =====
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: String,
}

impl Default for Settings {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            // --- General ---
            is_dark_mode: false,
            language: "en".to_string(),
            retention_month: 6,
            retention_policy: "delete".to_string(),

            // --- File-based TSDB ---
            enable_line_num_tracking: true,
            enable_index_file: true,
            max_storage_gb: 5,
            compression_enabled: true,

            // --- Metrics ---
            scrape_interval_sec: 60,
            metrics_batch_size: 500,
            enable_gpu_metrics: false,
            enable_network_metrics: true,

            // --- Alerts ---
            enable_cluster_health_alert: false,
            enable_rustcost_health_alert: false,
            global_alert_subject: "RustCost Alert".to_string(),
            linkback_url: None,
            email_recipients: vec![],
            slack_webhook_url: None,
            teams_webhook_url: None,

            // --- LLM ---
            llm_url: None,
            llm_token: None,
            llm_model: Some("gpt-4-turbo".to_string()),

            // --- Metadata ---
            created_at: now,
            updated_at: now,
            version: "1.0.0".to_string(),
        }
    }
}
