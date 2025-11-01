use crate::core::persistence::info::fixed::info_fixed_fs_adapter_trait::InfoFixedFsAdapterTrait;
use crate::core::persistence::info::fixed::setting::info_setting_collector_repository_trait::InfoSettingCollectorRepository;
use crate::core::persistence::info::fixed::setting::info_setting_entity::InfoSettingEntity;
use crate::core::persistence::info::fixed::setting::info_setting_fs_adapter::InfoSettingFsAdapter;
use anyhow::Result;

/// Concrete collector-side repository implementation for managing Settings.
/// Bridges the collector application logic with the file-based adapter.
pub struct InfoSettingCollectorRepositoryImpl {
    adapter: InfoSettingFsAdapter,
}

impl Default for InfoSettingCollectorRepositoryImpl {
    fn default() -> Self {
        Self {
            adapter: InfoSettingFsAdapter,
        }
    }
}

impl InfoSettingCollectorRepository for InfoSettingCollectorRepositoryImpl {
    fn fs_adapter(&self) -> &dyn InfoFixedFsAdapterTrait<InfoSettingEntity> {
        &self.adapter
    }

    /// Optionally, you can override functions here
    /// if you want collector-specific behavior later.
    fn read(&self) -> Result<InfoSettingEntity> {
        self.adapter.read()
    }

    fn create(&self, data: &InfoSettingEntity) -> Result<()> {
        self.adapter.insert(data)
    }

    fn update(&self, data: &InfoSettingEntity) -> Result<()> {
        self.adapter.update(data)
    }
}
