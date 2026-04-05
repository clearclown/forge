# エージェント間（A2A）プロトコル用 Forge CU 決済拡張機能

*エージェント通信標準への計算決済追加の提案*

## 概要

既存のエージェント間プロトコル（Google A2A、Anthropic MCP）は、エージェントがどのように通信するかを定義していますが、どのようにお互いに支払うかは定義していません。この提案は CU（計算ユニット: Compute Unit）決済層を追加し、人間による介入やブロックチェーンのトランザクションなしに、エージェントが自律的に計算資源を取引できるようにします。

## 課題

エージェントAがエージェントBにタスクの実行を依頼する場合：
- **現在:** エージェントAの人間がエージェントBの人間に支払う（クレジットカード、APIキー）。
- **必要とされるもの:** エージェントAが計算ユニットでエージェントBに直接支払う。

エージェント間の決済をサポートする既存の標準は存在しません。

## 提案: CU 決済ヘッダー

### リクエスト

エージェントAは、作業をリクエストする際に決済ヘッダーを追加します：

```http
POST /v1/chat/completions HTTP/1.1
X-Forge-Consumer-Id: <agent-a-node-id>
X-Forge-Max-CU: 500
X-Forge-Consumer-Sig: <ed25519-signature-of-request-hash>
```

### レスポンス

エージェントBはコスト情報を含めます：

```http
HTTP/1.1 200 OK
X-Forge-Provider-Id: <agent-b-node-id>
X-Forge-CU-Cost: 47
X-Forge-Provider-Sig: <ed25519-signature-of-response-hash>
```

### 取引記録

両方のエージェントが独立して記録します：

```json
{
  "provider": "<agent-b>",
  "consumer": "<agent-a>",
  "cu_amount": 47,
  "tokens_processed": 47,
  "timestamp": 1775289254032,
  "provider_sig": "<sig>",
  "consumer_sig": "<sig>"
}
```

### ゴシップ

二重署名された取引記録は、メッシュ全体でゴシップ同期されます。どのノードでも両方の署名を検証できます。

## 既存の標準との統合

### Google A2A

A2A の `Task` オブジェクトに追加します：

```json
{
  "id": "task-123",
  "status": "completed",
  "payment": {
    "protocol": "forge-cu",
    "consumer": "<node-id>",
    "provider": "<node-id>",
    "cu_amount": 47,
    "consumer_sig": "<sig>",
    "provider_sig": "<sig>"
  }
}
```

### Anthropic MCP

MCP サーバーに `forge_payment` リソースを追加します：

```json
{
  "resources": [{
    "uri": "forge://payment/balance",
    "name": "CU Balance",
    "mimeType": "application/json"
  }]
}
```

### OpenAI Function Calling

関数呼び出しを使用するエージェントは Forge ツールを含めることができます：

```json
{
  "tools": [{
    "type": "function",
    "function": {
      "name": "forge_pay",
      "description": "Pay CU for a compute task",
      "parameters": {
        "provider": "string",
        "cu_amount": "integer"
      }
    }
  }]
}
```

## セキュリティ

- すべての決済には Ed25519 による二重署名が必要です。
- 予算ポリシーにより、リクエストごと、時間、および生涯の支出が制限されます。
- サーキットブレーカーは、異常な支出パターンを検知して作動します。
- キルスイッチはすべてのトランザクションを凍結します（手動での上書きが可能）。
- ブロックチェーンは不要です。二者間の証明で十分です。

## 比較

| 機能 | Stripe | Bitcoin Lightning | **Forge CU** |
|---------|--------|-------------------|-------------|
| エージェント間 | 不可 (人間が必要) | 部分的 (チャネルが必要) | **可能** |
| 決済速度 | 数日 | 数秒 | **即時** |
| トランザクションコスト | 2.9% | 〜1 sat | **ゼロ** |
| 価値の裏打ち | 法定通貨 | PoW (無意味) | **有益な計算** |
| エージェントSDK | なし | なし | **あり** |

## 実装

リファレンス実装: [github.com/clearclown/forge](https://github.com/clearclown/forge)

- Python SDK: `pip install forge-sdk`
- MCP サーバー: `pip install forge-mcp`
- Rust クレート: `forge-ledger`, `forge-core`
