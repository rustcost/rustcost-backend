use super::info_container_entity::InfoContainerEntity;
use crate::core::persistence::info::dynamic::info_dynamic_fs_adapter_trait::InfoDynamicFsAdapterTrait;
use anyhow::Result;

/// API repository trait for containers.
///
/// The API can read and update container information, but typically does not
/// create or delete local files.
pub trait InfoContainerApiRepository: Send + Sync {
    fn fs_adapter(&self) -> &dyn InfoDynamicFsAdapterTrait<InfoContainerEntity>;

    /// Reads container info for the given container name.
    fn read(&self, container_name: &str) -> Result<InfoContainerEntity> {
        self.fs_adapter().read(container_name)
    }

    /// Updates container info for the given container name.
    fn update(&self, data: &InfoContainerEntity) -> Result<()> {
        self.fs_adapter().update(data)
    }
}
