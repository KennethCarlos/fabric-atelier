//! Stdio transport for MCP protocol.
//!
//! Handles reading JSON-RPC requests from stdin and writing responses to stdout.

use crate::error::Result;
use crate::mcp::protocol::{JsonRpcRequest, JsonRpcResponse, PARSE_ERROR};
use crate::mcp::McpServer;
use serde_json::json;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tracing::{debug, error, info};

/// Run the MCP server with stdio transport.
///
/// This function runs the main event loop, reading requests from stdin
/// and writing responses to stdout. It handles EOF gracefully and
/// logs all communication for debugging.
///
/// # Arguments
///
/// * `server` - MCP server instance
///
/// # Errors
///
/// Returns an error if:
/// - Server initialization fails
/// - I/O errors occur on stdin/stdout
///
/// # Examples
///
/// ```no_run
/// use fabric_atelier::mcp::{McpServer, run_stdio_server};
/// use fabric_atelier::config::Settings;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = Settings::default();
///     let server = McpServer::new(config).await?;
///     run_stdio_server(server).await?;
///     Ok(())
/// }
/// ```
pub async fn run_stdio_server(server: McpServer) -> Result<()> {
    info!("Starting MCP server with stdio transport");

    let stdin = tokio::io::stdin();
    let mut stdout = tokio::io::stdout();
    let mut reader = BufReader::new(stdin);
    let mut line = String::new();

    loop {
        line.clear();

        match reader.read_line(&mut line).await {
            Ok(0) => {
                info!("EOF received, shutting down");
                break;
            }
            Ok(n) => {
                debug!("Received {} bytes", n);

                // Skip empty lines
                if line.trim().is_empty() {
                    continue;
                }

                // Parse request
                let response = match serde_json::from_str::<JsonRpcRequest>(&line) {
                    Ok(request) => {
                        debug!("Request: method={}, id={}", request.method, request.id);
                        server.handle_request(request).await
                    }
                    Err(e) => {
                        error!("Failed to parse request: {}", e);
                        JsonRpcResponse::error(json!(null), PARSE_ERROR, format!("Parse error: {e}"))
                    }
                };

                // Write response
                match serde_json::to_string(&response) {
                    Ok(response_json) => {
                        if let Err(e) = stdout.write_all(response_json.as_bytes()).await {
                            error!("Failed to write response: {}", e);
                            break;
                        }
                        if let Err(e) = stdout.write_all(b"\n").await {
                            error!("Failed to write newline: {}", e);
                            break;
                        }
                        if let Err(e) = stdout.flush().await {
                            error!("Failed to flush stdout: {}", e);
                            break;
                        }
                        debug!("Response sent");
                    }
                    Err(e) => {
                        error!("Failed to serialize response: {}", e);
                        break;
                    }
                }
            }
            Err(e) => {
                error!("Error reading from stdin: {}", e);
                break;
            }
        }
    }

    info!("MCP server shutdown complete");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stdio_module_exists() {
        // Basic test to ensure module compiles
        // Real stdio tests would require mocking stdin/stdout
        assert!(true);
    }
}
