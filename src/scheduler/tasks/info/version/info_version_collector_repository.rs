use anyhow::Result;
use crate::core::persistence::info::fixed::info_fixed_fs_adapter_trait::InfoFixedFsAdapterTrait;
use crate::core::persistence::info::fixed::version::info_version_collector_repository_trait::InfoVersionCollectorRepository;
use crate::core::persistence::info::fixed::version::info_version_entity::InfoVersionEntity;
use crate::core::persistence::info::fixed::version::info_version_fs_adapter::InfoVersionFsAdapter;

/// Concrete collector-side repository implementation for managing Versions.
/// Bridges the collector application logic with the file-based adapter.
pub struct InfoVersionCollectorRepositoryImpl {
    adapter: InfoVersionFsAdapter,
}

impl Default for InfoVersionCollectorRepositoryImpl {
    fn default() -> Self {
        Self {
            adapter: InfoVersionFsAdapter,
        }
    }
}

impl InfoVersionCollectorRepository for InfoVersionCollectorRepositoryImpl {
    fn fs_adapter(&self) -> &dyn InfoFixedFsAdapterTrait<InfoVersionEntity> {
        &self.adapter
    }

    /// Optionally, you can override functions here
    /// if you want collector-specific behavior later.
    fn read(&self) -> Result<InfoVersionEntity> {
        self.adapter.read()
    }

    fn create(&self, data: &InfoVersionEntity) -> Result<()> {
        self.adapter.insert(data)
    }

    fn update(&self, data: &InfoVersionEntity) -> Result<()> {
        self.adapter.update(data)
    }
}
