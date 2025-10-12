# Fabric Atelier - Development Context

## Project Status: Initial Setup Complete ✅

**Last Updated**: October 11, 2025, 8:27 PM UTC-07:00  
**Current Phase**: Ready to begin MVP implementation  
**Repository**: https://github.com/copyleftdev/fabric-atelier

---

## What We've Built So Far

### ✅ Repository Structure
```
fabric-atelier/
├── README.md                  # Professional project overview
├── Cargo.toml                 # Rust dependencies configured
├── .gitignore                 # Rust project gitignore
├── SETUP_COMPLETE.md         # Setup verification document
├── DEVELOPMENT_CONTEXT.md    # This file - development context
├── docs/
│   ├── DESIGN_SPEC.md        # Complete Rust + Arrow architecture
│   ├── PROPOSAL.md           # Project vision & philosophy  
│   └── QUICKSTART.md         # 30-minute MVP implementation guide
└── data/
    └── fabric/               # Git submodule → danielmiessler/fabric
        └── data/patterns/    # 227 Fabric patterns (auto-synced)
```

### ✅ Git Configuration
- **Account**: copyleftdev (verified with `gh auth status`)
- **SSH Key**: `~/.ssh/id_ed25519_copyleftdev`
- **Git config**: `core.sshCommand` set to use copyleftdev key
- **Submodule**: Fabric repo linked at `data/fabric/`

### ✅ Documentation Complete
1. **DESIGN_SPEC.md** - Full Rust + Apache Arrow architecture
2. **PROPOSAL.md** - Project vision aligned with Daniel Miessler's philosophy
3. **QUICKSTART.md** - Step-by-step implementation guide
4. **README.md** - Professional project overview with features, roadmap

---

## Project Vision

**Fabric Atelier** is a high-performance MCP (Model Context Protocol) server that:
- Exposes Fabric's 227 patterns as MCP tools
- Uses Rust + Apache Arrow for sub-millisecond pattern discovery
- Implements semantic search via vectorized embeddings
- Maintains sync with upstream Fabric via git submodule

**Name Origin**: "Atelier" (French: workshop) - where craftsmen create with precision

---

## Technical Architecture

### Core Technologies
- **Language**: Rust 1.70+
- **Vector Engine**: Apache Arrow + DataFusion
- **Protocol**: MCP (JSON-RPC over stdio)
- **Embeddings**: OpenAI/Anthropic APIs
- **Cache**: Parquet format (columnar storage)

### Key Design Decisions

#### 1. Data Strategy: Git Submodule ✅
```bash
# Patterns live at:
data/fabric/data/patterns/

# Update from upstream:
git submodule update --remote data/fabric
```

**Why submodule?**
- ✅ Always in sync with Daniel's Fabric
- ✅ No data duplication
- ✅ Respects MIT license
- ✅ Single source of truth

#### 2. Performance Goals
- Binary size: ~8 MB (vs ~50 MB Go)
- Startup time: <10 ms
- Memory usage: ~25 MB
- Pattern search: <1 ms (Arrow SIMD)

#### 3. MCP Tools Exposed
- `fabric_find_pattern` - Semantic search
- `fabric_<pattern_name>` - Execute any pattern (227 tools)
  - `fabric_summarize`
  - `fabric_extract_wisdom`
  - `fabric_analyze_claims`
  - ... and 224 more

---

## Implementation Roadmap

### Phase 1: MVP (Next Steps) 🎯
**Goal**: Basic MCP server that executes patterns

**Tasks**:
1. Create `src/main.rs` - stdio JSON-RPC handler
2. Create `src/mcp/` - MCP protocol implementation
3. Create `src/fabric/` - Pattern loader & executor
4. Test with Claude Desktop

**Reference**: Follow `docs/QUICKSTART.md` for detailed steps

### Phase 2: Semantic Search
**Goal**: Arrow-based vector search

**Tasks**:
1. Create `src/vector/` - Embeddings & search
2. Generate embeddings for all patterns
3. Cache in Parquet format
4. Implement `fabric_find_pattern` tool

### Phase 3: Production Ready
**Goal**: Polish and optimize

**Tasks**:
1. SIMD acceleration for similarity search
2. Comprehensive testing
3. Benchmarks
4. CI/CD pipeline
5. Demo video

---

## Current Dependencies (Cargo.toml)

```toml
[dependencies]
tokio = { version = "1.40", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
arrow = "53.0"
parquet = "53.0"
datafusion = "42.0"
reqwest = { version = "0.12", features = ["json"] }
anyhow = "1.0"
thiserror = "1.0"
tracing = "0.1"
tracing-subscriber = "0.3"
config = "0.14"
toml = "0.8"
```

---

## Key Implementation Details

### Pattern Loading Strategy
```rust
// Read from submodule
let patterns_dir = PathBuf::from("data/fabric/data/patterns");

// Fallback to user's Fabric installation
let home = env::var("HOME")?;
let fallback = PathBuf::from(format!("{}/.config/fabric/patterns", home));
```

### Pattern Execution Strategy
```rust
// Option 1: Shell out to Fabric CLI (simpler, MVP)
Command::new("fabric")
    .arg("--pattern")
    .arg(pattern_name)
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()?

// Option 2: HTTP to Fabric REST API (future)
// POST http://localhost:8080/patterns/{name}/apply
```

### MCP Protocol Flow
```
1. Client (Claude) connects via stdio
2. Sends: {"jsonrpc":"2.0","method":"initialize"}
3. Server responds with capabilities
4. Client requests: {"method":"tools/list"}
5. Server returns all 227+ pattern tools
6. Client calls: {"method":"tools/call","params":{"name":"fabric_summarize"}}
7. Server executes pattern, returns result
```

---

## Development Commands

```bash
# Build project
cargo build                    # Debug build
cargo build --release          # Optimized build

# Run tests
cargo test

# Check code
cargo check
cargo clippy                   # Linting

# Format code
cargo fmt

# Run server (once implemented)
./target/release/fabric-atelier

# Update Fabric patterns
git submodule update --remote data/fabric
git add data/fabric
git commit -m "Update Fabric patterns"
git push
```

---

## Testing Strategy

### Manual Testing
```bash
# Test JSON-RPC protocol
echo '{"jsonrpc":"2.0","id":1,"method":"initialize"}' | ./target/release/fabric-atelier

# Test pattern listing
echo '{"jsonrpc":"2.0","id":2,"method":"tools/list"}' | ./target/release/fabric-atelier
```

### Claude Desktop Integration
**Config location**:
- macOS: `~/Library/Application Support/Claude/claude_desktop_config.json`
- Linux: `~/.config/Claude/claude_desktop_config.json`

**Config content**:
```json
{
  "mcpServers": {
    "fabric-atelier": {
      "command": "/absolute/path/to/fabric-atelier/target/release/fabric-atelier"
    }
  }
}
```

---

## Important Context from Original Discussion

### Why This Project Exists
1. **Problem**: Fabric's 227 patterns aren't accessible to MCP clients
2. **Solution**: Build MCP server to expose patterns as tools
3. **Approach**: Standalone repo (not fork) that references Fabric
4. **Goal**: Make Daniel's patterns available to entire MCP ecosystem

### Design Philosophy Alignment
- **Daniel's Vision**: "AI magnifies human creativity"
- **Our Extension**: "Make that magnification accessible everywhere"
- **Approach**: Respect Fabric's architecture, extend via protocol bridge

### Why Rust + Arrow?
1. **Performance**: Sub-millisecond pattern discovery
2. **Compelling Story**: "Blazingly fast" resonates with developers
3. **Technical Excellence**: Shows serious engineering
4. **Arrow**: Industry standard for analytics/vectorization

### Naming Choice: "Atelier"
- French for "workshop" or "studio"
- Evokes craftsmanship and artistry
- Extends Fabric metaphor naturally
- Professional, memorable, elegant

---

## Next Immediate Steps

### 1. Create Source Structure
```bash
mkdir -p src/{mcp,fabric,vector,bin}
touch src/main.rs
touch src/mcp/{mod.rs,server.rs,protocol.rs,tools.rs}
touch src/fabric/{mod.rs,patterns.rs,executor.rs}
```

### 2. Implement Basic MCP Server
**Start with**: `docs/QUICKSTART.md` - has complete working code

**Key files to create**:
- `src/main.rs` - stdio handler
- `src/mcp/server.rs` - MCP protocol
- `src/fabric/patterns.rs` - Pattern loading

### 3. Test Locally
```bash
cargo build --release
echo '{"jsonrpc":"2.0","id":1,"method":"initialize"}' | ./target/release/fabric-atelier
```

### 4. Configure Claude Desktop
Add MCP server config and test pattern execution

---

## Resources & References

### Documentation
- **MCP Spec**: https://modelcontextprotocol.io/docs/specification
- **Fabric Repo**: https://github.com/danielmiessler/fabric
- **Arrow Docs**: https://arrow.apache.org/rust/
- **Our Docs**: See `docs/` folder

### Pattern Access
```bash
# List all patterns
ls data/fabric/data/patterns

# Count patterns
ls data/fabric/data/patterns | wc -l  # 227

# View a pattern
cat data/fabric/data/patterns/summarize/system.md
```

### Example Patterns
- `summarize` - Summarize any content
- `extract_wisdom` - Extract insights from articles/videos
- `analyze_claims` - Fact-check and analyze claims
- `improve_writing` - Enhance writing quality
- `explain_code` - Explain code snippets

---

## Known Issues / Considerations

### None Yet - Fresh Start! ✅

This is a greenfield project. No technical debt, no legacy issues.

---

## Success Metrics

### Technical
- [ ] MCP server responds to initialize
- [ ] All 227 patterns exposed as tools
- [ ] Pattern execution works via Fabric CLI
- [ ] Semantic search implemented
- [ ] <1ms pattern discovery latency

### Community
- [ ] Demo video created
- [ ] Shared with Fabric community
- [ ] Positive feedback from Daniel
- [ ] 100+ GitHub stars
- [ ] Featured in MCP server directory

---

## Communication with Daniel

### When to Reach Out
**After**: MVP working + demo video created

### What to Show
1. Demo video of Claude using Fabric patterns
2. Performance benchmarks (sub-ms search)
3. Clean architecture respecting his work
4. Community interest (stars, feedback)

### Pitch Template
> "Hey Daniel, I built an MCP server in Rust that makes your Fabric patterns accessible to Claude Desktop and other MCP clients. It uses Apache Arrow for sub-millisecond semantic search across all 227 patterns.
>
> It references your repo as a git submodule, so it's always in sync and respects your work. Users get instant pattern discovery with SIMD-accelerated similarity search.
>
> [Demo video link]
>
> Would love your thoughts!"

---

## Environment Setup

### Required Tools
- [x] Rust 1.70+ installed
- [x] Fabric installed (`fabric --version`)
- [x] Git configured with copyleftdev account
- [x] GitHub CLI authenticated
- [ ] OpenAI or Anthropic API key (for embeddings - Phase 2)

### Verify Setup
```bash
# Check Rust
rustc --version
cargo --version

# Check Fabric
fabric --version
fabric --listpatterns | head -5

# Check patterns accessible
ls data/fabric/data/patterns | head -5

# Check git config
git config user.name  # Should be: copyleftdev
git config core.sshCommand  # Should use copyleftdev key
```

---

## Quick Reference: File Locations

### This Project
- **Patterns**: `data/fabric/data/patterns/`
- **Docs**: `docs/DESIGN_SPEC.md`, `docs/QUICKSTART.md`
- **Source**: `src/` (to be created)
- **Binary**: `target/release/fabric-atelier` (after build)

### System Locations
- **Fabric Config**: `~/.config/fabric/`
- **Fabric Patterns**: `~/.config/fabric/patterns/`
- **Claude Config**: `~/Library/Application Support/Claude/` (macOS)
- **SSH Keys**: `~/.ssh/id_ed25519_copyleftdev`

---

## Summary: Where We Are

**✅ Complete**:
- Repository created and pushed to GitHub
- Git submodule linking to Fabric (227 patterns accessible)
- Comprehensive documentation (3 guides)
- Cargo.toml configured with dependencies
- Project vision and architecture defined

**🎯 Next**:
- Create `src/` directory structure
- Implement basic MCP protocol handler
- Load patterns from submodule
- Execute patterns via Fabric CLI
- Test with Claude Desktop

**📚 Reference**:
Start with `docs/QUICKSTART.md` - it has complete working code for the MVP.

---

**Ready to build! Follow QUICKSTART.md for the 30-minute MVP implementation.** 🚀
