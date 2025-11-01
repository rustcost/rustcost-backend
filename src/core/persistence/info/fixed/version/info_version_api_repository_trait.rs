use super::info_version_entity::InfoVersionEntity;
use crate::core::persistence::info::fixed::info_fixed_fs_adapter_trait::InfoFixedFsAdapterTrait;
use anyhow::Result;

/// API repository trait for versions.
/// API can read and update, but usually not create/delete.
pub trait InfoVersionApiRepository: Send + Sync {
    fn fs_adapter(&self) -> &dyn InfoFixedFsAdapterTrait<InfoVersionEntity>;

    fn read(&self) -> Result<InfoVersionEntity> {
        self.fs_adapter().read()
    }

    fn update(&self, data: &InfoVersionEntity) -> Result<()> {
        self.fs_adapter().update(data)
    }
}
