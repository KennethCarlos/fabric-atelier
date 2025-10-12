//! MCP (Model Context Protocol) implementation.
//!
//! This module implements the MCP protocol for exposing Fabric patterns
//! as tools to MCP clients like Claude Desktop.

mod protocol;
mod server;
mod stdio;

pub use protocol::{JsonRpcError, JsonRpcRequest, JsonRpcResponse};
pub use server::McpServer;
pub use stdio::run_stdio_server;
