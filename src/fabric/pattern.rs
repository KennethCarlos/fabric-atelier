//! Pattern type definition.
//!
//! Represents a Fabric pattern with its metadata and content.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::SystemTime;

/// A Fabric pattern.
///
/// Represents a single pattern from the Fabric patterns directory.
/// Each pattern consists of a system prompt (required) and optional
/// user prompt, along with extracted metadata.
///
/// # Examples
///
/// ```
/// use fabric_atelier::fabric::Pattern;
/// use std::path::PathBuf;
///
/// let pattern = Pattern {
///     name: "summarize".to_string(),
///     description: "Summarize content".to_string(),
///     system_prompt: "You are a summarizer".to_string(),
///     user_prompt: None,
///     category: Some("writing".to_string()),
///     tags: vec!["summarization".to_string()],
///     path: PathBuf::from("patterns/summarize"),
///     modified: std::time::SystemTime::now(),
/// };
///
/// assert_eq!(pattern.tool_name(), "fabric_summarize");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    /// Pattern name (directory name).
    pub name: String,

    /// Human-readable description extracted from system prompt.
    pub description: String,

    /// System prompt content (from system.md).
    pub system_prompt: String,

    /// Optional user prompt content (from user.md).
    pub user_prompt: Option<String>,

    /// Optional category classification.
    pub category: Option<String>,

    /// Tags for categorization and search.
    pub tags: Vec<String>,

    /// Path to pattern directory.
    pub path: PathBuf,

    /// Last modified timestamp.
    pub modified: SystemTime,
}

impl Pattern {
    /// Get the MCP tool name for this pattern.
    ///
    /// Converts the pattern name to the MCP tool format by prefixing with "fabric_".
    ///
    /// # Examples
    ///
    /// ```
    /// use fabric_atelier::fabric::Pattern;
    ///
    /// # let pattern = Pattern {
    /// #     name: "summarize".to_string(),
    /// #     description: String::new(),
    /// #     system_prompt: String::new(),
    /// #     user_prompt: None,
    /// #     category: None,
    /// #     tags: vec![],
    /// #     path: std::path::PathBuf::new(),
    /// #     modified: std::time::SystemTime::now(),
    /// # };
    /// assert_eq!(pattern.tool_name(), "fabric_summarize");
    /// ```
    pub fn tool_name(&self) -> String {
        format!("fabric_{}", self.name)
    }

    /// Check if pattern matches a search query.
    ///
    /// Performs case-insensitive matching against name, description, and tags.
    ///
    /// # Arguments
    ///
    /// * `query` - Search query string
    ///
    /// # Examples
    ///
    /// ```
    /// use fabric_atelier::fabric::Pattern;
    ///
    /// # let pattern = Pattern {
    /// #     name: "summarize".to_string(),
    /// #     description: "Summarize content".to_string(),
    /// #     system_prompt: String::new(),
    /// #     user_prompt: None,
    /// #     category: None,
    /// #     tags: vec!["summarization".to_string()],
    /// #     path: std::path::PathBuf::new(),
    /// #     modified: std::time::SystemTime::now(),
    /// # };
    /// assert!(pattern.matches("summar"));
    /// assert!(pattern.matches("content"));
    /// assert!(pattern.matches("summarization"));
    /// assert!(!pattern.matches("security"));
    /// ```
    pub fn matches(&self, query: &str) -> bool {
        let query_lower = query.to_lowercase();

        self.name.to_lowercase().contains(&query_lower)
            || self.description.to_lowercase().contains(&query_lower)
            || self.tags.iter().any(|tag| tag.to_lowercase().contains(&query_lower))
            || self
                .category
                .as_ref()
                .map(|c| c.to_lowercase().contains(&query_lower))
                .unwrap_or(false)
    }

    /// Check if pattern has a user prompt.
    pub fn has_user_prompt(&self) -> bool {
        self.user_prompt.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_pattern() -> Pattern {
        Pattern {
            name: "summarize".to_string(),
            description: "Summarize content".to_string(),
            system_prompt: "You are a summarizer".to_string(),
            user_prompt: Some("Summarize this".to_string()),
            category: Some("writing".to_string()),
            tags: vec!["summarization".to_string(), "writing".to_string()],
            path: PathBuf::from("patterns/summarize"),
            modified: SystemTime::now(),
        }
    }

    #[test]
    fn test_tool_name() {
        let pattern = test_pattern();
        assert_eq!(pattern.tool_name(), "fabric_summarize");
    }

    #[test]
    fn test_matches_name() {
        let pattern = test_pattern();
        assert!(pattern.matches("summar"));
        assert!(pattern.matches("SUMMAR"));
    }

    #[test]
    fn test_matches_description() {
        let pattern = test_pattern();
        assert!(pattern.matches("content"));
    }

    #[test]
    fn test_matches_tags() {
        let pattern = test_pattern();
        assert!(pattern.matches("summarization"));
    }

    #[test]
    fn test_matches_category() {
        let pattern = test_pattern();
        assert!(pattern.matches("writing"));
    }

    #[test]
    fn test_no_match() {
        let pattern = test_pattern();
        assert!(!pattern.matches("security"));
    }

    #[test]
    fn test_has_user_prompt() {
        let pattern = test_pattern();
        assert!(pattern.has_user_prompt());

        let mut pattern_no_user = pattern.clone();
        pattern_no_user.user_prompt = None;
        assert!(!pattern_no_user.has_user_prompt());
    }
}
