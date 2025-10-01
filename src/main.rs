use deadpool_diesel::postgres::{Manager, Pool};
use deadpool_diesel::Runtime;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::net::SocketAddr;
use std::env;
use tracing_subscriber::prelude::*;

// Define modules for different parts of the application
mod config;
mod domain;
mod errors;
mod handlers;
mod infra;
mod routes;
mod state;
mod utils;

// Import necessary items from modules
use crate::config::config; // returns &'static Config
use crate::errors::{internal_error, AppError};
use crate::routes::app_router;
use crate::state::AppState;
use crate::utils::schedulers::node_scheduler::start_collector;

// Embed database migrations
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[tokio::main]
async fn main() {
    // Load environment variables from `.env` file
    dotenvy::dotenv().ok();

    // Initialize tracing for logging
    init_tracing();

    // Parse CLI arguments
    let args: Vec<String> = env::args().collect();

    // Load application configuration
    let app_config = config().await; // &'static Config

    // Create a connection manager for the database pool
    let manager = Manager::new(app_config.db_url().to_string(), Runtime::Tokio1);
    // Build the connection pool
    let pool = Pool::builder(manager)
        .build()
        .expect("Failed to create connection pool");

    // Handle subcommands
    if args.len() > 1 && args[1] == "migrate" {
        // Run migrations only
        if let Err(err) = run_migrations(&pool).await {
            tracing::error!("❌ Failed to run migrations: {:?}", err);
            std::process::exit(1);
        }
        tracing::info!("✅ Migrations complete");
        std::process::exit(0);
    }

    // Otherwise: start server
    run_server(app_config, pool).await;
}

/// Function to initialize tracing
fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "example_tokio_postgres=debug,axum_diesel_real_world=debug".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

/// Asynchronous function to run database migrations
async fn run_migrations(pool: &Pool) -> Result<(), AppError> {
    let conn = pool.get().await.map_err(internal_error)?;

    conn.interact(|conn_inner| {
        match conn_inner.run_pending_migrations(MIGRATIONS) {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Migration error: {:?}", e);
                Err(e)
            }
        }
    })
        .await
        .map_err(internal_error)? // error from interact
        .map_err(internal_error)?; // error from run_pending_migrations

    Ok(())
}

/// Run the Axum server
async fn run_server(app_config: &crate::config::Config, pool: Pool) {
    // Create application state containing the connection pool
    let state = AppState { pool };

    // Create the application router with the defined routes
    let app = app_router(state.clone());

    // Get server host and port from configuration
    let address = format!("{}:{}", app_config.server_host(), app_config.server_port());
    let socket_addr: SocketAddr = address.parse().expect("Unable to parse socket address");

    tracing::info!("🚀 Listening on http://{}", socket_addr);

    let listener = tokio::net::TcpListener::bind(socket_addr)
        .await
        .expect("Failed to bind");

    // Spawn background task
    tokio::spawn(async {
        if let Err(e) = start_collector().await {
            tracing::error!("Node collector task failed: {:?}", e);
        }
    });

    // Start the axum server
    axum::serve(listener, app)
        .await
        .expect("Server failed to run");
}
