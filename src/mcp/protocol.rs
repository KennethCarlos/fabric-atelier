//! MCP protocol type definitions.
//!
//! Implements JSON-RPC 2.0 types for the Model Context Protocol.

use serde::{Deserialize, Serialize};
use serde_json::Value;

// Standard JSON-RPC 2.0 error codes
pub const PARSE_ERROR: i32 = -32700;
pub const INVALID_REQUEST: i32 = -32600;
pub const METHOD_NOT_FOUND: i32 = -32601;
pub const INVALID_PARAMS: i32 = -32602;
pub const INTERNAL_ERROR: i32 = -32603;

/// JSON-RPC 2.0 request structure.
///
/// # Examples
///
/// ```json
/// {
///   "jsonrpc": "2.0",
///   "id": 1,
///   "method": "tools/list",
///   "params": {}
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    /// JSON-RPC version (always "2.0").
    pub jsonrpc: String,

    /// Request identifier (can be string or number).
    pub id: Value,

    /// Method name to invoke.
    pub method: String,

    /// Optional parameters.
    #[serde(default)]
    pub params: Value,
}

/// JSON-RPC 2.0 response structure.
///
/// Either `result` or `error` must be present, but not both.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    /// JSON-RPC version (always "2.0").
    pub jsonrpc: String,

    /// Request identifier (echoed from request).
    pub id: Value,

    /// Success result (mutually exclusive with error).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,

    /// Error result (mutually exclusive with result).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

/// JSON-RPC error object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    /// Error code.
    pub code: i32,

    /// Human-readable error message.
    pub message: String,

    /// Optional additional error data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl JsonRpcResponse {
    /// Create a success response.
    ///
    /// # Arguments
    ///
    /// * `id` - Request ID to echo
    /// * `result` - Result value
    pub fn success(id: Value, result: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(result),
            error: None,
        }
    }

    /// Create an error response.
    ///
    /// # Arguments
    ///
    /// * `id` - Request ID to echo
    /// * `code` - Error code
    /// * `message` - Error message
    pub fn error(id: Value, code: i32, message: impl Into<String>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(JsonRpcError {
                code,
                message: message.into(),
                data: None,
            }),
        }
    }

    /// Create an error response with additional data.
    ///
    /// # Arguments
    ///
    /// * `id` - Request ID to echo
    /// * `code` - Error code
    /// * `message` - Error message
    /// * `data` - Additional error data
    pub fn error_with_data(
        id: Value,
        code: i32,
        message: impl Into<String>,
        data: Value,
    ) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(JsonRpcError {
                code,
                message: message.into(),
                data: Some(data),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_success_response() {
        let response = JsonRpcResponse::success(json!(1), json!({"status": "ok"}));
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, json!(1));
        assert!(response.result.is_some());
        assert!(response.error.is_none());
    }

    #[test]
    fn test_error_response() {
        let response = JsonRpcResponse::error(json!(1), METHOD_NOT_FOUND, "Method not found");
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, json!(1));
        assert!(response.result.is_none());
        assert!(response.error.is_some());

        let error = response.error.unwrap();
        assert_eq!(error.code, METHOD_NOT_FOUND);
        assert_eq!(error.message, "Method not found");
    }

    #[test]
    fn test_request_serialization() {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: json!(1),
            method: "tools/list".to_string(),
            params: json!({}),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        assert!(serialized.contains("\"jsonrpc\":\"2.0\""));
        assert!(serialized.contains("\"method\":\"tools/list\""));
    }

    #[test]
    fn test_response_serialization() {
        let response = JsonRpcResponse::success(json!(1), json!({"tools": []}));
        let serialized = serde_json::to_string(&response).unwrap();
        assert!(serialized.contains("\"jsonrpc\":\"2.0\""));
        assert!(serialized.contains("\"result\""));
        assert!(!serialized.contains("\"error\""));
    }
}
