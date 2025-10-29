//! Core module
//!
//! This module combines logic previously in `lib` and `util`.
//! It contains foundational components and shared utilities used across the application,
//! such as API client configuration, models, errors, and core configuration handling.
//!
//! The `core` layer represents the application's internal logic — independent of any
//! specific runtime (CLI, HTTP server, etc.).

pub mod kube_client;
mod constants;