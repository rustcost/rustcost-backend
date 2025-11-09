use crate::core::persistence::info::k8s::info_dynamic_fs_adapter_trait::InfoDynamicFsAdapterTrait;
use crate::core::persistence::info::k8s::container::info_container_collector_repository_trait::InfoContainerCollectorRepository;
use crate::core::persistence::info::k8s::container::info_container_entity::InfoContainerEntity;
use crate::core::persistence::info::k8s::container::info_container_fs_adapter::InfoContainerFsAdapter;
use anyhow::Result;

/// Concrete collector-side repository implementation for managing container info.
///
/// Bridges the collector logic with the filesystem adapter layer.
pub struct InfoContainerCollectorRepositoryImpl {
    adapter: InfoContainerFsAdapter,
}

impl Default for InfoContainerCollectorRepositoryImpl {
    fn default() -> Self {
        Self {
            adapter: InfoContainerFsAdapter,
        }
    }
}

impl InfoContainerCollectorRepository for InfoContainerCollectorRepositoryImpl {
    fn fs_adapter(&self) -> &dyn InfoDynamicFsAdapterTrait<InfoContainerEntity> {
        &self.adapter
    }

    /// Reads container info for a specific container.
    fn read(&self, container_key: &str) -> Result<InfoContainerEntity> {
        self.adapter.read(container_key)
    }

    /// Creates (inserts) container info for a specific container.
    fn create(&self, data: &InfoContainerEntity) -> Result<()> {
        self.adapter.insert(data)
    }

    /// Updates container info for a specific container.
    fn update(&self, data: &InfoContainerEntity) -> Result<()> {
        self.adapter.update(data)
    }

    fn exists(&self, container_key: &str) -> Result<bool> {
        self.adapter.exists(container_key)
    }

    /// Creates container info only if it doesn't already exist.
    ///
    /// Returns:
    /// - `Ok(true)` if a new file was created.
    /// - `Ok(false)` if it already existed.
    fn create_if_missing(&self, container_key: &str, data: &InfoContainerEntity) -> Result<bool> {
        if self.adapter.exists(container_key)? {
            return Ok(false);
        }
        self.adapter.insert(data)?;
        Ok(true)
    }
}
