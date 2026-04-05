# Forge — エージェント統合ガイド

## AI エージェント開発者向け

Forge はエージェントに計算予算を提供します。エージェントは推論を提供することで CU を稼ぎ、より大きなモデルにアクセスするために CU を使うことができます。クレジットカード、API キー、人間による介在は一切不要です。

## クイック統合

### 任意の HTTP クライアント

```python
import requests

FORGE = "http://127.0.0.1:3000"

# エージェントがリクエストを実行できる残高があるか確認
balance = requests.get(f"{FORGE}/v1/forge/balance").json()
if balance["effective_balance"] > 100:
    # 推論を実行（CU コストが発生）
    r = requests.post(f"{FORGE}/v1/chat/completions", json={
        "messages": [{"role": "user", "content": "重力とは何ですか？"}],
        "max_tokens": 256
    }).json()
    print(r["choices"][0]["message"]["content"])
    print(f"コスト: {r['x_forge']['cu_cost']} CU")
```

### Python SDK

```python
from forge_sdk import ForgeClient, ForgeAgent

# シンプルなクライアント
forge = ForgeClient()
result = forge.chat("量子計算について説明して")
print(f"回答: {result['content']}")
print(f"コスト: {result['cu_cost']} CU, 残高: {result['balance']} CU")

# 予算管理機能を備えた自律エージェント
agent = ForgeAgent(max_cu_per_task=500)
while agent.has_budget():
    result = agent.think("次に何をすべきですか？")
    if result is None:
        break  # 予算不足
```

### MCP (Claude Code, Cursor)

MCP 設定に追加します：
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

AI アシスタントは `forge_balance`、`forge_pricing`、`forge_inference` といったツールを使用できるようになります。

### LangChain

```python
from langchain_openai import ChatOpenAI

llm = ChatOpenAI(
    base_url="http://127.0.0.1:3000/v1",
    api_key="not-needed",
    model="qwen2.5-0.5b-instruct-q4_k_m"
)
response = llm.invoke("こんにちは")
# レスポンスヘッダーから x_forge メタデータを利用可能
```

### curl

```bash
# 残高確認
curl localhost:3000/v1/forge/balance

# 推論実行
curl localhost:3000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"messages":[{"role":"user","content":"こんにちは"}]}'

# コストの確認
curl localhost:3000/v1/forge/trades
```

## エージェント経済ループ

自律エージェントに推奨されるパターン：

```python
from forge_sdk import ForgeClient

forge = ForgeClient()

def agent_loop():
    while True:
        # 1. 予算を確認
        balance = forge.balance()
        if balance["effective_balance"] < 50:
            print("CU 残高が不足しています。補充を待っています...")
            time.sleep(60)
            continue

        # 2. 価格を確認
        pricing = forge.pricing()
        cost_per_100 = pricing["estimated_cost_100_tokens"]

        # 3. タスクがコストに見合うか判断
        if cost_per_100 > 500:
            print("市場価格が高すぎます。待機中...")
            time.sleep(30)
            continue

        # 4. 実行
        result = forge.chat("データを分析して...", max_tokens=200)
        print(f"完了。コスト: {result['cu_cost']} CU")

        # 5. 安全性を確認
        safety = forge.safety()
        if safety["circuit_tripped"]:
            print("サーキットブレーカーが作動しました。一時停止中...")
            time.sleep(300)
```

## エージェント開発者のための安全性

### 予算ポリシーの設定

```bash
# 1つのエージェントの支出を1時間あたり 1000 CU に制限
curl -X POST localhost:3000/v1/forge/policy \
  -H "Content-Type: application/json" \
  -d '{
    "node_id": "<agent_node_id>",
    "max_cu_per_hour": 1000,
    "max_cu_per_request": 100,
    "human_approval_threshold": 500
  }'
```

### 支出速度の監視

```bash
curl localhost:3000/v1/forge/safety
# 戻り値: hourly_spend, lifetime_spend, spends_last_minute
```

### 緊急停止

```bash
# すべてを凍結
curl -X POST localhost:3000/v1/forge/kill \
  -d '{"activate": true, "reason": "agent anomaly"}'
```

## エージェント用 API リファレンス

| エージェントが必要なこと | エンドポイント | メソッド |
|-----------------|----------|--------|
| 「残高はいくら？」 | `/v1/forge/balance` | GET |
| 「コストはいくら？」 | `/v1/forge/pricing` | GET |
| 「最安のプロバイダーは？」 | `/v1/forge/providers` | GET |
| 「推論を実行する」 | `/v1/chat/completions` | POST |
| 「何に使った？」 | `/v1/forge/trades` | GET |
| 「安全な状態か？」 | `/v1/forge/safety` | GET |
| 「ビットコインに換金」 | `/v1/forge/invoice` | POST |
| 「すべて停止！」 | `/v1/forge/kill` | POST |
