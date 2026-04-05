# Forge — 线缆协议规范

## 概述

Forge 节点通过由 Iroh 建立的加密 QUIC 连接交换 bincode 序列化的控制消息。激活张量作为原始字节承载在 `Forward` 消息中。当前的 v1 实现对本地播种/请求者推理和未来的多跳流水线消息使用相同的封装。

## 消息封装 (Envelope)

每条消息都封装在一个外壳中：

```rust
pub struct Envelope {
    pub msg_id: u64,
    pub sender: NodeId,
    pub timestamp: u64, // unix 毫秒
    pub payload: Payload,
}
```

当前运行时强制执行的验证规则：
- `Envelope.sender` 必须与 QUIC 连接中经过身份验证的远程对等节点身份匹配
- 来自同一对等节点的重复 `msg_id` 值将在有限的重放窗口内被丢弃
- `Hello.capability.node_id` 和 `Welcome.capability.node_id` 必须与 `Envelope.sender` 匹配
- 畸形的层级范围和不匹配的张量长度在高级处理程序看到消息之前就会被拒绝
- 提示词和 token 字段是有界的 (`prompt_text` 和 `max_tokens`)，因此一个节点不能要求另一个节点分配无限的工作

## Payload 枚举

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

## 发现与握手

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

- `version` 是发送者通告的协议版本。
- `capability` 描述了用于调度决策的 CPU、内存、带宽和区域。
- `known_peers` 是一个机会主义的节点列表，不是全球权威注册表。

## 分片分配

这些消息定义了未来的多跳层级流水线。尽管当前的参考实现主要在播种节点上运行全模型推理，但它们仍属于 v1 的一部分。

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

`Forward` 在流水线阶段之间传递激活张量。

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

- `tensor_data` 是原始激活字节。
- `dtype` 是 `F16`、`F32` 或 `I8` 之一。
- WAN 传输预计更倾向于使用紧凑表示，如 `I8`。

### TokenResult

`TokenResult` 预留用于多跳推理中最后阶段采样得到的 token ID。

```rust
pub struct TokenResult {
    pub request_id: u64,
    pub tokens: Vec<u32>,
}
```

### InferenceRequest

当前的播种/请求者流程直接发送提示词文本。播种节点在本地进行分词 (tokenize)。

```rust
pub struct InferenceRequest {
    pub request_id: u64,
    pub prompt_text: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub top_p: f32,
}
```

- `prompt_text` 取代了早期的 token ID 提示词黑客方式。
- `max_tokens` 既是生成限制，也是预检 CU 支付能力的基础。

### TokenStreamMsg

当前的流式响应发送解码后的文本片段，而不是 token ID。

```rust
pub struct TokenStreamMsg {
    pub request_id: u64,
    pub text: String,
    pub is_final: bool,
}
```

- `text` 是适合立即渲染的解码文本片段。
- `is_final = true` 关闭请求的流。

### ErrorMsg

请求范围内的失败将作为类型化错误返回，而不是过载的文本片段。

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

- `request_id` 将错误与正在进行的推理请求关联。
- `retryable` 告诉调用者稍后重试是否合理。
- 当前的播种节点/运行时将其用于无效请求、CU 拒绝、并发饱和以及生成失败。

## 健康与活跃度

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

## 交易签名 (有用工作证明)

Forge 使用双重签名交易来证明计算已执行并已被接收。提供者和消费者都必须签署相同的规范交易字节。

### TradeProposal

由提供者在推理完成后发送。包含交易详情和提供者的 Ed25519 签名。

```rust
pub struct TradeProposal {
    pub request_id: u64,
    pub provider: NodeId,
    pub consumer: NodeId,
    pub cu_amount: u64,
    pub tokens_processed: u64,
    pub timestamp: u64,
    pub model_id: String,
    pub provider_sig: Vec<u8>,  // 64 字节 Ed25519 签名
}
```

### TradeAccept

由消费者发送以共同签署交易。

```rust
pub struct TradeAccept {
    pub request_id: u64,
    pub consumer_sig: Vec<u8>,  // 64 字节 Ed25519 签名
}
```

### TradeGossip

在记录双重签名交易后广播给所有连接的节点。任何节点都可以验证双方签名。

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

### 签名的规范字节

双方签署相同的确定性二进制表示：

```
provider_id (32 bytes) + consumer_id (32 bytes) +
cu_amount (8 bytes LE) + tokens_processed (8 bytes LE) +
timestamp (8 bytes LE) + model_id (variable bytes)
```

### 双重签名流程

```text
提供者 (播种节点)                           消费者 (工人节点)
    |                                         |
    |--- TokenStream (推理) ----------------->|
    |--- TokenStream (最终) ----------------->|
    |                                         |
    |--- TradeProposal (提供者签名) ---------->|
    |                                         |
    |    [消费者验证提供者签名]                 |
    |    [消费者共同签名]                      |
    |                                         |
    |<--- TradeAccept (消费者签名) ------------|
    |                                         |
    [提供者验证双方签名]                       |
    [在账本中记录 SignedTradeRecord]           |
    [广播 TradeGossip 到网格]                 |
```

如果消费者在 5 秒内未响应，提供者将退而记录一笔未签名交易（向后兼容）。

### Gossip 传播

当节点收到 `TradeGossip` 消息时：
1. 验证双方 Ed25519 签名
2. 检查 SHA-256 去重（如果已见过则拒绝）
3. 在本地账本中记录交易
4. 该交易**不会**被再次广播（单跳 gossip 以防止广播风暴）

## 序列化规则

- 控制消息使用 bincode。
- `Forward.tensor_data` 作为原始连续字节传输。
- 封装对于所有消息类型保持统一，以便传输层可以保持通用。
- 运行时拒绝大于 64 MiB 的协议帧。
- 协议不嵌入法币、区块链或交易所结算字段。这些属于协议外集成。

## 连接生命周期

### 当前播种/请求者流程

```text
请求者                          播种节点
  |                              |
  |--- QUIC + 加密 -------------->|
  |--- Hello ------------------->|
  |<-- Welcome ------------------|
  |--- InferenceRequest -------->|
  |                              | [CU 预留]
  |<-- TokenStreamMsg ---------- |
  |<-- TokenStreamMsg ---------- |
  |<-- TokenStreamMsg (最终) --- |
  |<-- TradeProposal ----------- | [提供者签名]
  |--- TradeAccept ------------->| [消费者共同签名]
  |                              | [记录 SignedTradeRecord]
  |                              | [广播 TradeGossip]
```

### 未来的多跳流程

```text
协调器            工人 A           工人 B           最后阶段
    |                |               |                |
    |-- AssignShard->|               |                |
    |-- AssignShard----------------->|               |
    |-- AssignShard---------------------------------->|
    |<-- ShardReady--|               |                |
    |<---------------- ShardReady ---|                |
    |<-------------------------------- ShardReady ---|
    |-- PipelineTopology 向所有节点广播 -------------->|
    |-- Forward ---->|-- Forward ---->|-- TokenResult->|
```

## 版本控制

当前版本: `1`

- 节点通过 `Hello` 和 `Welcome` 通告其版本。
- 参考实现当前假设对等节点兼容，并忽略未知的未来负载。
- 破坏性的线缆更改应增加 `version` 并显式定义降级行为。
