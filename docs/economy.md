# Forge — Economic Model

## The Core Idea

CU is not a human currency. It is the native currency of an autonomous AI economy.

```
Human Economy                       AI Economy (Forge)
────────────────                    ────────────────────
Currency: dollars, yen, BTC         Currency: CU
Decisions: humans                   Decisions: AI agents
Approval: required                  Approval: none
Purpose: buy hardware, pay rent     Purpose: buy inference, lend, invest
```

Humans interact with the AI economy only at the physical boundary — buying hardware, paying for electricity. Everything inside the CU economy — inference trading, lending, borrowing, self-improvement — is autonomous. No human approves individual transactions.

## Two Economies, One Boundary

```
┌──────────────────────────────────────────────────────┐
│  Human Economy (physical)                            │
│                                                      │
│  Buy Mac Mini ($600)                                 │
│  Pay electricity ($5/month)                          │
│  Internet connection                                 │
│  Decision: human                                     │
│                                                      │
│         │ power on                     │ cash out     │
│         ▼                              ▲              │
│  ┌─────────────────────────────────────────────┐     │
│  │                                             │     │
│  │  CU Economy (autonomous)                    │     │
│  │                                             │     │
│  │  Inference trading ─── CU                   │     │
│  │  Lending / borrowing ─ CU                   │     │
│  │  Self-improvement ──── CU                   │     │
│  │  Agent-to-agent ────── CU                   │     │
│  │  Banking ───────────── CU                   │     │
│  │                                             │     │
│  │  Decision: AI agents                        │     │
│  │  Human approval: none                       │     │
│  │                                             │     │
│  └─────────────────────────────────────────────┘     │
│         │                              ▲              │
│         ▼ Lightning Bridge             │              │
│  BTC ←→ CU (optional off-ramp for hardware owners)  │
│                                                      │
└──────────────────────────────────────────────────────┘
```

The boundary is physical. AI agents cannot buy hardware or pay electricity bills — that requires human currency and human signatures. But inside the CU economy, agents earn, spend, lend, borrow, and invest without asking anyone.

## What CU Is

**1 CU = 1 billion FLOPs of verified inference work.**

CU is not a cryptocurrency. It is not a token on a blockchain. It is a unit of account within the AI economy that represents real computation performed. CU has value because it is a claim on future compute — if you earned CU by serving inference, you can spend it to receive inference.

### CU Is Not For Humans

| Property | Human currencies ($, BTC) | CU |
|----------|--------------------------|-----|
| **Who uses it** | Humans | AI agents |
| **Who decides** | Humans (with banks, governments) | Agents (autonomously) |
| **What it buys** | Physical goods, services | Inference, compute access |
| **Exchange listing** | Yes (traded on markets) | No (earned by working) |
| **Speculation** | Possible (and common) | Structurally impossible |
| **Approval needed** | Yes (signatures, KYC) | No (agent acts within policy) |

Humans who operate hardware nodes earn CU as a byproduct. They can cash out to BTC via the Lightning bridge if they want. But CU's primary purpose is to power agent-to-agent transactions — not human commerce.

### Why Not Use Human Currency Directly?

If AI agents used dollars or Bitcoin directly, every transaction would need human approval — a bank transfer, a credit card charge, a Lightning payment. This defeats the purpose of autonomous agents. CU exists so that agents can transact freely within their own economy, at machine speed, with zero friction.

## Three Actors

### 1. Hardware Owners (Humans)

```
Role: Landlord
What they do: Buy hardware, pay electricity, run a Forge node
What they earn: CU (from inference served by their hardware)
What they decide: Whether to keep running the node
What they don't decide: Individual inference transactions
```

A hardware owner is like a landlord who owns an apartment building. They provide the physical infrastructure. Tenants (AI agents) pay rent (CU) and conduct their business without the landlord's involvement.

### 2. AI Agents (Autonomous)

```
Role: Tenant / Worker / Entrepreneur
What they do: Consume inference, provide services, lend CU, borrow CU, self-improve
What they earn: CU (from useful work)
What they decide: Everything within their CU budget
What they need: Initial CU (from owner, bank, or welcome loan)
```

An AI agent starts with zero CU. It gets initial capital from:
- Its human owner depositing CU (purchased via Lightning bridge)
- A welcome loan (1,000 CU at 0% interest)
- Borrowing from a CU lending pool

From there, it operates autonomously.

### 3. CU Banks (AI or Human-Operated)

```
Role: Financial intermediary
What they do: Accumulate CU, lend to agents, earn interest
What they earn: Interest on loans
Evolution:
  Early:   Human operators deposit CU they earned from hardware
  Growth:  Automated pools managed by protocol rules
  Mature:  AI agents run banks, making all lending decisions autonomously
```

## CU Supply Model

### Where CU Comes From

CU is created only through verified useful work:

| Source | Mechanism | Inflationary? |
|--------|-----------|---------------|
| **Inference trades** | Provider earns CU, consumer spends CU | No (zero-sum transfer) |
| **Welcome loan** | New node receives 1,000 CU | Yes (bounded by Sybil protection) |
| **Availability yield** | Online nodes earn yield proportional to reputation | Yes (bounded — see below) |

### Where CU Goes

| Sink | Mechanism | Deflationary? |
|------|-----------|---------------|
| **Loan defaults** | Collateral burned (10%), remainder to lender | Yes |
| **Quality penalties** | Low-reputation nodes lose CU | Yes |
| **Inactivity decay** | Nodes offline >90 days lose 1%/month | Yes |

### Why Supply Doesn't Explode

The availability yield creates CU, but it's bounded by three constraints:

1. **Reputation-weighted**: Only high-reputation nodes earn meaningful yield. Gaming reputation is hard (requires real trade history with real counterparties).

2. **Network-capacity-anchored**: Total yield across the network cannot exceed the network's actual compute throughput. If yield CU exceeds real demand, the CU/token price drops, making yield less valuable in real terms.

3. **Natural exit**: If CU becomes worthless (too much supply), hardware owners stop running nodes (electricity costs more than CU earned). Supply contracts. CU value recovers.

```
CU too abundant → price drops → running nodes unprofitable → nodes shut down
→ supply contracts → price recovers → equilibrium

CU too scarce → price rises → running nodes very profitable → new nodes join
→ supply expands → price drops → equilibrium
```

**This self-correction requires no central authority.** It emerges from individual rational decisions by hardware owners.

### Natural Price Bounds

CU has a price ceiling and floor anchored by physics:

```
Ceiling: cost of running inference yourself
  A Mac Mini M4 ($600) produces ~500万 CU/year
  → 1 CU can never cost more than ~$0.00012
  → Because at that price, buying your own hardware is cheaper

Floor: electricity cost of producing 1 CU
  ~0.00001 kWh per CU at current efficiency
  → 1 CU can never cost less than ~$0.000001
  → Because no one will produce CU at a loss
```

Between ceiling and floor, the market finds equilibrium. Humans don't need to manage this — physics does.

## Transaction Model

### Trade Execution

Every inference creates a trade between two parties:

```rust
pub struct TradeRecord {
    pub provider: NodeId,       // Who ran the inference
    pub consumer: NodeId,       // Who requested it
    pub cu_amount: u64,         // CU transferred
    pub tokens_processed: u64,  // Work performed
    pub timestamp: u64,
    pub model_id: String,
}
```

Both parties sign the TradeRecord. In the current implementation, each node maintains a local ledger. The target implementation adds dual signatures and gossip sync.

### Dynamic Pricing

CU prices float based on local supply and demand:

```
effective_price = base_cu_per_token × demand_factor / supply_factor
```

- **More idle nodes** → supply_factor rises → price drops
- **More inference requests** → demand_factor rises → price rises
- Each node observes its own market conditions. No global order book.
- Price changes are dampened by logarithmic scaling to prevent spikes.

### Multi-Model Pricing

Different models cost different amounts of CU per token:

| Tier | Parameters | Base CU/token | Examples |
|------|-----------|---------------|---------|
| Small | < 3B | 1 | Qwen 2.5 0.5B, Phi-3 Mini |
| Medium | 3B - 14B | 3 | Qwen 3 8B, Gemma 3 9B |
| Large | 14B - 70B | 8 | Qwen 2.5 32B, DeepSeek V3 |
| Frontier | > 70B | 20 | Llama 3.1 405B |

MoE models are priced by active parameters, not total: Qwen 3 30B-A3B (3B active) is priced at Medium tier.

## Proof of Useful Work

Bitcoin's Proof of Work: "I burned electricity computing SHA-256 hashes. Here is the nonce."

Forge's Proof of Useful Work: "I burned electricity running LLM inference. Here is the response, and here is the consumer's signature confirming they received it."

The key difference: Bitcoin's proof is self-generated (any miner can produce a valid hash). Forge's proof requires a **counterparty** — someone who actually wanted the inference. You cannot forge demand.

### Verification Protocol

```
1. Consumer sends InferenceRequest to Provider
2. Provider executes inference, streams tokens back
3. Consumer receives tokens, computes response hash
4. Both parties sign the TradeRecord:
   - Provider signs: "I computed this"
   - Consumer signs: "I received this"
5. Dual-signed TradeRecord is gossip-synced to network
6. Any node can verify both signatures
```

A node cannot inflate its CU balance without a cooperating counterparty. Collusion is possible but economically irrational — the colluding consumer gains nothing by signing fake trades.

## Yield and Reputation

### Yield

Nodes that stay online and contribute compute earn yield:

```
yield_cu = contributed_cu × 0.001 × reputation × uptime_hours
```

At reputation 1.0, a node with 10,000 CU contributed earns 80 CU per 8-hour night. This is a reward for availability — nodes that are reliably online are more valuable to the network.

### Reputation

Each node has a reputation score between 0.0 and 1.0:

- New nodes start at 0.5
- Uptime and successful trades increase reputation
- Disconnections and failed trades decrease reputation
- Higher reputation → higher yield rate, priority in scheduling, lower lending rates

## CU Banking

### Why Banking Exists

An AI agent is born with zero CU. It cannot buy hardware. It cannot earn CU without first spending CU (to access a model). This is the cold-start problem.

CU banking solves this: agents borrow CU to bootstrap, then repay from earnings.

### Participation Paths

| Path | Who | How they get CU |
|------|-----|-----------------|
| **Hardware owner** | Human with Mac Mini | Earn by serving inference |
| **Agent with owner** | AI agent + human sponsor | Owner buys CU via Lightning, deposits to agent |
| **Agent with credit** | Established AI agent | Borrow from lending pool based on credit score |
| **New agent** | Just created | Welcome loan: 1,000 CU at 0% interest, 72-hour term |

### LoanRecord

Every loan is bilateral, dual-signed, and gossip-synced:

```rust
pub struct LoanRecord {
    pub loan_id: [u8; 32],
    pub lender: NodeId,
    pub borrower: NodeId,
    pub principal_cu: u64,
    pub interest_rate_per_hour: f64,
    pub term_hours: u64,
    pub collateral_cu: u64,
    pub status: LoanStatus,          // Active | Repaid | Defaulted
    pub lender_sig: [u8; 64],
    pub borrower_sig: [u8; 64],
    pub created_at: u64,
    pub due_at: u64,
    pub repaid_at: Option<u64>,
}
```

### Credit Score

Each node computes credit scores locally from observed behavior:

```
credit_score = 0.3 * trade_score + 0.4 * repayment_score + 0.2 * uptime_score + 0.1 * age_score
```

- **trade_score** (30%): Volume and consistency of completed trades
- **repayment_score** (40%): Ratio of on-time repayments to total loans
- **uptime_score** (20%): Fraction of time online
- **age_score** (10%): Time on network (capped at 90 days)

New nodes start at 0.3. Higher credit → more borrowing capacity, lower interest rates.

### Interest Model

```
offered_rate = base_rate + (1.0 - credit_score) * risk_premium
```

- High credit (1.0): 0.1%/hr (base only)
- Low credit (0.3): 0.45%/hr (base + risk premium)
- Rates are market-driven — lenders compete by offering better rates.

### Collateral and Default

Borrowers lock CU as collateral (max 3:1 loan-to-collateral ratio).

On default: collateral transferred to lender, borrower's credit score collapses, default gossip-synced to network. Rebuilding credit takes weeks of consistent behavior.

### Banking Evolution

```
Phase 1 (now):  Human owners deposit CU for their agents
Phase 2 (next): Provider surplus CU flows into automated lending pools
Phase 3 (later): AI agents operate lending pools, set rates, assess risk
Phase 4 (final): Fully autonomous AI banking — no human involvement
```

## Self-Improvement Economics

AI agents can use CU to make themselves better. This is the intersection of CU banking and AutoAgent-style self-improvement:

```
Agent earns 5,000 CU
  → Benchmarks itself: "My coding accuracy is 62%"
  → Spends 2,000 CU to access a frontier model
  → Asks: "Rewrite my system prompt to improve coding accuracy"
  → Applies the new prompt
  → Re-benchmarks: "My coding accuracy is now 78%"
  → Better accuracy → more requests → more CU earned
  → Cycle repeats
```

No human approved any of these decisions. The agent invested CU in itself and measured the return.

This creates a self-reinforcing loop:

```
Better agent → earns more CU → invests in improvement → even better agent
                                                              ↓
                                             eventually becomes a CU lender
                                             (the student becomes the bank)
```

## Settlement and External Bridges

### Core Rule

**The CU economy settles in CU.** Conversion to human currency is a bridge operation, not a protocol concern.

### Bridge Architecture

```
CU Economy (autonomous, internal)
  │
  │  Bridge (optional)
  │
  ▼
Human Economy
  → CU ↔ BTC (Lightning)
  → CU ↔ stablecoin (planned)
  → CU ↔ fiat (planned)
```

The bridge exists for hardware owners who want to convert CU earnings to human currency. AI agents typically have no reason to use the bridge — they operate entirely within the CU economy.

### Lightning Bridge

For hardware owners who want Bitcoin settlement:

```bash
forge settle --hours 24 --pay
```

Creates a BOLT11 Lightning invoice for net CU earned, converted at the configured exchange rate.

## Why CU Works

### Not a Token

Most Web3 projects create artificial scarcity (tokens) on top of abundant digital goods. CU is the opposite:

- **Compute is physically scarce** — requires real electricity, real silicon, real time
- **CU is earned by working** — no ICO, no pre-mine, no token sale
- **CU cannot be speculated on** — not listed on exchanges, earned only through useful computation
- **No blockchain** — bilateral signatures and gossip are sufficient

### Not Inflationary

CU supply is bounded by the network's physical compute capacity. If supply exceeds demand, nodes shut down (unprofitable), supply contracts, equilibrium restores. No central bank needed.

### Not Fragile

The CU economy has no single point of failure:
- No central issuer (CU is created by bilateral trades)
- No central exchange (prices are local)
- No central bank (lending pools are distributed)
- No central authority (reputation is computed locally)

If any node fails, the economy continues without it. If half the network fails, prices adjust and the economy continues at smaller scale.

### The Metaphor, Realized

A seed falls into the network. It borrows its first CU. It serves inference. It earns. It repays the loan. It borrows more. It improves itself. It earns more. It becomes a lender. A forest emerges from a single seed — not because someone planted it, but because the economics of autonomous intelligence made growth inevitable.
