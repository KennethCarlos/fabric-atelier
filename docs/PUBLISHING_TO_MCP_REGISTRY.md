# Publishing Fabric Atelier to MCP Registries

## Most Popular MCP Registries

### 1. **Official MCP Registry** (RECOMMENDED) ⭐
- **URL**: https://github.com/modelcontextprotocol/registry
- **Status**: Official, community-driven
- **Servers**: Growing ecosystem
- **Backed by**: Anthropic + Community
- **Best for**: Maximum discoverability

### 2. **Smithery** 
- **URL**: https://smithery.ai
- **Servers**: 7,578+ servers
- **Features**: Semantic search, playground, one-click deploy
- **Best for**: Easy discovery and testing

### 3. **GitHub MCP Registry**
- **URL**: GitHub integration (coming soon)
- **Status**: In development by GitHub
- **Features**: Self-publishing, OSS community registry
- **Best for**: GitHub-native workflows

## Recommended Approach for Fabric Atelier

**Publish to BOTH:**
1. ✅ **Official MCP Registry** (primary)
2. ✅ **Smithery** (secondary, for discoverability)

## Publishing to Official MCP Registry

### Prerequisites

1. **Package your server** (choose one):
   - ✅ NPM package (Node.js/TypeScript)
   - ✅ PyPI package (Python)
   - ✅ Docker/OCI image (any language) ← **BEST FOR RUST**
   - ✅ NuGet package (.NET)

2. **Make it publicly accessible**
   - Published to public registry
   - Open source or closed source (both OK)
   - Not private/internal

### Step-by-Step: Docker/OCI Deployment (Recommended for Rust)

#### 1. Create Dockerfile

```dockerfile
# Multi-stage build for minimal image
FROM rust:1.75 as builder

WORKDIR /app
COPY . .

# Build release binary
RUN cargo build --release

# Runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/fabric-atelier /usr/local/bin/

# Copy patterns
COPY --from=builder /app/data /app/data

# Set working directory
WORKDIR /app

# Add MCP server name annotation
LABEL io.modelcontextprotocol.server.name="io.github.copyleftdev/fabric-atelier"

# Run the server
ENTRYPOINT ["fabric-atelier"]
```

#### 2. Build and Push to Docker Hub

```bash
# Build the image
docker build -t copyleftdev/fabric-atelier:1.0.0 .

# Tag as latest
docker tag copyleftdev/fabric-atelier:1.0.0 copyleftdev/fabric-atelier:latest

# Login to Docker Hub
docker login

# Push both tags
docker push copyleftdev/fabric-atelier:1.0.0
docker push copyleftdev/fabric-atelier:latest
```

#### 3. Install MCP Publisher CLI

```bash
# macOS/Linux with Homebrew
brew install mcp-publisher

# Or download binary
curl -L "https://github.com/modelcontextprotocol/registry/releases/download/latest/mcp-publisher_$(uname -s | tr '[:upper:]' '[:lower:]')_$(uname -m | sed 's/x86_64/amd64/;s/aarch64/arm64/').tar.gz" | tar xz mcp-publisher
sudo mv mcp-publisher /usr/local/bin/
```

#### 4. Create server.json

```bash
cd /home/sigma/Projects/fabric-atelier
mcp-publisher init
```

Edit the generated `server.json`:

```json
{
  "$schema": "https://static.modelcontextprotocol.io/schemas/2025-09-29/server.schema.json",
  "name": "io.github.copyleftdev/fabric-atelier",
  "title": "Fabric Atelier",
  "description": "AI-powered content processing with 226 Fabric patterns via MCP. Transform text with expert prompts for writing, analysis, security, coding, and more.",
  "version": "1.0.0",
  "license": "MIT",
  "homepage": "https://github.com/copyleftdev/fabric-atelier",
  "repository": {
    "type": "git",
    "url": "https://github.com/copyleftdev/fabric-atelier"
  },
  "author": {
    "name": "copyleftdev",
    "url": "https://github.com/copyleftdev"
  },
  "categories": [
    "content-generation",
    "text-processing",
    "ai-tools"
  ],
  "tags": [
    "fabric",
    "patterns",
    "writing",
    "analysis",
    "security",
    "coding",
    "llm",
    "ollama"
  ],
  "packages": [
    {
      "registryType": "docker",
      "identifier": "copyleftdev/fabric-atelier",
      "version": "1.0.0",
      "transport": {
        "type": "stdio"
      }
    }
  ]
}
```

#### 5. Authenticate with GitHub

```bash
# For io.github.* namespaces
mcp-publisher auth github
```

This will:
1. Open browser for GitHub OAuth
2. Authorize the MCP Publisher app
3. Save credentials locally

#### 6. Publish to Registry

```bash
mcp-publisher publish
```

This will:
1. Validate your `server.json`
2. Verify Docker image exists
3. Check the `io.modelcontextprotocol.server.name` label
4. Submit to registry
5. Return a URL to track publication status

#### 7. Verify Publication

```bash
# Check status
mcp-publisher status io.github.copyleftdev/fabric-atelier

# Or visit the registry
open https://registry.modelcontextprotocol.io/servers/io.github.copyleftdev/fabric-atelier
```

### Alternative: NPM Package (if you add Node.js wrapper)

If you want to publish as NPM package:

#### 1. Create package.json

```json
{
  "name": "@copyleftdev/fabric-atelier",
  "version": "1.0.0",
  "description": "AI-powered content processing with 226 Fabric patterns",
  "mcpName": "io.github.copyleftdev/fabric-atelier",
  "bin": {
    "fabric-atelier": "./bin/fabric-atelier"
  },
  "keywords": ["mcp", "fabric", "ai", "llm"],
  "author": "copyleftdev",
  "license": "MIT",
  "repository": {
    "type": "git",
    "url": "https://github.com/copyleftdev/fabric-atelier"
  }
}
```

#### 2. Publish to NPM

```bash
npm publish --access public
```

#### 3. Update server.json

```json
{
  "packages": [
    {
      "registryType": "npm",
      "identifier": "@copyleftdev/fabric-atelier",
      "version": "1.0.0",
      "transport": {
        "type": "stdio"
      }
    }
  ]
}
```

## Publishing to Smithery

### Option 1: Automatic (from GitHub)

1. **Push to GitHub**
   ```bash
   git push origin main
   ```

2. **Visit Smithery**
   - Go to https://smithery.ai/new
   - Connect GitHub repo
   - Smithery auto-detects MCP server
   - Click "Publish"

### Option 2: Manual

1. **Create smithery.json**
   ```json
   {
     "name": "fabric-atelier",
     "description": "AI-powered content processing with 226 Fabric patterns",
     "repository": "https://github.com/copyleftdev/fabric-atelier",
     "tags": ["fabric", "writing", "analysis", "security"],
     "install": {
       "docker": "docker run -i copyleftdev/fabric-atelier:latest"
     }
   }
   ```

2. **Submit via Smithery UI**
   - https://smithery.ai/new
   - Fill in form
   - Submit for review

## Automated Publishing with GitHub Actions

Create `.github/workflows/publish-mcp.yml`:

```yaml
name: Publish to MCP Registry

on:
  release:
    types: [published]

jobs:
  publish-docker:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Set up Docker Buildx
        uses: docker/setup-buildx-action@v3
      
      - name: Login to Docker Hub
        uses: docker/login-action@v3
        with:
          username: ${{ secrets.DOCKER_USERNAME }}
          password: ${{ secrets.DOCKER_TOKEN }}
      
      - name: Extract version
        id: version
        run: echo "VERSION=${GITHUB_REF#refs/tags/v}" >> $GITHUB_OUTPUT
      
      - name: Build and push
        uses: docker/build-push-action@v5
        with:
          context: .
          push: true
          tags: |
            copyleftdev/fabric-atelier:${{ steps.version.outputs.VERSION }}
            copyleftdev/fabric-atelier:latest
          labels: |
            io.modelcontextprotocol.server.name=io.github.copyleftdev/fabric-atelier
  
  publish-registry:
    needs: publish-docker
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install MCP Publisher
        run: |
          curl -L "https://github.com/modelcontextprotocol/registry/releases/download/latest/mcp-publisher_linux_amd64.tar.gz" | tar xz
          sudo mv mcp-publisher /usr/local/bin/
      
      - name: Update server.json version
        run: |
          VERSION=${GITHUB_REF#refs/tags/v}
          jq --arg version "$VERSION" '.version = $version | .packages[0].version = $version' server.json > server.json.tmp
          mv server.json.tmp server.json
      
      - name: Publish to MCP Registry
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        run: |
          mcp-publisher auth github --token $GITHUB_TOKEN
          mcp-publisher publish
```

## Package Structure Requirements

### For Docker Deployment

```
fabric-atelier/
├── Dockerfile                    # Multi-stage build
├── server.json                   # MCP registry metadata
├── README.md                     # User documentation
├── LICENSE                       # MIT license
├── Cargo.toml                    # Rust dependencies
├── src/                          # Source code
├── data/
│   └── fabric/                   # Fabric patterns (git submodule)
└── .github/
    └── workflows/
        └── publish-mcp.yml       # Auto-publish on release
```

### server.json Schema

Full schema: https://static.modelcontextprotocol.io/schemas/2025-09-29/server.schema.json

**Required fields:**
- `name` - Unique identifier (io.github.username/server-name)
- `title` - Display name
- `description` - What the server does
- `version` - Semantic version
- `packages` - Array of deployment methods

**Recommended fields:**
- `license` - License type
- `homepage` - Project website
- `repository` - Source code URL
- `author` - Creator info
- `categories` - For discovery
- `tags` - Keywords

## Testing Before Publishing

### 1. Test Locally with MCP Inspector

```bash
# Install MCP Inspector
npm install -g @modelcontextprotocol/inspector

# Test your server
mcp-inspector docker run -i copyleftdev/fabric-atelier:latest
```

### 2. Test with Claude Desktop

Add to `~/Library/Application Support/Claude/claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "fabric-atelier": {
      "command": "docker",
      "args": ["run", "-i", "copyleftdev/fabric-atelier:latest"]
    }
  }
}
```

### 3. Validate server.json

```bash
# Install ajv-cli
npm install -g ajv-cli

# Validate against schema
ajv validate \
  -s https://static.modelcontextprotocol.io/schemas/2025-09-29/server.schema.json \
  -d server.json
```

## Post-Publication

### 1. Update README

Add installation instructions:

```markdown
## Installation

### Via Claude Desktop

Add to your `claude_desktop_config.json`:

\`\`\`json
{
  "mcpServers": {
    "fabric-atelier": {
      "command": "docker",
      "args": ["run", "-i", "copyleftdev/fabric-atelier:latest"]
    }
  }
}
\`\`\`

### Via MCP Registry

\`\`\`bash
mcp install io.github.copyleftdev/fabric-atelier
\`\`\`
```

### 2. Create Documentation

- Usage examples
- Available patterns (all 226)
- Configuration options
- Troubleshooting

### 3. Promote

- Tweet about it
- Post on Reddit (r/ClaudeAI, r/LocalLLaMA)
- Share in MCP Discord
- Blog post

## Comparison: Registry Options

| Feature | Official MCP Registry | Smithery | GitHub Registry |
|---------|----------------------|----------|-----------------|
| **Status** | ✅ Live | ✅ Live | 🚧 Coming Soon |
| **Servers** | Growing | 7,578+ | TBD |
| **Discovery** | Search, categories | Semantic search | GitHub native |
| **Deployment** | Multi-platform | Docker focus | GitHub Actions |
| **Testing** | MCP Inspector | Playground | TBD |
| **Best For** | Official listing | Quick discovery | CI/CD workflows |

## Recommended Strategy

1. **Primary**: Official MCP Registry
   - Most authoritative
   - Best long-term support
   - Official tooling

2. **Secondary**: Smithery
   - Better discovery
   - Larger audience
   - Easy testing

3. **Future**: GitHub Registry
   - When available
   - Seamless CI/CD
   - GitHub-native

## Next Steps

1. ✅ Create Dockerfile
2. ✅ Build and test Docker image
3. ✅ Push to Docker Hub
4. ✅ Create server.json
5. ✅ Install mcp-publisher CLI
6. ✅ Authenticate with GitHub
7. ✅ Publish to registry
8. ✅ Verify publication
9. ✅ Update documentation
10. ✅ Promote!

## Resources

- **Official MCP Registry**: https://github.com/modelcontextprotocol/registry
- **Publishing Guide**: https://github.com/modelcontextprotocol/registry/blob/main/docs/guides/publishing/publish-server.md
- **Smithery**: https://smithery.ai
- **MCP Specification**: https://modelcontextprotocol.io/specification
- **MCP Inspector**: https://github.com/modelcontextprotocol/inspector
