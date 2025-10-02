use diesel::prelude::*;
use diesel::sql_types::{Double};
use anyhow::Result;
use chrono::{NaiveDate, NaiveDateTime, Utc};
use diesel::dsl::sql;

use crate::infra::db::connection::establish_connection;
use crate::infra::db::schema::{pods, pod_metrics};
use crate::domain::models::pod::{Pod, NewPod, PodMetric, NewPodMetric, UpdatePod};
use crate::infra::db::schema::pod_metrics::dsl::*;

/// =======================
/// Insert
/// =======================
pub fn insert_pod(new_pod: NewPod) -> Result<Pod> {
    let mut conn = establish_connection();

    diesel::insert_into(pods::table)
        .values(&new_pod)
        .on_conflict((pods::name, pods::namespace)) // ✅ composite key
        .do_update()
        .set(UpdatePod {
            node_id: new_pod.node_id,
            labels: new_pod.labels.clone(),
        })
        .returning(Pod::as_returning())
        .get_result(&mut conn)
        .map_err(Into::into)
}


pub fn insert_pod_metric(new_metric: NewPodMetric) -> Result<PodMetric> {
    let mut conn = establish_connection();
    diesel::insert_into(pod_metrics::table)
        .values(&new_metric)
        .returning(PodMetric::as_returning())
        .get_result(&mut conn)
        .map_err(Into::into)
}

/// =======================
/// Lists
/// =======================
pub fn get_pods() -> Result<Vec<Pod>> {
    let mut conn = establish_connection();
    pods::table.load::<Pod>(&mut conn).map_err(Into::into)
}

pub fn get_namespaces() -> Result<Vec<String>> {
    let mut conn = establish_connection();
    pod_metrics::table
        .select(pod_metrics::namespace)
        .distinct()
        .load::<String>(&mut conn)
        .map_err(Into::into)
}

/// =======================
/// Average metrics (with SQL CAST to f64)
/// =======================
fn today_start() -> NaiveDateTime {
    let today: NaiveDate = Utc::now().date_naive();
    today.and_hms_opt(0, 0, 0).unwrap()
}

/// average cpu usage today for a pod
pub fn get_avg_cpu_today_pod(pod_id_val: i32) -> Result<Option<f64>> {
    let mut conn = establish_connection();

    pod_metrics
        .filter(pod_id.eq(pod_id_val))
        .filter(timestamp.ge(today_start()))
        .select(sql::<Double>("CAST(AVG(cpu_mcores) AS DOUBLE PRECISION)"))
        .first::<f64>(&mut conn)
        .optional()
        .map_err(Into::into)
}
/// average memory usage today for a pod
pub fn get_avg_mem_today_pod(pod_id_val: i32) -> Result<Option<f64>> {
    let mut conn = establish_connection();

    pod_metrics
        .filter(pod_id.eq(pod_id_val))
        .filter(timestamp.ge(today_start()))
        .select(sql::<Double>("CAST(AVG(memory_bytes) AS DOUBLE PRECISION)"))
        .first::<f64>(&mut conn)
        .optional()
        .map_err(Into::into)
}

/// average cpu usage today for pods in a namespace
pub fn get_avg_cpu_today_namespace(ns: &str) -> Result<Option<f64>> {
    let mut conn = establish_connection();

    pod_metrics
        .filter(namespace.eq(ns))
        .filter(timestamp.ge(today_start()))
        .select(sql::<Double>("CAST(AVG(cpu_mcores) AS DOUBLE PRECISION)"))
        .first::<f64>(&mut conn)
        .optional()
        .map_err(Into::into)
}

/// average memory usage today for pods in a namespace
pub fn get_avg_mem_today_namespace(ns: &str) -> Result<Option<f64>> {
    let mut conn = establish_connection();

    pod_metrics
        .filter(namespace.eq(ns))
        .filter(timestamp.ge(today_start()))
        .select(sql::<Double>("CAST(AVG(memory_bytes) AS DOUBLE PRECISION)"))
        .first::<f64>(&mut conn)
        .optional()
        .map_err(Into::into)
}


/// =======================
/// Chart queries
/// =======================

/// Get CPU + Memory metrics for all pods between two times
pub fn get_pod_metrics_between(start: NaiveDateTime, end: NaiveDateTime) -> Result<Vec<PodMetric>> {
    let mut conn = establish_connection();
    pod_metrics::table
        .filter(pod_metrics::timestamp.ge(start))
        .filter(pod_metrics::timestamp.le(end))
        .order(pod_metrics::timestamp.asc())
        .load::<PodMetric>(&mut conn)
        .map_err(Into::into)
}

/// Get CPU + Memory metrics for a given namespace between two times
pub fn get_namespace_metrics_between(
    ns: &str,
    start: NaiveDateTime,
    end: NaiveDateTime,
) -> Result<Vec<PodMetric>> {
    let mut conn = establish_connection();
    pod_metrics::table
        .filter(pod_metrics::namespace.eq(ns))
        .filter(pod_metrics::timestamp.ge(start))
        .filter(pod_metrics::timestamp.le(end))
        .order(pod_metrics::timestamp.asc())
        .load::<PodMetric>(&mut conn)
        .map_err(Into::into)
}
