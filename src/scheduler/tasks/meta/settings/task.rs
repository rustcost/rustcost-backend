use anyhow::Result;
use super::{models::Settings, repository};
use std::path::Path;
use crate::scheduler::tasks::meta::settings::repository::{read_settings, write_settings};

/// Always re-read settings.rci every call; create if missing.
/// Load settings, create defaults if missing.
pub fn load_or_init_settings() -> Result<Settings> {
    if !Path::new(crate::scheduler::tasks::meta::settings::repository::PATH).exists() {
        let default = Settings::default();
        write_settings(default.clone())?;
        return Ok(default);
    }
    read_settings()
}
