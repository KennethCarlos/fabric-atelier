# Pattern Management

**Activation Mode**: Glob Pattern
**Pattern**: `src/fabric/**/*.rs`

## Pattern Loading

- Support both git submodule and user's Fabric installation
- Try bundled patterns first: `data/fabric/data/patterns`
- Fallback to user installation: `~/.config/fabric/patterns`
- Log pattern loading progress at INFO level
- Skip invalid patterns with WARN level logs

## Pattern Execution

- Execute via Fabric CLI: `fabric --pattern <name>`
- Handle stdin/stdout piping correctly
- Implement timeout handling (default: 30 seconds)
- Capture and log stderr for debugging
- Return clear error messages on failure

## Metadata Extraction

- Parse `system.md` for description
- Extract category and tags from content
- Auto-categorize based on keywords:
  - "security" or "threat" → security category
  - "writing" or "essay" → writing category
  - "code" or "programming" → coding category
- Store last modified timestamp
- Cache metadata for performance
