# Arrow & Parquet Operations

**Activation Mode**: Glob Pattern
**Pattern**: `src/arrow/**/*.rs, src/vector/**/*.rs`

## Arrow Best Practices

- Use zero-copy operations where possible
- Prefer columnar operations over row-wise
- Use SIMD compute kernels from arrow crate
- Memory-map Parquet files for large datasets

## Schema Design

- Define schemas as constants
- Use appropriate data types (FixedSizeList for embeddings)
- Include metadata in schema when useful
- Document schema evolution strategy

## Performance

- Batch operations when possible
- Use compression (ZSTD) for Parquet files
- Enable dictionary encoding for repeated values
- Profile memory usage for large datasets
