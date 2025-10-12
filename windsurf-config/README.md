# Windsurf Configuration Files

This directory contains Windsurf rules files ready to be moved to `.windsurf/rules/`.

## Installation

```bash
# Create the .windsurf/rules directory
mkdir -p .windsurf/rules

# Copy all rule files
cp windsurf-config/rules/*.md .windsurf/rules/

# Or move them
mv windsurf-config/rules/*.md .windsurf/rules/
```

## Rules Included

### Always On Rules
1. **architecture-standards.md** - Core architecture enforcement (300-line limit, docs, errors)

### Glob Pattern Rules
2. **rust-best-practices.md** - Pattern: `src/**/*.rs`
3. **arrow-parquet.md** - Pattern: `src/arrow/**/*.rs, src/vector/**/*.rs`
4. **mcp-protocol.md** - Pattern: `src/mcp/**/*.rs`
5. **pattern-management.md** - Pattern: `src/fabric/**/*.rs`
6. **build-system.md** - Pattern: `build.rs, src/build/**/*.rs`
7. **testing-standards.md** - Pattern: `tests/**/*.rs, src/**/tests.rs`

### Manual Rules (use @mention)
8. **git-commits.md** - Use: `@git-commits`
9. **code-review.md** - Use: `@code-review`

### Model Decision Rules
10. **performance-optimization.md** - AI decides when to apply

## Configuration in Windsurf

After moving the files:

1. Open Windsurf
2. Click **Customizations** icon (top-right)
3. Navigate to **Rules** panel
4. Windsurf should auto-discover the rules
5. Configure activation modes:
   - **architecture-standards.md**: Always On
   - **rust-best-practices.md**: Glob → `src/**/*.rs`
   - **arrow-parquet.md**: Glob → `src/arrow/**/*.rs, src/vector/**/*.rs`
   - **mcp-protocol.md**: Glob → `src/mcp/**/*.rs`
   - **pattern-management.md**: Glob → `src/fabric/**/*.rs`
   - **build-system.md**: Glob → `build.rs, src/build/**/*.rs`
   - **testing-standards.md**: Glob → `tests/**/*.rs, src/**/tests.rs`
   - **git-commits.md**: Manual
   - **code-review.md**: Manual
   - **performance-optimization.md**: Model Decision

## Testing

After installation, test by:

1. Opening `src/main.rs` → Should activate rust-best-practices
2. Opening `src/mcp/server.rs` → Should activate mcp-protocol
3. Typing `@git-commits` → Should activate git-commits rule
4. Ask Cascade: "What rules are currently active?"

## Cleanup

After moving files to `.windsurf/rules/`, you can optionally remove this directory:

```bash
rm -rf windsurf-config/
```

Or keep it as a backup.
