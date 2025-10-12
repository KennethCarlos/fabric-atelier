# Windsurf Workflows for Fabric Atelier

This document contains workflows to add to Windsurf for automated development tasks.

---

## Workflow: New Module Creation

**Trigger**: When creating a new Rust module

### Steps

1. **Create module directory structure**
   ```bash
   mkdir -p src/<module_name>
   touch src/<module_name>/mod.rs
   ```

2. **Add module-level documentation**
   ```rust
   //! Brief module summary.
   //!
   //! Detailed explanation of purpose and usage.
   //!
   //! # Examples
   //! ```
   //! // Usage example
   //! ```
   ```

3. **Export module in parent**
   ```rust
   // In src/lib.rs or parent mod.rs
   pub mod <module_name>;
   ```

4. **Create initial test file**
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
       
       #[test]
       fn test_placeholder() {
           // TODO: Add tests
       }
   }
   ```

5. **Update documentation**
   - Add module to `docs/MODULE_DESIGN.md`
   - Document module responsibility
   - Note file count and line limits

---

## Workflow: Add New Error Type

**Trigger**: When adding a new error variant

### Steps

1. **Add error variant to `src/error/types.rs`**
   ```rust
   #[derive(Error, Debug)]
   pub enum Error {
       // ... existing variants
       
       #[error("New error description: {context}")]
       NewError { context: String },
   }
   ```

2. **Add context helpers if needed**
   ```rust
   impl Error {
       pub fn new_error(context: impl Into<String>) -> Self {
           Self::NewError { context: context.into() }
       }
   }
   ```

3. **Update error documentation**
   - Document when this error occurs
   - Provide recovery suggestions
   - Add usage examples

4. **Add test cases**
   ```rust
   #[test]
   fn test_new_error() {
       let err = Error::new_error("test context");
       assert!(matches!(err, Error::NewError { .. }));
   }
   ```

---

## Workflow: Implement New Pattern Feature

**Trigger**: When adding functionality to pattern system

### Steps

1. **Design the feature**
   - Document in `docs/ARCHITECTURE.md`
   - Define interfaces
   - Consider performance implications

2. **Create feature branch**
   ```bash
   git checkout -b feature/<feature-name>
   ```

3. **Implement core logic**
   - Keep files under 300 lines
   - Add comprehensive documentation
   - Include error handling

4. **Add tests**
   - Unit tests for core logic
   - Integration tests for end-to-end
   - Edge case coverage

5. **Update documentation**
   - API documentation (rustdoc)
   - User-facing docs
   - Examples

6. **Run quality checks**
   ```bash
   cargo fmt
   cargo clippy
   cargo test
   cargo build --release
   ```

7. **Commit with proper message**
   ```
   feat: add <feature-name>
   
   - Implement core functionality
   - Add comprehensive tests
   - Update documentation
   
   Closes #<issue-number>
   ```

---

## Workflow: Performance Optimization

**Trigger**: When optimizing performance-critical code

### Steps

1. **Establish baseline**
   ```bash
   cargo bench --bench <benchmark-name> > baseline.txt
   ```

2. **Profile the code**
   ```bash
   cargo build --release
   perf record --call-graph=dwarf ./target/release/fabric-atelier
   perf report
   ```

3. **Identify bottlenecks**
   - Review profiling data
   - Focus on hot paths
   - Consider algorithmic improvements

4. **Implement optimization**
   - SIMD for vector operations
   - Parallel processing with rayon
   - Memory-mapped I/O
   - Zero-copy operations

5. **Benchmark improvements**
   ```bash
   cargo bench --bench <benchmark-name> > optimized.txt
   diff baseline.txt optimized.txt
   ```

6. **Document optimization**
   - Explain what was optimized
   - Show before/after metrics
   - Note any trade-offs

7. **Add regression tests**
   - Ensure optimization doesn't break functionality
   - Add performance tests to CI

---

## Workflow: Add Build-Time Feature

**Trigger**: When adding build-time code generation

### Steps

1. **Update `build.rs`**
   ```rust
   fn main() {
       println!("cargo:rerun-if-changed=<dependency>");
       
       if let Err(e) = build_feature() {
           println!("cargo:warning=Failed: {}", e);
       }
   }
   ```

2. **Implement build function**
   - Keep under 200 lines
   - Handle errors gracefully
   - Log progress with warnings

3. **Test build script**
   ```bash
   cargo clean
   cargo build
   ```

4. **Add environment variable controls**
   ```rust
   if env::var("SKIP_<FEATURE>").is_ok() {
       return Ok(());
   }
   ```

5. **Document build requirements**
   - Update `README.md`
   - Note any API keys needed
   - Explain skip flags

---

## Workflow: Hot Reload Testing

**Trigger**: When testing file watching and hot reload

### Steps

1. **Start in watch mode**
   ```bash
   cargo watch -x run
   ```

2. **Modify a pattern file**
   ```bash
   echo "# Test change" >> data/fabric/data/patterns/test/system.md
   ```

3. **Verify reload triggered**
   - Check logs for "Pattern change detected"
   - Verify cache rebuild
   - Confirm patterns reloaded

4. **Test error handling**
   - Introduce invalid pattern
   - Verify graceful handling
   - Check error messages

5. **Performance check**
   - Measure reload time
   - Should be < 100ms for single pattern
   - Log any slowdowns

---

## Workflow: Release Preparation

**Trigger**: When preparing a new release

### Steps

1. **Update version numbers**
   ```toml
   # Cargo.toml
   [package]
   version = "0.2.0"
   ```

2. **Update CHANGELOG.md**
   ```markdown
   ## [0.2.0] - 2025-01-15
   
   ### Added
   - New features
   
   ### Changed
   - Breaking changes
   
   ### Fixed
   - Bug fixes
   ```

3. **Run full test suite**
   ```bash
   cargo test --all-features
   cargo clippy -- -D warnings
   cargo fmt -- --check
   ```

4. **Build release binary**
   ```bash
   cargo build --release
   strip target/release/fabric-atelier
   ```

5. **Verify binary size and performance**
   ```bash
   ls -lh target/release/fabric-atelier  # Should be < 10 MB
   time ./target/release/fabric-atelier --version  # Should be < 50ms
   ```

6. **Create git tag**
   ```bash
   git tag -a v0.2.0 -m "Release version 0.2.0"
   git push origin v0.2.0
   ```

7. **Create GitHub release**
   - Upload binary
   - Include changelog
   - Add installation instructions

---

## Workflow: Debug Production Issue

**Trigger**: When investigating a bug report

### Steps

1. **Reproduce the issue**
   - Create minimal test case
   - Document steps to reproduce
   - Capture error messages

2. **Enable debug logging**
   ```bash
   RUST_LOG=debug ./target/release/fabric-atelier
   ```

3. **Add targeted logging**
   ```rust
   debug!("Variable state: {:?}", variable);
   ```

4. **Use debugger if needed**
   ```bash
   rust-gdb ./target/release/fabric-atelier
   ```

5. **Identify root cause**
   - Review logs
   - Check assumptions
   - Verify data flow

6. **Implement fix**
   - Minimal change to address root cause
   - Add regression test
   - Document fix in commit

7. **Verify fix**
   - Test original reproduction case
   - Run full test suite
   - Check for side effects

---

## Workflow: Code Review Process

**Trigger**: When reviewing a pull request

### Steps

1. **Check out PR branch**
   ```bash
   gh pr checkout <pr-number>
   ```

2. **Review code changes**
   - Check adherence to architecture rules
   - Verify documentation
   - Look for potential issues

3. **Run tests locally**
   ```bash
   cargo test
   cargo clippy
   cargo fmt -- --check
   ```

4. **Test functionality**
   - Build and run
   - Test new features
   - Verify bug fixes

5. **Check performance impact**
   ```bash
   cargo bench
   ```

6. **Provide feedback**
   - Be specific and constructive
   - Reference architecture docs
   - Suggest improvements

7. **Approve or request changes**
   - Use GitHub review interface
   - Explain reasoning
   - Offer to pair if complex

---

## Implementation Notes

### How to Add Workflows

1. Open Windsurf
2. Click **Customizations** icon
3. Navigate to **Workflows** panel
4. Click **+ New Workflow**
5. Copy workflow content
6. Set trigger conditions
7. Test workflow execution

### Workflow Best Practices

- Keep workflows focused on single tasks
- Make steps actionable and specific
- Include error handling steps
- Document expected outcomes
- Test workflows regularly

### Automation Opportunities

Consider automating these workflows with:
- Git hooks (pre-commit, pre-push)
- GitHub Actions
- `cargo-make` tasks
- Shell scripts in `scripts/` directory
