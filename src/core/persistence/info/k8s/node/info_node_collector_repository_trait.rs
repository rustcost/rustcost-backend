use super::info_node_entity::InfoNodeEntity;
use crate::core::persistence::info::k8s::info_dynamic_fs_adapter_trait::InfoDynamicFsAdapterTrait;
use anyhow::Result;

/// Collector repository trait for nodes.
///
/// Collectors may read, create, or update node info locally.
pub trait InfoNodeCollectorRepository: Send + Sync {
    fn fs_adapter(&self) -> &dyn InfoDynamicFsAdapterTrait<InfoNodeEntity>;

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
