# Adding Fabric Atelier to Windsurf

This guide shows you how to add your Fabric Atelier MCP server to Windsurf IDE.

## Method 1: Using the Plugin Store (Easiest)

1. **Open Windsurf**
2. **Access the Plugin Store**:
   - Click the `Plugins` icon in the top right menu in the Cascade panel
   - OR go to `Windsurf Settings` > `Cascade` > `Plugins`
3. **Search for "Fabric Atelier"** in the MCP Plugin Store
4. **Click Install** to expose the server and its 226 tools to Cascade
5. **Press the refresh button** after installation

## Method 2: Manual Configuration via mcp_config.json

If Fabric Atelier isn't in the Plugin Store yet, you can add it manually:

### Step 1: Locate the Config File

The MCP configuration file is located at:
```
~/.codeium/windsurf/mcp_config.json
```

### Step 2: Add Fabric Atelier Configuration

Add the following to your `mcp_config.json`:

```json
{
  "mcpServers": {
    "fabric-atelier": {
      "command": "docker",
      "args": [
        "run",
        "--rm",
        "-i",
        "--pull=always",
        "-e", "OLLAMA_BASE_URL=http://host.docker.internal:11434",
        "copyleftdev/fabric-atelier:latest"
      ]
    }
  }
}
```

### Step 3: Configure Environment Variables (Optional)

If you want to use OpenAI or Anthropic instead of Ollama:

**For OpenAI:**
```json
{
  "mcpServers": {
    "fabric-atelier": {
      "command": "docker",
      "args": [
        "run",
        "--rm",
        "-i",
        "--pull=always",
        "-e", "OPENAI_API_KEY=<YOUR_OPENAI_API_KEY>",
        "-e", "LLM_PROVIDER=openai",
        "-e", "OPENAI_MODEL=gpt-4",
        "copyleftdev/fabric-atelier:latest"
      ]
    }
  }
}
```

**For Anthropic:**
```json
{
  "mcpServers": {
    "fabric-atelier": {
      "command": "docker",
      "args": [
        "run",
        "--rm",
        "-i",
        "--pull=always",
        "-e", "ANTHROPIC_API_KEY=<YOUR_ANTHROPIC_API_KEY>",
        "-e", "LLM_PROVIDER=anthropic",
        "-e", "ANTHROPIC_MODEL=claude-3-5-sonnet-20241022",
        "copyleftdev/fabric-atelier:latest"
      ]
    }
  }
}
```

### Step 4: Restart Windsurf

After editing the config file, restart Windsurf for the changes to take effect.

## Managing Tools

Fabric Atelier provides 226 tools (one for each Fabric pattern). Windsurf has a limit of 100 total tools across all MCP servers.

### To manage which tools are enabled:

1. Go to the **Plugin Store**
2. Click on **Fabric Atelier**
3. Navigate to the **Tools** tab
4. Toggle the specific patterns you want to enable
5. OR click **Manage plugins** from `Windsurf Settings`

## Using Fabric Atelier in Cascade

Once installed, you can use Fabric patterns in your Cascade conversations:

### Example Prompts:

```
"Use fabric_write_essay_pg to write an essay about AI ethics"

"Analyze this code with fabric_analyze_claims"

"Extract insights from this document using fabric_extract_insights"

"Review this security report with fabric_analyze_threat_report"
```

### Available Patterns:

All 226 Fabric patterns are available, prefixed with `fabric_`. Some popular ones:

- **Writing**: `fabric_write_essay_pg`, `fabric_improve_writing`, `fabric_write_micro_essay`
- **Analysis**: `fabric_analyze_claims`, `fabric_analyze_paper`, `fabric_extract_insights`
- **Security**: `fabric_analyze_threat_report`, `fabric_analyze_malware`, `fabric_create_security_update`
- **Code**: `fabric_create_coding_project`, `fabric_explain_code`, `fabric_review_code`
- **Business**: `fabric_create_keynote`, `fabric_create_summary`, `fabric_extract_wisdom`

## Troubleshooting

### Docker Not Found
Make sure Docker is installed and running:
```bash
docker --version
docker ps
```

### Ollama Connection Issues
If using Ollama, ensure it's running:
```bash
ollama serve
```

And that the URL is correct:
- **Linux/macOS**: `http://host.docker.internal:11434`
- **Windows**: `http://host.docker.internal:11434`

### Tools Not Showing Up
1. Press the **refresh button** in the Plugin Store
2. Restart Windsurf
3. Check the Cascade panel for available tools

### Rate Limiting
If you have too many MCP tools enabled (>100), disable some unused plugins or tools.

## Configuration Examples

### Multiple MCP Servers

You can have multiple MCP servers configured:

```json
{
  "mcpServers": {
    "fabric-atelier": {
      "command": "docker",
      "args": [
        "run", "--rm", "-i", "--pull=always",
        "-e", "OLLAMA_BASE_URL=http://host.docker.internal:11434",
        "copyleftdev/fabric-atelier:latest"
      ]
    },
    "github": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-github"],
      "env": {
        "GITHUB_PERSONAL_ACCESS_TOKEN": "<YOUR_TOKEN>"
      }
    }
  }
}
```

## Team/Enterprise Settings

If you're on a Windsurf Team or Enterprise plan, admins can:
- Toggle MCP access for the team
- Whitelist approved MCP servers
- Configure at: https://windsurf.com/team/settings

## Resources

- **GitHub**: https://github.com/copyleftdev/fabric-atelier
- **Docker Hub**: https://hub.docker.com/r/copyleftdev/fabric-atelier
- **MCP Registry**: https://registry.modelcontextprotocol.io/servers/io.github.copyleftdev/fabric-atelier
- **Fabric Patterns**: https://github.com/danielmiessler/fabric

## Support

For issues or questions:
- Open an issue on GitHub: https://github.com/copyleftdev/fabric-atelier/issues
- Check the Fabric Atelier documentation in the repo
