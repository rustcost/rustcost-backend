//! Info domain DTOs

pub mod info_setting_upsert_request;
pub mod info_unit_price_upsert_request;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsDto {
    pub currency: Option<String>,
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitPriceDto {
    pub cpu_per_core_hour: f64,
    pub mem_per_gb_hour: f64,
    pub storage_per_gb_hour: f64,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionDto {
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfoDto {
    pub node_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodInfoDto {
    pub pod_uid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerInfoDto {
    pub id: String,
}

