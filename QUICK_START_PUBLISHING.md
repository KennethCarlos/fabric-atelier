# Quick Start: Publishing Fabric Atelier to MCP Registry

## TL;DR - 5 Commands to Publish

```bash
# 1. Build Docker image
docker build -t copyleftdev/fabric-atelier:0.1.0 .

# 2. Test locally
docker run -i copyleftdev/fabric-atelier:0.1.0

# 3. Push to Docker Hub
docker login
docker push copyleftdev/fabric-atelier:0.1.0

# 4. Install MCP Publisher
brew install mcp-publisher

# 5. Publish to registry
mcp-publisher auth github
mcp-publisher publish
```

## Most Popular MCP Registries

### 🏆 Official MCP Registry (RECOMMENDED)
- **URL**: https://github.com/modelcontextprotocol/registry
- **Status**: Official, backed by Anthropic
- **Best for**: Maximum discoverability
- **Publish with**: `mcp-publisher` CLI

### 🔍 Smithery
- **URL**: https://smithery.ai
- **Servers**: 7,578+ servers
- **Best for**: Easy discovery, testing playground
- **Publish with**: GitHub integration or web UI

## What We've Built

✅ **Dockerfile** - Multi-stage build, minimal image  
✅ **server.json** - MCP registry metadata  
✅ **.dockerignore** - Optimized build  
✅ **GitHub Actions** - Auto-publish on release  
✅ **Documentation** - Complete publishing guide  

## File Structure

```
fabric-atelier/
├── Dockerfile                           # Docker build
├── server.json                          # MCP metadata
├── .dockerignore                        # Build optimization
├── .github/workflows/publish-mcp.yml    # CI/CD
└── docs/PUBLISHING_TO_MCP_REGISTRY.md   # Full guide
```

## Manual Publishing (Step-by-Step)

### 1. Build & Test Docker Image

```bash
# Build
docker build -t copyleftdev/fabric-atelier:0.1.0 .

# Test
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | \
  docker run -i copyleftdev/fabric-atelier:0.1.0

# Should see: {"jsonrpc":"2.0","id":1,"result":{...}}
```

### 2. Push to Docker Hub

```bash
# Login
docker login

# Tag as latest
docker tag copyleftdev/fabric-atelier:0.1.0 copyleftdev/fabric-atelier:latest

# Push both tags
docker push copyleftdev/fabric-atelier:0.1.0
docker push copyleftdev/fabric-atelier:latest
```

### 3. Install MCP Publisher

```bash
# macOS/Linux
brew install mcp-publisher

# Or download binary
curl -L "https://github.com/modelcontextprotocol/registry/releases/download/latest/mcp-publisher_$(uname -s | tr '[:upper:]' '[:lower:]')_$(uname -m | sed 's/x86_64/amd64/;s/aarch64/arm64/').tar.gz" | tar xz
sudo mv mcp-publisher /usr/local/bin/
```

### 4. Authenticate

```bash
# For io.github.* namespaces
mcp-publisher auth github
# Opens browser for GitHub OAuth
```

### 5. Publish

```bash
# Validate first (optional)
npm install -g ajv-cli
ajv validate \
  -s https://static.modelcontextprotocol.io/schemas/2025-09-29/server.schema.json \
  -d server.json

# Publish
mcp-publisher publish

# Check status
mcp-publisher status io.github.copyleftdev/fabric-atelier
```

## Automated Publishing (GitHub Actions)

### Setup Secrets

1. Go to GitHub repo → Settings → Secrets
2. Add secrets:
   - `DOCKER_USERNAME` - Your Docker Hub username
   - `DOCKER_TOKEN` - Docker Hub access token

### Trigger Publishing

**Option 1: Create a Release**
```bash
git tag v0.1.0
git push origin v0.1.0
# Go to GitHub → Releases → Create release from tag
```

**Option 2: Manual Dispatch**
```bash
# Go to GitHub → Actions → "Publish to MCP Registry" → Run workflow
# Enter version: 0.1.0
```

The workflow will:
1. ✅ Build Docker image
2. ✅ Push to Docker Hub
3. ✅ Validate server.json
4. ✅ Publish to MCP registry
5. ✅ Verify publication

## Testing Before Publishing

### With MCP Inspector

```bash
# Install
npm install -g @modelcontextprotocol/inspector

# Test
mcp-inspector docker run -i copyleftdev/fabric-atelier:0.1.0
```

### With Claude Desktop

Add to `~/Library/Application Support/Claude/claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "fabric-atelier": {
      "command": "docker",
      "args": ["run", "-i", "copyleftdev/fabric-atelier:0.1.0"]
    }
  }
}
```

Restart Claude Desktop and test!

## After Publishing

### 1. Verify in Registry

Visit: https://registry.modelcontextprotocol.io/servers/io.github.copyleftdev/fabric-atelier

### 2. Update README

Add installation instructions:

```markdown
## Installation

### Via Docker

\`\`\`bash
docker pull copyleftdev/fabric-atelier:latest
\`\`\`

### Via Claude Desktop

Add to your \`claude_desktop_config.json\`:

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
```

### 3. Promote

- 🐦 Tweet about it
- 📝 Blog post
- 💬 Share in MCP Discord
- 📢 Post on Reddit (r/ClaudeAI, r/LocalLLaMA)

## Troubleshooting

### Docker build fails
```bash
# Check Docker is running
docker ps

# Check submodules
git submodule update --init --recursive

# Build with verbose output
docker build --progress=plain -t copyleftdev/fabric-atelier:0.1.0 .
```

### MCP Publisher auth fails
```bash
# Clear credentials
rm ~/.config/mcp-publisher/credentials.json

# Re-authenticate
mcp-publisher auth github
```

### server.json validation fails
```bash
# Check schema
cat server.json | jq .

# Validate
ajv validate \
  -s https://static.modelcontextprotocol.io/schemas/2025-09-29/server.schema.json \
  -d server.json
```

## Key Files Explained

### server.json
- **name**: `io.github.copyleftdev/fabric-atelier` (must match Docker label)
- **version**: Semantic version (0.1.0)
- **packages**: Docker deployment config
- **categories**: For discovery
- **tags**: Keywords for search

### Dockerfile
- **Multi-stage**: Builder + runtime (minimal size)
- **Label**: `io.modelcontextprotocol.server.name` (REQUIRED)
- **Entrypoint**: stdio transport

### GitHub Actions
- **Triggers**: Release or manual
- **Jobs**: Docker push + registry publish
- **Secrets**: Docker Hub credentials

## Resources

- **Full Guide**: [docs/PUBLISHING_TO_MCP_REGISTRY.md](docs/PUBLISHING_TO_MCP_REGISTRY.md)
- **Official Registry**: https://github.com/modelcontextprotocol/registry
- **Smithery**: https://smithery.ai
- **MCP Docs**: https://modelcontextprotocol.io

---

**Ready to publish?** Start with step 1! 🚀
