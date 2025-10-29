use anyhow::Result;
use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Write},
    path::Path,
};
use super::models::VersionInfo;

const PATH: &str = "data/meta/version.rci";

/// Read version.rci using buffered I/O.
pub fn read_version() -> Result<VersionInfo> {
    let file = File::open(PATH)?;
    let reader = BufReader::new(file);

    let mut v = VersionInfo {
        date: String::new(),
        major: String::new(),
        minor: String::new(),
        git_version: String::new(),
        git_commit: String::new(),
        build_date: String::new(),
        go_version: String::new(),
        compiler: String::new(),
        platform: String::new(),
    };

    for line in reader.lines() {
        let line = line?;
        if let Some((key, val)) = line.split_once(':') {
            let val = val.trim().to_string();
            match key {
                "DATE" => v.date = val,
                "MAJOR" => v.major = val,
                "MINOR" => v.minor = val,
                "GIT_VERSION" => v.git_version = val,
                "GIT_COMMIT" => v.git_commit = val,
                "BUILD_DATE" => v.build_date = val,
                "GO_VERSION" => v.go_version = val,
                "COMPILER" => v.compiler = val,
                "PLATFORM" => v.platform = val,
                _ => {}
            }
        }
    }

    Ok(v)
}

/// Write version.rci atomically (safe and durable).
pub fn write_version(v: &VersionInfo) -> Result<()> {
    if let Some(dir) = Path::new(PATH).parent() {
        fs::create_dir_all(dir)?;
    }

    let tmp_path = format!("{PATH}.tmp");
    {
        let mut tmp_file = File::create(&tmp_path)?;
        writeln!(tmp_file, "DATE:{}", v.date)?;
        writeln!(tmp_file, "MAJOR:{}", v.major)?;
        writeln!(tmp_file, "MINOR:{}", v.minor)?;
        writeln!(tmp_file, "GIT_VERSION:{}", v.git_version)?;
        writeln!(tmp_file, "GIT_COMMIT:{}", v.git_commit)?;
        writeln!(tmp_file, "BUILD_DATE:{}", v.build_date)?;
        writeln!(tmp_file, "GO_VERSION:{}", v.go_version)?;
        writeln!(tmp_file, "COMPILER:{}", v.compiler)?;
        writeln!(tmp_file, "PLATFORM:{}", v.platform)?;
        tmp_file.flush()?; // ensure data hits disk
    }

    // Atomic rename replaces the old file safely
    fs::rename(tmp_path, PATH)?;
    Ok(())
}
