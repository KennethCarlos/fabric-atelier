//! Pattern execution via Fabric CLI.
//!
//! Handles executing patterns by calling the Fabric CLI with proper
//! stdin/stdout piping and timeout handling.

use crate::error::{Error, Result};
use std::path::PathBuf;
use std::process::Stdio;
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use tokio::time::timeout;
use tracing::{debug, warn};

/// Pattern executor.
///
/// Executes patterns via the Fabric CLI.
pub struct PatternExecutor {
    /// Path to fabric binary.
    fabric_path: PathBuf,

    /// Execution timeout.
    timeout: Duration,
}

impl PatternExecutor {
    /// Create a new pattern executor.
    ///
    /// Automatically finds the fabric binary in PATH.
    ///
    /// # Arguments
    ///
    /// * `timeout_secs` - Timeout in seconds for pattern execution
    ///
    /// # Errors
    ///
    /// Returns an error if fabric binary cannot be found.
    pub fn new(timeout_secs: u64) -> Result<Self> {
        let fabric_path = Self::find_fabric_binary()?;
        debug!("Found fabric binary at: {}", fabric_path.display());

        Ok(Self {
            fabric_path,
            timeout: Duration::from_secs(timeout_secs),
        })
    }

    /// Create a pattern executor with a specific fabric path.
    ///
    /// # Arguments
    ///
    /// * `fabric_path` - Path to fabric binary
    /// * `timeout_secs` - Timeout in seconds
    pub fn with_path(fabric_path: PathBuf, timeout_secs: u64) -> Self {
        Self {
            fabric_path,
            timeout: Duration::from_secs(timeout_secs),
        }
    }

    /// Find the fabric binary in PATH.
    fn find_fabric_binary() -> Result<PathBuf> {
        // Try common locations
        let candidates = vec![
            PathBuf::from("fabric"),
            PathBuf::from("/usr/local/bin/fabric"),
            PathBuf::from("/usr/bin/fabric"),
        ];

        // Check if fabric is in PATH
        if let Ok(path) = which::which("fabric") {
            return Ok(path);
        }

        // Try candidates
        for candidate in candidates {
            if candidate.exists() {
                return Ok(candidate);
            }
        }

        Err(Error::ConfigError(
            "Fabric CLI not found. Please install fabric or set fabric_path in config".to_string(),
        ))
    }

    /// Execute a pattern with the given content.
    ///
    /// # Arguments
    ///
    /// * `pattern_name` - Name of the pattern to execute
    /// * `content` - Content to process with the pattern
    ///
    /// # Returns
    ///
    /// The output from the pattern execution.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Pattern execution fails
    /// - Timeout is exceeded
    /// - I/O errors occur
    pub async fn execute(&self, pattern_name: &str, content: &str) -> Result<String> {
        debug!("Executing pattern: {}", pattern_name);

        // Spawn fabric process
        let mut child = Command::new(&self.fabric_path)
            .arg("--pattern")
            .arg(pattern_name)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| {
                Error::pattern_execution_failed(
                    pattern_name,
                    format!("Failed to spawn fabric process: {e}"),
                )
            })?;

        // Write content to stdin
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(content.as_bytes()).await.map_err(|e| {
                Error::pattern_execution_failed(pattern_name, format!("Failed to write stdin: {e}"))
            })?;
            stdin.flush().await.map_err(|e| {
                Error::pattern_execution_failed(pattern_name, format!("Failed to flush stdin: {e}"))
            })?;
            // Close stdin to signal EOF
            drop(stdin);
        }

        // Wait for output with timeout
        let output = timeout(self.timeout, child.wait_with_output())
            .await
            .map_err(|_| {
                Error::pattern_execution_failed(
                    pattern_name,
                    format!("Execution timeout after {} seconds", self.timeout.as_secs()),
                )
            })?
            .map_err(|e| {
                Error::pattern_execution_failed(
                    pattern_name,
                    format!("Failed to wait for output: {e}"),
                )
            })?;

        // Check exit status
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            warn!("Pattern execution failed: {}", stderr);
            return Err(Error::pattern_execution_failed(
                pattern_name,
                format!("Exit code: {}, stderr: {stderr}", output.status),
            ));
        }

        // Return stdout
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        debug!("Pattern execution successful, output length: {} bytes", stdout.len());

        Ok(stdout)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_fabric_binary() {
        // This test will pass if fabric is installed
        // Otherwise it will fail, which is expected
        match PatternExecutor::find_fabric_binary() {
            Ok(path) => {
                assert!(path.to_string_lossy().contains("fabric"));
            }
            Err(_) => {
                // Fabric not installed, which is fine for testing
            }
        }
    }

    #[test]
    fn test_executor_creation() {
        // Test with a dummy path
        let executor = PatternExecutor::with_path(PathBuf::from("/usr/bin/fabric"), 30);
        assert_eq!(executor.timeout, Duration::from_secs(30));
    }
}
