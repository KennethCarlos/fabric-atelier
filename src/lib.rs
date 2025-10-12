//! Fabric Atelier - High-performance MCP server for Fabric patterns.
//!
//! This library provides the core functionality for exposing Fabric's 227+ patterns
//! as MCP tools with sub-millisecond semantic search capabilities.
//!
//! # Architecture
//!
//! - **MCP Layer**: JSON-RPC protocol over stdio
//! - **Fabric Layer**: Pattern loading and execution
//! - **Vector Layer**: Semantic search with Arrow/Parquet
//! - **Arrow Layer**: Columnar data operations
//!
//! # Examples
//!
//! ```no_run
//! use fabric_atelier::config::Settings;
//!
//! let settings = Settings::default();
//! println!("Server: {}", settings.server.name);
//! ```

pub mod config;
pub mod error;
pub mod fabric;
pub mod llm;
pub mod mcp;
pub mod utils;

// Re-export commonly used types
pub use error::{Error, Result};
