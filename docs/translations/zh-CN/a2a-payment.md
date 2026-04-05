# 代理间 (A2A) 协议的 Forge CU 支付扩展

*关于向代理通信标准添加计算支付的提案*

## 摘要

现有的代理间协议（Google A2A、Anthropic MCP）定义了代理如何通信，但未定义它们如何相互支付。本提案添加了一个 CU（计算单位: Compute Unit）支付层，使代理能够在没有人工干预或区块链交易的情况下自主交易计算资源。

## 问题

当代理 A 请求代理 B 执行任务时：
- **现状:** 代理 A 的人类向代理 B 的人类支付（信用卡、API 密钥）
- **需求:** 代理 A 直接以计算单位向代理 B 支付

目前没有任何现有标准支持代理间支付。

## 提案: CU 支付请求头

### 请求

代理 A 在请求工作时添加支付请求头：

```http
POST /v1/chat/completions HTTP/1.1
X-Forge-Consumer-Id: <agent-a-node-id>
X-Forge-Max-CU: 500
X-Forge-Consumer-Sig: <ed25519-signature-of-request-hash>
```

### 响应

代理 B 包含成本信息：

```http
HTTP/1.1 200 OK
X-Forge-Provider-Id: <agent-b-node-id>
X-Forge-CU-Cost: 47
X-Forge-Provider-Sig: <ed25519-signature-of-response-hash>
```

### 交易记录

双方代理独立记录：

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

双重签名的交易记录通过网格进行 gossip 同步。任何节点都可以验证双方签名。

## 与现有标准的集成

### Google A2A

添加到 A2A `Task` 对象中：

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

向 MCP 服务器添加 `forge_payment` 资源：

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

使用函数调用的代理可以包含 Forge 工具：

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

- 所有支付都需要双边 Ed25519 签名
- 预算政策限制每次请求、每小时和终生的支出
- 断路器在异常支出模式下跳闸
- 紧急开关冻结所有交易（人工干预）
- 无需区块链 — 双边证明即可

## 比较

| 特性 | Stripe | 比特币 Lightning | **Forge CU** |
|---------|--------|-------------------|-------------|
| 代理间支付 | 否 (需要人类) | 部分 (需要通道) | **是** |
| 结算速度 | 天 | 秒 | **即时** |
| 交易成本 | 2.9% | ~1 sat | **零** |
| 价值支撑 | 法定货币 | PoW (无用工作) | **有用计算** |
| 代理 SDK | 否 | 否 | **是** |

## 实现

参考实现: [github.com/clearclown/forge](https://github.com/clearclown/forge)

- Python SDK: `pip install forge-sdk`
- MCP 服务器: `pip install forge-mcp`
- Rust crates: `forge-ledger`, `forge-core`
