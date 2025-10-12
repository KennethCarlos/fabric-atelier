//! Logging configuration and setup.
//!
//! Configures `tracing` for structured logging throughout the application.

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Initialize logging with default configuration.
///
/// Sets up tracing with environment-based filtering.
/// Use `RUST_LOG` environment variable to control log levels.
///
/// # Examples
///
/// ```bash
/// RUST_LOG=debug cargo run
/// RUST_LOG=fabric_atelier=trace cargo run
/// ```
pub fn init() {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("fabric_atelier=info")),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

/// Initialize logging with custom filter.
///
/// # Arguments
///
/// * `filter` - Custom filter directive (e.g., "debug", "fabric_atelier=trace")
pub fn init_with_filter(filter: &str) {
    tracing_subscriber::registry()
        .with(EnvFilter::new(filter))
        .with(tracing_subscriber::fmt::layer())
        .init();
}
