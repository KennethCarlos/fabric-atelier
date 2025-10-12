# Git Commit Guidelines

**Activation Mode**: Manual
**Usage**: `@git-commits` when writing commit messages

## Commit Message Format

```
<type>: <subject>

<body>

<footer>
```

## Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, no logic change)
- `refactor`: Code refactoring (no feature change)
- `perf`: Performance improvements
- `test`: Test additions or changes
- `chore`: Build process or auxiliary tool changes

## Guidelines

- **Subject line**: Imperative mood, no period, max 50 characters
- **Body**: Explain WHAT and WHY, not HOW (wrap at 72 chars)
- **Footer**: Reference issues/PRs (e.g., "Closes #123", "Refs #456")

## Examples

```
feat: add SIMD-accelerated vector search

Implement cosine similarity using simsimd crate for 10x performance
improvement. Uses AVX2 instructions when available, falls back to
scalar implementation on unsupported platforms.

Closes #42
```

```
fix: handle empty pattern directory gracefully

Previously crashed with unwrap() when patterns directory was empty.
Now returns clear error message and logs warning.

Fixes #38
```
