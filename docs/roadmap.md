# Forge — Roadmap

## Phase 1: Local Inference ✅

- `forge-core`: Type system (NodeId, LayerRange, ModelManifest, PeerCapability)
- `forge-infer`: llama.cpp engine, GGUF loader, streaming token generation
- `forge-node`: HTTP API (/chat, /chat/stream, /health)
- `forge-cli`: `forge chat` command with model auto-download

## Phase 2: P2P Protocol ✅

- `forge-net`: Iroh transport, Noise encryption, peer connections
- `forge-proto`: 14 wire protocol message types (bincode + length-prefix)
- `forge-node`: Seed/Worker pipeline, inference request/response
- Integration tests: 2 nodes exchange Hello + multiple messages

## Phase 3: Remote Inference + Operator Ledger ✅

- `forge-ledger`: CU accounting, trade execution, reputation, yield, market pricing
- `forge-node`: Ledger integrated into inference pipeline
- CU balance checks before inference
- Trade records after completion
- HMAC-SHA256 ledger integrity

## Phase 4: Economic API ✅

- OpenAI-compatible API: `POST /v1/chat/completions`, `GET /v1/models`
- CU metering: every inference records a trade with `x_forge` extension
- Agent budget endpoints: `GET /v1/forge/balance`, `GET /v1/forge/pricing`
- CU→Lightning settlement bridge: `forge settle --pay`
- Seed model auto-resolve from HF Hub
- Graceful Ctrl-C shutdown with ledger persistence

## Phase 5: mesh-llm Fork Integration (next)

**Goal:** Replace Forge's inference layer with mesh-llm's proven distributed engine.

| Deliverable | Description |
|---|---|
| Fork mesh-llm | Create forge as a mesh-llm fork with economic layer |
| Integrate forge-ledger | Hook CU recording into mesh-llm's inference pipeline |
| Preserve economic API | Keep /v1/forge/* endpoints in the new codebase |
| Web console extension | Add CU balance and trade visibility to mesh-llm's console |
| Pipeline + MoE | Inherit mesh-llm's pipeline parallelism and expert sharding |
| Nostr discovery | Inherit mesh-llm's public mesh discovery |
| CREDITS.md | Document mesh-llm attribution |

## Phase 6: Proof of Useful Work

**Goal:** Make CU claims verifiable across the network.

| Deliverable | Description |
|---|---|
| Dual-sign protocol | Both provider and consumer sign each TradeRecord |
| Gossip sync | Signed trades propagate across the mesh |
| Fraud detection | Reject unsigned or mismatched trades |
| Reputation gossip | Share reputation scores across peers |
| Collusion resistance | Statistical anomaly detection on trade patterns |

## Phase 7: External Bridges

**Goal:** Let operators convert CU to external value.

| Deliverable | Description |
|---|---|
| Lightning bridge | Automated CU→sats settlement via LDK |
| Stablecoin adapter | CU→USDC/USDT conversion |
| Fiat adapter interface | Spec for bank-transfer settlement |
| Exchange rate service | Public CU/BTC and CU/USD rate feeds |
| Bitcoin anchoring | Optional: periodic Merkle root → OP_RETURN for immutable audit trail |

## Phase 8: Agent Autonomous Economy

**Goal:** Let AI agents manage their own compute lifecycle.

| Deliverable | Description |
|---|---|
| Budget policies | Human-set spend limits per agent |
| Autonomous trading | Agent decides when to buy/sell compute |
| Multi-model routing | Agent chooses model based on cost/quality tradeoff |
| Self-reinforcement | Agent earns CU → buys bigger model access → earns more CU |
| Inter-agent economy | Agents trade specialized compute (code model vs chat model) |

## Long-term

| Milestone | Description |
|---|---|
| SDK release | forge-node as embeddable Rust library with stable API |
| Protocol v2 | Lessons from v1, backward-compatible evolution |
| Cross-architecture | NVIDIA GPU, AMD ROCm, RISC-V support (via mesh-llm) |
| Federated training | Distributed fine-tuning, not just inference |
| Compute derivatives | Forward contracts on future compute capacity |

> The protocol is the platform. The computation is the currency.
