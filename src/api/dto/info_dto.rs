//! Info API DTOs

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct K8sListQuery {
    pub namespace: Option<String>,
    pub label_selector: Option<String>,
    pub node_name: Option<String>, // for pods by node
}