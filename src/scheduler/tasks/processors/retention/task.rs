use anyhow::Result;
use chrono::{Utc, Duration};
use std::fs;
use std::path::Path;
use tracing::info;

const BASE_PATH: &str = "/data/rustcost/tsdb";
const RETAIN_DAYS: i64 = 30;

pub async fn run() -> Result<()> {
    info!("Running retention cleanup...");
    let cutoff = Utc::now() - Duration::days(RETAIN_DAYS);

    for subdir in ["minute", "hour", "day"] {
        let dir = format!("{BASE_PATH}/{subdir}");
        cleanup_old_files(&dir, cutoff.timestamp())?;
    }

    Ok(())
}

fn cleanup_old_files<P: AsRef<Path>>(path: P, cutoff: i64) -> Result<()> {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let p = entry.path();
            if let Ok(meta) = entry.metadata() {
                if let Ok(modified) = meta.modified() {
                    if let Ok(secs) = modified.duration_since(std::time::UNIX_EPOCH) {
                        if secs.as_secs() < cutoff as u64 {
                            let _ = fs::remove_file(&p);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
