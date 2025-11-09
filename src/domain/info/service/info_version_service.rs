use anyhow::Result;
use serde_json::{json, Value};
use crate::core::persistence::info::fixed::version::info_version_api_repository_trait::InfoVersionApiRepository;
use crate::core::persistence::info::fixed::version::info_version_entity::InfoVersionEntity;
use crate::domain::info::repository::info_version_api_repository::InfoVersionApiRepositoryImpl;

pub async fn get_info_versions() -> Result<InfoVersionEntity> {
    let repo = InfoVersionApiRepositoryImpl::default();
    let entity = repo.read()?;
    Ok(entity)
}

pub async fn upsert_info_version() -> Result<Value> {
    // Until we introduce a DTO for updating, ensure the file exists
    // by reading current value and rewriting it.
    let repo = InfoVersionApiRepositoryImpl::default();
    let current = repo.read().unwrap_or_default();
    repo.update(&current)?;

    Ok(json!({
        "message": "Version updated successfully",
    }))
}
