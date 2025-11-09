use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::env;
use crate::domain::info::dto::info_setting_upsert_request::InfoSettingUpsertRequest;

/// Global configuration for RustCost.
///
/// This structure defines all configurable aspects of the system,
/// including UI preferences, TSDB settings, metric collection intervals,
/// alert integrations, and optional LLM connectivity.
///
/// The configuration is persisted as a simple key–value file (`settings.rci`)
/// and can be serialized/deserialized as YAML or JSON as needed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfoSettingEntity {
    // ===== General & UI =====
    /// Enables dark mode for the RustCost web UI.
    pub is_dark_mode: bool,

    /// Display language (e.g. `"en"`, `"ko"`).
    pub language: String,

    /// Number of months to retain metric data before applying retention policy.
    /// Minute data (files named YYYY-MM-DD)
    pub minute_retention_days: u32,

    /// Hour data (files named YYYY-MM)
    pub hour_retention_months: u32,

    /// Day data (files named YYYY)
    pub day_retention_years: u32,

    /// Retention behavior: `"delete"` or `"archive"`.
    pub retention_policy: String,

    // ===== File-based Persistence Options =====
    /// Whether to include line numbers when writing records.
    pub enable_line_num_tracking: bool,

    /// Whether to create `.idx` index sidecar files for faster reads.
    pub enable_index_file: bool,

    /// Maximum local storage capacity in gigabytes before cleanup triggers.
    pub max_storage_gb: u32,

    /// Enables on-disk compression (gzip or zstd).
    pub compression_enabled: bool,

    // ===== Metrics Collection =====
    /// Scrape interval in seconds (e.g. 60 = every minute).
    pub scrape_interval_sec: u32,

    /// Number of metrics batched together when written to disk.
    pub metrics_batch_size: u32,

    // ===== Alerts & Notifications =====
    /// Enable cluster-level health monitoring alerts.
    pub enable_cluster_health_alert: bool,

    /// Enable internal RustCost health alerts.
    pub enable_rustcost_health_alert: bool,

    /// Default subject line for alert notifications.
    pub global_alert_subject: String,

    /// Optional URL to include in alert messages for reference.
    pub linkback_url: Option<String>,

    /// Global list of alert email recipients.
    pub email_recipients: Vec<String>,

    /// Optional Slack webhook for alert delivery.
    pub slack_webhook_url: Option<String>,

    /// Optional Microsoft Teams webhook for alert delivery.
    pub teams_webhook_url: Option<String>,

    // ===== LLM Integration =====
    /// Endpoint for an external LLM API (e.g., OpenAI, Anthropic).
    pub llm_url: Option<String>,

    /// API token for authenticating with the LLM provider.
    /// Should be masked when displayed.
    pub llm_token: Option<String>,

    /// Default model to use for LLM queries.
    pub llm_model: Option<String>,

    // ===== Metadata =====
    /// Configuration creation timestamp (UTC).
    pub created_at: DateTime<Utc>,

    /// Last update timestamp (UTC).
    pub updated_at: DateTime<Utc>,

    /// Version identifier for the configuration format.
    pub version: String,


    // ===== Runtime =====
    pub runtime_type: RuntimeType,
    pub enable_k8s_api: bool,
    pub enable_container_exporter: bool,
    pub enable_gpu_exporter: bool,

    pub gpu_exporter_urls: Vec<String>,
    pub container_exporter_urls: Vec<String>,
    pub k8s_api_url: Option<String>,
}

impl Default for InfoSettingEntity {
    fn default() -> Self {
        let now = Utc::now();

        Self {
            // --- General & UI ---
            is_dark_mode: false,
            language: "en".into(),
            minute_retention_days: 7,
            hour_retention_months: 12,
            day_retention_years: 30,
            retention_policy: "delete".into(),

            // --- Persistence ---
            enable_line_num_tracking: true,
            enable_index_file: true,
            max_storage_gb: 5,
            compression_enabled: true,

            // --- Metrics ---
            scrape_interval_sec: 60,
            metrics_batch_size: 500,

            // --- Alerts ---
            enable_cluster_health_alert: false,
            enable_rustcost_health_alert: false,
            global_alert_subject: "RustCost Alert".into(),
            linkback_url: None,
            email_recipients: vec![],
            slack_webhook_url: None,
            teams_webhook_url: None,

            // --- LLM ---
            llm_url: None,
            llm_token: None,
            llm_model: Some("gpt-4-turbo".into()),

            // --- Metadata ---
            created_at: now,
            updated_at: now,
            version: "1.0.0".into(),

            // --- Runtime ---
            runtime_type: RuntimeType::default(),

            enable_k8s_api: env::var("RUSTCOST_ENABLE_K8S_API")
                .map(|v| v == "true")
                .unwrap_or(true),

            enable_container_exporter: env::var("RUSTCOST_ENABLE_CONTAINER_EXPORTER")
                .map(|v| v == "true")
                .unwrap_or(true),

            enable_gpu_exporter: env::var("RUSTCOST_ENABLE_GPU_EXPORTER")
                .map(|v| v == "true")
                .unwrap_or(false),

            gpu_exporter_urls: env::var("RUSTCOST_GPU_EXPORTER_URLS")
                .ok()
                .filter(|v| !v.trim().is_empty())
                .map(|v| v.split(',').map(|s| s.trim().to_string()).collect())
                .unwrap_or_else(Vec::new),

            container_exporter_urls: env::var("RUSTCOST_CONTAINER_EXPORTER_URLS")
                .ok()
                .filter(|v| !v.trim().is_empty())
                .map(|v| v.split(',').map(|s| s.trim().to_string()).collect())
                .unwrap_or_else(Vec::new),

            k8s_api_url: env::var("RUSTCOST_K8S_API_URL").ok(),
        }
    }
}
impl InfoSettingEntity {
    pub fn apply_update(&mut self, req: InfoSettingUpsertRequest) {
        // === General & UI ===
        if let Some(v) = req.is_dark_mode {
            self.is_dark_mode = v;
        }
        if let Some(v) = req.language {
            self.language = v;
        }
        if let Some(v) = req.minute_retention_days {
            self.minute_retention_days = v;
        }
        if let Some(v) = req.hour_retention_months {
            self.hour_retention_months = v;
        }
        if let Some(v) = req.day_retention_years {
            self.day_retention_years = v;
        }
        if let Some(v) = req.retention_policy {
            self.retention_policy = v;
        }

        // === File-based Persistence ===
        if let Some(v) = req.enable_line_num_tracking {
            self.enable_line_num_tracking = v;
        }
        if let Some(v) = req.enable_index_file {
            self.enable_index_file = v;
        }
        if let Some(v) = req.max_storage_gb {
            self.max_storage_gb = v;
        }
        if let Some(v) = req.compression_enabled {
            self.compression_enabled = v;
        }

        // === Metrics ===
        if let Some(v) = req.scrape_interval_sec {
            self.scrape_interval_sec = v;
        }
        if let Some(v) = req.metrics_batch_size {
            self.metrics_batch_size = v;
        }

        // === Alerts & Notifications ===
        if let Some(v) = req.enable_cluster_health_alert {
            self.enable_cluster_health_alert = v;
        }
        if let Some(v) = req.enable_rustcost_health_alert {
            self.enable_rustcost_health_alert = v;
        }
        if let Some(v) = req.global_alert_subject {
            self.global_alert_subject = v;
        }
        if let Some(v) = req.email_recipients {
            self.email_recipients = v;
        }

        // Optional URLs and tokens (normalize empty strings → None)
        if let Some(v) = normalize_string_opt(req.linkback_url) {
            self.linkback_url = v;
        }
        if let Some(v) = normalize_string_opt(req.slack_webhook_url) {
            self.slack_webhook_url = v;
        }
        if let Some(v) = normalize_string_opt(req.teams_webhook_url) {
            self.teams_webhook_url = v;
        }
        if let Some(v) = normalize_string_opt(req.llm_url) {
            self.llm_url = v;
        }
        if let Some(v) = normalize_string_opt(req.llm_token) {
            self.llm_token = v;
        }
        if let Some(v) = normalize_string_opt(req.llm_model) {
            self.llm_model = v;
        }
        if let Some(v) = normalize_string_opt(req.k8s_api_url) {
            self.k8s_api_url = v;
        }

        // === Runtime ===
        if let Some(v) = req.runtime_type {
            self.runtime_type = match v.to_lowercase().as_str() {
                "docker" => RuntimeType::Docker,
                "containerd" => RuntimeType::Containerd,
                "baremetal" => RuntimeType::BareMetal,
                _ => RuntimeType::K8s,
            };
        }
        if let Some(v) = req.enable_k8s_api {
            self.enable_k8s_api = v;
        }
        if let Some(v) = req.enable_container_exporter {
            self.enable_container_exporter = v;
        }
        if let Some(v) = req.enable_gpu_exporter {
            self.enable_gpu_exporter = v;
        }
        if let Some(v) = req.gpu_exporter_urls {
            self.gpu_exporter_urls = v;
        }
        if let Some(v) = req.container_exporter_urls {
            self.container_exporter_urls = v;
        }

        // === Update timestamp ===
        self.updated_at = Utc::now();
    }
}

fn normalize_string_opt(v: Option<String>) -> Option<Option<String>> {
    match v {
        Some(s) if s.trim().is_empty() => Some(None),
        Some(s) => Some(Some(s)),
        None => None,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuntimeType {
    #[serde(rename = "k8s")]
    K8s,
    #[serde(rename = "docker")]
    Docker,
    #[serde(rename = "containerd")]
    Containerd,
    #[serde(rename = "baremetal")]
    BareMetal,
}

impl Default for RuntimeType {
    fn default() -> Self {
        match env::var("RUSTCOST_RUNTIME_TYPE")
            .unwrap_or_else(|_| "k8s".to_string())
            .to_lowercase()
            .as_str()
        {
            "docker" => RuntimeType::Docker,
            "containerd" => RuntimeType::Containerd,
            "baremetal" => RuntimeType::BareMetal,
            _ => RuntimeType::K8s,
        }
    }
}