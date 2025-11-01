use crate::core::persistence::info::dynamic::info_dynamic_fs_adapter_trait::InfoDynamicFsAdapterTrait;
use crate::core::persistence::info::dynamic::node::info_node_collector_repository_trait::InfoNodeCollectorRepository;
use crate::core::persistence::info::dynamic::node::info_node_entity::InfoNodeEntity;
use crate::core::persistence::info::dynamic::node::info_node_fs_adapter::InfoNodeFsAdapter;
use anyhow::Result;

/// Concrete collector-side repository implementation for managing node info.
///
/// Bridges the collector logic with the filesystem adapter layer.
pub struct InfoNodeCollectorRepositoryImpl {
    adapter: InfoNodeFsAdapter,
}

impl Default for InfoNodeCollectorRepositoryImpl {
    fn default() -> Self {
        Self {
            adapter: InfoNodeFsAdapter,
        }
    }
}

impl InfoNodeCollectorRepository for InfoNodeCollectorRepositoryImpl {
    fn fs_adapter(&self) -> &dyn InfoDynamicFsAdapterTrait<InfoNodeEntity> {
        &self.adapter
    }

    /// Reads node info for a specific node.
    fn read(&self, node_name: &str) -> Result<InfoNodeEntity> {
        self.adapter.read(node_name)
    }

    /// Creates (inserts) node info for a specific node.
    fn create(&self, data: &InfoNodeEntity) -> Result<()> {
        self.adapter.insert(data)
    }

    /// Updates node info for a specific node.
    fn update(&self, data: &InfoNodeEntity) -> Result<()> {
        self.adapter.update(data)
    }

    fn exists(&self, node_name: &str) -> Result<bool> {
        self.adapter.exists(node_name)
    }

    /// Creates node info only if it doesn't already exist.
    ///
    /// Returns:
    /// - `Ok(true)` if a new file was created.
    /// - `Ok(false)` if it already existed.
    fn create_if_missing(&self, node_name: &str, data: &InfoNodeEntity) -> Result<bool> {
        if self.adapter.exists(node_name)? {
            return Ok(false);
        }
        self.adapter.insert(data)?;
        Ok(true)
    }
}
