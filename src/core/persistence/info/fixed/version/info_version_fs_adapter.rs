use super::info_version_entity::InfoVersionEntity;
use crate::core::persistence::info::fixed::info_fixed_fs_adapter_trait::InfoFixedFsAdapterTrait;
use anyhow::{Context, Result};
use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Write},
    path::Path,
};

/// Location of the persisted version file.
const PATH: &str = "data/info/version.rci";

/// File-based FS_ADAPTER implementation for the `VersionInfo` entity.
///
/// Provides lightweight read/write/update/delete operations for
/// the version file, using a simple key–value text format for
/// straightforward parsing and atomic write for durability.
pub struct InfoVersionFsAdapter;

impl InfoFixedFsAdapterTrait<InfoVersionEntity> for InfoVersionFsAdapter {
    /// Reads the version information file into memory.
    /// Returns default values if the file does not exist.
    fn read(&self) -> Result<InfoVersionEntity> {
        if !Path::new(PATH).exists() {
            return Ok(InfoVersionEntity::default());
        }

        let file = File::open(PATH).context("Failed to open version file")?;
        let reader = BufReader::new(file);
        let mut v = InfoVersionEntity::default();

        for line in reader.lines() {
            let line = line?;
            if let Some((key, val)) = line.split_once(':') {
                let key = key.trim().to_uppercase();
                let val = val.trim();

                match key.as_str() {
                    "DATE" => v.date = val.to_string(),
                    "MAJOR" => v.major = val.to_string(),
                    "MINOR" => v.minor = val.to_string(),
                    "GIT_VERSION" => v.git_version = val.to_string(),
                    "GIT_COMMIT" => v.git_commit = val.to_string(),
                    "BUILD_DATE" => v.build_date = val.to_string(),
                    "GO_VERSION" => v.go_version = val.to_string(),
                    "COMPILER" => v.compiler = val.to_string(),
                    "PLATFORM" => v.platform = val.to_string(),
                    _ => {}
                }
            }
        }

        Ok(v)
    }

    fn insert(&self, data: &InfoVersionEntity) -> Result<()> {
        self.write(data)
    }

    fn update(&self, data: &InfoVersionEntity) -> Result<()> {
        self.write(data)
    }

    fn delete(&self) -> Result<()> {
        if Path::new(PATH).exists() {
            fs::remove_file(PATH).context("Failed to delete version file")?;
        }
        Ok(())
    }
}

impl InfoVersionFsAdapter {
    /// Internal helper to atomically write the version file.
    fn write(&self, data: &InfoVersionEntity) -> Result<()> {
        if let Some(dir) = Path::new(PATH).parent() {
            fs::create_dir_all(dir).context("Failed to create version directory")?;
        }

        let tmp_path = format!("{PATH}.tmp");
        let mut f = File::create(&tmp_path).context("Failed to create temp file")?;

        writeln!(f, "DATE:{}", data.date)?;
        writeln!(f, "MAJOR:{}", data.major)?;
        writeln!(f, "MINOR:{}", data.minor)?;
        writeln!(f, "GIT_VERSION:{}", data.git_version)?;
        writeln!(f, "GIT_COMMIT:{}", data.git_commit)?;
        writeln!(f, "BUILD_DATE:{}", data.build_date)?;
        writeln!(f, "GO_VERSION:{}", data.go_version)?;
        writeln!(f, "COMPILER:{}", data.compiler)?;
        writeln!(f, "PLATFORM:{}", data.platform)?;
        f.flush()?;

        fs::rename(&tmp_path, PATH).context("Failed to finalize version file")?;
        Ok(())
    }
}
