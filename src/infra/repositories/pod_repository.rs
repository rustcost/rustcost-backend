use diesel::prelude::*;
use diesel::sql_types::{BigInt, Double, Text, Timestamptz};
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
#[derive(QueryableByName, Debug)]
pub struct NamespaceMetricBucket {
    #[diesel(sql_type = Timestamptz)]
    pub bucket: NaiveDateTime,
    #[diesel(sql_type = Double)]
    pub avg_cpu: f64,
    #[diesel(sql_type = Double)]
    pub avg_mem: f64,
}

pub fn get_namespace_metrics_between_bucketed(
    ns: &str,
    start: NaiveDateTime,
    end: NaiveDateTime,
    bucket: &str, // "minute", "hour", "day"
) -> Result<Vec<NamespaceMetricBucket>> {
    let mut conn = establish_connection();

    let query = format!(
        "
        SELECT
            date_trunc('{}', timestamp) AS bucket,
            AVG(cpu_mcores)::DOUBLE PRECISION AS avg_cpu,
            AVG(memory_bytes)::DOUBLE PRECISION AS avg_mem
        FROM pod_metrics
        WHERE namespace = $1
          AND timestamp BETWEEN $2 AND $3
        GROUP BY bucket
        ORDER BY bucket;
        ",
        bucket
    );

    diesel::sql_query(query)
        .bind::<Text, _>(ns)
        .bind::<Timestamptz, _>(start)
        .bind::<Timestamptz, _>(end)
        .load::<NamespaceMetricBucket>(&mut conn)
        .map_err(Into::into)
}
#[derive(QueryableByName, Debug)]
pub struct NamespaceMetricPoint {
    #[diesel(sql_type = Timestamptz)]
    pub bucket: NaiveDateTime,
    #[diesel(sql_type = Double)]
    pub avg_cpu: f64,
    #[diesel(sql_type = Double)]
    pub avg_mem: f64,
}
pub fn get_namespace_metrics_between(
    ns: &str,
    start: NaiveDateTime,
    end: NaiveDateTime,
) -> Result<Vec<NamespaceMetricPoint>> {
    let mut conn = establish_connection();

    let query = "
        SELECT
            date_trunc('minute', timestamp) AS bucket,  -- ✅ 버킷 단위 설정
            AVG(cpu_mcores)::DOUBLE PRECISION AS avg_cpu,
            AVG(memory_bytes)::DOUBLE PRECISION AS avg_mem
        FROM pod_metrics
        WHERE namespace = $1
          AND timestamp BETWEEN $2 AND $3
        GROUP BY bucket
        ORDER BY bucket ASC;
    ";

    diesel::sql_query(query)
        .bind::<Text, _>(ns)
        .bind::<Timestamptz, _>(start)
        .bind::<Timestamptz, _>(end)
        .load::<NamespaceMetricPoint>(&mut conn)
        .map_err(Into::into)
}

pub fn get_pods_by_namespace(ns: &str) -> Result<Vec<Pod>> {
    let mut conn = establish_connection();
    pods::table
        .filter(pods::namespace.eq(ns))
        .order(pods::name.asc())
        .load::<Pod>(&mut conn)
        .map_err(Into::into)
}

pub fn get_pod_metrics_between_by_id(
    pod_id_val: i32,
    start: NaiveDateTime,
    end: NaiveDateTime,
) -> Result<Vec<PodMetric>> {
    let mut conn = establish_connection();

    pod_metrics::table
        .filter(pod_metrics::pod_id.eq(pod_id_val))
        .filter(pod_metrics::timestamp.ge(start))
        .filter(pod_metrics::timestamp.le(end))
        .order(pod_metrics::timestamp.asc())
        .load::<PodMetric>(&mut conn)
        .map_err(Into::into)
}

#[derive(QueryableByName, Debug)]
pub struct PodMetricBucket {
    #[diesel(sql_type = Timestamptz)]
    pub bucket: NaiveDateTime,
    #[diesel(sql_type = Double)]
    pub avg_cpu: f64,
    #[diesel(sql_type = Double)]
    pub avg_mem: f64,
}

pub fn get_pod_metrics_between_by_id_bucketed(
    pod_id_val: i32,
    start: NaiveDateTime,
    end: NaiveDateTime,
    bucket: &str, // "minute", "hour", "day"
) -> Result<Vec<PodMetricBucket>> {
    let mut conn = establish_connection();

    let query = format!(
        "
        SELECT
            date_trunc('{}', timestamp) AS bucket,
            AVG(cpu_mcores)::DOUBLE PRECISION AS avg_cpu,
            AVG(memory_bytes)::DOUBLE PRECISION AS avg_mem
        FROM pod_metrics
        WHERE pod_id = $1
          AND timestamp BETWEEN $2 AND $3
        GROUP BY bucket
        ORDER BY bucket;
        ",
        bucket
    );

    diesel::sql_query(query)
        .bind::<BigInt, _>(pod_id_val as i64)
        .bind::<Timestamptz, _>(start)
        .bind::<Timestamptz, _>(end)
        .load::<PodMetricBucket>(&mut conn)
        .map_err(Into::into)
}