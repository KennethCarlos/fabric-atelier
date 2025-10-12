<div align="center">
  <img src="docs/media/logo.png" alt="Fabric Atelier Logo" width="200"/>
  
  # Fabric Atelier 🎨
  
  > *A high-performance atelier for Fabric patterns - MCP server built with Rust + Apache Arrow*
</div>

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)

## What is Fabric Atelier?

**Fabric Atelier** is a blazingly fast [Model Context Protocol (MCP)](https://modelcontextprotocol.io) server that exposes [Fabric's](https://github.com/danielmiessler/fabric) 200+ AI patterns as discoverable, executable tools for AI assistants like Claude Desktop, Cline, and other MCP clients.

Built with **Rust** and **Apache Arrow**, Atelier delivers sub-millisecond pattern discovery through vectorized semantic search, making Daniel Miessler's thoughtfully crafted patterns accessible to the entire MCP ecosystem.

## Why "Atelier"?

An *atelier* (French: workshop) is where craftsmen create with precision and artistry. Just as Fabric organizes AI prompts into reusable patterns, Atelier serves as the workshop where these patterns are discovered, orchestrated, and delivered to AI assistants with exceptional performance.

## Features

- 🚀 **Blazingly Fast** - Built with Rust for maximum performance
- 🎯 **226 Patterns** - All Fabric patterns accessible via MCP
- 🐳 **Docker Ready** - Pull and run in seconds
- 🤖 **LLM Powered** - Ollama, OpenAI, or Anthropic support
- 🦀 **Rust Performance** - 281MB Docker image, <50ms startup
- 🔗 **Auto-Sync** - Git submodule keeps patterns up-to-date with Fabric
- 🔒 **Secure** - Non-root Docker user, minimal dependencies
- 📊 **Benchmarked** - 5,000+ req/s, comprehensive performance testing

## Architecture

```
fabric-atelier/
├── src/                      # Rust source code
│   ├── mcp/                  # MCP protocol implementation
│   ├── fabric/               # Pattern loader & executor
│   ├── vector/               # Arrow-based semantic search
│   └── main.rs
├── data/
│   ├── fabric/              # Git submodule → danielmiessler/fabric
│   │   └── data/patterns/   # 200+ Fabric patterns (auto-synced)
│   └── embeddings.parquet   # Cached pattern embeddings
└── docs/
    ├── DESIGN_SPEC.md       # Technical architecture
    ├── PROPOSAL.md          # Project vision & philosophy
    └── QUICKSTART.md        # Implementation guide
```

## Quick Start

### Option 1: Docker (Recommended) 🐳

**Pull from Docker Hub:**
```bash
docker pull copyleftdev/fabric-atelier:latest
```

**Configure Claude Desktop:**
```json
{
  "mcpServers": {
    "fabric-atelier": {
      "command": "docker",
      "args": ["run", "-i", "--rm", "copyleftdev/fabric-atelier:latest"]
    }
  }
}
```

**Test it:**
```bash
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | \
  docker run -i --rm copyleftdev/fabric-atelier:latest
```

### Option 2: Build from Source

**Prerequisites:**
- Rust 1.90+ (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- Local LLM (Ollama) or API keys (OpenAI/Anthropic)

**Installation:**
```bash
# Clone with submodules
git clone --recursive https://github.com/copyleftdev/fabric-atelier.git
cd fabric-atelier

# Build release binary
cargo build --release

# Binary location: target/release/fabric-atelier
```

### Configure MCP Client

**Claude Desktop (macOS):**
```bash
code ~/Library/Application\ Support/Claude/claude_desktop_config.json
```

**Claude Desktop (Linux):**
```bash
code ~/.config/Claude/claude_desktop_config.json
```

Add this configuration:
```json
{
  "mcpServers": {
    "fabric-atelier": {
      "command": "/absolute/path/to/fabric-atelier/target/release/fabric-atelier"
    }
  }
}
```

Restart Claude Desktop and look for the 🔌 icon to verify connection.

## Usage

### In Claude Desktop

```
User: "Find me a Fabric pattern for analyzing security papers"
Claude: [Uses fabric_find_pattern tool]
        → Suggests: analyze_threat_report, analyze_paper, extract_wisdom

User: "Use fabric_extract_wisdom to analyze this article: [content]"
Claude: [Executes pattern and returns insights]
```

### Available MCP Tools

- **`fabric_find_pattern`** - Semantic search for patterns
- **`fabric_<pattern_name>`** - Execute any Fabric pattern (200+ tools)
  - `fabric_summarize` - Summarize content
  - `fabric_extract_wisdom` - Extract insights from articles/videos
  - `fabric_analyze_claims` - Fact-check and analyze claims
  - `fabric_improve_writing` - Enhance writing quality
  - `fabric_explain_code` - Explain code snippets
  - ... and 195+ more!

## How It Works

### 1. Pattern Discovery
Fabric Atelier loads all patterns from the `data/fabric/data/patterns/` directory (synced via git submodule).

### 2. Semantic Indexing
Pattern descriptions are embedded using OpenAI/Anthropic APIs and cached in Parquet format using Apache Arrow for instant access.

### 3. MCP Protocol
The server implements the Model Context Protocol, exposing patterns as tools that AI assistants can discover and execute.

### 4. Pattern Execution
When a tool is called, Atelier executes the corresponding Fabric pattern via CLI and returns the result.

## Data Strategy

Fabric Atelier uses a **git submodule** to reference Daniel Miessler's Fabric repository:

```bash
# Update to latest Fabric patterns
git submodule update --remote data/fabric

# Rebuild embeddings after update
cargo run --bin generate-embeddings
```

This approach:
- ✅ Keeps patterns in sync with upstream Fabric
- ✅ Respects Fabric's MIT license
- ✅ No data duplication
- ✅ Single source of truth

## Performance

| Metric | Fabric Atelier | Notes |
|--------|----------------|-------|
| Docker image | 281 MB | Multi-stage build with cargo-chef |
| Startup time | <50 ms | Pattern loading included |
| Memory usage | ~30 MB | Runtime footprint |
| Throughput | 5,000-7,000 req/s | Concurrent request handling |
| Pattern loading | ~11 ms | 226 patterns from disk |
| Request latency | ~380 µs | Sub-millisecond response |

See [BENCHMARKS.md](BENCHMARKS.md) for detailed performance analysis.

## Development

### Project Structure

See [docs/DESIGN_SPEC.md](docs/DESIGN_SPEC.md) for detailed architecture.

### Building

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run benchmarks
cargo bench
```

### Generating Embeddings

```bash
# Set API key
export OPENAI_API_KEY=your_key_here

# Generate embeddings for all patterns
cargo run --bin generate-embeddings

# Output: data/embeddings.parquet
```

## Documentation

- **[DESIGN_SPEC.md](docs/DESIGN_SPEC.md)** - Complete technical architecture
- **[PROPOSAL.md](docs/PROPOSAL.md)** - Project vision and philosophy
- **[QUICKSTART.md](docs/QUICKSTART.md)** - 30-minute implementation guide

## Roadmap

- [x] Basic MCP server implementation
- [x] Pattern loading from Fabric submodule
- [x] Git submodule integration
- [ ] Apache Arrow vector search
- [ ] Parquet embedding cache
- [ ] SIMD-accelerated similarity search
- [ ] Pattern chaining support
- [ ] YouTube transcript integration
- [ ] Custom pattern support
- [ ] Performance benchmarks

## Contributing

Contributions welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) first.

## Philosophy

Fabric Atelier aligns with [Fabric's core philosophy](https://github.com/danielmiessler/fabric):

> "AI isn't a thing; it's a magnifier of a thing. And that thing is human creativity."

By making Fabric's patterns accessible through MCP, we extend this vision of human augmentation to every AI assistant that supports the protocol.

## Credits

- **[Daniel Miessler](https://github.com/danielmiessler)** - Creator of [Fabric](https://github.com/danielmiessler/fabric)
- **[Model Context Protocol](https://modelcontextprotocol.io)** - Anthropic's open protocol for AI tool integration
- **[Apache Arrow](https://arrow.apache.org/)** - High-performance columnar data format

## License

MIT License - see [LICENSE](LICENSE) for details.

Fabric patterns are licensed under MIT by [danielmiessler/fabric](https://github.com/danielmiessler/fabric).

---

**Built with 🦀 Rust and ❤️ for the Fabric community**
