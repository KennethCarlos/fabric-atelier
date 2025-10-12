//! Fabric Atelier MCP Server entry point.

use fabric_atelier::{config::Settings, utils::logging, Result};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    logging::init();

    // Load configuration
    let settings = Settings::load()?;

    info!(
        "Starting {} v{}",
        settings.server.name, settings.server.version
    );

    // TODO: Initialize MCP server
    info!("Server initialized successfully");

    Ok(())
}
