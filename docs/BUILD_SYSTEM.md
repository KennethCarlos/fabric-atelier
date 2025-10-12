# Build System Design

## Overview

The build system generates Arrow/Parquet cache at **compile time**, enabling sub-50ms startup.

## Build Script (`build.rs`)

```rust
use std::env;
use std::path::PathBuf;

fn main() {
    // Rerun if patterns change
    println!("cargo:rerun-if-changed=data/fabric/data/patterns");
    
    // Skip embeddings for fast builds
    if env::var("SKIP_EMBEDDINGS").is_ok() {
        println!("cargo:warning=Skipping embedding generation");
        return;
    }
    
    let patterns_dir = PathBuf::from("data/fabric/data/patterns");
    
    if !patterns_dir.exists() {
        println!("cargo:warning=Patterns directory not found");
        return;
    }
    
    // Run indexing
    if let Err(e) = build_cache(&patterns_dir) {
        println!("cargo:warning=Failed to build cache: {}", e);
    }
}

fn build_cache(patterns_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:warning=Building pattern cache...");
    
    // 1. Load all patterns
    let patterns = scan_patterns(patterns_dir)?;
    println!("cargo:warning=Found {} patterns", patterns.len());
    
    // 2. Generate embeddings (if API key available)
    let embeddings = if let Ok(api_key) = env::var("OPENAI_API_KEY") {
        println!("cargo:warning=Generating embeddings...");
        generate_embeddings(&patterns, &api_key)?
    } else {
        println!("cargo:warning=No API key, skipping embeddings");
        vec![]
    };
    
    // 3. Build Arrow RecordBatch
    let batch = build_record_batch(&patterns, &embeddings)?;
    
    // 4. Write to Parquet
    write_parquet(&batch, "data/cache/embeddings.parquet")?;
    
    println!("cargo:warning=Cache built successfully");
    Ok(())
}
```

## Build Commands

```bash
# Full build with embeddings
OPENAI_API_KEY=sk-... cargo build --release

# Fast build (skip embeddings)
SKIP_EMBEDDINGS=1 cargo build --release

# Development build
cargo build

# Clean rebuild
cargo clean && cargo build --release
```

## Hot Reload System

### File Watcher (`src/fabric/watcher.rs`)

```rust
use notify::{Watcher, RecursiveMode, Event};
use std::path::Path;
use tokio::sync::mpsc;

pub struct PatternWatcher {
    watcher: notify::RecommendedWatcher,
    rx: mpsc::Receiver<Event>,
}

impl PatternWatcher {
    pub fn new(patterns_dir: &Path) -> Result<Self> {
        let (tx, rx) = mpsc::channel(100);
        
        let mut watcher = notify::recommended_watcher(move |res| {
            if let Ok(event) = res {
                let _ = tx.blocking_send(event);
            }
        })?;
        
        watcher.watch(patterns_dir, RecursiveMode::Recursive)?;
        
        Ok(Self { watcher, rx })
    }
    
    pub async fn watch(&mut self) -> Option<Event> {
        self.rx.recv().await
    }
}
```

### Hot Reload Loop

```rust
pub async fn run_with_hot_reload(server: Arc<McpServer>) -> Result<()> {
    let patterns_dir = server.patterns_dir();
    let mut watcher = PatternWatcher::new(&patterns_dir)?;
    
    tokio::spawn(async move {
        while let Some(event) = watcher.watch().await {
            info!("Pattern change detected: {:?}", event);
            
            // Rebuild cache
            if let Err(e) = rebuild_cache(&patterns_dir).await {
                error!("Failed to rebuild cache: {}", e);
                continue;
            }
            
            // Reload patterns
            if let Err(e) = server.reload_patterns().await {
                error!("Failed to reload patterns: {}", e);
            } else {
                info!("Patterns reloaded successfully");
            }
        }
    });
    
    Ok(())
}
```

## Development Workflow

### Watch Mode

```bash
# Install cargo-watch
cargo install cargo-watch

# Run with auto-reload
cargo watch -x run

# Run tests on change
cargo watch -x test
```

### Script (`scripts/watch.sh`)

```bash
#!/bin/bash
set -e

echo "Starting Fabric Atelier in watch mode..."

# Watch for changes and rebuild
cargo watch \
    --clear \
    --watch src \
    --watch data/fabric/data/patterns \
    --shell 'SKIP_EMBEDDINGS=1 cargo run'
```

## Cache Generation

### Parquet Schema

```rust
use arrow::datatypes::{DataType, Field, Schema};

pub fn embedding_schema() -> Schema {
    Schema::new(vec![
        Field::new("pattern_name", DataType::Utf8, false),
        Field::new("description", DataType::Utf8, false),
        Field::new("category", DataType::Utf8, true),
        Field::new("tags", DataType::List(
            Box::new(Field::new("item", DataType::Utf8, true))
        ), true),
        Field::new("embedding", DataType::FixedSizeList(
            Box::new(Field::new("item", DataType::Float32, false)),
            1536  // text-embedding-3-small dimension
        ), false),
        Field::new("modified", DataType::Timestamp(TimeUnit::Second, None), false),
    ])
}
```

### Write Parquet

```rust
use parquet::arrow::arrow_writer::ArrowWriter;
use parquet::file::properties::WriterProperties;

pub fn write_parquet(batch: &RecordBatch, path: &str) -> Result<()> {
    let file = File::create(path)?;
    
    let props = WriterProperties::builder()
        .set_compression(Compression::ZSTD)
        .set_dictionary_enabled(true)
        .build();
    
    let mut writer = ArrowWriter::try_new(file, batch.schema(), Some(props))?;
    writer.write(batch)?;
    writer.close()?;
    
    Ok(())
}
```

## Performance Optimizations

### 1. Memory-Mapped I/O

```rust
use memmap2::Mmap;

pub fn load_with_mmap(path: &Path) -> Result<RecordBatch> {
    let file = File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    
    let reader = ParquetReader::new(Cursor::new(&mmap[..]))?;
    let batch = reader.read()?;
    
    Ok(batch)
}
```

### 2. Parallel Pattern Loading

```rust
use rayon::prelude::*;

pub fn load_patterns_parallel(dir: &Path) -> Result<Vec<Pattern>> {
    let entries: Vec<_> = fs::read_dir(dir)?
        .filter_map(|e| e.ok())
        .collect();
    
    let patterns = entries
        .par_iter()
        .filter_map(|entry| load_pattern(&entry.path()).ok())
        .collect();
    
    Ok(patterns)
}
```

### 3. Incremental Updates

```rust
pub async fn update_cache_incremental(
    cache_path: &Path,
    changed_patterns: &[Pattern]
) -> Result<()> {
    // Load existing cache
    let mut batch = load_parquet(cache_path)?;
    
    // Update changed patterns
    for pattern in changed_patterns {
        update_pattern_in_batch(&mut batch, pattern)?;
    }
    
    // Write back
    write_parquet(&batch, cache_path)?;
    
    Ok(())
}
```

## Crate Recommendations

### File Watching
- **`notify`** - Cross-platform file system watcher
- **`hotwatch`** - Simplified file watching API

### SIMD Optimization
- **`simsimd`** - SIMD-accelerated similarity metrics
- **`faer`** - High-performance linear algebra
- **`arrow`** - Built-in SIMD compute kernels

### Build Tools
- **`cargo-watch`** - Auto-rebuild on changes
- **`cargo-make`** - Task runner for complex builds

## CI/CD Integration

```yaml
# .github/workflows/build.yml
name: Build

on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
        with:
          submodules: recursive
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Build (skip embeddings)
        run: SKIP_EMBEDDINGS=1 cargo build --release
      
      - name: Run tests
        run: cargo test
```
