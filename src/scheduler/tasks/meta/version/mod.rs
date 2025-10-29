/* Entry point */
pub mod task;

/* Builds API client (token, cert, base URL) */
mod client;
/* Maps K8s API objects → internal models */
mod mapper;
/* Data structures */
pub mod models;
/* Persists metrics to file/TSDB */
mod repository;