use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
    pub retention_month: u32,

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

    /// Collect GPU metrics (if available).
    pub enable_gpu_metrics: bool,

    /// Collect network-related metrics.
    pub enable_network_metrics: bool,

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
}

impl Default for InfoSettingEntity {
    fn default() -> Self {
        let now = Utc::now();

        Self {
            // --- General & UI ---
            is_dark_mode: false,
            language: "en".into(),
            retention_month: 6,
            retention_policy: "delete".into(),

            // --- Persistence ---
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
        }
    }
}
