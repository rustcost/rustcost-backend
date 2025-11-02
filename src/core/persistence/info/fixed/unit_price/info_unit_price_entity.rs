use chrono::Utc;
use serde::{Serialize, Deserialize};
use crate::core::persistence::info::fixed::setting::info_setting_entity::InfoSettingEntity;

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
}


impl Default for InfoUnitPriceEntity {
    fn default() -> Self {
        Self {
            cpu_core_hour: 0.031 / (30.0 * 24.0),         // Convert rough monthly → hourly
            cpu_spot_core_hour: 0.006 / (30.0 * 24.0),
            memory_gb_hour: 0.004 / (30.0 * 24.0),
            memory_spot_gb_hour: 0.001 / (30.0 * 24.0),
            gpu_hour: 0.90 / (30.0 * 24.0),
            gpu_spot_hour: 0.25 / (30.0 * 24.0),
            storage_gb_hour: 0.00005 / (30.0 * 24.0),
            network_local_gb: 0.01,
            network_regional_gb: 0.01,
            network_external_gb: 0.12,
        }
    }
}