use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::domain::info::dto::info_unit_price_upsert_request::InfoUnitPriceUpsertRequest;

/// Represents per-unit pricing configuration for system resource usage.
///
/// Each field defines the cost of consuming a single unit of that resource.
/// These values can be customized to match your environment or cloud provider.
/// Typically, the cost is expressed per hour of usage or per GB transferred.
///
/// Example use:
/// Combine this configuration with resource metrics (e.g., [`MetricNodeEntity`])
/// to estimate the operational cost of a node, pod, or container.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfoUnitPriceEntity {
    // --- CPU ---
    /// Price per CPU core-hour
    pub cpu_core_hour: f64,
    /// Price per CPU core-hour for spot, preemptible, or discounted nodes
    pub cpu_spot_core_hour: f64,

    // --- Memory ---
    /// Price per GB-hour of memory
    pub memory_gb_hour: f64,
    /// Price per GB-hour of memory on spot/preemptible nodes
    pub memory_spot_gb_hour: f64,

    // --- GPU ---
    /// Price per GPU-hour
    pub gpu_hour: f64,
    /// Price per GPU-hour for spot or preemptible GPUs
    pub gpu_spot_hour: f64,

    // --- Storage ---
    /// Price per GB-hour of storage usage
    pub storage_gb_hour: f64,

    // --- Network ---
    /// Price per GB transferred within the same availability zone
    pub network_local_gb: f64,
    /// Price per GB transferred within the same region
    pub network_regional_gb: f64,
    /// Price per GB transferred to external networks (internet egress)
    pub network_external_gb: f64,

    /// Last update timestamp (UTC).
    pub updated_at: DateTime<Utc>,
}

impl InfoUnitPriceEntity {
    pub fn apply_update(&mut self, req: InfoUnitPriceUpsertRequest) {
        if let Some(v) = req.cpu_core_hour { self.cpu_core_hour = v; }
        if let Some(v) = req.cpu_spot_core_hour { self.cpu_spot_core_hour = v; }
        if let Some(v) = req.memory_gb_hour { self.memory_gb_hour = v; }
        if let Some(v) = req.memory_spot_gb_hour { self.memory_spot_gb_hour = v; }
        if let Some(v) = req.gpu_hour { self.gpu_hour = v; }
        if let Some(v) = req.gpu_spot_hour { self.gpu_spot_hour = v; }
        if let Some(v) = req.storage_gb_hour { self.storage_gb_hour = v; }
        if let Some(v) = req.network_local_gb { self.network_local_gb = v; }
        if let Some(v) = req.network_regional_gb { self.network_regional_gb = v; }
        if let Some(v) = req.network_external_gb { self.network_external_gb = v; }
    }
}

impl Default for InfoUnitPriceEntity {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            cpu_core_hour: 0.031 / (30.0 * 24.0),         // Convert rough monthly → hour
            cpu_spot_core_hour: 0.006 / (30.0 * 24.0),
            memory_gb_hour: 0.004 / (30.0 * 24.0),
            memory_spot_gb_hour: 0.001 / (30.0 * 24.0),
            gpu_hour: 0.90 / (30.0 * 24.0),
            gpu_spot_hour: 0.25 / (30.0 * 24.0),
            storage_gb_hour: 0.00005 / (30.0 * 24.0),
            network_local_gb: 0.01,
            network_regional_gb: 0.01,
            network_external_gb: 0.12,
            updated_at: now,
        }
    }
}