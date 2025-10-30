use anyhow::Result;
use crate::core::persistence::info::dynamic::info_dynamic_fs_adapter_trait::InfoDynamicFsAdapterTrait;
use super::info_node_entity::InfoNodeEntity;

/// API repository trait for nodes.
///
/// The API can read and update node information, but typically does not
/// create or delete local files.
pub trait InfoNodeApiRepository: Send + Sync {
    fn fs_adapter(&self) -> &dyn InfoDynamicFsAdapterTrait<InfoNodeEntity>;

    /// Reads node info for the given node name.
    fn read(&self, node_name: &str) -> Result<InfoNodeEntity> {
        self.fs_adapter().read(node_name)
    }

    /// Updates node info for the given node name.
    fn update(&self, node_name: &str, data: &InfoNodeEntity) -> Result<()> {
        self.fs_adapter().update(node_name, data)
    }
}
