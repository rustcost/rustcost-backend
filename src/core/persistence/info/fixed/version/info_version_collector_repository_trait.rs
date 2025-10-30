use anyhow::Result;
use crate::core::persistence::info::fixed::info_fixed_fs_adapter_trait::InfoFixedFsAdapterTrait;
use super::info_version_entity::InfoVersionEntity;

/// Collector repository trait for versions.
/// Collector may read and occasionally create/update versions locally.
pub trait InfoVersionCollectorRepository: Send + Sync {
    fn fs_adapter(&self) -> &dyn InfoFixedFsAdapterTrait<InfoVersionEntity>;

    fn read(&self) -> Result<InfoVersionEntity> {
        self.fs_adapter().read()
    }

    fn create(&self, data: &InfoVersionEntity) -> Result<()> {
        self.fs_adapter().insert(data)
    }

    fn update(&self, data: &InfoVersionEntity) -> Result<()> {
        self.fs_adapter().update(data)
    }
}
