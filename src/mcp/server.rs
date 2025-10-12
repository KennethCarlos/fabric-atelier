//! MCP server implementation.
//!
//! Handles MCP protocol requests and dispatches them to appropriate handlers.

use crate::config::Settings;
use crate::error::Result;
use crate::fabric::{Pattern, PatternLoader};
use crate::mcp::protocol::{JsonRpcRequest, JsonRpcResponse, INTERNAL_ERROR, METHOD_NOT_FOUND};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

/// MCP server state.
///
/// Holds all state needed to handle MCP requests.
/// Uses Arc<RwLock<>> for thread-safe shared access.
pub struct McpServer {
    /// Pattern loader for loading patterns.
    pattern_loader: Arc<PatternLoader>,

    /// Cached patterns (loaded once, reused).
    patterns: Arc<RwLock<Vec<Pattern>>>,

    /// Server configuration.
    config: Arc<Settings>,
}

impl McpServer {
    /// Create a new MCP server instance.
    ///
    /// # Arguments
    ///
    /// * `config` - Server configuration
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Pattern loading fails
    /// - Configuration is invalid
    pub async fn new(config: Settings) -> Result<Self> {
        let config = Arc::new(config);

        info!("Initializing MCP server");

        // Initialize pattern loader
        let pattern_loader = Arc::new(PatternLoader::new()?);

        // Load patterns
        let patterns = pattern_loader.load_all().await?;
        info!("Loaded {} patterns", patterns.len());

        Ok(Self {
            pattern_loader,
            patterns: Arc::new(RwLock::new(patterns)),
            config,
        })
    }

    /// Handle an incoming JSON-RPC request.
    ///
    /// This is the main entry point for request processing.
    /// Dispatches to the appropriate handler based on the method name.
    ///
    /// # Arguments
    ///
    /// * `request` - JSON-RPC request to handle
    pub async fn handle_request(&self, request: JsonRpcRequest) -> JsonRpcResponse {
        debug!("Handling request: method={}", request.method);

        match request.method.as_str() {
            "initialize" => self.handle_initialize(request.id).await,
            "tools/list" => self.handle_tools_list(request.id).await,
            "tools/call" => self.handle_tools_call(request.id, request.params).await,
            _ => self.method_not_found(request.id),
        }
    }

    /// Handle initialize request.
    ///
    /// Returns server capabilities and information.
    async fn handle_initialize(&self, id: Value) -> JsonRpcResponse {
        debug!("Handling initialize");

        JsonRpcResponse::success(
            id,
            json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": self.config.server.name,
                    "version": self.config.server.version
                }
            }),
        )
    }

    /// Handle tools/list request.
    ///
    /// Returns list of all available pattern tools.
    async fn handle_tools_list(&self, id: Value) -> JsonRpcResponse {
        debug!("Handling tools/list");

        let patterns = self.patterns.read().await;

        let tools: Vec<Value> = patterns
            .iter()
            .map(|pattern| {
                json!({
                    "name": pattern.tool_name(),
                    "description": pattern.description,
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "content": {
                                "type": "string",
                                "description": "The content to process with this pattern"
                            }
                        },
                        "required": ["content"]
                    }
                })
            })
            .collect();

        JsonRpcResponse::success(id, json!({ "tools": tools }))
    }

    /// Handle tools/call request.
    ///
    /// Executes a pattern tool.
    ///
    /// # Arguments
    ///
    /// * `id` - Request ID
    /// * `params` - Request parameters containing tool name and arguments
    async fn handle_tools_call(&self, id: Value, params: Value) -> JsonRpcResponse {
        debug!("Handling tools/call");

        // Extract tool name
        let tool_name = match params.get("name").and_then(|v| v.as_str()) {
            Some(name) => name,
            None => {
                return JsonRpcResponse::error(id, INTERNAL_ERROR, "Missing tool name");
            }
        };

        // Extract pattern name from tool name (remove "fabric_" prefix)
        let pattern_name = match tool_name.strip_prefix("fabric_") {
            Some(name) => name,
            None => {
                return JsonRpcResponse::error(
                    id,
                    INTERNAL_ERROR,
                    format!("Invalid tool name: {tool_name}"),
                );
            }
        };

        // Extract content
        let content = match params
            .get("arguments")
            .and_then(|args| args.get("content"))
            .and_then(|c| c.as_str())
        {
            Some(c) => c,
            None => {
                return JsonRpcResponse::error(id, INTERNAL_ERROR, "Missing content argument");
            }
        };

        // Find pattern
        let patterns = self.patterns.read().await;
        let pattern = match patterns.iter().find(|p| p.name == pattern_name) {
            Some(p) => p,
            None => {
                return JsonRpcResponse::error(
                    id,
                    INTERNAL_ERROR,
                    format!("Pattern not found: {pattern_name}"),
                );
            }
        };

        debug!("Executing pattern '{}' with {} bytes of content", pattern.name, content.len());

        // Return the pattern's system prompt and user content for the AI to process
        // The MCP client (Claude) will use the system prompt to process the content
        let prompt_text = if let Some(ref user_prompt) = pattern.user_prompt {
            format!(
                "# System Prompt\n\n{}\n\n# User Prompt Template\n\n{}\n\n# Content to Process\n\n{}",
                pattern.system_prompt, user_prompt, content
            )
        } else {
            format!(
                "# System Prompt\n\n{}\n\n# Content to Process\n\n{}",
                pattern.system_prompt, content
            )
        };

        debug!("Pattern execution successful, returning prompt with {} bytes", prompt_text.len());
        JsonRpcResponse::success(
            id,
            json!({
                "content": [{
                    "type": "text",
                    "text": prompt_text
                }]
            }),
        )
    }

    /// Handle unknown method.
    fn method_not_found(&self, id: Value) -> JsonRpcResponse {
        JsonRpcResponse::error(id, METHOD_NOT_FOUND, "Method not found")
    }

    /// Reload patterns from disk.
    ///
    /// Useful for hot reload functionality.
    pub async fn reload_patterns(&self) -> Result<()> {
        info!("Reloading patterns");
        let patterns = self.pattern_loader.load_all().await?;
        *self.patterns.write().await = patterns;
        info!("Patterns reloaded");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    async fn test_server() -> McpServer {
        let config = Settings::default();
        // Note: This will fail if patterns directory doesn't exist
        // In real tests, we'd use a mock or test fixtures
        McpServer::new(config).await.unwrap()
    }

    #[tokio::test]
    async fn test_initialize() {
        let server = test_server().await;
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: json!(1),
            method: "initialize".to_string(),
            params: json!({}),
        };

        let response = server.handle_request(request).await;
        assert!(response.error.is_none());
        assert!(response.result.is_some());

        let result = response.result.unwrap();
        assert!(result.get("protocolVersion").is_some());
        assert!(result.get("capabilities").is_some());
        assert!(result.get("serverInfo").is_some());
    }

    #[tokio::test]
    async fn test_method_not_found() {
        let server = test_server().await;
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: json!(1),
            method: "unknown/method".to_string(),
            params: json!({}),
        };

        let response = server.handle_request(request).await;
        assert!(response.result.is_none());
        assert!(response.error.is_some());

        let error = response.error.unwrap();
        assert_eq!(error.code, METHOD_NOT_FOUND);
    }
}
