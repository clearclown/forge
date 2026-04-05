# Forge — 路線圖

## 階段 1: 本地推理 ✅

- `forge-core`: 類型系統 (NodeId, LayerRange, ModelManifest, PeerCapability)
- `forge-infer`: llama.cpp 引擎, GGUF 加載器, 流式 token 生成
- `forge-node`: HTTP API (/chat, /chat/stream, /health)
- `forge-cli`: 帶有模型自動下載功能的 `forge chat` 命令

## 階段 2: P2P 協定 ✅

- `forge-net`: Iroh 傳輸, Noise 加密, 節點連接
- `forge-proto`: 14 種線纜協定消息類型 (bincode + 長度前綴)
- `forge-node`: 播種/工人流水線, 推理請求/響應
- 集成測試: 2 個節點交換 Hello + 多條消息

## 階段 3: 遠端推理 + 操作員帳本 ✅

- `forge-ledger`: CU 核算、交易執行、聲譽、收益、市場定價
- `forge-node`: 帳本集成到推理流水線中
- 推理前的 CU 餘額檢查
- 完成後的交易記錄
- HMAC-SHA256 帳本完整性保護

## 階段 4: 經濟 API ✅

- 相容 OpenAI 的 API: `POST /v1/chat/completions`, `GET /v1/models`
- CU 計量: 每次推理記錄一筆帶有 `x_forge` 擴充的交易
- 代理預算端點: `GET /v1/forge/balance`, `GET /v1/forge/pricing`
- CU→Lightning 結算橋接: `forge settle --pay`
- 從 HF Hub 自動解析播種模型
- 支援帳本持久化的優雅 Ctrl-C 停機

## 階段 5: mesh-llm 分叉集成 (下一階段)

**目標:** 使用 mesh-llm 成熟的分散式引擎替換 Forge 的推理層。

| 交付物 | 描述 |
|---|---|
| 分叉 mesh-llm | 創建集成經濟層的 mesh-llm 分叉版本 forge |
| 集成 forge-ledger | 將 CU 記錄掛鉤到 mesh-llm 的推理流水線中 |
| 保留經濟 API | 在新代碼庫中保留 /v1/forge/* 端點 |
| Web 控制台擴充 | 在 mesh-llm 的控制台中添加 CU 餘額和交易可見性 |
| 流水線 + MoE | 繼承 mesh-llm 的流水線並行和專家分片功能 |
| Nostr 發現 | 繼承 mesh-llm 的公共網格發現機制 |
| CREDITS.md | 記錄對 mesh-llm 的歸功說明 |

## 階段 6: 有用工作證明

**目標:** 使 CU 聲明在網絡中可驗證。

| 交付物 | 描述 |
|---|---|
| 雙重簽名協定 | 提供者和消費者都對每個 TradeRecord 進行簽名 |
| Gossip 同步 | 簽名後的交易在網格中傳播 |
| 欺詐檢測 | 拒絕未簽名或不匹配的交易 |
| 聲譽 Gossip | 在節點間共享聲譽分數 |
| 共謀抗性 | 對交易模式進行統計異常檢測 |

## 階段 7: 外部橋接

**目標:** 讓操作員能夠將 CU 轉換為外部價值。

| 交付物 | 描述 |
|---|---|
| Lightning 橋接 | 透過 LDK 實現自動化的 CU→sats 結算 |
| 穩定幣適配器 | CU→USDC/USDT 轉換 |
| 法幣適配器接口 | 銀行轉帳結算規範 |
| 匯率服務 | 公共 CU/BTC 和 CU/USD 匯率推介 |
| 比特幣錨定 | 可選: 定期的默克爾根 → OP_RETURN，用於不可篡改的審計追蹤 |

## 階段 8: 代理自主經濟

**目標:** 让 AI 代理管理自己的計算生命週期。

| 交付物 | 描述 |
|---|---|
| 預算政策 | 由人類設置的每個代理的支出限制 |
| 自主交易 | 代理決定何時買賣計算資源 |
| 多模型路由 | 代理基於成本/質量權衡選擇模型 |
| 自我強化 | 代理賺取 CU → 購買更大型模型訪問權限 → 賺取更多 CU |
| 代理間經濟 | 代理交易專門的計算能力（代碼模型 vs 聊天模型） |

## 長期計劃

| 里程碑 | 描述 |
|---|---|
| SDK 發布 | forge-node 作為具有穩定 API 的可嵌入 Rust 庫 |
| 協定 v2 | 基於 v1 經驗的向後相容演進 |
| 跨架構支援 | 支援 NVIDIA GPU, AMD ROCm, RISC-V (透過 mesh-llm) |
| 聯邦訓練 | 分散式微調，而不僅僅是推理 |
| 計算衍生品 | 關於未來計算能力的遠期合約 |

> 協定即平台。計算即貨幣。
