use std::sync::Arc;

use anyhow::Result;

use crate::core::persistence::info::fixed::version::info_version_entity::InfoVersionEntity;
use crate::core::persistence::info::fixed::version::info_version_repository_trait::InfoVersionRepository;

/// Domain service trait for version operations
pub trait VersionService: Send + Sync {
    fn get(&self) -> Result<InfoVersionEntity>;
    fn create(&self, data: &InfoVersionEntity) -> Result<()>;
    fn update(&self, data: &InfoVersionEntity) -> Result<()>;
}

/// Concrete service that depends only on the repository trait
pub struct VersionServiceImpl<R: InfoVersionRepository> {
    repo: Arc<R>,
}

impl<R: InfoVersionRepository> VersionServiceImpl<R> {
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }
}

impl<R: InfoVersionRepository> VersionService for VersionServiceImpl<R> {
    fn get(&self) -> Result<InfoVersionEntity> {
        self.repo.get()
    }

    fn create(&self, data: &InfoVersionEntity) -> Result<()> {
        self.repo.insert(data)
    }

    fn update(&self, data: &InfoVersionEntity) -> Result<()> {
        self.repo.update(data)
    }
}
