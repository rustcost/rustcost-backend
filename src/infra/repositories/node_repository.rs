use diesel::prelude::*;
use crate::domain::models::node::{NodeModel, NodeEntity, NewNodeEntity};
use crate::infra::db::schema::nodes;
use crate::infra::errors::{adapt_infra_error, InfraError};

pub fn insert(conn: &mut PgConnection, new_node: &NewNodeEntity) -> Result<NodeModel, InfraError> {
    let entity: NodeEntity = diesel::insert_into(nodes::table)
        .values(new_node)
        .get_result(conn)
        .map_err(adapt_infra_error)?;

    Ok(adapt_entity_to_model(entity))
}

pub fn get(conn: &mut PgConnection, id: i32) -> Result<NodeModel, InfraError> {
    let entity = nodes::table
        .filter(nodes::id.eq(id))
        .select(NodeEntity::as_select())
        .first::<NodeEntity>(conn)
        .map_err(adapt_infra_error)?;

    Ok(adapt_entity_to_model(entity))
}

#[derive(Debug)]
pub struct  NodesFilter {
    pub name_contains: Option<String>,
    pub architecture: Option<String>,
}

pub fn get_all(conn: &mut PgConnection, filter: NodesFilter) -> Result<Vec<NodeModel>, InfraError> {
    let mut query = nodes::table.into_boxed::<diesel::pg::Pg>();

    if let Some(name) = filter.name_contains {
        query = query.filter(nodes::name.ilike(format!("%{}%", name)));
    }

    if let Some(arch) = filter.architecture {
        query = query.filter(nodes::architecture.eq(arch));
    }

    let results = query
        .select(NodeEntity::as_select())
        .load::<NodeEntity>(conn)
        .map_err(adapt_infra_error)?;

    Ok(results.into_iter().map(adapt_entity_to_model).collect())
}

fn adapt_entity_to_model(entity: NodeEntity) -> NodeModel {
    NodeModel {
        id: entity.id,
        name: entity.name,
        cpu_capacity: entity.cpu_capacity,
        memory_capacity: entity.memory_capacity,
        kubelet_version: entity.kubelet_version,
        os_image: entity.os_image,
        architecture: entity.architecture,
        created_at: entity.created_at,
    }
}