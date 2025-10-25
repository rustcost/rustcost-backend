use std::{env, fs, path::Path};
use tracing_appender::rolling;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

pub fn init_tracing() -> tracing_appender::non_blocking::WorkerGuard {
    // Read log directory from env or fallback
    let log_dir = env::var("LOG_DIR").unwrap_or_else(|_| "data/logs".to_string());
    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());

    if !Path::new(&log_dir).exists() {
        fs::create_dir_all(&log_dir).expect("Failed to create log directory");
    }

    // Daily rotation
    let file_appender = rolling::daily(&log_dir, "app.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let fmt_layer = fmt::layer()
        .with_writer(non_blocking)
        .with_target(false)
        .with_level(true)
        .with_ansi(false);

    let filter_layer = EnvFilter::new(log_level);

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();

    tracing::info!(
        "✅ Tracing initialized — daily logs in {}/app.log.YYYY-MM-DD",
        log_dir
    );

    guard
}
