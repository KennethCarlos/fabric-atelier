# Windsurf Rules for Fabric Atelier

This document contains rules to add to Windsurf via the Customizations panel.

---

## Global Rule: Architecture Standards

**Activation Mode**: Always On

### Modular Design
- No file exceeds 300 lines
- Split files into submodules when approaching 250 lines
- Each module has a single, well-defined responsibility
- Extract helper functions to separate files

### Code Organization
- Standard library imports first
- External crate imports second (alphabetical)
- Internal crate imports third (alphabetical)
- Constants after imports
- Type aliases before main types

### Documentation Requirements
- Every public function requires rustdoc comments
- Include: brief summary, arguments, returns, errors, examples, performance notes
- Module-level documentation explains purpose and usage
- Complex algorithms need inline WHY comments (not WHAT)

### Error Handling
- Use `thiserror` for custom error types
- Add context with `anyhow::Context`
- Never use `unwrap()` or `expect()` in production code paths
- Return `Result` for all fallible operations

### Performance
- Use `#[inline]` for hot path functions < 10 lines
- Use `rayon` for data parallelism
- Avoid unnecessary allocations
- Prefer zero-copy operations

---

## Workspace Rule: Rust Best Practices

**Activation Mode**: Glob Pattern: `src/**/*.rs`

### Naming Conventions
- Types: `PascalCase`
- Functions: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE`
- Modules: `snake_case`

### Async Code
- Use `tokio` for async runtime
- Prefer `async fn` over `impl Future`
- Use `tokio::spawn` for concurrent tasks

### Testing
- Unit tests in `#[cfg(test)] mod tests`
- Integration tests in `tests/` directory
- Use `#[tokio::test]` for async tests

### Logging
- Use `tracing` for structured logging
- Use `#[instrument]` for function tracing
- Log levels: ERROR (unrecoverable), WARN (recoverable), INFO (milestones), DEBUG (detailed), TRACE (verbose)

---

## Workspace Rule: Arrow/Parquet Operations

**Activation Mode**: Glob Pattern: `src/arrow/**/*.rs, src/vector/**/*.rs`

### Arrow Best Practices
- Use zero-copy operations where possible
- Prefer columnar operations over row-wise
- Use SIMD compute kernels from arrow crate
- Memory-map Parquet files for large datasets

### Schema Design
- Define schemas as constants
- Use appropriate data types (FixedSizeList for embeddings)
- Include metadata in schema when useful

### Performance
- Batch operations when possible
- Use compression (ZSTD) for Parquet files
- Enable dictionary encoding for repeated values

---

## Workspace Rule: Build System

**Activation Mode**: Glob Pattern: `build.rs, src/build/**/*.rs`

### Build Script Rules
- Check for `SKIP_EMBEDDINGS` environment variable
- Fail gracefully if patterns directory missing
- Log warnings for build-time issues
- Generate cache in `data/cache/` directory

### Build-Time Optimization
- Generate Arrow/Parquet cache at compile time
- Embed metadata in binary when appropriate
- Validate pattern structure during build

---

## Workspace Rule: MCP Protocol

**Activation Mode**: Glob Pattern: `src/mcp/**/*.rs`

### Protocol Compliance
- Follow JSON-RPC 2.0 specification exactly
- Always include `jsonrpc: "2.0"` field
- Echo request `id` in response
- Use standard error codes (PARSE_ERROR: -32700, METHOD_NOT_FOUND: -32601, etc.)

### Error Handling
- Provide clear, actionable error messages
- Include error context in `data` field when helpful
- Never expose internal implementation details in errors

---

## Workspace Rule: Pattern Management

**Activation Mode**: Glob Pattern: `src/fabric/**/*.rs`

### Pattern Loading
- Support both git submodule and user's Fabric installation
- Try bundled patterns first, fallback to `~/.config/fabric/patterns`
- Log pattern loading progress
- Skip invalid patterns with warnings

### Pattern Execution
- Execute via Fabric CLI (`fabric --pattern <name>`)
- Handle stdin/stdout piping correctly
- Implement timeout handling
- Capture and log stderr for debugging

### Metadata Extraction
- Parse `system.md` for description
- Extract category and tags from content
- Auto-categorize based on keywords
- Store last modified timestamp

---

## Workspace Rule: Testing Standards

**Activation Mode**: Glob Pattern: `tests/**/*.rs, src/**/tests.rs`

### Test Organization
- Unit tests: `#[cfg(test)] mod tests` in same file
- Integration tests: `tests/integration/` directory
- Benchmarks: `benches/` directory

### Test Quality
- Test both success and error cases
- Use descriptive test names
- Include edge cases
- Mock external dependencies

### Async Testing
- Use `#[tokio::test]` for async tests
- Test concurrent operations
- Verify proper cleanup

---

## Workspace Rule: Documentation

**Activation Mode**: Glob Pattern: `docs/**/*.md`

### Documentation Style
- Use clear, concise language
- Include code examples
- Provide context and rationale
- Keep examples up-to-date with code

### Structure
- Start with overview
- Include table of contents for long docs
- Use headings for organization
- Add diagrams where helpful

---

## Workspace Rule: Git Commits

**Activation Mode**: Manual (use @mention when committing)

### Commit Message Format
```
<type>: <subject>

<body>

<footer>
```

### Types
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Test additions/changes
- `chore`: Build process or auxiliary tool changes

### Guidelines
- Subject line: imperative mood, no period, max 50 chars
- Body: explain WHAT and WHY, not HOW
- Reference issues/PRs in footer

---

## Workspace Rule: Code Review

**Activation Mode**: Manual (use @mention during reviews)

### Review Checklist
- [ ] All public APIs documented
- [ ] Complex logic has WHY comments
- [ ] No file exceeds 300 lines
- [ ] Tests pass (`cargo test`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Code formatted (`cargo fmt`)
- [ ] Error messages clear and actionable
- [ ] No `unwrap()` in production paths
- [ ] Performance considered
- [ ] Security implications reviewed

---

## Implementation Notes

### How to Add These Rules

1. Open Windsurf
2. Click **Customizations** icon (top right)
3. Navigate to **Rules** panel
4. Click **+ Workspace** for workspace-specific rules
5. Click **+ Global** for global rules
6. Copy the rule content from above
7. Set the **Activation Mode** as specified
8. For Glob patterns, add the pattern in the UI

### Rule Activation Modes

- **Always On**: Applied to all Cascade interactions
- **Glob Pattern**: Applied when working with matching files
- **Manual**: Activated via `@mention` in Cascade
- **Model Decision**: AI decides when to apply based on description

### Best Practices

- Start with "Always On" rules for core architecture
- Use Glob patterns for language/framework-specific rules
- Use Manual mode for workflow-specific rules (commits, reviews)
- Keep each rule under 12000 characters
- Group related rules with XML tags if needed
