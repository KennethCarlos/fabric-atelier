# Fabric Atelier - Production Architecture

**Version**: 1.0.0  
**Status**: Design Phase  
**Last Updated**: October 11, 2025

---

## Executive Summary

Fabric Atelier is a **production-grade, high-performance MCP server** that exposes Fabric's 227+ patterns with sub-millisecond semantic search.

### Core Principles

1. **Radical Modularity** - No file exceeds 300 lines
2. **Documentation-Driven** - Every public API is documented
3. **Performance First** - SIMD-optimized, zero-copy operations
4. **Build-Time Optimization** - Arrow cache generated at compile time

### Performance Targets

| Metric | Target |
|--------|--------|
| Binary Size | < 10 MB |
| Startup Time | < 50 ms |
| Memory Usage | < 30 MB |
| Pattern Search | < 1 ms |
| Pattern Load | < 100 ms |

---

## System Architecture

```
┌─────────────────────────────────────────────────────────┐
│                  Claude Desktop (MCP Client)             │
└────────────────────────┬────────────────────────────────┘
                         │ JSON-RPC over stdio
                         ▼
┌─────────────────────────────────────────────────────────┐
│              Fabric Atelier MCP Server                   │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │ MCP Protocol │──│ Pattern Mgmt │──│ Vector Search│  │
│  │   Handler    │  │   Engine     │  │   Engine     │  │
│  └──────────────┘  └──────────────┘  └──────────────┘  │
│         │                  │                  │          │
│  ┌──────▼──────────────────▼──────────────────▼──────┐  │
│  │        Apache Arrow / DataFusion Core             │  │
│  └───────────────────────────────────────────────────┘  │
└────────────────────────┬────────────────────────────────┘
                         │
        ┌────────────────┼────────────────┐
        ▼                ▼                ▼
┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│   Fabric     │  │   Parquet    │  │  Embedding   │
│   Patterns   │  │    Cache     │  │   Provider   │
│ (Submodule)  │  │ (Build-time) │  │ (OpenAI/etc) │
└──────────────┘  └──────────────┘  └──────────────┘
```

---

## Directory Structure

See [MODULE_STRUCTURE.md](./MODULE_STRUCTURE.md) for complete module breakdown.

```
fabric-atelier/
├── src/
│   ├── main.rs              # Entry point (< 100 lines)
│   ├── mcp/                 # MCP Protocol Layer
│   ├── fabric/              # Fabric Integration
│   ├── vector/              # Vector Search
│   ├── arrow/               # Arrow Operations
│   ├── build/               # Build-time Processing
│   ├── config/              # Configuration
│   ├── error/               # Error Types
│   └── utils/               # Utilities
├── build.rs                 # Build-time indexing
├── data/fabric/             # Git submodule
└── data/cache/              # Generated at build time
```

---

## Design Philosophy

### 1. Radical Modularity

**Rule**: No file exceeds 300 lines. Each module has one responsibility.

**Benefits**:
- Easy to understand and test
- Clear dependency graph
- Parallel development friendly
- Simple code review

### 2. Build-Time Optimization

**Strategy**: Generate Arrow cache at compile time, not runtime.

**Process**:
1. `build.rs` scans Fabric patterns during compilation
2. Generates embeddings (if API key available)
3. Creates Parquet cache file
4. Embeds metadata in binary

**Result**: Sub-50ms startup time (just load pre-built cache)

### 3. Hot Reload Support

**Implementation**: File watcher monitors pattern directory

```rust
// Detects changes to patterns and rebuilds cache
PatternWatcher::new()
    .watch("data/fabric/data/patterns")
    .on_change(|path| {
        rebuild_cache(path)?;
        reload_patterns()?;
    })
```

**Crates**: `notify` (cross-platform file watching), `hotwatch`

### 4. SIMD-Optimized Search

**Strategy**: Use platform-specific SIMD instructions for vector math

**Crates**:
- `simsimd` - SIMD-accelerated similarity metrics
- `arrow` - Built-in SIMD compute kernels
- `faer` - High-performance linear algebra

**Performance**: 10-100x faster than scalar operations

---

## Module Responsibilities

### MCP Protocol Layer (`src/mcp/`)

**Purpose**: Handle JSON-RPC communication over stdio

**Files**:
- `protocol.rs` - Type definitions (< 200 lines)
- `server.rs` - Server implementation (< 250 lines)
- `handlers.rs` - Request handlers (< 200 lines)
- `tools.rs` - Tool definitions (< 150 lines)
- `stdio.rs` - Stdio transport (< 150 lines)

### Fabric Integration (`src/fabric/`)

**Purpose**: Load and execute Fabric patterns

**Files**:
- `pattern.rs` - Pattern type (< 150 lines)
- `loader.rs` - Pattern loading (< 250 lines)
- `parser.rs` - Markdown parsing (< 200 lines)
- `executor.rs` - Pattern execution (< 200 lines)
- `metadata.rs` - Metadata extraction (< 150 lines)
- `watcher.rs` - File watching (< 200 lines)

### Vector Search (`src/vector/`)

**Purpose**: Semantic pattern discovery

**Files**:
- `embeddings.rs` - Embedding generation (< 200 lines)
- `search.rs` - Similarity search (< 250 lines)
- `cache.rs` - Parquet cache (< 200 lines)
- `simd.rs` - SIMD optimizations (< 150 lines)
- `index.rs` - Vector index (< 200 lines)

### Arrow Operations (`src/arrow/`)

**Purpose**: Arrow/Parquet data operations

**Files**:
- `schema.rs` - Schema definitions (< 150 lines)
- `builder.rs` - RecordBatch builders (< 200 lines)
- `reader.rs` - Parquet reader (< 150 lines)
- `writer.rs` - Parquet writer (< 150 lines)

### Build System (`src/build/`)

**Purpose**: Build-time pattern indexing

**Files**:
- `indexer.rs` - Pattern indexing (< 200 lines)
- `embedder.rs` - Batch embedding (< 200 lines)
- `validator.rs` - Pattern validation (< 150 lines)

---

## Data Flow

### Build Time

```
1. build.rs runs
   ↓
2. Scan data/fabric/data/patterns/
   ↓
3. Load all patterns
   ↓
4. Generate embeddings (if API key set)
   ↓
5. Build Arrow RecordBatch
   ↓
6. Write data/cache/embeddings.parquet
   ↓
7. Embed metadata in binary
```

### Runtime

```
1. Binary starts
   ↓
2. Load embeddings.parquet (memory-mapped)
   ↓
3. Initialize MCP server (< 50ms)
   ↓
4. Listen on stdio
   ↓
5. Handle requests:
   - tools/list → Return all patterns
   - tools/call → Execute pattern via Fabric CLI
   - fabric_find_pattern → Vector search (< 1ms)
```

### Hot Reload

```
1. File watcher detects change
   ↓
2. Rebuild affected patterns
   ↓
3. Update Arrow cache
   ↓
4. Reload in-memory index
   ↓
5. Log reload completion
```

---

## Technology Stack

### Core Dependencies

```toml
[dependencies]
# Async runtime
tokio = { version = "1.40", features = ["full"] }

# MCP Protocol
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Arrow ecosystem
arrow = "53.0"
parquet = "53.0"
datafusion = "42.0"

# SIMD-optimized operations
simsimd = "5.0"      # SIMD similarity metrics
faer = "0.19"        # Linear algebra

# File watching
notify = "6.1"       # Cross-platform file watcher

# Configuration
config = "0.14"
toml = "0.8"

# Error handling
anyhow = "1.0"
thiserror = "1.0"

# Logging
tracing = "0.1"
tracing-subscriber = "0.3"

# Utilities
once_cell = "1.19"   # Lazy static initialization
rayon = "1.10"       # Data parallelism
```

### Build Dependencies

```toml
[build-dependencies]
reqwest = { version = "0.12", features = ["blocking", "json"] }
tokio = { version = "1.40", features = ["rt", "macros"] }
```

---

## Performance Optimization Strategies

### 1. Memory-Mapped I/O

```rust
use memmap2::Mmap;

// Memory-map Parquet file for zero-copy access
let file = File::open("data/cache/embeddings.parquet")?;
let mmap = unsafe { Mmap::map(&file)? };
let reader = ParquetReader::new(Cursor::new(&mmap[..]))?;
```

### 2. SIMD Acceleration

```rust
use simsimd::SpatialSimilarity;

// Use SIMD for cosine similarity
let similarity = f32::cosine(&query_embedding, &pattern_embedding)?;
```

### 3. Parallel Processing

```rust
use rayon::prelude::*;

// Parallel pattern loading
let patterns: Vec<Pattern> = entries
    .par_iter()
    .filter_map(|entry| load_pattern(entry).ok())
    .collect();
```

### 4. Lazy Initialization

```rust
use once_cell::sync::Lazy;

static VECTOR_INDEX: Lazy<VectorSearch> = Lazy::new(|| {
    VectorSearch::load_from_cache().expect("Failed to load index")
});
```

---

## Error Handling Strategy

### Custom Error Types

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Pattern directory not found")]
    PatternDirectoryNotFound,
    
    #[error("Invalid embedding dimension: expected {expected}, got {actual}")]
    InvalidEmbeddingDimension { expected: usize, actual: usize },
    
    #[error("Pattern not found: {0}")]
    PatternNotFound(String),
    
    #[error("Fabric execution failed: {0}")]
    FabricExecutionFailed(String),
    
    #[error(transparent)]
    Io(#[from] std::io::Error),
    
    #[error(transparent)]
    Arrow(#[from] arrow::error::ArrowError),
}
```

### Error Context

```rust
use anyhow::Context;

// Add context to errors
load_pattern(&path)
    .context(format!("Failed to load pattern from {}", path.display()))?;
```

---

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_pattern_parsing() {
        let content = "# IDENTITY\nYou are a summarizer";
        let desc = extract_description(content);
        assert_eq!(desc, "You are a summarizer");
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_mcp_protocol() {
    let server = McpServer::new(test_config()).await.unwrap();
    let request = JsonRpcRequest {
        jsonrpc: "2.0".into(),
        id: 1.into(),
        method: "tools/list".into(),
        params: json!({}),
    };
    let response = server.handle_request(request).await;
    assert!(response.error.is_none());
}
```

### Benchmarks

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_search(c: &mut Criterion) {
    let index = VectorSearch::load_from_cache().unwrap();
    let query = vec![0.1; 1536];
    
    c.bench_function("vector_search_top_10", |b| {
        b.iter(|| index.search(black_box(&query), 10))
    });
}
```

---

## Configuration

See [CONFIGURATION.md](./CONFIGURATION.md) for complete configuration reference.

```toml
# config/default.toml
[server]
name = "fabric-atelier"
version = "0.1.0"

[fabric]
patterns_dir = "data/fabric/data/patterns"
executor = "cli"  # or "api"

[embeddings]
provider = "openai"
model = "text-embedding-3-small"
dimension = 1536
cache_path = "data/cache/embeddings.parquet"

[performance]
parallel_loading = true
simd_enabled = true
memory_mapped_io = true
```

---

## Build System

### Build Script (`build.rs`)

```rust
fn main() {
    // Rerun if patterns change
    println!("cargo:rerun-if-changed=data/fabric/data/patterns");
    
    // Skip embeddings for fast builds
    if env::var("SKIP_EMBEDDINGS").is_ok() {
        return;
    }
    
    // Index patterns and generate cache
    if let Err(e) = build_cache() {
        println!("cargo:warning=Failed to build cache: {}", e);
    }
}
```

### Build Commands

```bash
# Full build with embeddings
OPENAI_API_KEY=sk-... cargo build --release

# Fast build (skip embeddings)
SKIP_EMBEDDINGS=1 cargo build --release

# Development build with hot reload
cargo watch -x run
```

---

## Next Steps

See [IMPLEMENTATION_PLAN.md](./IMPLEMENTATION_PLAN.md) for detailed roadmap.

1. ✅ Architecture design
2. → Implement core modules
3. → Build system integration
4. → Testing & benchmarking
5. → Documentation & examples
