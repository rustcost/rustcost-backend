use anyhow::Result;
use std::path::Path;
use crate::core::persistence::info::fixed::setting::info_setting_collector_repository_trait::InfoSettingCollectorRepository;
use crate::core::persistence::info::fixed::setting::info_setting_entity::InfoSettingEntity;
use crate::scheduler::tasks::info::settings::info_setting_collector_repository::InfoSettingCollectorRepositoryImpl;

/// Always re-read settings.rci every call; create if missing.
/// Load settings, create defaults if missing.
pub fn load_or_init_settings() -> Result<InfoSettingEntity> {
    let repo = InfoSettingCollectorRepositoryImpl::default();
    let path = Path::new("data/info/settings.rci");

    if !path.exists() {
        let default = InfoSettingEntity::default();
        repo.create(&default).expect("CREATE FS INFO SETTING FAILED");
        return Ok(default);
    }

    repo.read()
}
