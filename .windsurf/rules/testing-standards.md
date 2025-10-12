# Testing Standards

**Activation Mode**: Glob Pattern
**Pattern**: `tests/**/*.rs, src/**/tests.rs`

## Test Organization

- Unit tests: `#[cfg(test)] mod tests` in same file
- Integration tests: `tests/integration/` directory
- Benchmarks: `benches/` directory
- Test fixtures: `tests/fixtures/` directory

## Test Quality

- Test both success and error cases
- Use descriptive test names (test_<what>_<when>_<expected>)
- Include edge cases (empty input, max values, etc.)
- Mock external dependencies
- Avoid test interdependencies

## Async Testing

- Use `#[tokio::test]` for async tests
- Test concurrent operations
- Verify proper cleanup (no resource leaks)
- Test timeout behavior

## Assertions

- Use specific assertions (assert_eq!, assert!(matches!(...)))
- Provide helpful error messages
- Test error types, not just error occurrence
- Use property-based testing for complex logic
