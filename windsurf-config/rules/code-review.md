# Code Review Checklist

**Activation Mode**: Manual
**Usage**: `@code-review` when reviewing code

## Review Checklist

### Documentation
- [ ] All public APIs have rustdoc comments
- [ ] Complex logic has WHY comments (not WHAT)
- [ ] Module-level documentation is clear
- [ ] Examples are provided for non-trivial APIs

### Code Quality
- [ ] No file exceeds 300 lines
- [ ] Naming is clear and consistent
- [ ] Code follows Rust idioms
- [ ] No unnecessary complexity

### Testing
- [ ] Tests pass (`cargo test`)
- [ ] New features have tests
- [ ] Edge cases are covered
- [ ] No test interdependencies

### Error Handling
- [ ] No `unwrap()` or `expect()` in production code
- [ ] Errors have clear, actionable messages
- [ ] Proper use of `Result` types
- [ ] Context added with `anyhow::Context`

### Performance
- [ ] No obvious performance issues
- [ ] Appropriate data structures used
- [ ] SIMD used for vector operations where applicable
- [ ] No unnecessary allocations

### Security
- [ ] No hardcoded secrets
- [ ] Input validation present
- [ ] No SQL injection risks
- [ ] Sensitive data not logged

### Build & CI
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] Build succeeds (`cargo build --release`)
- [ ] Documentation builds (`cargo doc`)

## Review Comments

When providing feedback:
- Be specific and constructive
- Reference architecture docs when applicable
- Suggest improvements, don't just point out issues
- Acknowledge good practices
- Offer to pair on complex issues
