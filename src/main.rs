use std::net::SocketAddr;
use tokio::sync::broadcast;

// --- Modules ---
mod config;
mod logging;
mod domain;
mod api;
mod errors;
mod handlers;
mod routes;
mod scheduler;
pub mod core;

// --- Imports ---
use crate::config::config;
// &'fixed Config
use crate::routes::app_router;
use crate::scheduler::schedule::{run_day_loop, run_hour_loop, run_minute_loop};
use crate::scheduler::scheduler_start_all_tasks;
use tracing::{info, error};

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
    let debug_mode = std::env::var("DEBUG_MODE")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    info!("🚀 Listening on http://{}", socket_addr);

    let listener = tokio::net::TcpListener::bind(socket_addr)
        .await
        .expect("Failed to bind");

    // Keep the sender ALIVE for whole function lifetime
    let (shutdown_tx, mut shutdown_rx) = broadcast::channel::<()>(16);

    if debug_mode {
        let sched_rx = shutdown_rx.resubscribe();
        // run_minute_loop(&mut broadcast::channel::<()>(1).1).await;
        // run_hour_loop(&mut broadcast::channel::<()>(5).1).await;
        run_day_loop(&mut broadcast::channel::<()>(5).1).await;
    } else {
        // Run the scheduler as a background task that blocks until it receives shutdown
        let sched_rx = shutdown_rx.resubscribe();
        tokio::spawn(async move {
            scheduler_start_all_tasks(sched_rx).await;
        });
    }



    // Graceful shutdown: Ctrl+C => send shutdown => server stops
    let shutdown_tx_clone = shutdown_tx.clone();
    let server = axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            // Wait for Ctrl+C
            let _ = tokio::signal::ctrl_c().await;
            info!("🔻 Ctrl+C received, sending shutdown...");
            let _ = shutdown_tx_clone.send(());
        });

    // Also listen for a shutdown message to finish this function if needed
    tokio::select! {
        result = server => {
            if let Err(e) = result {
                error!(?e, "Server failed");
            }
        }
        _ = shutdown_rx.recv() => {
            info!("🔻 Shutdown received; exiting run_server");
        }
    }

}
