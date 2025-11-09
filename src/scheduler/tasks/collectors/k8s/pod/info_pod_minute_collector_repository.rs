use crate::core::persistence::info::k8s::info_dynamic_fs_adapter_trait::InfoDynamicFsAdapterTrait;
use crate::core::persistence::info::k8s::pod::info_pod_collector_repository_trait::InfoPodCollectorRepository;
use crate::core::persistence::info::k8s::pod::info_pod_entity::InfoPodEntity;
use crate::core::persistence::info::k8s::pod::info_pod_fs_adapter::InfoPodFsAdapter;
use anyhow::Result;

/// Concrete collector-side repository implementation for managing pod info.
///
/// Bridges the collector logic with the filesystem adapter layer.
pub struct InfoPodCollectorRepositoryImpl {
    adapter: InfoPodFsAdapter,
}

impl Default for InfoPodCollectorRepositoryImpl {
    fn default() -> Self {
        Self {
            adapter: InfoPodFsAdapter,
        }
    }
}

impl InfoPodCollectorRepository for InfoPodCollectorRepositoryImpl {
    fn fs_adapter(&self) -> &dyn InfoDynamicFsAdapterTrait<InfoPodEntity> {
        &self.adapter
    }

    /// Reads pod info for a specific pod.
    fn read(&self, pod_uid: &str) -> Result<InfoPodEntity> {
        self.adapter.read(pod_uid)
    }

    /// Creates (inserts) pod info for a specific pod.
    fn create(&self, data: &InfoPodEntity) -> Result<()> {
        self.adapter.insert(data)
    }

    /// Updates pod info for a specific pod.
    fn update(&self, data: &InfoPodEntity) -> Result<()> {
        self.adapter.update(data)
    }

    fn exists(&self, pod_uid: &str) -> Result<bool> {
        self.adapter.exists(pod_uid)
    }

    /// Creates pod info only if it doesn't already exist.
    ///
    /// Returns:
    /// - `Ok(true)` if a new file was created.
    /// - `Ok(false)` if it already existed.
    fn create_if_missing(&self, pod_uid: &str, data: &InfoPodEntity) -> Result<bool> {
        if self.adapter.exists(pod_uid)? {
            return Ok(false);
        }
        self.adapter.insert(data)?;
        Ok(true)
    }
}
