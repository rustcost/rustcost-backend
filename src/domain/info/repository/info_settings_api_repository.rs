use anyhow::Result;
use crate::core::persistence::info::fixed::info_fixed_fs_adapter_trait::InfoFixedFsAdapterTrait;
use crate::core::persistence::info::fixed::setting::info_setting_api_repository_trait::InfoSettingApiRepository;
use crate::core::persistence::info::fixed::setting::info_setting_entity::InfoSettingEntity;
use crate::core::persistence::info::fixed::setting::info_setting_fs_adapter::InfoSettingFsAdapter;

/// Concrete collector-side repository implementation for managing container info.
///
/// Bridges the collector logic with the filesystem adapter layer.
pub struct InfoSettingApiRepositoryImpl {
    adapter: InfoSettingFsAdapter,
}

impl Default for InfoSettingApiRepositoryImpl {
    fn default() -> Self {
        Self {
            adapter: InfoSettingFsAdapter,
        }
    }
}

impl InfoSettingApiRepository for InfoSettingApiRepositoryImpl {
    fn fs_adapter(&self) -> &dyn InfoFixedFsAdapterTrait<InfoSettingEntity> {
        &self.adapter
    }

    fn read(&self) -> Result<InfoSettingEntity> {
        self.adapter.read()
    }

    fn update(&self, data: &InfoSettingEntity) -> Result<()> {
        self.adapter.update(data)
    }
}
