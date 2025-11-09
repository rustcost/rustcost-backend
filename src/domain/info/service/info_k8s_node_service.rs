use anyhow::Result;
use crate::core::persistence::info::k8s::node::info_node_api_repository_trait::InfoNodeApiRepository;
use crate::core::persistence::info::k8s::node::info_node_entity::InfoNodeEntity;
use crate::domain::info::repository::info_k8s_node_api_repository::InfoK8sNodeApiRepositoryImpl;


pub async fn get_info_k8s_node(node_name: String) -> Result<InfoNodeEntity> {
    let repo = InfoK8sNodeApiRepositoryImpl::default();
    repo.read(&node_name)
}
