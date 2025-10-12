#!/bin/bash
# Comprehensive security checks for Fabric Atelier

set -e

echo "🔒 Running Security Checks for Fabric Atelier"
echo "=============================================="
echo ""

# 1. Cargo Audit - Check for known vulnerabilities
echo "📋 1. Checking for known vulnerabilities (cargo audit)..."
cargo audit || echo "⚠️  Warning: Some advisories found (see above)"
echo ""

# 2. Cargo Deny - License and dependency checks
echo "📋 2. Checking licenses and dependencies (cargo deny)..."
cargo deny check licenses sources bans || echo "⚠️  Warning: Some issues found (see above)"
echo ""

# 3. Cargo Clippy - Lint checks
echo "📋 3. Running lint checks (cargo clippy)..."
cargo clippy --all-targets --all-features -- -D warnings || echo "⚠️  Warning: Clippy warnings found"
echo ""

# 4. Cargo Test - Run all tests
echo "📋 4. Running tests..."
cargo test --all-features || echo "❌ Tests failed"
echo ""

# 5. Check for outdated dependencies
echo "📋 5. Checking for outdated dependencies..."
cargo outdated || echo "⚠️  Some dependencies may be outdated"
echo ""

echo "✅ Security checks complete!"
echo ""
echo "Summary:"
echo "- cargo audit: Checks for known security vulnerabilities"
echo "- cargo deny: Validates licenses and dependency sources"
echo "- cargo clippy: Lints for common mistakes and improvements"
echo "- cargo test: Ensures all tests pass"
echo "- cargo outdated: Identifies outdated dependencies"
