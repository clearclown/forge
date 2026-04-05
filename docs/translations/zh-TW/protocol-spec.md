# Forge — 線纜協定規範

## 概述

Forge 節點透過由 Iroh 建立的加密 QUIC 連接交換 bincode 序列化的控制消息。激活張量作為原始字節承載在 `Forward` 消息中。當前的 v1 實現對本地播種/請求者推理和未來的多跳流水線消息使用相同的封裝。

## 消息封裝 (Envelope)

每條消息都封裝在一個外殼中：

```rust
pub struct Envelope {
    pub msg_id: u64,
    pub sender: NodeId,
    pub timestamp: u64, // unix 毫秒
    pub payload: Payload,
}
```

當前運行時強制執行的驗證規則：
- `Envelope.sender` 必須與 QUIC 連接中經過身份驗證的遠端對等節點身份匹配
- 來自同一對等節點的重複 `msg_id` 值將在有限的重放窗口內被丟棄
- `Hello.capability.node_id` 和 `Welcome.capability.node_id` 必須與 `Envelope.sender` 匹配
- 畸形的層級範圍和不匹配的張量長度在高級處理程序看到消息之前就會被拒絶
- 提示詞和 token 字段是有界的 (`prompt_text` 和 `max_tokens`)，因此一個節點不能要求另一個節點分配無限的工作

## Payload 枚舉

```rust
pub enum Payload {
    Hello(Hello),
    Welcome(Welcome),
    AssignShard(AssignShard),
    ShardReady(ShardReady),
    PipelineTopology(PipelineTopologyMsg),
    Forward(Forward),
    TokenResult(TokenResult),
    InferenceRequest(InferenceRequest),
    TokenStream(TokenStreamMsg),
    Error(ErrorMsg),
    Heartbeat(Heartbeat),
    Ping(Ping),
    Pong(Pong),
    Leaving(Leaving),
    Rebalance(Rebalance),
}
```

## 發現與握手

```rust
pub struct Hello {
    pub version: u16,
    pub capability: PeerCapability,
}

pub struct Welcome {
    pub version: u16,
    pub capability: PeerCapability,
    pub known_peers: Vec<PeerInfo>,
}

pub struct PeerInfo {
    pub node_id: NodeId,
    pub addr: String,
}
```

- `version` 是發送者通告的協定版本。
- `capability` 描述了用於調度決策的 CPU、內存、頻寬和區域。
- `known_peers` 是一個機會主義的節點列表，不是全球權威註冊表。

## 分片分配

這些消息定義了未來的多跳層級流水線。儘管當前的參考實現主要在播種節點上運行全模型推理，但它們仍屬於 v1 的一部分。

```rust
pub struct AssignShard {
    pub model_id: ModelId,
    pub model_source: String,
    pub layer_range: LayerRange,
    pub pipeline_position: u8,
    pub upstream: Option<NodeId>,
    pub downstream: Option<NodeId>,
}

pub struct ShardReady {
    pub model_id: ModelId,
    pub layer_range: LayerRange,
    pub load_time_ms: u64,
}

pub struct PipelineTopologyMsg {
    pub model_id: ModelId,
    pub stages: Vec<PipelineStage>,
}
```

## 推理消息

### Forward

`Forward` 在流水線階段之間傳遞激活張量。

```rust
pub struct Forward {
    pub request_id: u64,
    pub sequence_pos: u32,
    pub tensor_meta: TensorMeta,
    #[serde(with = "serde_bytes")]
    pub tensor_data: Vec<u8>,
}

pub struct TensorMeta {
    pub shape: Vec<u32>,
    pub dtype: DType,
    pub byte_len: u32,
}
```

- `tensor_data` 是原始激活字節。
- `dtype` 是 `F16`、`F32` 或 `I8` 之一。
- WAN 傳輸預計更傾向於使用緊湊表示，如 `I8`。

### TokenResult

`TokenResult` 預留用於多跳推理中最後階段採樣得到的 token ID。

```rust
pub struct TokenResult {
    pub request_id: u64,
    pub tokens: Vec<u32>,
}
```

### InferenceRequest

當前的播種/請求者流程直接發送提示詞文本。播種節點在本地進行分詞 (tokenize)。

```rust
pub struct InferenceRequest {
    pub request_id: u64,
    pub prompt_text: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub top_p: f32,
}
```

- `prompt_text` 取代了早期的 token ID 提示詞黑客方式。
- `max_tokens` 既是生成限制，也是預檢 CU 支付能力的基礎。

### TokenStreamMsg

當前的流式回應發送解碼後的文本片段，而不是 token ID。

```rust
pub struct TokenStreamMsg {
    pub request_id: u64,
    pub text: String,
    pub is_final: bool,
}
```

- `text` 是適合立即渲染的解碼文本片段。
- `is_final = true` 關閉請求的流。

### ErrorMsg

請求範圍內的失敗將作為類型化錯誤返回，而不是過載的文本片段。

```rust
pub enum ErrorCode {
    InvalidRequest,
    InsufficientBalance,
    Busy,
    Internal,
}

pub struct ErrorMsg {
    pub request_id: u64,
    pub code: ErrorCode,
    pub message: String,
    pub retryable: bool,
}
```

- `request_id` 將錯誤與正在進行的推理請求關聯。
- `retryable` 告訴調用者稍後重試是否合理。
- 當前的播種節點/運行時將其用於無效請求、CU 拒絶、併發飽和以及生成失敗。

## 健康與活躍度

```rust
pub struct Heartbeat {
    pub uptime_sec: u64,
    pub load: f32,
    pub memory_free_gb: f32,
    pub battery_pct: Option<u8>,
}

pub struct Ping {
    pub sent_at: u64,
}

pub struct Pong {
    pub ping_sent_at: u64,
    pub received_at: u64,
}
```

## 集群管理

```rust
pub enum LeaveReason {
    Shutdown,
    LowBattery,
    UserRequest,
}

pub struct Leaving {
    pub reason: LeaveReason,
    pub drain_time_ms: u64,
}

pub enum RebalanceReason {
    NodeJoined,
    NodeLeft,
    ModelUpgrade,
}

pub struct Rebalance {
    pub new_topology: PipelineTopologyMsg,
    pub reason: RebalanceReason,
}
```

## 交易簽名 (有用工作證明)

Forge 使用雙重簽名交易來證明計算已執行並已被接收。提供者和消費者都必須簽署相同的規範交易字節。

### TradeProposal

由提供者在推理完成后發送。包含交易詳情和提供者的 Ed25519 簽名。

```rust
pub struct TradeProposal {
    pub request_id: u64,
    pub provider: NodeId,
    pub consumer: NodeId,
    pub cu_amount: u64,
    pub tokens_processed: u64,
    pub timestamp: u64,
    pub model_id: String,
    pub provider_sig: Vec<u8>,  // 64 字節 Ed25519 簽名
}
```

### TradeAccept

由消費者發送以共同簽署交易。

```rust
pub struct TradeAccept {
    pub request_id: u64,
    pub consumer_sig: Vec<u8>,  // 64 字節 Ed25519 簽名
}
```

### TradeGossip

在記錄雙重簽名交易後廣播給所有連接的節點。任何節點都可以驗證雙方簽名。

```rust
pub struct TradeGossip {
    pub provider: NodeId,
    pub consumer: NodeId,
    pub cu_amount: u64,
    pub tokens_processed: u64,
    pub timestamp: u64,
    pub model_id: String,
    pub provider_sig: Vec<u8>,
    pub consumer_sig: Vec<u8>,
}
```

### 簽名的規範字節

雙方簽署相同的確定性二進制表示：

```
provider_id (32 bytes) + consumer_id (32 bytes) +
cu_amount (8 bytes LE) + tokens_processed (8 bytes LE) +
timestamp (8 bytes LE) + model_id (variable bytes)
```

### 雙重簽名流程

```text
提供者 (播種節點)                           消費者 (工人節點)
    |                                         |
    |--- TokenStream (推理) ----------------->|
    |--- TokenStream (最終) ----------------->|
    |                                         |
    |--- TradeProposal (提供者簽名) ---------->|
    |                                         |
    |    [消費者驗證提供者簽名]                 |
    |    [消費者共同簽名]                      |
    |                                         |
    |<--- TradeAccept (消費者簽名) ------------|
    |                                         |
    [提供者驗證雙方簽名]                       |
    [在帳本中記錄 SignedTradeRecord]           |
    [廣播 TradeGossip 到網格]                 |
```

如果消費者在 5 秒內未響應，提供者將退而記錄一筆未簽名交易（向後相容）。

### Gossip 傳播

當節點收到 `TradeGossip` 消息時：
1. 驗證雙方 Ed25519 簽名
2. 檢查 SHA-256 去重（如果已見過則拒絶）
3. 在本地帳本中記錄交易
4. 該交易**不會**被再次廣播（單跳 gossip 以防止廣播風暴）

## 序列化規則

- 控制消息使用 bincode。
- `Forward.tensor_data` 作為原始連續字節傳輸。
- 封裝對於所有消息類型保持統一，以便傳輸層可以保持通用。
- 運行時拒絶大於 64 MiB 的協定幀。
- 協定不嵌入法幣、區塊鏈或交易所結算字段。這些屬於協定外集成。

## 連接生命週期

### 當前播種/請求者流程

```text
請求者                          播種節點
  |                              |
  |--- QUIC + 加密 -------------->|
  |--- Hello ------------------->|
  |<-- Welcome ------------------|
  |--- InferenceRequest -------->|
  |                              | [CU 預留]
  |<-- TokenStreamMsg ---------- |
  |<-- TokenStreamMsg ---------- |
  |<-- TokenStreamMsg (最終) --- |
  |<-- TradeProposal ----------- | [提供者簽名]
  |--- TradeAccept ------------->| [消費者共同簽名]
  |                              | [記錄 SignedTradeRecord]
  |                              | [廣播 TradeGossip]
```

### 未來的多跳流程

```text
協調器            工人 A           工人 B           最後階段
    |                |               |                |
    |-- AssignShard->|               |                |
    |-- AssignShard----------------->|               |
    |-- AssignShard---------------------------------->|
    |<-- ShardReady--|               |                |
    |<---------------- ShardReady ---|                |
    |<-------------------------------- ShardReady ---|
    |-- PipelineTopology 向所有節點廣播 -------------->|
    |-- Forward ---->|-- Forward ---->|-- TokenResult->|
```

## 版本控制

當前版本: `1`

- 節點透過 `Hello` 和 `Welcome` 通告其版本。
- 參考實現當前假設對等節點相容，並忽略未知的未來負載。
- 破壞性的線纜更改應增加 `version` 並顯式定義降級行為。
