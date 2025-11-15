use super::info_version_entity::InfoVersionEntity;
use crate::core::persistence::info::fixed::info_fixed_fs_adapter_trait::InfoFixedFsAdapterTrait;
use anyhow::{Context, Result};
use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Write},
};
use crate::core::persistence::storage_path::{info_version_path};

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
        let path = info_version_path();

        if !path.exists() {
            return Ok(InfoVersionEntity::default());
        }

        let file = File::open(&path).context("Failed to open version file")?;
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
        let path = info_version_path();

        if path.exists() {
            fs::remove_file(&path).context("Failed to delete version file")?;
        }
        Ok(())
    }
}

impl InfoVersionFsAdapter {
    /// Internal helper to atomically write the version file.
    fn write(&self, data: &InfoVersionEntity) -> Result<()> {
        use std::io::Write;

        let path = info_version_path();

        // Ensure parent directory exists
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)
                .context("Failed to create version directory")?;
        }

        // Temporary file: version.tmp
        let tmp_path = path.with_extension("tmp");

        // Open temporary file
        let mut f = File::create(&tmp_path)
            .context("Failed to create temporary version file")?;

        // ----- Write all fields -----
        writeln!(f, "DATE:{}", data.date)?;
        writeln!(f, "MAJOR:{}", data.major)?;
        writeln!(f, "MINOR:{}", data.minor)?;
        writeln!(f, "GIT_VERSION:{}", data.git_version)?;
        writeln!(f, "GIT_COMMIT:{}", data.git_commit)?;
        writeln!(f, "BUILD_DATE:{}", data.build_date)?;
        writeln!(f, "GO_VERSION:{}", data.go_version)?;
        writeln!(f, "COMPILER:{}", data.compiler)?;
        writeln!(f, "PLATFORM:{}", data.platform)?;

        // ----- Flush + sync -----
        f.flush()?;
        f.sync_all()
            .context("Failed to fsync temporary version file")?;

        // ----- Atomic rename -----
        fs::rename(&tmp_path, &path)
            .context("Failed to finalize version file atomically")?;

        // ----- fsync directory (required for true durability) -----
        #[cfg(unix)]
        if let Some(dir) = path.parent() {
            let dir_file = File::open(dir)
                .context("Failed to open version directory for fsync")?;
            dir_file.sync_all()
                .context("Failed to fsync version directory")?;
        }

        Ok(())
    }
}
