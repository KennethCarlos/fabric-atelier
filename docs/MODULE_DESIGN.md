# Module Design

## Directory Structure

```
src/
├── main.rs              # Entry point (< 100 lines)
├── lib.rs               # Library exports
├── mcp/                 # MCP Protocol Layer
│   ├── mod.rs
│   ├── protocol.rs      # JSON-RPC types (< 200 lines)
│   ├── server.rs        # Server implementation (< 250 lines)
│   ├── handlers.rs      # Request handlers (< 200 lines)
│   ├── tools.rs         # Tool definitions (< 150 lines)
│   └── stdio.rs         # Stdio transport (< 150 lines)
├── fabric/              # Fabric Integration
│   ├── mod.rs
│   ├── pattern.rs       # Pattern type (< 150 lines)
│   ├── loader.rs        # Pattern loading (< 250 lines)
│   ├── parser.rs        # Markdown parsing (< 200 lines)
│   ├── executor.rs      # Pattern execution (< 200 lines)
│   ├── metadata.rs      # Metadata extraction (< 150 lines)
│   └── watcher.rs       # File watching (< 200 lines)
├── vector/              # Vector Search
│   ├── mod.rs
│   ├── embeddings.rs    # Embedding generation (< 200 lines)
│   ├── search.rs        # Similarity search (< 250 lines)
│   ├── cache.rs         # Parquet cache (< 200 lines)
│   ├── simd.rs          # SIMD optimizations (< 150 lines)
│   └── index.rs         # Vector index (< 200 lines)
├── arrow/               # Arrow Operations
│   ├── mod.rs
│   ├── schema.rs        # Schema definitions (< 150 lines)
│   ├── builder.rs       # RecordBatch builders (< 200 lines)
│   ├── reader.rs        # Parquet reader (< 150 lines)
│   └── writer.rs        # Parquet writer (< 150 lines)
├── build/               # Build-time Processing
│   ├── mod.rs
│   ├── indexer.rs       # Pattern indexing (< 200 lines)
│   ├── embedder.rs      # Batch embedding (< 200 lines)
│   └── validator.rs     # Pattern validation (< 150 lines)
├── config/              # Configuration
│   ├── mod.rs
│   ├── settings.rs      # Settings types (< 150 lines)
│   └── loader.rs        # Config loading (< 100 lines)
├── error/               # Error Handling
│   ├── mod.rs
│   └── types.rs         # Error types (< 200 lines)
└── utils/               # Utilities
    ├── mod.rs
    ├── logging.rs       # Logging setup (< 100 lines)
    └── metrics.rs       # Metrics (< 150 lines)
```

## Module Responsibilities

### MCP Layer (`src/mcp/`)
**Purpose**: JSON-RPC protocol over stdio

- `protocol.rs` - Type definitions (JsonRpcRequest, JsonRpcResponse)
- `server.rs` - Server state and lifecycle
- `handlers.rs` - Method handlers (initialize, tools/list, tools/call)
- `tools.rs` - Tool schema generation
- `stdio.rs` - Stdin/stdout transport

### Fabric Layer (`src/fabric/`)
**Purpose**: Pattern loading and execution

- `pattern.rs` - Pattern struct and methods
- `loader.rs` - Filesystem scanning and loading
- `parser.rs` - Markdown parsing (system.md, user.md)
- `executor.rs` - Execute via Fabric CLI
- `metadata.rs` - Extract description, tags, category
- `watcher.rs` - File system watching with `notify` crate

### Vector Layer (`src/vector/`)
**Purpose**: Semantic search

- `embeddings.rs` - Generate embeddings via OpenAI/Anthropic
- `search.rs` - Cosine similarity search
- `cache.rs` - Parquet read/write
- `simd.rs` - SIMD-optimized similarity using `simsimd`
- `index.rs` - In-memory vector index

### Arrow Layer (`src/arrow/`)
**Purpose**: Arrow/Parquet operations

- `schema.rs` - Define Arrow schemas
- `builder.rs` - Build RecordBatches
- `reader.rs` - Read Parquet files
- `writer.rs` - Write Parquet files

### Build Layer (`src/build/`)
**Purpose**: Compile-time indexing

- `indexer.rs` - Scan patterns at build time
- `embedder.rs` - Batch generate embeddings
- `validator.rs` - Validate pattern structure

## Key Design Patterns

### 1. Builder Pattern
```rust
let config = Settings::builder()
    .patterns_dir("data/fabric/data/patterns")
    .embedding_provider("openai")
    .build()?;
```

### 2. Arc + RwLock for Shared State
```rust
pub struct McpServer {
    pattern_manager: Arc<PatternManager>,
    vector_search: Arc<RwLock<VectorSearch>>,
}
```

### 3. Lazy Initialization
```rust
use once_cell::sync::Lazy;

static VECTOR_INDEX: Lazy<VectorSearch> = Lazy::new(|| {
    VectorSearch::load_from_cache().expect("Failed to load")
});
```

### 4. Error Context
```rust
use anyhow::Context;

load_pattern(&path)
    .context(format!("Failed to load pattern: {}", path.display()))?;
```

## Module Dependencies

```
main.rs
  └── mcp::server
        ├── fabric::loader
        │     ├── fabric::pattern
        │     ├── fabric::parser
        │     └── fabric::metadata
        ├── fabric::executor
        └── vector::search
              ├── vector::embeddings
              ├── vector::cache
              │     └── arrow::*
              └── vector::simd
```

## File Size Guidelines

When a file approaches 300 lines:

1. **Extract helpers** - Move utility functions to separate file
2. **Split by responsibility** - Create submodules for distinct concerns
3. **Create traits** - Abstract common behavior
4. **Use composition** - Break into smaller types

Example:
```rust
// Before: loader.rs (350 lines)
// After:
// loader.rs (200 lines) - Main logic
// scanner.rs (100 lines) - Directory scanning
// cache.rs (50 lines) - Caching logic
```
