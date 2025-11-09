use super::info_unit_price_entity::InfoUnitPriceEntity;
use crate::core::persistence::info::fixed::info_fixed_fs_adapter_trait::InfoFixedFsAdapterTrait;
use crate::core::persistence::storage_path::info_unit_price_path;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Write},
};

/// File-based adapter for reading and writing [`InfoUnitPriceEntity`] data.
///
/// Uses a simple `key: value` text format with all keys in snake_case.
pub struct InfoUnitPriceFsAdapter;

impl InfoFixedFsAdapterTrait<InfoUnitPriceEntity> for InfoUnitPriceFsAdapter {
    /// Reads the unit price configuration from disk.
    /// Returns default values if the file does not exist.
    fn read(&self) -> Result<InfoUnitPriceEntity> {
        let path = info_unit_price_path();

        if !path.exists() {
            return Ok(InfoUnitPriceEntity::default());
        }

        let file = File::open(&path).context("Failed to open unit price file")?;
        let reader = BufReader::new(file);
        let mut entity = InfoUnitPriceEntity::default();

        for line in reader.lines() {
            let line = line?;
            if let Some((key, val)) = line.split_once(':') {
                let key = key.trim().to_lowercase(); // normalize key
                let val = val.trim();

                match key.as_str() {
                    // CPU
                    "cpu_core_hour" => entity.cpu_core_hour = val.parse().unwrap_or_default(),
                    "cpu_spot_core_hour" => entity.cpu_spot_core_hour = val.parse().unwrap_or_default(),

                    // Memory
                    "memory_gb_hour" => entity.memory_gb_hour = val.parse().unwrap_or_default(),
                    "memory_spot_gb_hour" => entity.memory_spot_gb_hour = val.parse().unwrap_or_default(),

                    // GPU
                    "gpu_hour" => entity.gpu_hour = val.parse().unwrap_or_default(),
                    "gpu_spot_hour" => entity.gpu_spot_hour = val.parse().unwrap_or_default(),

                    // Storage
                    "storage_gb_hour" => entity.storage_gb_hour = val.parse().unwrap_or_default(),

                    // Network
                    "network_local_gb" => entity.network_local_gb = val.parse().unwrap_or_default(),
                    "network_regional_gb" => entity.network_regional_gb = val.parse().unwrap_or_default(),
                    "network_external_gb" => entity.network_external_gb = val.parse().unwrap_or_default(),

                    // Updated timestamp
                    "updated_at" => {
                        if let Ok(parsed) = DateTime::parse_from_rfc3339(val) {
                            entity.updated_at = parsed.with_timezone(&Utc);
                        }
                    }

                    _ => {}
                }
            }
        }

        Ok(entity)
    }

    fn insert(&self, data: &InfoUnitPriceEntity) -> Result<()> {
        self.write(data)
    }

    fn update(&self, data: &InfoUnitPriceEntity) -> Result<()> {
        self.write(data)
    }

    fn delete(&self) -> Result<()> {
        let path = info_unit_price_path();

        if path.exists() {
            fs::remove_file(&path).context("Failed to delete unit price file")?;
        }

        Ok(())
    }
}

impl InfoUnitPriceFsAdapter {
    /// Writes the unit price configuration to disk atomically.
    /// All keys are written in snake_case for consistency.
    fn write(&self, data: &InfoUnitPriceEntity) -> Result<()> {
        let path = info_unit_price_path();

        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir).context("Failed to create unit price directory")?;
        }

        let tmp_path = path.with_extension("tmp");
        let mut f = File::create(&tmp_path).context("Failed to create temp file")?;

        // CPU
        writeln!(f, "cpu_core_hour:{}", data.cpu_core_hour)?;
        writeln!(f, "cpu_spot_core_hour:{}", data.cpu_spot_core_hour)?;

        // Memory
        writeln!(f, "memory_gb_hour:{}", data.memory_gb_hour)?;
        writeln!(f, "memory_spot_gb_hour:{}", data.memory_spot_gb_hour)?;

        // GPU
        writeln!(f, "gpu_hour:{}", data.gpu_hour)?;
        writeln!(f, "gpu_spot_hour:{}", data.gpu_spot_hour)?;

        // Storage
        writeln!(f, "storage_gb_hour:{}", data.storage_gb_hour)?;

        // Network
        writeln!(f, "network_local_gb:{}", data.network_local_gb)?;
        writeln!(f, "network_regional_gb:{}", data.network_regional_gb)?;
        writeln!(f, "network_external_gb:{}", data.network_external_gb)?;

        // Timestamp
        writeln!(f, "updated_at:{}", data.updated_at.to_rfc3339())?;

        f.flush()?;
        fs::rename(&tmp_path, &path).context("Failed to finalize unit price file")?;
        Ok(())
    }
}
