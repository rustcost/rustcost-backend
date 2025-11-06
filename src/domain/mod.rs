//! Domain layer: core business logic and entities
//!
//! Folders:
//! - metrics: domain entities/services/usecases for metrics
//! - info: domain entities/services/usecases for static and dynamic info
//! - system: domain for system health/backup/etc.
//! - common: shared domain types and services

pub mod metrics;
pub mod info;
pub mod system;
pub mod common;

