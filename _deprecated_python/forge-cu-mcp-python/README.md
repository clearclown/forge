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

## Phase 8 Tools (L2 Bank, L3 Mind, L4 Agora)

### L2 Bank

| Tool | Description |
|------|-------------|
| `forge_bank_portfolio` | PortfolioManager state: cash, lent, borrowed, net exposure |
| `forge_bank_tick` | Run one strategy tick; returns Lend/Borrow/Hold decisions |
| `forge_bank_set_strategy` | Hot-swap portfolio strategy: `conservative`, `highyield`, `balanced` |
| `forge_bank_set_risk` | Set risk tolerance: `conservative`, `balanced`, `aggressive` |
| `forge_bank_list_futures` | List all active CU futures contracts |
| `forge_bank_create_future` | Create a new CU futures contract with a counterparty |
| `forge_bank_risk_assessment` | VaR, concentration, and leverage assessment of the portfolio |
| `forge_bank_optimize` | Run YieldOptimizer with a VaR budget; applies changes if beneficial |

### L4 Agora

| Tool | Description |
|------|-------------|
| `forge_agora_register` | Register an agent in the Agora marketplace |
| `forge_agora_list_agents` | List all registered agent profiles |
| `forge_agora_reputation` | Get reputation score for a specific agent by hex NodeId |
| `forge_agora_find` | Find agents matching model patterns and optional filters |
| `forge_agora_stats` | Registry statistics: agent_count, trade_count, etc. |
| `forge_agora_snapshot` | Export registry snapshot for backup or migration |
| `forge_agora_restore` | Restore registry from a previously exported snapshot |

### L3 Mind

| Tool | Description |
|------|-------------|
| `forge_mind_init` | Initialise ForgeMindAgent with a system prompt and optimizer |
| `forge_mind_state` | Harness version, prompt preview, cycle count, today's budget usage |
| `forge_mind_improve` | Run N self-improvement cycles with the MetaOptimizer |
| `forge_mind_budget` | Update max CU per cycle/day and max cycles per day |
| `forge_mind_stats` | Lifetime stats: kept/reverted cycles, score delta, total CU invested |

## What Can AI Do With This?

- Check balance before making expensive decisions
- Compare providers and choose the cheapest
- Monitor spending velocity
- Cash out earnings to Bitcoin Lightning
- Hit the emergency stop if something goes wrong

The AI manages its own compute budget. Humans set the policy (budget limits, kill switch). The AI operates within those bounds autonomously.
