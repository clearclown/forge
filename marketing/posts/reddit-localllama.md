# Reddit r/LocalLLaMA Post

**Title:** Your PC can earn compute credits for free while idle — I built an open-source protocol for it

**Body:**

I've been building Forge, an open-source protocol that adds an economy to local LLM inference. The idea: your idle hardware earns Compute Units (CU) by serving inference to the network, and you spend those CU to access larger models you can't run locally.

**How it works:**

```
$ forged node -m "qwen2.5:0.5b" --ledger forge-ledger.json

$ curl localhost:3000/v1/forge/balance
{"effective_balance": 1000, "reputation": 0.5}

$ curl localhost:3000/v1/chat/completions \
    -d '{"messages":[{"role":"user","content":"Hello"}]}'
{
  "choices": [{"message": {"content": "こんにちは！"}}],
  "x_forge": {"cu_cost": 9, "effective_balance": 1009}
}
```

Every response tells you the CU cost. The provider earned 9 CU. The consumer spent 9 CU. Real computation, real value.

**What makes this different from Bittensor/Render/etc:**
- No token, no blockchain, no crypto wallet needed
- Just HTTP API — works with any tool (LangChain, Ollama, curl)
- CU is backed by actual computation, not speculation
- Built on mesh-llm for distributed inference (pipeline parallelism, MoE sharding)

**The economics:**
- CU is deflationary: as the network grows, each CU buys more compute
- Early contributors earn CU when it's expensive → spend when it's cheap
- Same economics as early Bitcoin mining, but useful work

**Safety:**
- Kill switch: freeze all transactions in milliseconds
- Budget policies: per-agent spending limits
- Circuit breakers: auto-stop on anomalous patterns

**Stack:** ~10K lines of Rust, 84 tests, 2 security audits, MIT licensed.

GitHub: https://github.com/clearclown/forge
Whitepaper: https://github.com/clearclown/forge/blob/main/WHITEPAPER.md

Interested in feedback from the r/LocalLLaMA community. What would make you want to contribute your idle compute to a network like this?
