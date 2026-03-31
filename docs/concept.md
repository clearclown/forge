# Forge — Concept & Problem Statement

## The Problem

Autonomous AI agents (Claude Code, OpenClaw, etc.) cannot procure external compute resources without human intervention. When an agent runs out of local compute capacity, a human must manually set up cloud infrastructure and handle payments.

This "human in the loop" fundamentally limits agent autonomy.

Meanwhile, personal PCs and workstations worldwide sit idle ~90% of the time, consuming power without contributing value.

## Why P2P?

> Long-tail supply (idle home PCs) × autonomous micro-demand (AI agents) = a combination that does not fit centralized cloud models.
> P2P compute markets are the best way to connect this supply and demand.

## Why Not Existing Solutions?

| Project | Focus | Gap |
|---|---|---|
| Golem | Humans renting cheap compute | Buyer is not an AI |
| Akash | Decentralized IaaS | Designed for human operators |
| io.net | GPU supply optimization | Demand side (AI autonomous buying) unaddressed |
| Render | GPU rendering | Domain-specific, not a general protocol |
| Bittensor | Decentralized AI inference | Not an autonomous procurement layer for external agents |

**In one sentence:** Existing solutions optimize the supply side. Forge autonomizes the demand side (AI).

## The Forge Vision

> Forge is the infrastructure for the era when AI buys compute — not humans.

When Forge reaches its long-term vision:
- AI agents autonomously detect compute bottlenecks
- Call `forge_run` with task, budget, and deadline
- Forge dispatches to verified worker nodes worldwide
- Results are verified by consensus before delivery
- Payment happens automatically
- No human needed at any step
