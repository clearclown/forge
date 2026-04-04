# Forge — Concept & Vision

## The Problem Is Not Distributed Inference

Projects like [mesh-llm](https://github.com/michaelneale/mesh-llm), Petals, and Exo have shown that you can split LLM inference across multiple devices over a network. The hard engineering of pipeline parallelism, expert sharding, and mesh coordination is largely solved.

The unsolved problem is: **why would anyone contribute their hardware?**

mesh-llm pools GPUs beautifully — but if you run your Mac Mini as a mesh node for a year, you get nothing. No record of contribution, no priority access, no economic return. The network runs on goodwill. Goodwill doesn't scale.

## The Insight: Compute Is Money

Every monetary system is backed by scarcity. Gold is scarce because geology. Oil is scarce because extraction costs energy. Bitcoin is scarce because mining burns electricity on SHA-256 hashes.

But Bitcoin's scarcity is artificial — the computation is purposeless. The hashes secure the ledger but produce nothing useful.

LLM inference is different. When a Forge node spends electricity to answer someone's question, that computation has **intrinsic value**. Someone wanted that answer badly enough to request it. The electricity was not wasted — it produced intelligence.

```
Bitcoin:   electricity → useless hashing → artificial scarcity → value
Forge:     electricity → useful inference → real utility → value
```

This is the **Compute Standard (計算本位制)**: a monetary system where the unit of value is backed by verified useful computation.

## What Forge Is

Forge is mesh-llm with an economy.

The inference layer (networking, model distribution, API) comes from mesh-llm. Forge adds:

1. **CU Ledger** — Every inference creates a trade. Provider earns CU, consumer spends CU. Dual-signed by both parties.
2. **Dynamic Pricing** — CU per token floats with local supply and demand. More idle nodes → cheaper. More requests → more expensive.
3. **Proof of Useful Work** — CU is earned by performing real inference, not by solving arbitrary puzzles.
4. **Agent Budget API** — AI agents can query their balance, estimate costs, and make autonomous spending decisions.
5. **External Bridges** — CU can optionally be exchanged for Bitcoin (Lightning), stablecoins, or fiat through adapter layers outside the protocol.

## Why Not Just Use Bitcoin?

We considered making Bitcoin/Lightning the primary settlement layer. We decided against it.

| Concern | Explanation |
|---------|-------------|
| **Philosophical inconsistency** | Rewarding useful work in a currency backed by useless work |
| **External dependency** | If Bitcoin's security breaks (quantum computing, regulatory), Forge's economy breaks too |
| **Efficiency** | Lightning channel management is overhead for per-inference micropayments |
| **Self-sufficiency** | CU has value because the computation itself is useful — it doesn't need external validation |

Bitcoin remains available as an **off-ramp** for operators who need external liquidity. But the protocol's native economy runs on CU.

## Why CU Has Value

CU is not a speculative token. It is a **claim on future compute**.

If you earned 10,000 CU by serving inference, you can spend those CU to buy inference from any other node on the network. The value is not abstract — it is the ability to make a machine think for you.

This makes CU a **productive asset**, not a store of value:

```
Apartment building          Mac Mini on Forge
───────────────────         ──────────────────
Asset: building             Asset: compute hardware
Cost: maintenance           Cost: electricity
Revenue: rent               Revenue: CU from inference
Yield: rent - maintenance   Yield: CU earned - electricity
Idle = lost income          Idle = wasted potential
```

Unlike Bitcoin (digital gold — holds value but produces nothing), CU is like a rental property — it generates yield by performing useful work.

## AI Agents as Economic Actors

The most important consumer of Forge's economy is not humans — it's AI agents.

An agent running a small local model (1.5B parameters on a phone) has limited intelligence. But if it can earn CU by lending idle compute and spend CU to access larger models, it can autonomously expand its own capabilities:

```
Small agent (phone, 1.5B)
  → idle overnight → lends CPU → earns CU
  → morning: needs complex reasoning
  → checks /v1/forge/balance → has 5,000 CU
  → checks /v1/forge/pricing → 70B model costs 2,000 CU for 500 tokens
  → buys 70B inference → gets smarter answer
  → uses answer to make better trading decisions
  → earns more CU next cycle
```

This is the self-reinforcement loop: agents that make good economic decisions grow stronger, which lets them make even better decisions.

No human needs to approve individual transactions. The agent operates within a budget policy set by its owner. The protocol provides the market; the agent provides the strategy.

## Comparison

| Project | Inference | Economy | Agent Autonomy |
|---------|-----------|---------|----------------|
| **mesh-llm** | Distributed (pipeline + MoE) | None | Blackboard messaging only |
| **Petals** | Distributed (collaborative) | None | None |
| **Ollama** | Local only | None | None |
| **Together AI** | Centralized | Pay-per-token (corporate) | API access only |
| **Bitcoin** | N/A | PoW (useless work) | None |
| **Golem** | Batch compute | GNT token | Human-directed |
| **Forge** | Distributed (mesh-llm) | **CU (useful work)** | **Autonomous budget management** |

## The Metaphor

A seed falls into the network. It earns its first CU by lending idle cycles overnight. With those CU, it buys access to a larger model. It becomes smarter. It finds more efficient trades. More CU. A bigger model. A forest emerges from a single seed — not because someone planted it, but because the economics made growth inevitable.
