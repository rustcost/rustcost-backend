use anyhow::Result;
use crate::core::persistence::info::k8s::info_dynamic_fs_adapter_trait::InfoDynamicFsAdapterTrait;
use crate::core::persistence::info::k8s::container::info_container_api_repository_trait::InfoContainerApiRepository;
use crate::core::persistence::info::k8s::container::info_container_entity::InfoContainerEntity;
use crate::core::persistence::info::k8s::container::info_container_fs_adapter::InfoContainerFsAdapter;

/// API-side repository implementation for container info.
pub struct InfoK8sContainerApiRepositoryImpl {
    adapter: InfoContainerFsAdapter,
}

impl Default for InfoK8sContainerApiRepositoryImpl {
    fn default() -> Self {
        Self { adapter: InfoContainerFsAdapter }
    }
}

impl InfoContainerApiRepository for InfoK8sContainerApiRepositoryImpl {
    fn fs_adapter(&self) -> &dyn InfoDynamicFsAdapterTrait<InfoContainerEntity> {
        &self.adapter
    }

    fn read(&self, container_key: &str) -> Result<InfoContainerEntity> {
        self.adapter.read(container_key)
    }

    fn update(&self, data: &InfoContainerEntity) -> Result<()> {
        self.adapter.update(data)
    }
}

