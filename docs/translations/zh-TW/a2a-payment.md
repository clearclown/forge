# 代理間 (A2A) 協定的 Forge CU 支付擴充

*關於向代理通訊標準添加計算支付的提案*

## 摘要

現有的代理間協定（Google A2A、Anthropic MCP）定義了代理如何通訊，但未定義它們如何相互支付。本提案添加了一個 CU（計算單位: Compute Unit）支付層，使代理能夠在沒有人工干預或區塊鏈交易的情況下自主交易計算資源。

## 問題

當代理 A 請求代理 B 執行任務時：
- **現狀:** 代理 A 的人類向代理 B 的人類支付（信用卡、API 金鑰）
- **需求:** 代理 A 直接以計算單位向代理 B 支付

目前沒有任何現有標準支援代理間支付。

## 提案: CU 支付請求頭

### 請求

代理 A 在請求工作時添加支付請求頭：

```http
POST /v1/chat/completions HTTP/1.1
X-Forge-Consumer-Id: <agent-a-node-id>
X-Forge-Max-CU: 500
X-Forge-Consumer-Sig: <ed25519-signature-of-request-hash>
```

### 回應

代理 B 包含成本資訊：

```http
HTTP/1.1 200 OK
X-Forge-Provider-Id: <agent-b-node-id>
X-Forge-CU-Cost: 47
X-Forge-Provider-Sig: <ed25519-signature-of-response-hash>
```

### 交易記錄

雙方代理獨立記錄：

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

### Gossip

雙重簽名的交易記錄透過網格進行 gossip 同步。任何節點都可以驗證雙方簽名。

## 與現有標準的集成

### Google A2A

添加到 A2A `Task` 對象中：

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

向 MCP 伺服器添加 `forge_payment` 資源：

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

使用函數調用的代理可以包含 Forge 工具：

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

## 安全性

- 所有支付都需要雙邊 Ed25519 簽名
- 預算政策限制每次請求、每小時和終生的支出
- 斷路器在異常支出模式下跳閘
- 緊急開關凍結所有交易（人工干預）
- 無需區塊鏈 — 雙邊證明即可

## 比較

| 特性 | Stripe | 比特幣 Lightning | **Forge CU** |
|---------|--------|-------------------|-------------|
| 代理間支付 | 否 (需要人類) | 部分 (需要通道) | **是** |
| 結算速度 | 天 | 秒 | **即時** |
| 交易成本 | 2.9% | ~1 sat | **零** |
| 價值支撐 | 法定貨幣 | PoW (無用工作) | **有用計算** |
| 代理 SDK | 否 | 否 | **是** |

## 實現

參考實現: [github.com/clearclown/forge](https://github.com/clearclown/forge)

- Python SDK: `pip install forge-sdk`
- MCP 伺服器: `pip install forge-mcp`
- Rust crates: `forge-ledger`, `forge-core`
