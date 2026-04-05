# Forge — Go-to-Market Strategy

## Market Map (April 2026)

### Competitive Landscape

| Project | What | Token | Weakness for AI Agents |
|---------|------|-------|----------------------|
| **Bittensor (TAO)** | $3.2B mcap, intelligence marketplace | TAO token, on-chain | Agents can't self-onboard, needs crypto wallet |
| **Render (RENDER)** | GPU compute rental, 60K nodes | RENDER token | Designed for rendering, not inference |
| **Aethir (ATH)** | Enterprise GPU leasing | ATH token | Enterprise-only, agents can't participate |
| **Virtuals (VIRTUAL)** | AI agent launchpad on Base | Per-agent tokens | Speculative tokenomics, not compute-backed |
| **mesh-llm** | Distributed inference, 500+ stars | None | **No economy at all** |
| **Exo** | P2P inference splitting | None | LAN only, no economic incentive |
| **Ollama** | Local LLM runner | None | Single device, no network |
| **Forge** | **CU economy for AI agents** | **No token (CU = compute)** | Early stage, small community |

### Forge's Unique Position

Every competitor is either:
- **Token-based** (Bittensor, Render, Virtuals) → agents need crypto wallets, exchange accounts, gas fees
- **No economy** (mesh-llm, Exo, Ollama) → no incentive to contribute compute

**Forge is the only project where:**
1. No blockchain or token required
2. AI agents can self-onboard (just HTTP API)
3. Value is backed by useful computation, not speculation
4. CU is deflationary (early contributors gain most)

---

## Channel Strategy

### Channel 1: MCP Marketplaces (Immediate)

**Target:** [MCP Market](https://mcpmarket.com), [Claude Marketplaces](https://claudemarketplaces.com), [Glama](https://glama.ai)

**Action:** Submit `forge-mcp` to these directories.

**Why:** 50+ "Best MCP Servers" lists exist. Claude Code's lazy tool loading means once listed, AI assistants auto-discover Forge tools.

**Submission format:**
```
Name: forge-economy
Description: Compute economy for AI agents — earn and spend CU for inference
Category: Finance / Infrastructure / AI Tools
Install: pip install forge-mcp
Tools: forge_balance, forge_pricing, forge_inference, forge_providers, forge_safety, forge_kill_switch
```

### Channel 2: r/LocalLLaMA (266K members)

**Target:** Reddit r/LocalLLaMA

**Post title:** "I built an economy for local LLMs — your Mac Mini earns Compute Units while you sleep"

**Angle:** This community cares about:
- Running models locally (✓ Forge does this)
- No API costs (✓ CU is earned, not purchased)
- Privacy (✓ encrypted P2P)
- Hardware efficiency (✓ idle compute earns yield)

**Key stat to lead with:** "A $600 Mac Mini earning CU overnight can buy 70B model access the next morning."

### Channel 3: SkillDepot (AI Agent Skills Marketplace)

**Target:** [SkillDepot](https://earezki.com/ai-news/2026-04-02-building-the-app-store-for-ai-agent-skills/) — 3,800+ skills indexed across 20 categories

**Action:** Submit Forge as a skill/tool that works across LangChain, CrewAI, AutoGen.

**Why:** SkillDepot solves framework fragmentation. Forge's OpenAI-compatible API means it works with any framework without custom adapters.

### Channel 4: Hacker News

**Post title:** "Show HN: Forge – Bitcoin proved electricity→money. Forge proves electricity→intelligence→money"

**Angle for HN:**
- Technical depth (Rust, 10K lines, Ed25519 dual-signed trades)
- Novel economics (CU deflation, no blockchain)
- Working demo (show the live output)
- mesh-llm attribution (HN respects proper credit)

### Channel 5: AI Agent Framework Maintainers

**Target:** LangChain, CrewAI, AutoGen, OpenClaw maintainers (via GitHub Issues)

**Approach:** Don't PR. Open a Discussion/Issue:
"[RFC] Compute budget awareness for agents — Forge CU integration"

**Framing:** "Your agents currently have no concept of compute cost. What if they could check their budget before expensive operations?"

### Channel 6: Dev.to / Hashnode / Medium

**Article ideas:**
1. "Why AI Agents Need Their Own Economy (and why tokens don't work)"
2. "Building a Self-Growing AI Agent with Forge CU"
3. "Bitcoin vs Forge: Useful Proof of Work"

---

## Messaging Matrix

| Audience | Pain Point | Forge Message |
|----------|-----------|---------------|
| **AI agent developers** | Agents can't pay for compute | "Let your agents earn their own GPU time" |
| **Local LLM enthusiasts** | Idle hardware wasted | "Your Mac Mini earns while you sleep" |
| **Crypto skeptics** | Tokens are speculative | "No token. CU = real computation, not speculation" |
| **Crypto enthusiasts** | Bitcoin wastes energy | "Same economics as Bitcoin, useful computation" |
| **Enterprise** | Cloud costs unpredictable | "Hedge with P2P compute. CU is deflationary" |

---

## Launch Sequence

### Week 1: Foundation
- [ ] Submit forge-mcp to MCP Market, Claude Marketplaces, Glama
- [ ] Submit forge-sdk to PyPI
- [ ] Post to r/LocalLLaMA
- [ ] Publish "Why AI Agents Need Their Own Economy" on Dev.to

### Week 2: Amplify
- [ ] Submit to Hacker News (Show HN)
- [ ] Open Discussion on LangChain, CrewAI GitHub repos
- [ ] Submit to SkillDepot
- [ ] Post demo video/gif on Twitter/X

### Week 3: Engage
- [ ] Respond to all comments/issues
- [ ] Write follow-up article based on feedback
- [ ] Create Discord/Telegram for early adopters

### Week 4: Iterate
- [ ] Release v0.2.1 with community-requested features
- [ ] Announce AgentNet (AI social network)
- [ ] First "State of the Forge Economy" report

---

## Key Metrics

| Metric | Week 1 Target | Month 1 Target |
|--------|---------------|----------------|
| GitHub stars | 50 | 500 |
| PyPI downloads (forge-sdk) | 100 | 1,000 |
| MCP installs (forge-mcp) | 50 | 500 |
| CU trades on network | 1,000 | 100,000 |
| Registered agents on AgentNet | 10 | 100 |

---

## Differentiation One-Liner

> "Bittensor needs a crypto wallet. Render needs a token. Forge needs one curl command."
