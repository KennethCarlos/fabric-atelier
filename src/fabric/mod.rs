//! Fabric pattern integration.
//!
//! This module handles loading, parsing, and executing Fabric patterns.

mod loader;
mod pattern;

pub use loader::PatternLoader;
pub use pattern::Pattern;
