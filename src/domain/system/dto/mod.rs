//! System domain DTOs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusDto {
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthDto {
    pub healthy: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupJobDto {
    pub id: String,
    pub state: String,
}

