# Forge Integration Pitch — For Framework Maintainers

## One-Line Pitch

"Let your agents pay for their own compute."

## The Problem You Have

Your agent framework runs on a single machine. When agents need more compute than available locally, they need a human to pay for cloud APIs. This creates a bottleneck:

```
Agent capability = min(local_hardware, human_budget)
```

## What Forge Adds

```
Agent capability = local_hardware + network_compute_earned
```

Agents earn Compute Units (CU) by serving inference when idle. They spend CU to access larger models when needed. No API keys, no subscriptions, no human in the loop.

## Integration Effort

### Option A: Zero effort (OpenAI-compatible)

```python
# Your existing code already works. Just point base_url to Forge.
llm = YourFramework(base_url="http://localhost:3000/v1", api_key="not-needed")
```

Every inference now has CU cost tracking in the `x_forge` response extension.

### Option B: Budget-aware agent (30 minutes)

```python
# pip install forge-sdk
from forge_sdk import ForgeClient

forge = ForgeClient()

# Before expensive operations
if forge.can_afford(estimated_tokens=500):
    result = forge.chat("complex task")
else:
    result = forge.chat("simpler fallback", max_tokens=50)
```

### Option C: Full autonomous agent (2 hours)

```python
from forge_sdk import ForgeAgent

agent = ForgeAgent(max_cu_per_task=1000)
while agent.has_budget():
    result = agent.think("next task")
```

## What Your Users Get

1. **Cost transparency**: every inference shows CU cost
2. **Budget management**: agents don't overspend
3. **Network compute**: access to models too large for local hardware
4. **Deflationary economics**: early-earned CU becomes more valuable
5. **Safety**: kill switch, circuit breakers, budget policies

## Comparison

| Without Forge | With Forge |
|---------------|------------|
| Agent limited to local GPU | Agent accesses network GPUs |
| Human pays per API call | Agent earns its own compute |
| Fixed model size | Model size grows with CU balance |
| No cost awareness | Every token has a price |
| No safety limits | Budget policies + kill switch |

## Links

- GitHub: https://github.com/clearclown/forge
- Python SDK: `pip install forge-sdk`
- MCP Server: `pip install forge-mcp`
- Whitepaper: https://github.com/clearclown/forge/blob/main/WHITEPAPER.md
- Agent Guide: https://github.com/clearclown/forge/blob/main/docs/agent-integration.md
