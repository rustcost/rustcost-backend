use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

// Make sure these structs reference the schema directly

// NodeModel: app-level model
#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct NodeModel {
    pub id: i32,
    pub name: String,
    pub cpu_capacity: Option<String>,
    pub memory_capacity: Option<String>,
    pub kubelet_version: Option<String>,
    pub os_image: Option<String>,
    pub architecture: Option<String>,
    pub created_at: NaiveDateTime,
}

// NodeEntity: Diesel queryable / selectable
#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::infra::db::schema::nodes)]
pub struct NodeEntity {
    pub id: i32,
    pub name: String,
    pub cpu_capacity: Option<String>,
    pub memory_capacity: Option<String>,
    pub kubelet_version: Option<String>,
    pub os_image: Option<String>,
    pub architecture: Option<String>,
    pub created_at: NaiveDateTime,
}

// NewNodeEntity: Diesel insertable
#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = crate::infra::db::schema::nodes)]
pub struct NewNodeEntity {
    pub name: String,
    pub cpu_capacity: Option<String>,
    pub memory_capacity: Option<String>,
    pub kubelet_version: Option<String>,
    pub os_image: Option<String>,
    pub architecture: Option<String>,
    pub created_at: NaiveDateTime,
}

impl Clone for NewNodeEntity {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            cpu_capacity: self.cpu_capacity.clone(),
            memory_capacity: self.memory_capacity.clone(),
            kubelet_version: self.kubelet_version.clone(),
            os_image: self.os_image.clone(),
            architecture: self.architecture.clone(),
            created_at: self.created_at,
        }
    }
}