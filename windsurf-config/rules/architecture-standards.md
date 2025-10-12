# Architecture Standards

**Activation Mode**: Always On

## Modular Design

- No file exceeds 300 lines
- Split files into submodules when approaching 250 lines
- Each module has a single, well-defined responsibility
- Extract helper functions to separate files

## Code Organization

- Standard library imports first
- External crate imports second (alphabetical)
- Internal crate imports third (alphabetical)
- Constants after imports
- Type aliases before main types

## Documentation Requirements

- Every public function requires rustdoc comments
- Include: brief summary, arguments, returns, errors, examples, performance notes
- Module-level documentation explains purpose and usage
- Complex algorithms need inline WHY comments (not WHAT)

## Error Handling

- Use `thiserror` for custom error types
- Add context with `anyhow::Context`
- Never use `unwrap()` or `expect()` in production code paths
- Return `Result` for all fallible operations

## Performance

- Use `#[inline]` for hot path functions < 10 lines
- Use `rayon` for data parallelism
- Avoid unnecessary allocations
- Prefer zero-copy operations
