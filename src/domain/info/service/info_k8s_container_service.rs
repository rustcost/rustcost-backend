use anyhow::Result;
use crate::core::persistence::info::dynamic::container::info_container_api_repository_trait::InfoContainerApiRepository;
use crate::core::persistence::info::dynamic::container::info_container_entity::InfoContainerEntity;
use crate::domain::info::repository::info_k8s_container_api_repository::InfoK8sContainerApiRepositoryImpl;


pub async fn get_info_k8s_container(id: String) -> Result<InfoContainerEntity> {
    let repo = InfoK8sContainerApiRepositoryImpl::default();
    repo.read(&id)
}

