# MCP Schema Documentation - Best Practices

## Research Summary: MCP Schema & Documentation Standards

### Key Findings

**YES** - There IS a standardized schema for MCP, but it's **TypeScript-based**, not OpenAPI/Swagger.

## Official MCP Schema

### 1. **TypeScript Schema (Authoritative)**

The MCP protocol is defined by an **official TypeScript schema**:

- **Location**: `https://github.com/modelcontextprotocol/specification/blob/main/schema/2025-06-18/schema.ts`
- **Status**: This is the **authoritative source of truth**
- **Format**: TypeScript type definitions with JSDoc comments
- **Versioning**: Date-based (e.g., `2025-06-18`)

**Example from the official schema:**
```typescript
export interface JSONRPCRequest extends Request {
  jsonrpc: typeof JSONRPC_VERSION;
  id: RequestId;
}

export interface InitializeRequest extends Request {
  method: "initialize";
  params: {
    protocolVersion: string;
    capabilities: ClientCapabilities;
    clientInfo: Implementation;
  };
}
```

### 2. **JSON-RPC 2.0 Foundation**

MCP is built on **JSON-RPC 2.0**, which provides:
- Standard request/response format
- Error handling conventions
- Notification mechanism

## Current Documentation Approaches

### ✅ **Best Practice: TypeScript Schema + Markdown Docs**

The MCP community uses:

1. **TypeScript Schema** (`schema.ts`)
   - Type-safe definitions
   - JSDoc comments for documentation
   - Compile-time validation
   - IDE autocomplete support

2. **Markdown Documentation** (modelcontextprotocol.io)
   - Human-readable specification
   - Examples and tutorials
   - Architecture diagrams
   - Best practices

3. **Schema Reference Pages**
   - Auto-generated from TypeScript
   - Comprehensive type listings
   - Cross-referenced

### 🔄 **Emerging: OpenAPI Bridges**

Several projects are bridging MCP ↔ OpenAPI:

1. **`openapi-mcp`** (https://github.com/jedisct1/openapi-mcp)
   - Converts OpenAPI specs → MCP servers
   - Exposes REST APIs as MCP tools
   - One-way: OpenAPI → MCP

2. **`mcp-openapi-schema`** (https://github.com/hannesj/mcp-openapi-schema)
   - Provides OpenAPI schema for MCP servers
   - Allows REST API access to MCP

3. **MCP-to-OpenAPI Proxy** (Open WebUI)
   - Exposes MCP servers via OpenAPI endpoints
   - Allows standard HTTP clients to use MCP

## Why NOT OpenAPI for MCP?

### MCP-Specific Requirements

1. **Bidirectional Communication**
   - MCP supports server→client notifications
   - OpenAPI is request→response only

2. **Stateful Sessions**
   - MCP has initialization/lifecycle
   - OpenAPI is stateless

3. **Dynamic Capabilities**
   - MCP negotiates capabilities at runtime
   - OpenAPI is static

4. **Streaming & Progress**
   - MCP supports progress notifications
   - OpenAPI has limited streaming support

## Best Practices for Fabric Atelier

### ✅ **Recommended Approach**

1. **Use TypeScript Interfaces** (like official MCP)
   ```typescript
   // Define your server's capabilities
   export interface FabricAtelierCapabilities {
     tools: {
       listChanged: true
     }
   }
   ```

2. **Provide JSON Schema** (for validation)
   ```json
   {
     "$schema": "http://json-schema.org/draft-07/schema#",
     "type": "object",
     "properties": {
       "name": { "type": "string" },
       "arguments": {
         "type": "object",
         "properties": {
           "content": { "type": "string" }
         }
       }
     }
   }
   ```

3. **Document with Markdown**
   - Server capabilities
   - Available tools
   - Usage examples
   - Error codes

4. **Optional: Generate OpenAPI** (for REST access)
   - If you want HTTP/REST access
   - Use `mcp-openapi-schema` or similar
   - Complement, don't replace MCP

### 📋 **Documentation Structure**

```
docs/
├── MCP_SPECIFICATION.md       # Your server's MCP implementation
├── TOOLS_REFERENCE.md          # All 226 patterns documented
├── EXAMPLES.md                 # Usage examples
└── schema/
    ├── mcp-schema.ts           # TypeScript definitions
    ├── tools-schema.json       # JSON Schema for tools
    └── openapi.yaml            # Optional: REST API spec
```

## Example: Documenting Fabric Atelier

### TypeScript Schema (Recommended)

```typescript
/**
 * Fabric Atelier MCP Server
 * 
 * Exposes 226 Fabric patterns as MCP tools for AI-powered content processing.
 */

export interface FabricAtelierTool {
  /** Tool name in format: fabric_{pattern_name} */
  name: string;
  
  /** Human-readable description of the pattern */
  description: string;
  
  /** Input schema for the tool */
  inputSchema: {
    type: "object";
    properties: {
      content: {
        type: "string";
        description: "The content to process with this pattern";
      };
    };
    required: ["content"];
  };
}

export interface FabricAtelierCapabilities {
  tools: {
    /** Supports dynamic tool list updates */
    listChanged: true;
  };
}
```

### JSON Schema for Tool Input

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "Fabric Pattern Tool Input",
  "type": "object",
  "properties": {
    "content": {
      "type": "string",
      "description": "The content to process with this pattern",
      "minLength": 1
    }
  },
  "required": ["content"],
  "additionalProperties": false
}
```

### Markdown Documentation

```markdown
# Fabric Atelier MCP Server

## Overview

Fabric Atelier exposes 226 Fabric patterns as MCP tools.

## Capabilities

- ✅ Tools: 226 patterns available
- ✅ Dynamic tool list (listChanged notifications)
- ✅ LLM-powered execution (Ollama/OpenAI/Anthropic)

## Available Tools

### `fabric_write_essay_pg`

Writes essays in Paul Graham's distinctive style.

**Input:**
```json
{
  "content": "Your topic or draft content"
}
```

**Output:**
- Concise, clear essay in PG's style
- 500-2000 words typical
```

## Comparison: MCP vs OpenAPI

| Feature | MCP | OpenAPI |
|---------|-----|---------|
| **Protocol** | JSON-RPC 2.0 | HTTP REST |
| **Communication** | Bidirectional | Request/Response |
| **State** | Stateful sessions | Stateless |
| **Schema** | TypeScript | YAML/JSON |
| **Streaming** | Native support | Limited |
| **Notifications** | Server→Client | Not supported |
| **Discovery** | Dynamic capabilities | Static spec |
| **Best For** | AI tool integration | Web APIs |

## Tools for MCP Schema Management

### 1. **TypeScript** (Official)
```bash
npm install @modelcontextprotocol/sdk
```

### 2. **JSON Schema Validation**
```bash
npm install ajv
```

### 3. **OpenAPI Bridge** (Optional)
```bash
npm install openapi-mcp
```

### 4. **Documentation Generation**
```bash
npm install typedoc  # For TypeScript
npm install @redocly/cli  # For OpenAPI
```

## Recommendations for Fabric Atelier

### Phase 1: Core Documentation ✅

1. **Create `docs/MCP_SPECIFICATION.md`**
   - Server capabilities
   - Protocol version
   - Initialization flow

2. **Create `docs/TOOLS_REFERENCE.md`**
   - All 226 patterns
   - Input/output schemas
   - Examples

3. **Create `schema/tools-schema.json`**
   - JSON Schema for validation
   - Used by clients for validation

### Phase 2: Advanced (Optional)

1. **TypeScript Definitions**
   - Generate `.d.ts` files
   - Publish to npm
   - Enable TypeScript clients

2. **OpenAPI Spec** (if REST access needed)
   - Generate from MCP schema
   - Use `openapi-mcp` or similar
   - Enables HTTP/REST clients

3. **Interactive Documentation**
   - Use Redoc or Swagger UI
   - Host at `/docs` endpoint
   - Live API testing

## Conclusion

**Best Practice for MCP Servers:**

1. ✅ **Primary**: TypeScript schema + Markdown docs (like official MCP)
2. ✅ **Validation**: JSON Schema for runtime validation
3. ✅ **Optional**: OpenAPI spec for REST access
4. ❌ **Don't**: Use OpenAPI as primary schema (loses MCP features)

**For Fabric Atelier:**
- Follow official MCP patterns
- Document all 226 tools clearly
- Provide JSON Schema for validation
- Consider OpenAPI bridge if REST access needed

## References

- **Official MCP Spec**: https://modelcontextprotocol.io/specification
- **TypeScript Schema**: https://github.com/modelcontextprotocol/specification/blob/main/schema/
- **OpenAPI Bridge**: https://github.com/jedisct1/openapi-mcp
- **JSON-RPC 2.0**: https://www.jsonrpc.org/specification
