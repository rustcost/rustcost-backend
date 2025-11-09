use anyhow::Result;
use crate::core::persistence::info::fixed::info_fixed_fs_adapter_trait::InfoFixedFsAdapterTrait;
use crate::core::persistence::info::fixed::version::info_version_api_repository_trait::InfoVersionApiRepository;
use crate::core::persistence::info::fixed::version::info_version_entity::InfoVersionEntity;
use crate::core::persistence::info::fixed::version::info_version_fs_adapter::InfoVersionFsAdapter;

/// API-side repository implementation for version info.
pub struct InfoVersionApiRepositoryImpl {
    adapter: InfoVersionFsAdapter,
}

impl Default for InfoVersionApiRepositoryImpl {
    fn default() -> Self {
        Self { adapter: InfoVersionFsAdapter }
    }
}

impl InfoVersionApiRepository for InfoVersionApiRepositoryImpl {
    fn fs_adapter(&self) -> &dyn InfoFixedFsAdapterTrait<InfoVersionEntity> {
        &self.adapter
    }

    fn read(&self) -> Result<InfoVersionEntity> {
        self.adapter.read()
    }

    fn update(&self, data: &InfoVersionEntity) -> Result<()> {
        self.adapter.update(data)
    }
}

