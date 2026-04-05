# Forge — 通信プロトコル仕様

## 概要

Forge ノードは、Iroh によって確立された暗号化 QUIC 接続を介して、bincode でシリアル化された制御メッセージを交換します。アクティベーションテンソルは、`Forward` メッセージ内の生のバイトデータとして運ばれます。現在の v1 実装では、ローカルのシード/リクエスター間の推論と、将来のマルチホップパイプラインメッセージの両方で同じエンベロープを使用します。

## メッセージエンベロープ

すべてのメッセージはエンベロープでラップされています：

```rust
pub struct Envelope {
    pub msg_id: u64,
    pub sender: NodeId,
    pub timestamp: u64, // unix millis (ミリ秒)
    pub payload: Payload,
}
```

現在のランタイムで適用される検証ルール：
- `Envelope.sender` は、QUIC 接続から認証されたリモートピアの ID と一致しなければならない。
- 同じピアからの重複した `msg_id` は、制限されたリプレイウィンドウ内で破棄される。
- `Hello.capability.node_id` および `Welcome.capability.node_id` は `Envelope.sender` と一致しなければならない。
- 不正な形式のレイヤー範囲や不一致のテンソル長は、上位のハンドラーに渡される前に拒否される。
- 1つのピアが他方に無制限の作業を割り当てることができないよう、プロンプトとトークンのフィールドは制限される (`prompt_text` および `max_tokens`)。

## ペイロード (Payload) Enum

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

## 探索とハンドシェイク

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

- `version`: 送信者が提示するプロトコルバージョン。
- `capability`: スケジューリング決定のための CPU、メモリ、帯域幅、およびリージョンの記述。
- `known_peers`: 便宜的なピアリストであり、グローバルに信頼されたレジストリではない。

## シャード割り当て

これらのメッセージは、将来のマルチホップレイヤーパイプラインを定義します。現在のリファレンス実装は主にシード上でモデル全体の推論を実行しますが、これらは v1 の一部として含まれています。

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

## 推論メッセージ

### Forward

`Forward` は、パイプラインステージ間でアクティベーションテンソルを運びます。

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

- `tensor_data`: 生のアクティベーションバイトデータ。
- `dtype`: `F16`, `F32`, または `I8` のいずれか。
- WAN 転送では、`I8` のようなコンパクトな表現が好まれることが期待されます。

### TokenResult

`TokenResult` は、マルチホップ推論における最終ステージのサンプリング済みトークン ID のために予約されています。

```rust
pub struct TokenResult {
    pub request_id: u64,
    pub tokens: Vec<u32>,
}
```

### InferenceRequest

現在のシード/リクエスターフローでは、プロンプトテキストを直接送信します。シードがローカルでトークナイズを行います。

```rust
pub struct InferenceRequest {
    pub request_id: u64,
    pub prompt_text: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub top_p: f32,
}
```

- `prompt_text`: 以前のトークン ID プロンプト方式に代わるもの。
- `max_tokens`: 生成制限であると同時に、プリフライト（実行前）の CU 支払能力チェックの根拠となります。

### TokenStreamMsg

現在のストリーミングレスポンスでは、トークン ID ではなくデコードされたテキストフラグメントを送信します。

```rust
pub struct TokenStreamMsg {
    pub request_id: u64,
    pub text: String,
    pub is_final: bool,
}
```

- `text`: 即座にレンダリング可能なデコード済みテキストフラグメント。
- `is_final = true`: リクエストのストリームを終了します。

### ErrorMsg

リクエストスコープの失敗は、テキストフラグメントではなく、型定義されたエラーとして返されます。

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

- `request_id`: エラーを実行中の推論リクエストに関連付けます。
- `retryable`: 後で再試行することが妥当かどうかを呼び出し側に伝えます。
- 現在のシード/ランタイムは、無効なリクエスト、CU 拒否、並列処理の飽和、および生成の失敗にこれを使用します。

## ヘルスチェックと生存確認

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

## クラスタ管理

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

## 取引署名 (有益な仕事の証明)

Forge は、計算が実行され、受け取られたことを証明するために、二重署名された取引を使用します。プロバイダーとコンシューマーの両方が、同じカノニカル（標準的）な取引バイトデータに署名する必要があります。

### TradeProposal (取引提案)

推論完了後にプロバイダーによって送信されます。取引の詳細とプロバイダーの Ed25519 署名が含まれます。

```rust
pub struct TradeProposal {
    pub request_id: u64,
    pub provider: NodeId,
    pub consumer: NodeId,
    pub cu_amount: u64,
    pub tokens_processed: u64,
    pub timestamp: u64,
    pub model_id: String,
    pub provider_sig: Vec<u8>,  // 64バイトの Ed25519 署名
}
```

### TradeAccept (取引承諾)

取引に副署（カウンターサイン）するためにコンシューマーによって送信されます。

```rust
pub struct TradeAccept {
    pub request_id: u64,
    pub consumer_sig: Vec<u8>,  // 64バイトの Ed25519 署名
}
```

### TradeGossip (取引ゴシップ)

二重署名された取引が記録された後、接続されているすべてのピアにブロードキャストされます。どのノードでも両方の署名を検証できます。

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

### 署名用の標準バイトデータ (Canonical Bytes)

両当事者は、同じ決定論的なバイナリ表現に署名します：

```
provider_id (32 bytes) + consumer_id (32 bytes) +
cu_amount (8 bytes LE) + tokens_processed (8 bytes LE) +
timestamp (8 bytes LE) + model_id (variable bytes)
```

### 二重署名のフロー

```text
プロバイダー (シード)                       コンシューマー (ワーカー)
    |                                         |
    |--- TokenStream (推論) ----------------->|
    |--- TokenStream (最終) ----------------->|
    |                                         |
    |--- TradeProposal (プロバイダー署名) ---->|
    |                                         |
    |    [コンシューマーがプロバイダー署名を検証] |
    |    [コンシューマーが副署を行う]           |
    |                                         |
    |<--- TradeAccept (コンシューマー署名) ----|
    |                                         |
    [プロバイダーが両方の署名を検証]           |
    [SignedTradeRecord を台帳に記録]          |
    [TradeGossip をメッシュにブロードキャスト] |
```

コンシューマーが 5 秒以内に応答しない場合、プロバイダーは未署名の取引の記録にフォールバックします（後方互換性のため）。

### ゴシップの伝播

ノードが `TradeGossip` メッセージを受信した場合：
1. 両方の Ed25519 署名を検証する。
2. SHA-256 による重複排除をチェックする（既知の場合は拒否）。
3. 取引をローカル台帳に記録する。
4. 取引は再ブロードキャスト**されない**（ブロードキャストストームを防ぐためのシングルホップゴシップ）。

## シリアル化ルール

- 制御メッセージには bincode を使用。
- `Forward.tensor_data` は生の連続したバイトとして送信。
- トランスポート層を汎用的に保つため、エンベロープはすべてのメッセージタイプで共通。
- ランタイムは 64 MiB を超えるプロトコルフレームを拒否。
- プロトコルには法定通貨、ブロックチェーン、または交換決済のフィールドを含まない。これらはプロトコル外の統合に属する。

## 接続ライフサイクル

### 現在のシード/リクエスターフロー

```text
リクエスター                      シード
  |                              |
  |--- QUIC + 暗号化 ----------->|
  |--- Hello ------------------->|
  |<-- Welcome ------------------|
  |--- InferenceRequest -------->|
  |                              | [CU 予約]
  |<-- TokenStreamMsg ---------- |
  |<-- TokenStreamMsg ---------- |
  |<-- TokenStreamMsg (最終) --- |
  |<-- TradeProposal ----------- | [プロバイダー署名]
  |--- TradeAccept ------------->| [コンシューマー副署]
  |                              | [SignedTradeRecord 記録]
  |                              | [TradeGossip ブロードキャスト]
```

### 将来のマルチホップフロー

```text
コーディネーター     ワーカー A       ワーカー B       最終ステージ
    |                |               |                |
    |-- AssignShard->|               |                |
    |-- AssignShard----------------->|               |
    |-- AssignShard---------------------------------->|
    |<-- ShardReady--|               |                |
    |<---------------- ShardReady ---|                |
    |<-------------------------------- ShardReady ---|
    |-- PipelineTopology 全員へブロードキャスト ------>|
    |-- Forward ---->|-- Forward ---->|-- TokenResult->|
```

## バージョニング

現在のバージョン: `1`

- ピアは `Hello` および `Welcome` を通じてバージョンを通知。
- リファレンス実装は現在、互換性のあるピアを想定し、未知の将来のペイロードは無視する。
- 破壊的なワイヤ変更では `version` をインクリメントし、ダウングレード時の動作を明示的に定義しなければならない。
