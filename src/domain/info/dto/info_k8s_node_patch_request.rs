
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct InfoK8sNodePatchRequest {
    // --- Team / Service metadata (NEW) ---
    pub team: Option<String>,
    pub service: Option<String>,
    pub env: Option<String>, // "dev", "stage", "prod"
}
