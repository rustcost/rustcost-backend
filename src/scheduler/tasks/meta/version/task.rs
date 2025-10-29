use anyhow::Result;
use std::path::Path;
use super::{client, repository, models::VersionInfo};

/// If version.rci exists → read it; otherwise fetch and create it.
pub async fn load_or_init_version() -> Result<VersionInfo> {
    let path = "data/meta/version.rci";

    if Path::new(path).exists() {
        repository::read_version()
    } else {
        let info = client::fetch_version().await?;
        repository::write_version(&info)?;
        Ok(info)
    }
}
