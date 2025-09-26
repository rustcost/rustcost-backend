use deadpool_diesel::postgres::{Manager, Pool};
use deadpool_diesel::Runtime;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use std::net::SocketAddr;
use tracing_subscriber::prelude::*;
use diesel_migrations::MigrationHarness;

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
use crate::config::config;
use crate::errors::{internal_error, AppError};
use crate::routes::app_router;
use crate::state::AppState;
use crate::utils::schedulers::node_scheduler::start_node_collector;

// Embed database migrations
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

// Main asynchronous function to start the application
#[tokio::main]
async fn main() {
    // Load environment variables from `.env` file
    dotenvy::dotenv().ok();

    // Initialize tracing for logging
    init_tracing();

    // Load application configuration
    let app_config = config().await;

    // Create a connection manager for the database pool
    let manager = Manager::new(app_config.db_url().to_string(), Runtime::Tokio1);
    // Build the connection pool
    let pool = Pool::builder(manager)
        .build()
        .expect("Failed to create connection pool");

    // Run database migrations
    if let Err(err) = run_migrations(&pool).await {
        tracing::error!("Failed to run migrations: {:?}", err);
        // Consider exiting the application or handling the error appropriately
        return;
    }

    // Create application state containing the connection pool
    let state = AppState { pool };

    // Create the application router with the defined routes
    let app = app_router(state.clone());

    // Get server host and port from configuration
    let host = app_config.server_host();
    let port = app_config.server_port();
    let address = format!("{}:{}", host, port);
    // Parse the address into a SocketAddr
    let socket_addr: SocketAddr = address.parse().expect("Unable to parse socket address");

    // Log the server address
    tracing::info!("listening on http://{}", socket_addr);

    // Bind the server to the specified address
    let listener = tokio::net::TcpListener::bind(socket_addr)
        .await
        .expect("Failed to bind");

    // Spawn background task
    tokio::spawn(async {
        if let Err(e) = start_node_collector().await {
            tracing::error!("Node collector task failed: {:?}", e);
        }
    });

    // Start the axum server
    axum::serve(listener, app)
        .await
        .expect("Server failed to run");
}

// Function to initialize tracing
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

// Asynchronous function to run database migrations
async fn run_migrations(pool: &Pool) -> Result<(), AppError> {
    let conn = pool.get().await.map_err(internal_error)?;

    conn.interact(|conn_inner| {
        match conn_inner.run_pending_migrations(MIGRATIONS) {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Migration error: {:?}", e); // 👈 log full error
                Err(e)
            }
        }
    })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    Ok(())
}