use super::info_container_entity::InfoContainerEntity;
use crate::core::persistence::info::k8s::info_dynamic_fs_adapter_trait::InfoDynamicFsAdapterTrait;
use anyhow::Result;

/// Collector repository trait for containers.
///
/// Collectors may read, create, or update container info locally.
pub trait InfoContainerCollectorRepository: Send + Sync {
    fn fs_adapter(&self) -> &dyn InfoDynamicFsAdapterTrait<InfoContainerEntity>;

    /// Creates container info for a specific container.
    fn create(&self, data: &InfoContainerEntity) -> Result<()> {
        self.fs_adapter().insert(data)
    }

    /// Updates container info for a specific container.
    fn update(&self, data: &InfoContainerEntity) -> Result<()> {
        self.fs_adapter().update(data)
    }
    fn exists(&self, container_name: &str) -> Result<bool>;

    fn create_if_missing(&self, container_name: &str, data: &InfoContainerEntity) -> Result<bool>;

}
