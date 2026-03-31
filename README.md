# Forge

> Compute bought by AI, not humans.

**The first MCP-native compute marketplace for AI agents.**

Forge is a P2P compute market protocol that enables AI agents to autonomously procure, execute, verify, and settle compute tasks — without human intervention.

## Why Forge?

Existing distributed compute markets (Golem, Akash, io.net, etc.) are all designed for **humans** to rent cheap infrastructure. Forge is different: the **buyer is an AI agent**.

When Claude Code detects a heavy parallel test suite, it calls `forge_run`. Forge distributes the task to surplus PCs worldwide, verifies results by majority vote, and returns only verified outputs — all without human involvement.

## Status

🚧 **Concept phase** — architecture and spec finalized, implementation not started.

## MVP Target

- **First buyer**: Autonomous development agents (Claude Code, OpenClaw)
- **First task type**: Deterministic CPU tasks (pytest, repo analysis)
- **Supply side**: Surplus home PCs
- **Verification**: Majority vote (2-of-3 nodes)
- **Settlement**: Off-chain ledger (blockchain later)
- **Interface**: MCP tool (`forge_run`)

## Architecture

```
┌─────────────────────────────────────┐
│  Upper Layer: Agent Interface       │
│  MCP Tool (forge_run)               │
│  Single API for AI agents           │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│  Lower Layer: Compute Market        │
│  Broker → Worker × N → Verify → Settle │
└─────────────────────────────────────┘
```

## Quick Start (coming soon)

```python
from forge import compute

result = await compute.run(
    task_type="pytest",
    command="pytest tests/",
    budget_max=0.5,
    timeout_sec=300
)
```

## Docs

- [Concept & Problem Statement](docs/concept.md)
- [Architecture](docs/architecture.md)
- [MCP Tool Spec](docs/mcp-spec.md)
- [Roadmap](docs/roadmap.md)

## License

MIT
