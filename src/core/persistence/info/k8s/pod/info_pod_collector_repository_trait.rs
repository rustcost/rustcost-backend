use super::info_pod_entity::InfoPodEntity;
use crate::core::persistence::info::k8s::info_dynamic_fs_adapter_trait::InfoDynamicFsAdapterTrait;
use anyhow::Result;

/// Collector repository trait for pods.
///
/// Collectors may read, create, or update pod info locally.
pub trait InfoPodCollectorRepository: Send + Sync {
    fn fs_adapter(&self) -> &dyn InfoDynamicFsAdapterTrait<InfoPodEntity>;

    /// Reads pod info for a specific pod.
    fn read(&self, pod_name: &str) -> Result<InfoPodEntity> {
        self.fs_adapter().read(pod_name)
    }

    /// Creates pod info for a specific pod.
    fn create(&self, data: &InfoPodEntity) -> Result<()> {
        self.fs_adapter().insert(data)
    }

    /// Updates pod info for a specific pod.
    fn update(&self, data: &InfoPodEntity) -> Result<()> {
        self.fs_adapter().update(data)
    }
    fn exists(&self, pod_name: &str) -> Result<bool>;

    fn create_if_missing(&self, pod_name: &str, data: &InfoPodEntity) -> Result<bool>;

}
