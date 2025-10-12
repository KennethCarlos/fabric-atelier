//! Application settings and configuration.
//!
//! Defines the configuration structure and loading logic.

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Application configuration.
///
/// # Examples
///
/// ```
/// use fabric_atelier::config::Settings;
///
/// let settings = Settings::default();
/// assert_eq!(settings.server.name, "fabric-atelier");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// Server configuration.
    pub server: ServerConfig,

    /// Fabric integration configuration.
    pub fabric: FabricConfig,

    /// Embeddings configuration.
    pub embeddings: EmbeddingsConfig,

    /// Performance configuration.
    pub performance: PerformanceConfig,
}

/// Server configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Server name.
    pub name: String,

    /// Server version.
    pub version: String,
}

/// Fabric integration configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FabricConfig {
    /// Path to patterns directory (optional, auto-detected if not set).
    pub patterns_dir: Option<String>,

    /// Fabric CLI executable path (optional, uses PATH if not set).
    pub cli_path: Option<String>,

    /// Pattern execution timeout in seconds.
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,
}

/// Embeddings configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingsConfig {
    /// Embedding provider (e.g., "openai", "anthropic").
    #[serde(default = "default_provider")]
    pub provider: String,

    /// Model name for embeddings.
    #[serde(default = "default_model")]
    pub model: String,

    /// Embedding dimension.
    #[serde(default = "default_dimension")]
    pub dimension: usize,

    /// Path to embeddings cache file.
    #[serde(default = "default_cache_path")]
    pub cache_path: String,
}

/// Performance configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Enable parallel pattern loading.
    #[serde(default = "default_true")]
    pub parallel_loading: bool,

    /// Enable SIMD optimizations.
    #[serde(default = "default_true")]
    pub simd_enabled: bool,

    /// Enable memory-mapped I/O for cache files.
    #[serde(default = "default_true")]
    pub memory_mapped_io: bool,
}

impl Settings {
    /// Load settings from a TOML file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the TOML configuration file
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or parsed.
    pub fn from_file(path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
        let content = std::fs::read_to_string(&path).map_err(|e| {
            Error::ConfigError(format!("Failed to read config file '{}': {}", path.display(), e))
        })?;

        toml::from_str(&content).map_err(|e| {
            Error::ConfigError(format!("Failed to parse config file '{}': {}", path.display(), e))
        })
    }

    /// Load settings with defaults and optional config file.
    ///
    /// Tries to load from `config/default.toml`, falls back to defaults.
    pub fn load() -> Result<Self> {
        let default_path = PathBuf::from("config/default.toml");

        if default_path.exists() {
            Self::from_file(default_path)
        } else {
            Ok(Self::default())
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                name: "fabric-atelier".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
            fabric: FabricConfig {
                patterns_dir: None,
                cli_path: None,
                timeout_secs: default_timeout(),
            },
            embeddings: EmbeddingsConfig {
                provider: default_provider(),
                model: default_model(),
                dimension: default_dimension(),
                cache_path: default_cache_path(),
            },
            performance: PerformanceConfig {
                parallel_loading: true,
                simd_enabled: true,
                memory_mapped_io: true,
            },
        }
    }
}

// Default value functions
fn default_timeout() -> u64 {
    30
}

fn default_provider() -> String {
    "openai".to_string()
}

fn default_model() -> String {
    "text-embedding-3-small".to_string()
}

fn default_dimension() -> usize {
    1536
}

fn default_cache_path() -> String {
    "data/cache/embeddings.parquet".to_string()
}

fn default_true() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let settings = Settings::default();
        assert_eq!(settings.server.name, "fabric-atelier");
        assert_eq!(settings.fabric.timeout_secs, 30);
        assert_eq!(settings.embeddings.dimension, 1536);
        assert!(settings.performance.simd_enabled);
    }

    #[test]
    fn test_settings_serialization() {
        let settings = Settings::default();
        let toml = toml::to_string(&settings).unwrap();
        assert!(toml.contains("fabric-atelier"));
    }
}
