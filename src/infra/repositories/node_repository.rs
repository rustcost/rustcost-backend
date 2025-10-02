use diesel::prelude::*;
use chrono::{NaiveDateTime, Utc};
use anyhow::Result;

use crate::infra::db::connection::establish_connection;
use crate::infra::db::schema::{nodes, node_metrics};
use crate::infra::db::schema::nodes::dsl::*;
use crate::infra::db::schema::node_metrics::dsl::*;

use crate::domain::models::node::{Node, NewNode, NodeMetric, NewNodeMetric};
use diesel::sql_types::Double;
use diesel::dsl::sql;

/// Insert new node

pub fn insert_node(new_node: NewNode) -> Result<Node> {
    let mut conn = establish_connection();

    diesel::insert_into(nodes::table)
        .values(&new_node)
        .on_conflict(nodes::name)
        .do_update()
        .set(nodes::labels.eq(new_node.labels.as_ref())) // ✅ borrow Option<serde_json::Value>
        .returning(Node::as_returning())
        .get_result(&mut conn)
        .map_err(Into::into)
}




/// Insert a new node metric
pub fn insert_node_metric(new_metric: NewNodeMetric) -> Result<NodeMetric> {
    let mut conn = establish_connection();

    diesel::insert_into(node_metrics::table)
        .values(&new_metric)
        .returning(NodeMetric::as_returning())
        .get_result(&mut conn)
        .map_err(Into::into)
}

/// Get all nodes
pub fn get_nodes() -> Result<Vec<Node>> {
    let mut conn = establish_connection();
    nodes.load::<Node>(&mut conn).map_err(Into::into)
}

/// Get average CPU usage today (all nodes)
pub fn get_avg_cpu_today() -> Result<Option<f64>> {
    let mut conn = establish_connection();
    let today = Utc::now().date_naive();

    node_metrics
        .filter(timestamp.ge(today.and_hms_opt(0, 0, 0).unwrap()))
        .select(sql::<Double>("CAST(AVG(cpu_mcores) AS DOUBLE PRECISION)"))
        .first::<f64>(&mut conn)       // ask for f64
        .optional()                    // wrap it into Option<f64>
        .map_err(Into::into)
}



/// Get average Memory usage today (all nodes)
pub fn get_avg_memory_today() -> Result<Option<f64>> {
    let mut conn = establish_connection();
    let today = Utc::now().date_naive();

    node_metrics
        .filter(timestamp.ge(today.and_hms_opt(0, 0, 0).unwrap()))
        .select(sql::<Double>("CAST(AVG(memory_bytes) AS DOUBLE PRECISION)"))
        .first::<f64>(&mut conn)
        .optional()
        .map_err(Into::into)
}

/// Get CPU + Memory data between given times (for chart)
pub fn get_metrics_between(start: NaiveDateTime, end: NaiveDateTime) -> Result<Vec<NodeMetric>> {
    let mut conn = establish_connection();

    node_metrics
        .filter(timestamp.ge(start))
        .filter(timestamp.le(end))
        .order(timestamp.asc())
        .load::<NodeMetric>(&mut conn)
        .map_err(Into::into)
}
