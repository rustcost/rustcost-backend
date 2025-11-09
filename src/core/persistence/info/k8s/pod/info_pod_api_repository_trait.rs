use super::info_pod_entity::InfoPodEntity;
use crate::core::persistence::info::k8s::info_dynamic_fs_adapter_trait::InfoDynamicFsAdapterTrait;
use anyhow::Result;

/// API repository trait for pods.
///
/// The API can read and update pod information, but typically does not
/// create or delete local files.
pub trait InfoPodApiRepository: Send + Sync {
    fn fs_adapter(&self) -> &dyn InfoDynamicFsAdapterTrait<InfoPodEntity>;

    /// Reads pod info for the given pod name.
    fn read(&self, pod_name: &str) -> Result<InfoPodEntity> {
        self.fs_adapter().read(pod_name)
    }

    /// Updates pod info for the given pod name.
    fn update(&self, data: &InfoPodEntity) -> Result<()> {
        self.fs_adapter().update(data)
    }
}
