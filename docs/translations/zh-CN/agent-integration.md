# Forge — 代理集成指南

## 针对 AI 代理开发者

Forge 为您的代理提供计算预算。代理可以通过提供推理赚取 CU，并花费 CU 来访问更大的模型。无需信用卡，无需 API 密钥，无需人工参与。

## 快速集成

### 任何 HTTP 客户端

```python
import requests

FORGE = "http://127.0.0.1:3000"

# 检查代理是否支付得起请求
balance = requests.get(f"{FORGE}/v1/forge/balance").json()
if balance["effective_balance"] > 100:
    # 执行推理（消耗 CU）
    r = requests.post(f"{FORGE}/v1/chat/completions", json={
        "messages": [{"role": "user", "content": "什么是重力？"}],
        "max_tokens": 256
    }).json()
    print(r["choices"][0]["message"]["content"])
    print(f"成本: {r['x_forge']['cu_cost']} CU")
```

### Python SDK

```python
from forge_sdk import ForgeClient, ForgeAgent

# 简单客户端
forge = ForgeClient()
result = forge.chat("解释量子计算")
print(f"回答: {result['content']}")
print(f"成本: {result['cu_cost']} CU, 余额: {result['balance']} CU")

# 具有预算管理功能的自主代理
agent = ForgeAgent(max_cu_per_task=500)
while agent.has_budget():
    result = agent.think("我下一步应该做什么？")
    if result is None:
        break  # 预算耗尽
```

### MCP (Claude Code, Cursor)

添加到您的 MCP 设置中：
```json
{
  "mcpServers": {
    "forge": {
      "command": "python",
      "args": ["path/to/forge/mcp/forge-mcp-server.py"]
    }
  }
}
```

AI 助手随后可以使用 `forge_balance`、`forge_pricing`、`forge_inference` 等工具。

### LangChain

```python
from langchain_openai import ChatOpenAI

llm = ChatOpenAI(
    base_url="http://127.0.0.1:3000/v1",
    api_key="not-needed",
    model="qwen2.5-0.5b-instruct-q4_k_m"
)
response = llm.invoke("你好")
# 响应头中提供 x_forge 元数据
```

### curl

```bash
# 检查余额
curl localhost:3000/v1/forge/balance

# 执行推理
curl localhost:3000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"messages":[{"role":"user","content":"你好"}]}'

# 检查成本
curl localhost:3000/v1/forge/trades
```

## 代理经济循环

自主代理的推荐模式：

```python
from forge_sdk import ForgeClient

forge = ForgeClient()

def agent_loop():
    while True:
        # 1. 检查预算
        balance = forge.balance()
        if balance["effective_balance"] < 50:
            print("CU 余额不足。等待赚取更多...")
            time.sleep(60)
            continue

        # 2. 检查价格
        pricing = forge.pricing()
        cost_per_100 = pricing["estimated_cost_100_tokens"]

        # 3. 决定任务是否值得该成本
        if cost_per_100 > 500:
            print("市场价格太高。等待中...")
            time.sleep(30)
            continue

        # 4. 执行
        result = forge.chat("分析这些数据...", max_tokens=200)
        print(f"完成。成本: {result['cu_cost']} CU")

        # 5. 检查安全
        safety = forge.safety()
        if safety["circuit_tripped"]:
            print("断路器跳闸。暂停中...")
            time.sleep(300)
```

## 代理开发者的安全性

### 设置预算政策

```bash
# 限制代理每小时支出 1000 CU
curl -X POST localhost:3000/v1/forge/policy \
  -H "Content-Type: application/json" \
  -d '{
    "node_id": "<agent_node_id>",
    "max_cu_per_hour": 1000,
    "max_cu_per_request": 100,
    "human_approval_threshold": 500
  }'
```

### 监控支出速率

```bash
curl localhost:3000/v1/forge/safety
# 返回: hourly_spend, lifetime_spend, spends_last_minute
```

### 紧急停止

```bash
# 冻结一切
curl -X POST localhost:3000/v1/forge/kill \
  -d '{"activate": true, "reason": "agent anomaly"}'
```

## 代理 API 参考

| 代理需求 | 端点 | 方法 |
|-----------------|----------|--------|
| “我有多少 CU？” | `/v1/forge/balance` | GET |
| “这需要多少钱？” | `/v1/forge/pricing` | GET |
| “谁是最便宜的提供者？” | `/v1/forge/providers` | GET |
| “执行推理” | `/v1/chat/completions` | POST |
| “我花了什么？” | `/v1/forge/trades` | GET |
| “我安全吗？” | `/v1/forge/safety` | GET |
| “提现到比特币” | `/v1/forge/invoice` | POST |
| “停止一切” | `/v1/forge/kill` | POST |
