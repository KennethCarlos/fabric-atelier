# Build System

**Activation Mode**: Glob Pattern
**Pattern**: `build.rs, src/build/**/*.rs`

## Build Script Rules

- Check for `SKIP_EMBEDDINGS` environment variable
- Fail gracefully if patterns directory missing
- Log warnings for build-time issues using `cargo:warning=`
- Generate cache in `data/cache/` directory
- Use `cargo:rerun-if-changed=` for dependencies

## Build-Time Optimization

- Generate Arrow/Parquet cache at compile time
- Embed metadata in binary when appropriate
- Validate pattern structure during build
- Skip expensive operations in CI (respect SKIP flags)

## Error Handling

- Never fail the build for optional features
- Provide clear warning messages
- Document required environment variables
- Test build script separately from main code
