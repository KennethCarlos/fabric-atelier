//! Error types for Fabric Atelier.
//!
//! This module provides a unified error type using `thiserror` for
//! structured error handling throughout the application.

mod types;

pub use types::Error;
pub type Result<T> = std::result::Result<T, Error>;
