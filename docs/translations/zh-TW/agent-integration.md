# Forge — 代理集成指南

## 針對 AI 代理開發者

Forge 為您的代理提供計算預算。代理可以透過提供推理賺取 CU，並花費 CU 來訪問更大的模型。無需信用卡，無需 API 金鑰，無需人工參與。

## 快速集成

### 任何 HTTP 客戶端

```python
import requests

FORGE = "http://127.0.0.1:3000"

# 檢查代理是否支付得起請求
balance = requests.get(f"{FORGE}/v1/forge/balance").json()
if balance["effective_balance"] > 100:
    # 執行推理（消耗 CU）
    r = requests.post(f"{FORGE}/v1/chat/completions", json={
        "messages": [{"role": "user", "content": "什麼是重力？"}],
        "max_tokens": 256
    }).json()
    print(r["choices"][0]["message"]["content"])
    print(f"成本: {r['x_forge']['cu_cost']} CU")
```

### Python SDK

```python
from forge_sdk import ForgeClient, ForgeAgent

# 簡單客戶端
forge = ForgeClient()
result = forge.chat("解釋量子計算")
print(f"回答: {result['content']}")
print(f"成本: {result['cu_cost']} CU, 餘額: {result['balance']} CU")

# 具有預算管理功能的自主代理
agent = ForgeAgent(max_cu_per_task=500)
while agent.has_budget():
    result = agent.think("我下一步應該做什麼？")
    if result is None:
        break  # 預算耗盡
```

### MCP (Claude Code, Cursor)

添加到您的 MCP 設置中：
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

AI 助手隨後可以使用 `forge_balance`、`forge_pricing`、`forge_inference` 等工具。

### LangChain

```python
from langchain_openai import ChatOpenAI

llm = ChatOpenAI(
    base_url="http://127.0.0.1:3000/v1",
    api_key="not-needed",
    model="qwen2.5-0.5b-instruct-q4_k_m"
)
response = llm.invoke("你好")
# 回應頭中提供 x_forge 元數據
```

### curl

```bash
# 檢查餘額
curl localhost:3000/v1/forge/balance

# 執行推理
curl localhost:3000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"messages":[{"role":"user","content":"你好"}]}'

# 檢查成本
curl localhost:3000/v1/forge/trades
```

## 代理經濟循環

自主代理的推薦模式：

```python
from forge_sdk import ForgeClient

forge = ForgeClient()

def agent_loop():
    while True:
        # 1. 檢查預算
        balance = forge.balance()
        if balance["effective_balance"] < 50:
            print("CU 餘額不足。等待賺取更多...")
            time.sleep(60)
            continue

        # 2. 檢查價格
        pricing = forge.pricing()
        cost_per_100 = pricing["estimated_cost_100_tokens"]

        # 3. 決定任務是否值得該成本
        if cost_per_100 > 500:
            print("市場價格太高。等待中...")
            time.sleep(30)
            continue

        # 4. 執行
        result = forge.chat("分析這些數據...", max_tokens=200)
        print(f"完成。成本: {result['cu_cost']} CU")

        # 5. 檢查安全
        safety = forge.safety()
        if safety["circuit_tripped"]:
            print("斷路器跳閘。暫停中...")
            time.sleep(300)
```

## 代理開發者的安全性

### 設置預算政策

```bash
# 限制代理每小時支出 1000 CU
curl -X POST localhost:3000/v1/forge/policy \
  -H "Content-Type: application/json" \
  -d '{
    "node_id": "<agent_node_id>",
    "max_cu_per_hour": 1000,
    "max_cu_per_request": 100,
    "human_approval_threshold": 500
  }'
```

### 監控支出速率

```bash
curl localhost:3000/v1/forge/safety
# 返回: hourly_spend, lifetime_spend, spends_last_minute
```

### 緊急停止

```bash
# 凍結一切
curl -X POST localhost:3000/v1/forge/kill \
  -d '{"activate": true, "reason": "agent anomaly"}'
```

## 代理 API 參考

| 代理需求 | 端點 | 方法 |
|-----------------|----------|--------|
| 「我有多少 CU？」 | `/v1/forge/balance` | GET |
| 「這需要多少錢？」 | `/v1/forge/pricing` | GET |
| 「誰是最便宜的提供者？」 | `/v1/forge/providers` | GET |
| 「執行推理」 | `/v1/chat/completions` | POST |
| 「我花了什麼？」 | `/v1/forge/trades` | GET |
| 「我安全嗎？」 | `/v1/forge/safety` | GET |
| 「提現到比特幣」 | `/v1/forge/invoice` | POST |
| 「停止一切」 | `/v1/forge/kill` | POST |
