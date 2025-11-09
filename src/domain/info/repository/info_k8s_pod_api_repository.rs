use anyhow::Result;
use crate::core::persistence::info::dynamic::info_dynamic_fs_adapter_trait::InfoDynamicFsAdapterTrait;
use crate::core::persistence::info::dynamic::pod::info_pod_api_repository_trait::InfoPodApiRepository;
use crate::core::persistence::info::dynamic::pod::info_pod_entity::InfoPodEntity;
use crate::core::persistence::info::dynamic::pod::info_pod_fs_adapter::InfoPodFsAdapter;

/// API-side repository implementation for pod info.
pub struct InfoK8sPodApiRepositoryImpl {
    adapter: InfoPodFsAdapter,
}

impl Default for InfoK8sPodApiRepositoryImpl {
    fn default() -> Self {
        Self { adapter: InfoPodFsAdapter }
    }
}

impl InfoPodApiRepository for InfoK8sPodApiRepositoryImpl {
    fn fs_adapter(&self) -> &dyn InfoDynamicFsAdapterTrait<InfoPodEntity> {
        &self.adapter
    }

    fn read(&self, pod_uid: &str) -> Result<InfoPodEntity> {
        self.adapter.read(pod_uid)
    }

    fn update(&self, data: &InfoPodEntity) -> Result<()> {
        self.adapter.update(data)
    }
}

