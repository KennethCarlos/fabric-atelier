//! Error type definitions.
//!
//! Defines all error variants that can occur in Fabric Atelier.
//! Uses `thiserror` for ergonomic error handling.

use thiserror::Error;

/// Main error type for Fabric Atelier.
///
/// All errors in the application are represented by this enum.
/// Each variant provides context about what went wrong.
#[derive(Error, Debug)]
pub enum Error {
    /// Pattern directory not found at any expected location.
    #[error("Pattern directory not found. Tried: {locations:?}")]
    PatternDirectoryNotFound { locations: Vec<String> },

    /// Pattern file is invalid or malformed.
    #[error("Invalid pattern '{name}': {reason}")]
    InvalidPattern { name: String, reason: String },

    /// Pattern not found by name.
    #[error("Pattern '{0}' not found")]
    PatternNotFound(String),

    /// Failed to execute pattern via Fabric CLI.
    #[error("Failed to execute pattern '{pattern}': {message}")]
    PatternExecutionFailed { pattern: String, message: String },

    /// Embedding dimension mismatch.
    #[error("Invalid embedding dimension: expected {expected}, got {actual}")]
    InvalidEmbeddingDimension { expected: usize, actual: usize },

    /// Invalid Arrow schema.
    #[error("Invalid Arrow schema: {0}")]
    InvalidArrowSchema(String),

    /// MCP protocol error.
    #[error("MCP protocol error: {0}")]
    McpProtocolError(String),

    /// Configuration error.
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// I/O error.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// Arrow error.
    #[error(transparent)]
    Arrow(#[from] arrow::error::ArrowError),

    /// Parquet error.
    #[error(transparent)]
    Parquet(#[from] parquet::errors::ParquetError),

    /// JSON serialization error.
    #[error(transparent)]
    Json(#[from] serde_json::Error),

    /// TOML deserialization error.
    #[error(transparent)]
    Toml(#[from] toml::de::Error),
}

impl Error {
    /// Create a pattern execution error.
    pub fn pattern_execution_failed(
        pattern: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self::PatternExecutionFailed {
            pattern: pattern.into(),
            message: message.into(),
        }
    }

    /// Create an invalid pattern error.
    pub fn invalid_pattern(name: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::InvalidPattern {
            name: name.into(),
            reason: reason.into(),
        }
    }

    /// Create a pattern directory not found error.
    pub fn pattern_directory_not_found(locations: Vec<String>) -> Self {
        Self::PatternDirectoryNotFound { locations }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_not_found() {
        let err = Error::PatternNotFound("summarize".to_string());
        assert_eq!(err.to_string(), "Pattern 'summarize' not found");
    }

    #[test]
    fn test_invalid_embedding_dimension() {
        let err = Error::InvalidEmbeddingDimension {
            expected: 1536,
            actual: 768,
        };
        assert_eq!(
            err.to_string(),
            "Invalid embedding dimension: expected 1536, got 768"
        );
    }

    #[test]
    fn test_pattern_execution_failed() {
        let err = Error::pattern_execution_failed("summarize", "timeout");
        assert!(err.to_string().contains("summarize"));
        assert!(err.to_string().contains("timeout"));
    }
}
