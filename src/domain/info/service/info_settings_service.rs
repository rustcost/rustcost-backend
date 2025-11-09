use anyhow::Result;
use serde_json::{Value};
use crate::core::persistence::info::fixed::setting::info_setting_api_repository_trait::InfoSettingApiRepository;
use crate::core::persistence::info::fixed::setting::info_setting_entity::InfoSettingEntity;
use crate::domain::info::dto::info_setting_upsert_request::InfoSettingUpsertRequest;
use crate::domain::info::repository::info_settings_api_repository::InfoSettingApiRepositoryImpl;


pub async fn get_info_settings() -> Result<InfoSettingEntity> {
    let repo = InfoSettingApiRepositoryImpl::default();
    let settings = repo.read()?;
    Ok(settings)
}

pub async fn upsert_info_settings(req: InfoSettingUpsertRequest) -> Result<Value> {
    let repo = InfoSettingApiRepositoryImpl::default();

    let mut settings = repo.read()?;
    settings.apply_update(req);

    repo.update(&settings)?; // ✅ now clean

    Ok(serde_json::json!({
        "message": "Settings updated successfully",
        "updated_at": settings.updated_at.to_rfc3339(),
    }))
}

