//! Pattern loading and management.
//!
//! Handles loading patterns from the filesystem with support for
//! multiple locations and parallel loading.

use crate::error::{Error, Result};
use crate::fabric::Pattern;
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::{debug, info, warn};

/// Pattern loader.
///
/// Handles loading patterns from the filesystem and keeping them in sync.
/// Supports both git submodule and user's Fabric installation.
pub struct PatternLoader {
    /// Path to patterns directory.
    patterns_dir: PathBuf,
}

impl PatternLoader {
    /// Create a new pattern loader.
    ///
    /// Automatically resolves the patterns directory from multiple locations:
    /// 1. Git submodule: `data/fabric/data/patterns`
    /// 2. User's Fabric installation: `~/.config/fabric/patterns`
    ///
    /// # Errors
    ///
    /// Returns an error if no patterns directory is found.
    pub fn new() -> Result<Self> {
        let patterns_dir = Self::resolve_patterns_dir()?;
        info!("Pattern directory: {}", patterns_dir.display());

        Ok(Self { patterns_dir })
    }

    /// Create a pattern loader with a specific directory.
    ///
    /// # Arguments
    ///
    /// * `patterns_dir` - Path to the patterns directory
    pub fn with_directory(patterns_dir: PathBuf) -> Self {
        Self { patterns_dir }
    }

    /// Resolve the patterns directory path.
    ///
    /// Tries multiple locations in order:
    /// 1. Git submodule (data/fabric/data/patterns)
    /// 2. User's Fabric installation (~/.config/fabric/patterns)
    fn resolve_patterns_dir() -> Result<PathBuf> {
        let mut tried_locations = Vec::new();

        // Try git submodule
        let submodule_path = PathBuf::from("data/fabric/data/patterns");
        tried_locations.push(submodule_path.display().to_string());
        if submodule_path.exists() {
            return Ok(submodule_path);
        }

        // Try user's Fabric installation
        if let Ok(home) = std::env::var("HOME") {
            let user_path = PathBuf::from(format!("{home}/.config/fabric/patterns"));
            tried_locations.push(user_path.display().to_string());
            if user_path.exists() {
                return Ok(user_path);
            }
        }

        Err(Error::pattern_directory_not_found(tried_locations))
    }

    /// Load all patterns from the patterns directory.
    ///
    /// Scans the directory and loads all valid patterns.
    /// Invalid patterns are logged and skipped.
    ///
    /// # Errors
    ///
    /// Returns an error if the directory cannot be read.
    pub async fn load_all(&self) -> Result<Vec<Pattern>> {
        info!("Loading patterns from {}", self.patterns_dir.display());

        let mut entries = fs::read_dir(&self.patterns_dir).await?;
        let mut patterns = Vec::new();

        while let Some(entry) = entries.next_entry().await? {
            if entry.file_type().await?.is_dir() {
                match self.load_pattern(&entry.path()).await {
                    Ok(pattern) => {
                        debug!("Loaded pattern: {}", pattern.name);
                        patterns.push(pattern);
                    }
                    Err(e) => {
                        warn!("Failed to load pattern from {:?}: {}", entry.path(), e);
                    }
                }
            }
        }

        info!("Loaded {} patterns", patterns.len());
        Ok(patterns)
    }

    /// Load a single pattern from a directory.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the pattern directory
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - system.md file is missing or cannot be read
    /// - Pattern name cannot be extracted from path
    async fn load_pattern(&self, path: &Path) -> Result<Pattern> {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| Error::invalid_pattern("unknown", "Invalid path"))?
            .to_string();

        // Read system.md (required)
        let system_path = path.join("system.md");
        let system_prompt = fs::read_to_string(&system_path).await.map_err(|e| {
            Error::invalid_pattern(&name, format!("Failed to read system.md: {e}"))
        })?;

        // Read user.md (optional)
        let user_path = path.join("user.md");
        let user_prompt = fs::read_to_string(&user_path).await.ok();

        // Extract description from system prompt
        let description = extract_description(&system_prompt);

        // Auto-extract category and tags
        let (category, tags) = extract_metadata(&system_prompt);

        // Get last modified time
        let metadata = fs::metadata(path).await?;
        let modified = metadata.modified()?;

        Ok(Pattern {
            name,
            description,
            system_prompt,
            user_prompt,
            category,
            tags,
            path: path.to_path_buf(),
            modified,
        })
    }

    /// Get the patterns directory path.
    pub fn patterns_dir(&self) -> &Path {
        &self.patterns_dir
    }
}

/// Extract description from system prompt content.
///
/// Looks for "# IDENTITY" or "# PURPOSE" section and extracts the first paragraph.
fn extract_description(content: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();

    // Look for IDENTITY or PURPOSE section
    for (i, line) in lines.iter().enumerate() {
        if line.contains("# IDENTITY") || line.contains("# PURPOSE") {
            // Get next non-empty line
            for j in (i + 1)..lines.len() {
                let desc = lines[j].trim();
                if !desc.is_empty() && !desc.starts_with('#') {
                    return desc.to_string();
                }
            }
        }
    }

    // Fallback: first non-empty, non-heading paragraph
    content
        .lines()
        .skip_while(|l| l.trim().is_empty() || l.starts_with('#'))
        .take_while(|l| !l.trim().is_empty())
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .take(200)
        .collect()
}

/// Extract metadata (category and tags) from content.
///
/// Uses simple heuristics based on keywords in the content.
fn extract_metadata(content: &str) -> (Option<String>, Vec<String>) {
    let mut category = None;
    let mut tags = Vec::new();
    let lower = content.to_lowercase();

    // Security category
    if lower.contains("security") || lower.contains("threat") || lower.contains("vulnerability") {
        category = Some("security".to_string());
        tags.push("security".to_string());
    }

    // Writing category
    if lower.contains("writing") || lower.contains("essay") || lower.contains("article") {
        if category.is_none() {
            category = Some("writing".to_string());
        }
        tags.push("writing".to_string());
    }

    // Coding category
    if lower.contains("code") || lower.contains("programming") || lower.contains("software") {
        if category.is_none() {
            category = Some("coding".to_string());
        }
        tags.push("coding".to_string());
    }

    // Analysis tag
    if lower.contains("analyze") || lower.contains("analysis") || lower.contains("extract") {
        tags.push("analysis".to_string());
    }

    // Summarization tag
    if lower.contains("summarize") || lower.contains("summary") {
        tags.push("summarization".to_string());
    }

    // Deduplicate tags
    tags.sort();
    tags.dedup();

    (category, tags)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_description_with_identity() {
        let content = "# IDENTITY\n\nYou are a helpful assistant.\n\n# STEPS";
        let desc = extract_description(content);
        assert_eq!(desc, "You are a helpful assistant.");
    }

    #[test]
    fn test_extract_description_with_purpose() {
        let content = "# PURPOSE\n\nSummarize content effectively.\n\n# OUTPUT";
        let desc = extract_description(content);
        assert_eq!(desc, "Summarize content effectively.");
    }

    #[test]
    fn test_extract_description_fallback() {
        let content = "This is a pattern.\n\nIt does things.";
        let desc = extract_description(content);
        assert!(desc.contains("This is a pattern"));
    }

    #[test]
    fn test_extract_metadata_security() {
        let content = "Analyze security threats and vulnerabilities";
        let (category, tags) = extract_metadata(content);
        assert_eq!(category, Some("security".to_string()));
        assert!(tags.contains(&"security".to_string()));
        assert!(tags.contains(&"analysis".to_string()));
    }

    #[test]
    fn test_extract_metadata_writing() {
        let content = "Help with writing essays and articles";
        let (category, tags) = extract_metadata(content);
        assert_eq!(category, Some("writing".to_string()));
        assert!(tags.contains(&"writing".to_string()));
    }

    #[test]
    fn test_extract_metadata_coding() {
        let content = "Analyze code and programming patterns";
        let (category, tags) = extract_metadata(content);
        assert_eq!(category, Some("coding".to_string()));
        assert!(tags.contains(&"coding".to_string()));
        assert!(tags.contains(&"analysis".to_string()));
    }

    #[test]
    fn test_extract_metadata_summarization() {
        let content = "Summarize content and extract key points";
        let (_category, tags) = extract_metadata(content);
        assert!(tags.contains(&"summarization".to_string()));
        assert!(tags.contains(&"analysis".to_string()));
    }
}
