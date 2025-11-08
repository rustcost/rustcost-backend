use super::info_unit_price_entity::InfoUnitPriceEntity;
use crate::core::persistence::info::fixed::info_fixed_fs_adapter_trait::InfoFixedFsAdapterTrait;
use anyhow::{Context, Result};
use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Write},
};
use crate::core::persistence::storage_path::info_unit_price_path;

/// File-based adapter for reading and writing [`InfoUnitPriceEntity`] data.
///
/// Uses a simple `key: value` text format for durability and transparency.
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
                let key = key.trim().to_lowercase();
                let val: f64 = val.trim().parse().unwrap_or_default();

                match key.as_str() {
                    // CPU
                    "cpu_core_hour" => entity.cpu_core_hour = val,
                    "cpu_spot_core_hour" => entity.cpu_spot_core_hour = val,

                    // Memory
                    "memory_gb_hour" => entity.memory_gb_hour = val,
                    "memory_spot_gb_hour" => entity.memory_spot_gb_hour = val,

                    // GPU
                    "gpu_hour" => entity.gpu_hour = val,
                    "gpu_spot_hour" => entity.gpu_spot_hour = val,

                    // Storage
                    "storage_gb_hour" => entity.storage_gb_hour = val,

                    // Network
                    "network_local_gb" => entity.network_local_gb = val,
                    "network_regional_gb" => entity.network_regional_gb = val,
                    "network_external_gb" => entity.network_external_gb = val,

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
    fn write(&self, data: &InfoUnitPriceEntity) -> Result<()> {
        let path = info_unit_price_path();

        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir).context("Failed to create unit price directory")?;
        }

        let tmp_path = path.join(".tmp");
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

        f.flush()?;
        fs::rename(&tmp_path, &path).context("Failed to finalize unit price file")?;
        Ok(())
    }
}
