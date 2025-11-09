use serde::{Deserialize, Serialize};
use validator::Validate;

/// Represents an upsert (create/update) request for InfoSettingEntity.
/// All fields are optional to allow partial updates.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct InfoSettingUpsertRequest {
    // ===== General & UI =====
    /// Enables dark mode for the RustCost web UI.
    pub is_dark_mode: Option<bool>,

    /// Display language (e.g. "en", "ko").
    #[validate(length(min = 2, max = 10))]
    pub language: Option<String>,

    /// Number of days to retain minute-level metric data.
    pub minute_retention_days: Option<u32>,

    /// Number of months to retain hour-level metric data.
    pub hour_retention_months: Option<u32>,

    /// Number of years to retain day-level metric data.
    pub day_retention_years: Option<u32>,

    /// Retention behavior: "delete" or "archive".
    #[validate(length(min = 3))]
    pub retention_policy: Option<String>,

    // ===== File-based Persistence Options =====
    /// Whether to include line numbers when writing records.
    pub enable_line_num_tracking: Option<bool>,

    /// Whether to create `.idx` index sidecar files for faster reads.
    pub enable_index_file: Option<bool>,

    /// Maximum local storage capacity in gigabytes before cleanup triggers.
    pub max_storage_gb: Option<u32>,

    /// Enables on-disk compression (gzip or zstd).
    pub compression_enabled: Option<bool>,

    // ===== Metrics Collection =====
    /// Scrape interval in seconds (e.g. 60 = every minute).
    pub scrape_interval_sec: Option<u32>,

    /// Number of metrics batched together when written to disk.
    pub metrics_batch_size: Option<u32>,

    // ===== Alerts & Notifications =====
    /// Enable cluster-level health monitoring alerts.
    pub enable_cluster_health_alert: Option<bool>,

    /// Enable internal RustCost health alerts.
    pub enable_rustcost_health_alert: Option<bool>,

    /// Default subject line for alert notifications.
    #[validate(length(min = 1, max = 100))]
    pub global_alert_subject: Option<String>,

    /// Optional URL to include in alert messages for reference.
    #[validate(url)]
    pub linkback_url: Option<String>,

    /// Global list of alert email recipients.
    pub email_recipients: Option<Vec<String>>,

    /// Optional Slack webhook for alert delivery.
    #[validate(url)]
    pub slack_webhook_url: Option<String>,

    /// Optional Microsoft Teams webhook for alert delivery.
    #[validate(url)]
    pub teams_webhook_url: Option<String>,

    // ===== LLM Integration =====
    /// Endpoint for an external LLM API (e.g., OpenAI, Anthropic).
    #[validate(url)]
    pub llm_url: Option<String>,

    /// API token for authenticating with the LLM provider.
    /// Should be masked when displayed.
    #[validate(length(min = 10))]
    pub llm_token: Option<String>,

    /// Default model to use for LLM queries.
    #[validate(length(min = 2))]
    pub llm_model: Option<String>,

    // ===== Runtime =====
    /// Runtime environment type, e.g. "standalone", "k8s".
    pub runtime_type: Option<String>,

    /// Enable Kubernetes API metrics collection.
    pub enable_k8s_api: Option<bool>,

    /// Enable container exporter.
    pub enable_container_exporter: Option<bool>,

    /// Enable GPU exporter.
    pub enable_gpu_exporter: Option<bool>,

    /// GPU exporter endpoint URLs.
    pub gpu_exporter_urls: Option<Vec<String>>,

    /// Container exporter endpoint URLs.
    pub container_exporter_urls: Option<Vec<String>>,

    /// Optional Kubernetes API endpoint.
    #[validate(url)]
    pub k8s_api_url: Option<String>,
}
