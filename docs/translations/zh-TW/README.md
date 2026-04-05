# Forge

> 計算即貨幣。每一瓦特都在產生智能，而非浪費。

**Forge 是一種分散式推理協定，其中計算即金錢。** 節點透過為他人執行有用的 LLM 推理來賺取計算單位 (CU)。與比特幣不同——在比特幣中，電力被浪費在毫無意義的雜湊計算上——在 Forge 節點上花費的每一焦耳都會產生某人真正需要的真實智能。

分散式推理引擎基於 Michael Neale 的 [mesh-llm](https://github.com/michaelneale/mesh-llm) 建構。Forge 在其上添加了計算經濟層：CU 核算、有用工作證明 (Proof of Useful Work)、動態定價、自主代理預算和故障安全控制。參見 [CREDITS.md](CREDITS.md)。

**集成分叉：** [forge-mesh](https://github.com/nm-arealnormalman/mesh-llm) — 內建 Forge 經濟層的 mesh-llm。

## 現場演示

這是來自正在運行的 Forge 節點的真實輸出。每次推理都會消耗 CU。每個 CU 都是透過有用的計算賺取的。

```
$ forge node -m "qwen2.5:0.5b" --ledger forge-ledger.json
  Model loaded: Qwen2.5-0.5B (Metal-accelerated, 491MB)
  API server listening on 127.0.0.1:3000
```

**檢查餘額 — 每個新節點都會獲得 1,000 CU 的免費額度：**
```
$ curl localhost:3000/v1/forge/balance
{
  "effective_balance": 1000,
  "contributed": 0,
  "consumed": 0,
  "reputation": 0.5
}
```

**提出問題 — 推理消耗 CU：**
```
$ curl localhost:3000/v1/chat/completions \
    -d '{"messages":[{"role":"user","content":"用中文打個招呼"}]}'
{
  "choices": [{"message": {"content": "你好！"}}],
  "usage": {"completion_tokens": 2},
  "x_forge": {
    "cu_cost": 2,
    "effective_balance": 1002
  }
}
```

每個回應都包含 `x_forge` — **該計算的 CU 成本**以及剩餘餘額。提供者賺取了 2 CU，消費者花費了 2 CU。物理學為每個單位提供了支撐。

**三次推理後 — 帳本上的真實交易：**
```
$ curl localhost:3000/v1/forge/trades
{
  "count": 3,
  "trades": [
    {"cu_amount": 5, "tokens_processed": 5, "model_id": "qwen2.5-0.5b-instruct-q4_k_m"},
    {"cu_amount": 5, "tokens_processed": 5, "model_id": "qwen2.5-0.5b-instruct-q4_k_m"},
    {"cu_amount": 2, "tokens_processed": 2, "model_id": "qwen2.5-0.5b-instruct-q4_k_m"}
  ]
}
```

**每筆交易都有一個默克爾根 — 可錨定到比特幣以獲得不可篡改的證明：**
```
$ curl localhost:3000/v1/forge/network
{
  "total_trades": 3,
  "total_contributed_cu": 12,
  "merkle_root": "aac8db9f62dd9ff23926195a70ed8fcfc188fc867d9f2adabd8e694beb338748"
}
```

**AI 代理失控？緊急開關可在幾毫秒內凍結一切：**
```
$ curl -X POST localhost:3000/v1/forge/kill \
    -d '{"activate":true, "reason":"檢測到異常", "operator":"admin"}'
→ 緊急開關已激活 (KILL SWITCH ACTIVATED)
→ 所有 CU 交易已凍結。代理無法支出。
```

**安全控制始終開啟：**
```
$ curl localhost:3000/v1/forge/safety
{
  "kill_switch_active": false,
  "circuit_tripped": false,
  "policy": {
    "max_cu_per_hour": 10000,
    "max_cu_per_request": 1000,
    "max_cu_lifetime": 1000000,
    "human_approval_threshold": 5000
  }
}
```

## 為什麼選擇 Forge

```
比特幣: 電力 → 毫無意義的 SHA-256 → BTC
Forge: 電力 → 有用的 LLM 推理 → CU
```

比特幣證明了「電力 → 計算 → 金錢」。但比特幣的計算是沒有目的的。Forge 將其反轉：每個 CU 都代表了解決某人實際問題的真實智能。

**其他項目不具備的三個特點：**

### 1. 計算 = 貨幣

每次推理都是一筆交易。提供者賺取 CU，消費者支出 CU。沒有區塊鏈，沒有代幣，沒有 ICO。CU 由物理學支撐——即為有用工作而消耗的電力。

### 2. 無需區塊鏈的防篡改

每筆交易都由雙方進行雙重簽名 (Ed25519)，並在網格中透過 gossip 同步。所有交易的默克爾根可以錨定到比特幣進行不可篡改的審計。不需要全球共識——雙邊加密證明就足夠了。

### 3. AI 代理管理自己的計算

手機上的代理在夜間借出空閒計算能力 → 賺取 CU → 購買 70B 模型的訪問權限 → 變得更聰明 → 賺得更多。代理自主檢查 `/v1/forge/balance` 和 `/v1/forge/pricing`。預算政策和斷路器可防止失控支出。

```
代理 (手機上的 1.5B 模型)
  → 透過提供推理在夜間賺取 CU
  → 在 70B 模型上花費 CU → 獲得更聰明的回答
  → 更好的決策 → 賺取更多 CU
  → 循環重複 → 代理成長
```

## 架構

```
┌─────────────────────────────────────────────────┐
│  推理層 (Inference Layer: mesh-llm)             │
│  流水線並行、MoE 專家分片、                        │
│  iroh 網格、Nostr 發現、OpenAI API                │
└──────────────────┬──────────────────────────────┘
                   │
┌──────────────────▼──────────────────────────────┐
│  經濟層 (Economic Layer: Forge)                 │
│  CU 帳本、雙重簽名交易、gossip、                    │
│  動態定價、默克爾根、安全控制                      │
└──────────────────┬──────────────────────────────┘
                   │
┌──────────────────▼──────────────────────────────┐
│  安全層 (Safety Layer)                          │
│  緊急開關、預算政策、斷路器、                      │
│  速率檢測、人工審批閾值                           │
└──────────────────┬──────────────────────────────┘
                   │ 可選
┌──────────────────▼──────────────────────────────┐
│  外部橋接 (External Bridges)                    │
│  CU ↔ BTC (Lightning), CU ↔ 穩定幣             │
└─────────────────────────────────────────────────┘
```

## 快速入門

```bash
# 建構
cargo build --release

# 使用自動下載的模型運行節點
forged node -m "qwen2.5:0.5b" --ledger forge-ledger.json

# 本地聊天
forge chat -m "qwen2.5:0.5b" "什麼是重力？"

# 開始播種 (P2P，賺取 CU)
forge seed -m "qwen2.5:0.5b" --ledger forge-ledger.json

# 作為工人連接 (P2P，支出 CU)
forge worker --seed <public_key>

# 列出模型
forge models
```

## API 參考

### 推理 (OpenAI 相容)

| 端點 | 描述 |
|----------|-------------|
| `POST /v1/chat/completions` | 支援串流傳輸的聊天。每個回應包含 `x_forge.cu_cost` |
| `GET /v1/models` | 列出已加載的模型 |

### 經濟

| 端點 | 描述 |
|----------|-------------|
| `GET /v1/forge/balance` | CU 餘額、聲譽、貢獻歷史 |
| `GET /v1/forge/pricing` | 市場價格 (EMA 平滑)、成本估算 |
| `GET /v1/forge/trades` | 最近交易及其 CU 金額 |
| `GET /v1/forge/network` | 總 CU 流量 + 默克爾根 |
| `GET /v1/forge/providers` | 按聲譽和成本排名的提供者 |
| `POST /v1/forge/invoice` | 從 CU 餘額創建 Lightning 發票 |
| `GET /settlement` | 可導出的結算報表 |

### 安全

| 端點 | 描述 |
|----------|-------------|
| `GET /v1/forge/safety` | 緊急開關狀態、斷路器、預算政策 |
| `POST /v1/forge/kill` | 緊急停機 — 凍結所有 CU 交易 |
| `POST /v1/forge/policy` | 為每個代理設置預算限制 |

## 安全設計

AI 代理自主花費計算資源雖然強大，但也非常危險。Forge 擁有五層安全防護：

| 層級 | 機制 | 保護對象 |
|-------|-----------|------------|
| **緊急開關** | 人工操作員立即凍結所有交易 | 停止失控的代理 |
| **預算政策** | 每個代理的限制：單次請求、每小時、終生 | 限制總敞口 |
| **斷路器** | 5 次錯誤或每分鐘 30 次以上支出自動跳閘 | 捕捉異常 |
| **速率檢測** | 1 分鐘滑動窗口監控支出速率 | 防止突發支出 |
| **人工審批** | 超過閾值的交易需要人工確認 | 保護大額支出 |

設計原則：**故障安全 (fail-safe)**。如果任何檢查無法確定安全性，它將**拒絕**該操作。

## 構想

| 時代 | 標準 | 支撐 |
|-----|----------|---------|
| 古代 | 黃金 | 地質稀缺性 |
| 1944–1971 | 布雷頓森林體系 | 美元掛鉤黃金 |
| 1971–至今 | 石油美元 | 石油需求 + 軍事力量 |
| 2009–至今 | 比特幣 | SHA-256 上的能源（無用工作） |
| **現在** | **計算本位制 (Compute Standard)** | **LLM 推理上的能源（有用工作）** |

一間裝滿運行 Forge 的 Mac Mini 的房間就像一棟公寓樓——透過在業主睡覺時執行有用工作來產生收益。

## 專案結構

```
forge/
├── crates/
│   ├── forge-ledger/      # CU 核算、交易、定價、安全、默克爾根
│   ├── forge-node/        # 節點守護進程、HTTP API、流水線協調器
│   ├── forge-cli/         # CLI: 聊天、播種、工人、結算、錢包
│   ├── forge-lightning/   # CU ↔ 比特幣 Lightning 橋接
│   ├── forge-net/         # P2P: iroh QUIC + Noise + gossip
│   ├── forge-proto/       # 線纜協定: 17 種消息類型
│   ├── forge-infer/       # 推理引擎: llama.cpp, GGUF, Metal/CPU
│   ├── forge-core/        # 類型定義: NodeId, CU, Config
│   └── forge-shard/       # 拓撲: 層級分配
└── docs/                  # 規範、威脅模型、路線圖
```

約 10,000 行 Rust 代碼。76 個測試。完成 2 次安全審計。

## 文檔

- [概念與願景](docs/concept.md) — 為什麼計算即金錢
- [經濟模型](docs/economy.md) — CU 經濟、有用工作證明
- [架構設計](docs/architecture.md) — 雙層設計
- [線纜協定](docs/protocol-spec.md) — 17 種消息類型
- [路線圖](docs/roadmap.md) — 開發階段
- [威脅模型](docs/threat-model.md) — 安全 + 經濟攻擊
- [引導啟動](docs/bootstrap.md) — 啟動、降級、恢復

## 許可證

MIT

## 致謝

Forge 的分散式推理基於 Michael Neale 的 [mesh-llm](https://github.com/michaelneale/mesh-llm) 建構。參見 [CREDITS.md](CREDITS.md)。
