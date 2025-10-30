use anyhow::Result;
use crate::core::persistence::info::fixed::info_fixed_fs_adapter_trait::InfoFixedFsAdapterTrait;
use super::info_setting_entity::InfoSettingEntity;

/// API repository trait for settings.
/// API can read and update, but usually not create/delete.
pub trait InfoSettingApiRepository: Send + Sync {
    fn fs_adapter(&self) -> &dyn InfoFixedFsAdapterTrait<InfoSettingEntity>;

    fn read(&self) -> Result<InfoSettingEntity> {
        self.fs_adapter().read()
    }

    fn update(&self, data: &InfoSettingEntity) -> Result<()> {
        self.fs_adapter().update(data)
    }
}
