use anyhow::Result;

use super::info_version_entity::InfoVersionEntity;

/// Unified repository interface used by the domain service layer.
///
/// Implementations may be backed by filesystem adapters, databases,
/// or remote APIs, but the domain only depends on this trait.
pub trait InfoVersionRepository: Send + Sync {
    fn get(&self) -> Result<InfoVersionEntity>;
    fn insert(&self, data: &InfoVersionEntity) -> Result<()>;
    fn update(&self, data: &InfoVersionEntity) -> Result<()>;
}

