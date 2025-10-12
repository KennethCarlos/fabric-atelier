//! Fabric Atelier MCP Server entry point.

use fabric_atelier::{
    config::Settings,
    mcp::{run_stdio_server, McpServer},
    utils::logging,
    Result,
};
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

    // Initialize MCP server
    let server = McpServer::new(settings).await?;
    info!("MCP server initialized successfully");

    // Run stdio transport
    run_stdio_server(server).await?;

    Ok(())
}
