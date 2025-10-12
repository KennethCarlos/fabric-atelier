# Code Style Guide

## Core Principles

1. **No file exceeds 300 lines** - Split into submodules when approaching limit
2. **Every public API is documented** - rustdoc comments required
3. **Explicit over implicit** - No magic, clear behavior
4. **Comments explain WHY, not WHAT** - Code should be self-documenting

## Documentation Standards

### Module Documentation
```rust
//! Brief module summary.
//!
//! Detailed explanation of purpose and usage.
//!
//! # Examples
//! ```
//! use fabric_atelier::vector::VectorSearch;
//! let search = VectorSearch::load()?;
//! ```
```

### Function Documentation
```rust
/// Brief function summary.
///
/// # Arguments
/// * `query` - Query embedding vector
/// * `top_k` - Number of results
///
/// # Returns
/// Sorted search results (highest score first)
///
/// # Errors
/// Returns error if query dimension mismatches
///
/// # Performance
/// < 1ms for 227 patterns using SIMD
pub fn search(&self, query: &[f32], top_k: usize) -> Result<Vec<SearchResult>>
```

### Inline Comments
```rust
// Good: Explain WHY
// Use cosine similarity because it's scale-invariant
let similarity = cosine_similarity(&a, &b);

// Bad: State the obvious
// Calculate cosine similarity
let similarity = cosine_similarity(&a, &b);
```

## Naming Conventions

- Types: `PascalCase` (VectorSearch, PatternManager)
- Functions: `snake_case` (load_patterns, compute_similarity)
- Constants: `SCREAMING_SNAKE_CASE` (DEFAULT_TOP_K)
- Modules: `snake_case` (vector_search, pattern_loader)

## Error Handling

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Pattern directory not found")]
    PatternDirectoryNotFound,
    
    #[error("Invalid dimension: expected {expected}, got {actual}")]
    InvalidDimension { expected: usize, actual: usize },
    
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
```

## Logging

```rust
use tracing::{debug, info, warn, error, instrument};

#[instrument(skip(self))]
pub async fn load_patterns(&self) -> Result<Vec<Pattern>> {
    info!("Loading patterns from {}", self.dir.display());
    let patterns = self.scan().await?;
    debug!("Found {} patterns", patterns.len());
    Ok(patterns)
}
```

## Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pattern_name() {
        let pattern = Pattern::new("summarize");
        assert_eq!(pattern.tool_name(), "fabric_summarize");
    }
}

#[tokio::test]
async fn test_async_load() {
    let manager = PatternManager::new(config()).await.unwrap();
    assert!(manager.patterns().await.len() > 200);
}
```
