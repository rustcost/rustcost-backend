use anyhow::Result;
use chrono::{Datelike, Utc};
use std::fs;
use std::path::PathBuf;
use tracing::{debug, error};

const BASE_PATH: &str = "/data/rustcost/tsdb";

pub async fn run() -> Result<()> {
    debug!("Running daily aggregation task...");

    let now = Utc::now();
    let day_path = format!("{BASE_PATH}/day/{:04}-{:02}-{:02}.tsv",
                           now.year(), now.month(), now.day());
    let hour_dir = format!("{BASE_PATH}/hour/{:04}-{:02}-{:02}/",
                           now.year(), now.month(), now.day());

    let files = read_hour_files(&hour_dir);
    if files.is_empty() {
        debug!("No hourly files to aggregate today");
        return Ok(());
    }

    let mut aggregated: Vec<String> = Vec::new();
    for file in files {
        if let Ok(contents) = fs::read_to_string(&file) {
            aggregated.push(contents);
        }
    }

    fs::create_dir_all(PathBuf::from(&day_path).parent().unwrap())?;
    fs::write(&day_path, aggregated.join("\n"))?;

    debug!(path = %day_path, "Daily aggregation complete");
    Ok(())
}

fn read_hour_files(base: &str) -> Vec<PathBuf> {
    let mut result = Vec::new();
    if let Ok(entries) = fs::read_dir(base) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.ends_with(".tsv") {
                    result.push(path);
                }
            }
        }
    }
    result
}
