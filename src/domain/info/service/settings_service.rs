use anyhow::Result;
use serde_json::{json, Value};
use crate::core::persistence::info::fixed::setting::info_setting_api_repository_trait::InfoSettingApiRepository;
use crate::core::persistence::info::fixed::setting::info_setting_entity::InfoSettingEntity;
use crate::domain::info::repository::info_settings_api_repository::InfoSettingApiRepositoryImpl;


pub async fn get_settings() -> Result<InfoSettingEntity> {
    let repo = InfoSettingApiRepositoryImpl::default();
    let settings = repo.read()?;
    Ok(settings)
}

pub async fn upsert_settings(data: InfoSettingEntity) -> Result<Value> {
    let repo = InfoSettingApiRepositoryImpl::default();
    repo.update(&data)?;
    Ok(json!({
        "updated": true,
        "version": data.version,
        "updated_at": data.updated_at.to_rfc3339()
    }))
}

