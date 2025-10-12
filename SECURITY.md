# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Security Tools & Checks

Fabric Atelier uses multiple security tools to ensure code quality and safety:

### 1. Cargo Audit
Checks for known security vulnerabilities in dependencies.

```bash
cargo audit
```

### 2. Cargo Deny
Validates licenses, checks for banned dependencies, and ensures all dependencies come from trusted sources.

```bash
cargo deny check
```

Configuration: `deny.toml`

### 3. Cargo Clippy
Lints for common mistakes, performance issues, and security concerns.

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

### 4. Fuzzing with cargo-fuzz
Automated fuzzing to find edge cases and potential crashes.

```bash
# Fuzz MCP protocol parsing
cargo fuzz run mcp_protocol

# Fuzz pattern search
cargo fuzz run pattern_search
```

### 5. Comprehensive Security Check Script

Run all security checks at once:

```bash
./scripts/security-check.sh
```

## Known Advisories

### Transitive Dependencies

- **paste (RUSTSEC-2024-0436)**: Unmaintained crate, transitive dependency through datafusion. 
  - Status: Monitoring for alternatives
  - Impact: Low - used only at compile time for macros
  - Mitigation: Will be replaced when datafusion updates

## Reporting a Vulnerability

If you discover a security vulnerability, please:

1. **DO NOT** open a public issue
2. Email: don@codetestcode.io
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

### Response Timeline

- **Initial Response**: Within 48 hours
- **Status Update**: Within 7 days
- **Fix Timeline**: Depends on severity
  - Critical: 1-7 days
  - High: 7-14 days
  - Medium: 14-30 days
  - Low: Next release cycle

## Security Best Practices

### For Users

1. **Always use the latest version** - Security fixes are released promptly
2. **Verify Docker images** - Use official images from Docker Hub
3. **Secure API keys** - Never commit API keys to version control
4. **Use environment variables** - For sensitive configuration
5. **Review permissions** - MCP servers have access to your filesystem

### For Contributors

1. **Run security checks** before submitting PRs
2. **Never commit secrets** - Use `.env` files (gitignored)
3. **Validate input** - All user input should be validated
4. **Use safe Rust** - Avoid `unsafe` blocks unless absolutely necessary
5. **Document security implications** - In code comments and PRs

## Dependency Security

### Allowed Licenses

- MIT
- Apache-2.0
- BSD-2-Clause / BSD-3-Clause
- ISC
- Unicode-DFS-2016
- Unicode-3.0
- Zlib
- 0BSD
- CC0-1.0

### Banned Licenses

- GPL-2.0 / GPL-3.0
- AGPL-3.0

### Trusted Sources

All dependencies must come from:
- crates.io (official Rust package registry)
- Approved git repositories (case-by-case basis)

## Security Features

### Built-in Security

1. **Non-root Docker user** - Containers run as non-privileged user
2. **Minimal dependencies** - Reduced attack surface
3. **Input validation** - All MCP requests are validated
4. **Type safety** - Rust's type system prevents many common vulnerabilities
5. **Memory safety** - No buffer overflows or use-after-free bugs

### Network Security

- **No outbound connections** (except to configured LLM providers)
- **Stdio transport** - No network ports exposed by default
- **Sandboxed execution** - Docker isolation

## Audit Log

| Date       | Version | Change                                    |
|------------|---------|-------------------------------------------|
| 2025-10-11 | 0.1.0   | Initial security policy and tooling setup |

## References

- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)
- [OWASP Secure Coding Practices](https://owasp.org/www-project-secure-coding-practices-quick-reference-guide/)
- [MCP Security Best Practices](https://modelcontextprotocol.io/docs/concepts/security)
