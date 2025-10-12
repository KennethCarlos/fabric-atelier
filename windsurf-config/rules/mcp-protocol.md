# MCP Protocol

**Activation Mode**: Glob Pattern
**Pattern**: `src/mcp/**/*.rs`

## Protocol Compliance

- Follow JSON-RPC 2.0 specification exactly
- Always include `jsonrpc: "2.0"` field
- Echo request `id` in response
- Use standard error codes:
  - PARSE_ERROR: -32700
  - INVALID_REQUEST: -32600
  - METHOD_NOT_FOUND: -32601
  - INVALID_PARAMS: -32602
  - INTERNAL_ERROR: -32603

## Error Handling

- Provide clear, actionable error messages
- Include error context in `data` field when helpful
- Never expose internal implementation details in errors
- Log errors with full context for debugging

## Request Handling

- Validate all input parameters
- Handle malformed JSON gracefully
- Implement proper timeout handling
- Support concurrent requests
