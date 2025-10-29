use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};

/// Build the main application router
pub fn app_router() -> Router {
    Router::new()
        // Root route
        .route("/", get(root))
        // Health check
        .route("/health", get(health_check))
        // Mount node-related routes under /api/v1
        // .nest("/api/v1/nodes", node_handler::node_routes()
        // .nest("/api/v1/pods", pod_handler::pod_routes()

        // Fallback handler for 404
        .fallback(handler_404)
        // Attach shared application state ONCE here
}

// Handler for root
async fn root() -> &'static str {
    "Server is running!"
}

// Handler for health check
async fn health_check() -> &'static str {
    "OK"
}

// Handler for 404 Not Found
async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "The requested resource was not found",
    )
}
