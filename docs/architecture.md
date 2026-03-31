# Forge — Architecture

## Two-Layer Design

### Upper Layer: Agent Interface

The agent-facing layer. Forge is exposed as a single MCP tool `forge_run`. Agents specify task type, input, budget cap, timeout, and verification mode — that is it.

Compatible with: Claude Code, OpenAI Agents SDK, AutoGen, and any MCP-enabled runtime.

### Lower Layer: Compute Market

The actual compute market. Worker nodes run isolated execution environments on surplus PCs. Results are verified by replication + majority vote.

## Data Flow

```
AI Agent (Claude Code, etc.)
  |
  v forge_run(task, budget, deadline, verification)
Forge Broker (MVP: centralized mock)
  |
  v Replicate task to 3 nodes
Worker x 3 (surplus PCs)
  |
  v Execute in rootless Docker (MVP) / Wasm (future)
Verification Layer
  |
  v Majority vote (2/3 match -> accept, mismatch -> reputation penalty)
Settlement Layer
  |
  v Reward verified nodes (off-chain ledger)
AI Agent <- Receives verified result
```

## Layer Responsibilities

| Layer | Role | MVP | Future |
|---|---|---|---|
| Agent Interface | Order entry point | MCP Tool (forge_run) | A2A, SDK |
| Marketplace | Worker discovery and dispatch | Central Broker | P2P Discovery |
| Execution | Isolated execution | rootless Docker | Wasm |
| Verification | Result verification | Majority vote (3 nodes) | zk-proof / TEE |
| Settlement | Reward distribution | Off-chain ledger | Base / Solana |
| Reputation | Node trust score | off-chain DB | on-chain |

## Worker Node Schema

```json
{
  "node_id": "forge_node_jp_001",
  "cpu": "Ryzen 9 6900HS",
  "memory_gb": 16,
  "gpu_vram_gb": 0,
  "region": "JP",
  "power_cost_hint": "low|medium|high",
  "reputation": 0.95,
  "stake": 10.0
}
```
