use anyhow::Result;
use crate::core::persistence::info::dynamic::info_dynamic_fs_adapter_trait::InfoDynamicFsAdapterTrait;
use super::info_node_entity::InfoNodeEntity;

/// Collector repository trait for nodes.
///
/// Collectors may read, create, or update node info locally.
pub trait InfoNodeCollectorRepository: Send + Sync {
    fn fs_adapter(&self) -> &dyn InfoDynamicFsAdapterTrait<InfoNodeEntity>;

    /// Reads node info for a specific node.
    fn read(&self, node_name: &str) -> Result<InfoNodeEntity> {
        self.fs_adapter().read(node_name)
    }

    /// Creates node info for a specific node.
    fn create(&self, data: &InfoNodeEntity) -> Result<()> {
        self.fs_adapter().insert(data)
    }

    /// Updates node info for a specific node.
    fn update(&self, data: &InfoNodeEntity) -> Result<()> {
        self.fs_adapter().update(data)
    }
    fn exists(&self, node_name: &str) -> Result<bool>;

    fn create_if_missing(&self, node_name: &str, data: &InfoNodeEntity) -> Result<bool>;

}
