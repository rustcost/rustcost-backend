use anyhow::Result;

pub mod version;
pub mod settings;

/// Combined data returned from meta init each minute
#[derive(Debug, Clone)]
pub struct MetaState {
    pub version: version::models::VersionInfo,
    pub settings: settings::models::Settings,
}

/// Ensures version.rci and settings.rci exist.
/// Always returns the latest settings (re-read every call).
pub async fn load_meta_state() -> Result<MetaState> {
    // --- Version: create if missing, otherwise read existing ---
    let version_info = version::task::load_or_init_version().await?;

    // --- Settings: create if missing, always re-read ---
    let settings_info = settings::task::load_or_init_settings()?;

    Ok(MetaState {
        version: version_info,
        settings: settings_info,
    })
}
