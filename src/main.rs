use std::{env, fs, net::SocketAddr, path::Path};
use tokio::sync::broadcast;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

// --- Modules ---
mod config;
mod logging;
mod domain;
mod errors;
mod handlers;
mod routes;
mod scheduler;
mod core;

// --- Imports ---
use crate::config::config;
// &'static Config
use crate::routes::app_router;
use crate::scheduler::schedule::run_minute_loop;
use crate::scheduler::scheduler_start_all_tasks;

// --- Entry Point ---
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let _log_guard = logging::init_tracing();

    let app_config = config().await;
    run_server(app_config).await;
}

/// ✅ Initialize tracing (logs stored in file)


/// ✅ Run the Axum server
async fn run_server(app_config: &crate::config::Config) {
    let app = app_router();
    let address = format!("{}:{}", app_config.server_host(), app_config.server_port());
    let socket_addr: SocketAddr = address.parse().expect("Invalid socket address");
    let debug_mode = std::env::var("DEBUG_MODE").is_ok();

    tracing::info!("🚀 Listening on http://{}", socket_addr);

    let listener = tokio::net::TcpListener::bind(socket_addr)
        .await
        .expect("Failed to bind");



    if debug_mode {
        run_minute_loop(&mut tokio::sync::broadcast::channel::<()>(1).1).await;
    } else {
        let (shutdown_tx, shutdown_rx) = broadcast::channel::<()>(16);
        scheduler_start_all_tasks(shutdown_rx).await;
    }




    axum::serve(listener, app)
        .await
        .expect("Server failed to run");
}
