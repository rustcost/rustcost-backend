use anyhow::Result;
use crate::core::persistence::info::k8s::pod::info_pod_api_repository_trait::InfoPodApiRepository;
use crate::core::persistence::info::k8s::pod::info_pod_entity::InfoPodEntity;
use crate::domain::info::repository::info_k8s_pod_api_repository::InfoK8sPodApiRepositoryImpl;


pub async fn get_info_k8s_pod(pod_uid: String) -> Result<InfoPodEntity> {
    let repo = InfoK8sPodApiRepositoryImpl::default();
    repo.read(&pod_uid)
}
