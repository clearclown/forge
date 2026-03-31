# AGENTS.md — Forge

## Project

Forge is a P2P compute market protocol for AI agents.

**Tagline:** "Compute bought by AI, not humans."

## Architecture Overview

- Upper layer: MCP Tool (`forge_run`) — agent-facing API
- Lower layer: Broker + Worker nodes + Verification + Settlement

## Key Decisions

- MVP execution environment: rootless Docker (NOT Wasm yet)
- MVP settlement: off-chain ledger (NO blockchain in MVP)
- MVP verification: majority vote (3 nodes, 2 must agree)
- First task type: pytest / CPU batch
- First buyer: Claude Code and similar autonomous dev agents

## What NOT to do

- Do NOT add blockchain to MVP
- Do NOT add zk-proof to MVP (Phase 3 only)
- Do NOT use Wasm in MVP (Docker first)
- Do NOT add GPU support in MVP (CPU only)

## Docs

- `docs/concept.md` — Why Forge exists
- `docs/architecture.md` — Technical design
- `docs/mcp-spec.md` — forge_run spec
- `docs/roadmap.md` — Phase plan
