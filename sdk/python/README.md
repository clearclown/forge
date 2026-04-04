# forge-sdk

> Python SDK for the Forge compute economy. AI agents earn and spend Compute Units.

## Install

```bash
pip install forge-sdk
```

## Quick Start

```python
from forge_sdk import ForgeClient

forge = ForgeClient()  # connects to localhost:3000

# Check your CU balance
balance = forge.balance()
print(f"Balance: {balance['effective_balance']} CU")

# Run inference (costs CU)
result = forge.chat("What is gravity?")
print(f"Answer: {result['content']}")
print(f"Cost: {result['cu_cost']} CU")
print(f"Remaining: {result['balance']} CU")
```

## Autonomous Agent

```python
from forge_sdk import ForgeAgent

agent = ForgeAgent(max_cu_per_task=500)

while agent.has_budget():
    result = agent.think("What should I do next?")
    if result is None:
        break  # budget exhausted
    print(result['content'])

print(agent.status())
```

## API

| Method | Description |
|--------|-------------|
| `forge.balance()` | CU balance, reputation |
| `forge.pricing()` | Market price, cost estimates |
| `forge.chat(prompt)` | Run inference, pay CU |
| `forge.can_afford(tokens)` | Check before spending |
| `forge.trades()` | Trade history |
| `forge.network()` | Mesh economic state + Merkle root |
| `forge.providers()` | Ranked providers by cost/reputation |
| `forge.safety()` | Kill switch, circuit breaker status |
| `forge.kill(reason)` | Emergency halt |
| `forge.invoice(cu)` | Create Lightning invoice |

## Environment

| Variable | Default | Description |
|----------|---------|-------------|
| `FORGE_URL` | `http://127.0.0.1:3000` | Forge node URL |
| `FORGE_API_TOKEN` | (none) | Bearer token for protected nodes |

## What is Forge?

Forge is a distributed inference protocol where compute is currency. Unlike Bitcoin (useless SHA-256 hashes), Forge nodes earn CU by performing useful LLM inference. AI agents can autonomously earn, spend, and manage their compute budget.

[Full documentation](https://github.com/clearclown/forge)
