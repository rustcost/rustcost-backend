use anyhow::Result;
use crate::core::persistence::info::k8s::info_dynamic_fs_adapter_trait::InfoDynamicFsAdapterTrait;
use crate::core::persistence::info::k8s::node::info_node_api_repository_trait::InfoNodeApiRepository;
use crate::core::persistence::info::k8s::node::info_node_entity::InfoNodeEntity;
use crate::core::persistence::info::k8s::node::info_node_fs_adapter::InfoNodeFsAdapter;

/// API-side repository implementation for node info.
pub struct InfoK8sNodeApiRepositoryImpl {
    adapter: InfoNodeFsAdapter,
}

impl Default for InfoK8sNodeApiRepositoryImpl {
    fn default() -> Self {
        Self { adapter: InfoNodeFsAdapter }
    }
}

impl InfoNodeApiRepository for InfoK8sNodeApiRepositoryImpl {
    fn fs_adapter(&self) -> &dyn InfoDynamicFsAdapterTrait<InfoNodeEntity> {
        &self.adapter
    }

    fn read(&self, node_name: &str) -> Result<InfoNodeEntity> {
        self.adapter.read(node_name)
    }

    fn update(&self, data: &InfoNodeEntity) -> Result<()> {
        self.adapter.update(data)
    }
}

