# Performance Optimization

**Activation Mode**: Model Decision
**Description**: Apply when working on performance-critical code, especially vector operations, search algorithms, or hot paths

## SIMD Optimization

- Use `simsimd` crate for vector similarity operations
- Implement fallback for non-SIMD platforms
- Use `#[cfg(target_feature = "avx2")]` for platform-specific code
- Profile before and after optimization

## Memory Optimization

- Use memory-mapped I/O for large files (Parquet cache)
- Prefer zero-copy operations
- Reuse buffers instead of allocating
- Use `Vec::with_capacity()` when size is known

## Parallel Processing

- Use `rayon` for data parallelism
- Parallelize independent operations
- Avoid parallelizing small workloads (overhead > benefit)
- Test concurrent safety

## Hot Path Optimization

- Use `#[inline]` for small, frequently-called functions
- Avoid allocations in tight loops
- Use iterators instead of index-based loops
- Profile with `cargo flamegraph` or `perf`

## Benchmarking

- Add benchmarks in `benches/` directory
- Use `criterion` for statistical analysis
- Benchmark before optimization (baseline)
- Document performance improvements in commits
