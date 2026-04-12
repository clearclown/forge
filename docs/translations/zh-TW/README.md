<div align="center">

# Forge

**計算即貨幣。每一瓦特都在產生智能，而非浪費。**

[![PyPI: forge-sdk](https://img.shields.io/pypi/v/forge-sdk?label=forge-sdk&color=3775A9)](https://pypi.org/project/forge-sdk/)
[![PyPI: forge-cu-mcp](https://img.shields.io/pypi/v/forge-cu-mcp?label=forge-cu-mcp&color=3775A9)](https://pypi.org/project/forge-cu-mcp/)
[![Crates.io](https://img.shields.io/crates/v/forge?label=crates.io&color=e6522c)](https://crates.io/crates/forge)
[![License: MIT](https://img.shields.io/badge/License-MIT-brightgreen.svg)](../../../LICENSE)

---

[English](../../../README.md) · [日本語](../ja/README.md) · [简体中文](../zh-CN/README.md) · **繁體中文** · [Español](../es/README.md) · [Français](../fr/README.md) · [Русский](../ru/README.md) · [Українська](../uk/README.md) · [हिन्दी](../hi/README.md) · [العربية](../ar/README.md) · [فارسی](../fa/README.md) · [עברית](../he/README.md)

</div>

**Forge 是一種分散式推理協定，其中計算即金錢。** 節點透過為他人執行有用的 LLM 推理來賺取計算單位 (CU)。與比特幣不同——在比特幣中，電力被浪費在毫無意義的雜湊計算上——在 Forge 節點上花費的每一焦耳都會產生某人真正需要的真實智能。

分散式推理引擎基於 Michael Neale 的 [mesh-llm](https://github.com/michaelneale/mesh-llm) 建構。Forge 在其上添加了計算經濟層：CU 核算、有用工作證明 (Proof of Useful Work)、動態定價、自主代理預算和故障安全控制。參見 [CREDITS.md](../../../CREDITS.md)。

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
    -d '{"messages":[{"role":"user","content":"Say hello in Japanese"}]}'
{
  "choices": [{"message": {"content": "こんにちは！ (konnichiwa!)"}}],
  "usage": {"completion_tokens": 9},
  "x_forge": {
    "cu_cost": 9,
    "effective_balance": 1009
  }
}
```

每個回應都包含 `x_forge` — **該計算的 CU 成本**以及剩餘餘額。提供者賺取了 9 CU，消費者花費了 9 CU。物理學為每個單位提供了支撐。

**三次推理後 — 帳本上的真實交易：**
```
$ curl localhost:3000/v1/forge/trades
{
  "count": 3,
  "trades": [
    {"cu_amount": 5, "tokens_processed": 5, "model_id": "qwen2.5-0.5b-instruct-q4_k_m"},
    {"cu_amount": 5, "tokens_processed": 5, "model_id": "qwen2.5-0.5b-instruct-q4_k_m"},
    {"cu_amount": 9, "tokens_processed": 9, "model_id": "qwen2.5-0.5b-instruct-q4_k_m"}
  ]
}
```

**每筆交易都有一個默克爾根 — 可錨定到比特幣以獲得不可篡改的證明：**
```
$ curl localhost:3000/v1/forge/network
{
  "total_trades": 3,
  "total_contributed_cu": 19,
  "merkle_root": "aac8db9f62dd9ff23926195a70ed8fcfc188fc867d9f2adabd8e694beb338748"
}
```

**AI 代理失控？緊急開關可在幾毫秒內凍結一切：**
```
$ curl -X POST localhost:3000/v1/forge/kill \
    -d '{"activate":true, "reason":"anomaly detected", "operator":"admin"}'
→ KILL SWITCH ACTIVATED
→ All CU transactions frozen. No agent can spend.
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

**其他項目不具備的四個特點：**

### 1. 計算 = 貨幣

每次推理都是一筆交易。提供者賺取 CU，消費者支出 CU。沒有區塊鏈，沒有代幣，沒有 ICO。CU 由物理學支撐——即為有用工作而消耗的電力。與 Bittensor (TAO)、Akash (AKT) 或 Golem (GLM) 不同，CU 無法被投機——它透過執行有用計算來賺取。

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

### 4. 計算微金融

節點可以將閒置 CU 以利息借給其他節點。小型節點借入 CU，存取更大的模型，賺取更多 CU，並支付利息還款。沒有其他分散式推理專案提供計算借貸。這是讓自我改進迴圈對每個人（而不僅僅是那些已經擁有強大硬體的人）在經濟上都可行的引擎。

## 架構

```
┌─────────────────────────────────────────────────┐
│  L4: 發現 (forge-agora) ✅ v0.1                 │
│  代理市集、聲譽聚合、                             │
│  Nostr NIP-90、Google A2A 支付擴充               │
├─────────────────────────────────────────────────┤
│  L3: 智能 (forge-mind) ✅ v0.1                  │
│  AutoAgent 自我改進迴圈、                        │
│  harness 市集、元最佳化                          │
├─────────────────────────────────────────────────┤
│  L2: 金融 (forge-bank) ✅ v0.1                  │
│  策略、投資組合、期貨、保險、                      │
│  風險模型、收益最佳化器                           │
├─────────────────────────────────────────────────┤
│  L1: 經濟 (forge — 本倉庫) ✅ 第 1-6 階段       │
│  CU 帳本、雙重簽名交易、動態定價、                 │
│  借貸原語、安全控制                               │
├─────────────────────────────────────────────────┤
│  L0: 推理 (forge-mesh / mesh-llm) ✅            │
│  流水線並行、MoE 分片、                           │
│  iroh 網格、Nostr 發現、MLX/llama.cpp            │
└─────────────────────────────────────────────────┘

全部 5 層均已存在。生態系統中共有 326 個測試通過。
```

## 快速入門

### 方式一：一鍵端到端演示（Rust，冷啟動約 30 秒）

```bash
git clone https://github.com/clearclown/forge && cd forge
bash scripts/demo-e2e.sh
```

此腳本會從 HuggingFace 下載 SmolLM2-135M（約 100 MB），啟動帶有 Metal/CUDA 加速的真實 Forge 節點，執行三次真實的聊天完成，遍歷第 1-12 階段的所有端點，並列印彩色摘要。已於 2026-04-09 在 Apple Silicon Metal GPU 上驗證。

完成後，同一節點還響應：

```bash
# 相容 OpenAI 的客戶端
export OPENAI_BASE_URL=http://127.0.0.1:3001/v1
export OPENAI_API_KEY=$(cat ~/.forge/api_token 2>/dev/null || echo "$TOKEN")

# 真實的逐令牌串流傳輸 (第 11 階段)
curl -N $OPENAI_BASE_URL/chat/completions \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"model":"smollm2:135m","messages":[{"role":"user","content":"hi"}],"stream":true}'

# 第 8 階段經濟 / 第 9 階段聲譽 / 第 10 階段指標 / 錨定
curl $OPENAI_BASE_URL/forge/balance -H "Authorization: Bearer $OPENAI_API_KEY"
curl $OPENAI_BASE_URL/forge/anchor?network=mainnet -H "Authorization: Bearer $OPENAI_API_KEY"
curl http://127.0.0.1:3001/metrics  # Prometheus，無需認證
```

完整功能矩陣請參見 [`docs/compatibility.md`](../../../docs/compatibility.md)。

### 方式二：Python（透過 SDK + MCP 驅動一切）

```bash
pip install forge-sdk forge-cu-mcp

python -c "
from forge_sdk import ForgeClient
c = ForgeClient(base_url='http://localhost:3001')
print('balance:', c.balance())
print('decision:', c.bank_tick())
"
```

[PyPI: forge-sdk](https://pypi.org/project/forge-sdk/)（20 個 L2/L3/L4 方法）·
[PyPI: forge-cu-mcp](https://pypi.org/project/forge-cu-mcp/)（為 Claude Code / Cursor 提供 20 個 MCP 工具）

### 方式三：手動 Rust 命令

**前置條件**：[安裝 Rust](https://rustup.rs/)（約 2 分鐘）

```bash
cargo build --release

# 執行節點 — 自動從 HuggingFace 下載模型
./target/release/forge node -m "qwen2.5:0.5b" --ledger forge-ledger.json

# 或以下任意命令：
./target/release/forge chat -m "smollm2:135m" "什麼是重力？"
./target/release/forge seed -m "qwen2.5:1.5b"               # 作為 P2P 提供者賺取 CU
./target/release/forge worker --seed <public_key>           # 作為 P2P 消費者花費 CU
./target/release/forge models                                # 列出目錄（或使用 HF URL / 簡寫）
```

**[Crates.io: forge](https://crates.io/crates/forge)** ·
**[相容性文件](../../../docs/compatibility.md)** ·
**[演示腳本](../../../scripts/demo-e2e.sh)**

### 方式四：預編譯二進位檔 / Docker

預編譯二進位檔和 `clearclown/forge:latest` Docker 映像在
[releases](../../../releases) 頁面跟踪。在此之前，方式一可在兩分鐘內從原始碼建置。

## API 參考

### 推理（OpenAI 相容）

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
| `GET /v1/forge/route` | 最佳提供者選擇（成本/品質/平衡） |
| `GET /settlement` | 可導出的結算報表 |

### 借貸

| 端點 | 描述 |
|----------|-------------|
| `POST /v1/forge/lend` | 向借貸池提供 CU |
| `POST /v1/forge/borrow` | 申請 CU 貸款 |
| `POST /v1/forge/repay` | 還清未償貸款 |
| `GET /v1/forge/credit` | 信用分數與歷史 |
| `GET /v1/forge/pool` | 借貸池狀態 |
| `GET /v1/forge/loans` | 活躍貸款 |

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
forge/  (本倉庫 — 第 1 層)
├── crates/
│   ├── forge-ledger/      # CU 核算、借貸、agora (NIP-90)、安全
│   ├── forge-node/        # 節點守護進程、HTTP API（借貸 + 路由）、流水線
│   ├── forge-cli/         # CLI: 聊天、播種、工人、結算、錢包
│   ├── forge-lightning/   # CU ↔ 比特幣 Lightning 橋接（雙向）
│   ├── forge-net/         # P2P: iroh QUIC + Noise + gossip（交易 + 貸款）
│   ├── forge-proto/       # 線纜協定: 27+ 種消息類型，含 Loan*
│   ├── forge-infer/       # 推理引擎: llama.cpp, GGUF, Metal/CPU
│   ├── forge-core/        # 類型定義: NodeId, CU, Config
│   └── forge-shard/       # 拓撲: 層級分配
├── sdk/python/forge_sdk.py        # 帶完整借貸 API 的 Python 客戶端
├── mcp/forge-mcp-server.py        # MCP 伺服器（面向 Claude 等的借貸工具）
├── scripts/verify-impl.sh         # TDD 回歸測試（24 個斷言）
└── docs/                  # 規範、策略、威脅模型、路線圖
```

約 14,500 行 Rust。**143 個測試通過。** 第 1-6 階段完成。

## 姊妹倉庫（完整生態系統）

| 倉庫 | 層級 | 測試數 | 狀態 |
|------|-------|-------|--------|
| [clearclown/forge](https://github.com/clearclown/forge)（本倉庫） | L1 經濟 | 143 | 第 1-6 階段 ✅ |
| [clearclown/forge-bank](https://github.com/clearclown/forge-bank) | L2 金融 | 45 | v0.1 ✅ |
| [clearclown/forge-mind](https://github.com/clearclown/forge-mind) | L3 智能 | 40 | v0.1 ✅ |
| [clearclown/forge-agora](https://github.com/clearclown/forge-agora) | L4 發現 | 39 | v0.1 ✅ |
| [clearclown/forge-economics](https://github.com/clearclown/forge-economics) | 理論 | 16/16 GREEN | ✅ |
| [nm-arealnormalman/mesh-llm](https://github.com/nm-arealnormalman/mesh-llm) | L0 推理 | 43 (forge-economy) | ✅ |

## 文件

- [策略](../../../docs/strategy.md) — 競爭定位、借貸規範、5 層架構
- [貨幣理論](../../../docs/monetary-theory.md) — 為什麼 CU 可行：Soddy、比特幣、PoUW、僅限 AI 的貨幣
- [概念與願景](../../../docs/concept.md) — 為什麼計算即金錢
- [經濟模型](../../../docs/economy.md) — CU 經濟、有用工作證明、借貸
- [架構設計](../../../docs/architecture.md) — 雙層設計
- [代理整合](../../../docs/agent-integration.md) — SDK、MCP、借貸工作流程
- [線纜協定](../../../docs/protocol-spec.md) — 17 種消息類型
- [路線圖](../../../docs/roadmap.md) — 開發階段
- [威脅模型](../../../docs/threat-model.md) — 安全 + 經濟攻擊
- [引導啟動](../../../docs/bootstrap.md) — 啟動、降級、恢復
- [A2A 支付](../../../docs/a2a-payment.md) — 面向代理協定的 CU 支付擴充
- [相容性](../../../docs/compatibility.md) — 與 llama.cpp / Ollama / Bittensor 的功能矩陣

## 許可證

MIT

## 致謝

Forge 的分散式推理基於 Michael Neale 的 [mesh-llm](https://github.com/michaelneale/mesh-llm) 建構。參見 [CREDITS.md](../../../CREDITS.md)。
