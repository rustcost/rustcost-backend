use anyhow::Result;
use crate::api::dto::info_dto::K8sListQuery;
use crate::core::persistence::info::k8s::container::info_container_api_repository_trait::InfoContainerApiRepository;
use crate::core::persistence::info::k8s::container::info_container_entity::InfoContainerEntity;
use crate::core::persistence::info::k8s::pod::info_pod_entity::InfoPodEntity;
use crate::domain::info::repository::info_k8s_container_api_repository::InfoK8sContainerApiRepositoryImpl;


pub async fn get_info_k8s_container(id: String) -> Result<InfoContainerEntity> {
    let repo = InfoK8sContainerApiRepositoryImpl::default();
    repo.read(&id)
}
pub async fn list_k8s_containers(p0: K8sListQuery) -> Result<Vec<InfoContainerEntity>> {
    todo!()
}