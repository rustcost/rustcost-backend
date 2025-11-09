use serde::{Deserialize, Serialize};
use validator::Validate;

/// Represents an upsert (create/update) request for `InfoUnitPriceEntity`.
///
/// All fields are optional to allow partial updates.
/// Each value represents the price per *unit* of resource usage (usually per hour).
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct InfoUnitPriceUpsertRequest {
    // --- CPU ---
    /// Price per CPU core-hour.
    pub cpu_core_hour: Option<f64>,

    /// Price per CPU core-hour for spot, preemptible, or discounted nodes.
    pub cpu_spot_core_hour: Option<f64>,

    // --- Memory ---
    /// Price per GB-hour of memory.
    pub memory_gb_hour: Option<f64>,

    /// Price per GB-hour of memory on spot/preemptible nodes.
    pub memory_spot_gb_hour: Option<f64>,

    // --- GPU ---
    /// Price per GPU-hour.
    pub gpu_hour: Option<f64>,

    /// Price per GPU-hour for spot or preemptible GPUs.
    pub gpu_spot_hour: Option<f64>,

    // --- Storage ---
    /// Price per GB-hour of storage usage.
    pub storage_gb_hour: Option<f64>,

    // --- Network ---
    /// Price per GB transferred within the same availability zone.
    pub network_local_gb: Option<f64>,

    /// Price per GB transferred within the same region.
    pub network_regional_gb: Option<f64>,

    /// Price per GB transferred to external networks (internet egress).
    pub network_external_gb: Option<f64>,
}
