/* Entry point */
mod task;
pub use task::run;

/* Maps K8s API objects → internal models */
/* Data structures */
pub mod summary_dto;
pub mod node;
mod pod;
mod container;
