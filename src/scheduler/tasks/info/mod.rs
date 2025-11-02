use anyhow::Result;

pub mod version;
pub mod settings;
pub mod unit_price;

use crate::core::persistence::info::fixed::setting::info_setting_entity::InfoSettingEntity;
use crate::core::persistence::info::fixed::version::info_version_entity::InfoVersionEntity;

/// Combined data returned from info init each minute
#[derive(Debug, Clone)]
pub struct InfoSate {
    pub version: InfoVersionEntity,
    pub settings: InfoSettingEntity,
}

/// Ensures version.rci and settings.rci exist.
/// Always returns the latest settings (re-read every call).
pub async fn load_info_state() -> Result<InfoSate> {
    // --- Version: create if missing, otherwise read existing ---
    let version_info = version::task::load_or_init_version().await?;

    // --- Settings: create if missing, always re-read ---
    let settings_info = settings::task::load_or_init_settings()?;

    unit_price::task::load_or_init_unit_price()?;

    Ok(InfoSate {
        version: version_info,
        settings: settings_info,
    })
}
