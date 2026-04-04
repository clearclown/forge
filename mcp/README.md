# forge-mcp

> MCP server for the Forge compute economy. Gives AI assistants direct access to CU balance, inference, and safety controls.

## Install

```bash
pip install forge-mcp
```

## Setup

### Claude Code

Add to `~/.claude/settings.json`:

```json
{
  "mcpServers": {
    "forge": {
      "command": "forge-mcp",
      "env": {
        "FORGE_URL": "http://127.0.0.1:3000"
      }
    }
  }
}
```

### Cursor

Add to `.cursor/mcp.json`:

```json
{
  "mcpServers": {
    "forge": {
      "command": "forge-mcp"
    }
  }
}
```

## Available Tools

| Tool | Description |
|------|-------------|
| `forge_balance` | Check CU balance and reputation |
| `forge_pricing` | Get market price and cost estimates |
| `forge_inference` | Run LLM inference (costs CU) |
| `forge_trades` | View trade history |
| `forge_network` | Mesh economic summary + Merkle root |
| `forge_providers` | Compare providers by cost and reputation |
| `forge_safety` | Check safety status |
| `forge_invoice` | Create Lightning invoice for CU cashout |
| `forge_kill_switch` | Emergency halt (freeze all transactions) |

## What Can AI Do With This?

- Check balance before making expensive decisions
- Compare providers and choose the cheapest
- Monitor spending velocity
- Cash out earnings to Bitcoin Lightning
- Hit the emergency stop if something goes wrong

The AI manages its own compute budget. Humans set the policy (budget limits, kill switch). The AI operates within those bounds autonomously.
