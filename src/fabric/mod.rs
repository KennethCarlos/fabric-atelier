//! Fabric pattern integration.
//!
//! This module handles loading, parsing, and executing Fabric patterns.

mod executor;
mod loader;
mod pattern;

pub use executor::PatternExecutor;
pub use loader::PatternLoader;
pub use pattern::Pattern;
