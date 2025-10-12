# Security Guide - Quick Reference

## Running Security Checks

### All-in-One Script
```bash
./scripts/security-check.sh
```

### Individual Tools

**Check for vulnerabilities:**
```bash
cargo audit
```

**Validate licenses and dependencies:**
```bash
cargo deny check
```

**Run lints:**
```bash
cargo clippy --all-targets --all-features -- -D warnings
```

**Run tests:**
```bash
cargo test --all-features
```

## Fuzzing

### Run Fuzz Tests

**Fuzz MCP protocol parsing (run for 60 seconds):**
```bash
cargo fuzz run mcp_protocol -- -max_total_time=60
```

**Fuzz pattern search:**
```bash
cargo fuzz run pattern_search -- -max_total_time=60
```

**Run all fuzz targets:**
```bash
cargo fuzz list | xargs -I {} cargo fuzz run {} -- -max_total_time=30
```

### View Fuzz Results
```bash
# Check for crashes
ls fuzz/artifacts/

# Reproduce a crash
cargo fuzz run mcp_protocol fuzz/artifacts/mcp_protocol/crash-<hash>
```

## CI/CD Security

Security checks run automatically on:
- Every push to master
- Every pull request
- Daily at 00:00 UTC

View results: `.github/workflows/security.yml`

## Quick Security Checklist

Before releasing:
- [ ] `cargo audit` passes
- [ ] `cargo deny check` passes
- [ ] `cargo clippy` has no warnings
- [ ] All tests pass
- [ ] Fuzz tests run for at least 5 minutes each
- [ ] No new dependencies without review
- [ ] SECURITY.md updated if needed

## Common Issues

### Unmaintained Dependencies
Check transitive dependencies:
```bash
cargo tree | grep <crate-name>
```

### License Issues
Add to `deny.toml` if license is acceptable:
```toml
[licenses]
allow = [
    "MIT",
    "Apache-2.0",
    # Add new license here
]
```

### Multiple Versions
Check why:
```bash
cargo tree -d
```

## Resources

- [SECURITY.md](../SECURITY.md) - Full security policy
- [deny.toml](../deny.toml) - Dependency configuration
- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)
