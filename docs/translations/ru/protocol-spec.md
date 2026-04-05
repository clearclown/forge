# Forge — Спецификация протокола передачи

## Обзор

Узлы Forge обмениваются сообщениями управления, сериализованными в bincode, через зашифрованные QUIC-соединения, устанавливаемые Iroh. Тензоры активации передаются как сырые байты внутри сообщений `Forward`. Текущая реализация v1 использует один и тот же конверт для локального инференса (seed/requester) и для будущих многошаговых сообщений пайплайна.

## Конверт сообщения (Envelope)

Каждое сообщение упаковано в конверт:

```rust
pub struct Envelope {
    pub msg_id: u64,
    pub sender: NodeId,
    pub timestamp: u64, // unix millis
    pub payload: Payload,
}
```

Правила валидации, применяемые текущей средой выполнения:
- `Envelope.sender` должен соответствовать аутентифицированному идентификатору удаленного пира из QUIC-соединения.
- дублирующиеся значения `msg_id` от одного и того же пира отбрасываются в рамках ограниченного окна защиты от повторов.
- `Hello.capability.node_id` и `Welcome.capability.node_id` должны совпадать с `Envelope.sender`.
- некорректные диапазоны слоев и несовпадающие длины тензоров отклоняются до того, как обработчики более высокого уровня увидят сообщение.
- поля промпта и токенов ограничены (`prompt_text` и `max_tokens`), чтобы один пир не мог заставить другого выделить неограниченный объем работы.

## Перечисление полезной нагрузки (Payload Enum)

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

## Поиск и рукопожатие (Handshake)

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

- `version` — версия протокола, объявляемая отправителем.
- `capability` — описывает CPU, память, пропускную способность и регион для принятия решений о планировании.
- `known_peers` — оппортунистический список пиров, не является глобально авторитетным реестром.

## Назначение шардов

Эти сообщения определяют будущий многошаговый пайплайн слоев. Они являются частью v1, хотя текущая эталонная реализация в основном запускает инференс всей модели на сиде (seed).

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

## Сообщения инференса

### Forward

`Forward` переносит тензор активации между стадиями пайплайна.

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

- `tensor_data` — сырые байты активации.
- `dtype` — один из `F16`, `F32` или `I8`.
- Ожидается, что WAN-транспорт будет предпочитать компактные представления, такие как `I8`.

### TokenResult

`TokenResult` зарезервирован для ID сэмплированных токенов финальной стадии в многошаговом инференсе.

```rust
pub struct TokenResult {
    pub request_id: u64,
    pub tokens: Vec<u32>,
}
```

### InferenceRequest

Текущий поток seed/requester отправляет текст промпта напрямую. Сид выполняет токенизацию локально.

```rust
pub struct InferenceRequest {
    pub request_id: u64,
    pub prompt_text: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub top_p: f32,
}
```

- `prompt_text` заменяет ранний хак с промптом в виде ID токенов.
- `max_tokens` является одновременно лимитом генерации и основой для предварительной проверки платежеспособности в CU.

### TokenStreamMsg

Текущий потоковый ответ отправляет декодированные фрагменты текста, а не ID токенов.

```rust
pub struct TokenStreamMsg {
    pub request_id: u64,
    pub text: String,
    pub is_final: bool,
}
```

- `text` — декодированный фрагмент текста, пригодный для немедленного отображения.
- `is_final = true` закрывает поток для данного запроса.

### ErrorMsg

Ошибки на уровне запроса возвращаются как типизированные ошибки, а не перегруженные фрагменты текста.

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

- `request_id` связывает ошибку с запущенным запросом инференса.
- `retryable` сообщает вызывающей стороне, имеет ли смысл повторить попытку позже.
- Текущий сид/среда выполнения использует это для невалидных запросов, отклонения по CU, насыщения параллелизма и сбоев генерации.

## Здоровье и живучесть

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

## Управление кластером

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

## Подписание сделок (Proof of Useful Work)

Forge использует сделки с двойной подписью, чтобы доказать, что вычисления были выполнены и получены. И провайдер, и потребитель должны подписать одни и те же канонические байты сделки.

### TradeProposal

Отправляется провайдером после завершения инференса. Содержит детали сделки и Ed25519-подпись провайдера.

```rust
pub struct TradeProposal {
    pub request_id: u64,
    pub provider: NodeId,
    pub consumer: NodeId,
    pub cu_amount: u64,
    pub tokens_processed: u64,
    pub timestamp: u64,
    pub model_id: String,
    pub provider_sig: Vec<u8>,  // 64-байтная подпись Ed25519
}
```

### TradeAccept

Отправляется потребителем для подтверждения сделки своей подписью.

```rust
pub struct TradeAccept {
    pub request_id: u64,
    pub consumer_sig: Vec<u8>,  // 64-байтная подпись Ed25519
}
```

### TradeGossip

Транслируется всем подключенным пирам после записи сделки с двойной подписью. Любой узел может проверить обе подписи.

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

### Канонические байты для подписания

Обе стороны подписывают одно и то же детерминированное бинарное представление:

```
provider_id (32 байта) + consumer_id (32 байта) +
cu_amount (8 байт LE) + tokens_processed (8 байт LE) +
timestamp (8 байт LE) + model_id (переменное количество байт)
```

### Поток двойного подписания (Dual-Sign Flow)

```text
Провайдер (Seed)                         Потребитель (Worker)
    |                                         |
    |--- TokenStream (инференс) -------------->|
    |--- TokenStream (финал) ----------------->|
    |                                         |
    |--- TradeProposal (provider_sig) ------->|
    |                                         |
    |    [потребитель проверяет provider_sig] |
    |    [потребитель ставит контр-подпись]   |
    |                                         |
    |<--- TradeAccept (consumer_sig) ---------|
    |                                         |
    [провайдер проверяет обе подписи]         |
    [записывает SignedTradeRecord в леджер]   |
    [транслирует TradeGossip в сеть]          |
```

Если потребитель не отвечает в течение 5 секунд, провайдер переходит к записи неподписанной сделки (обратная совместимость).

### Распространение Gossip

Когда узел получает сообщение `TradeGossip`:
1. Проверяет обе подписи Ed25519.
2. Проверяет дедупликацию SHA-256 (отклоняет, если уже видел).
3. Записывает сделку в локальный леджер.
4. Сделка НЕ пересылается дальше (gossip на один переход для предотвращения штормов).

## Правила сериализации

- Сообщения управления используют bincode.
- `Forward.tensor_data` передается как сырые непрерывные байты.
- Конверт остается единообразным для всех типов сообщений, чтобы транспорт мог оставаться универсальным.
- Среда выполнения отклоняет кадры протокола размером более 64 МиБ.
- Протокол не содержит полей для фиата, блокчейна или биржевых расчетов. Они относятся к внепротокольным интеграциям.

## Жизненный цикл соединения

### Текущий поток seed/requester

```text
Requester                       Seed
  |                              |
  |--- QUIC + шифрование ------->|
  |--- Hello ------------------->|
  |<-- Welcome ------------------|
  |--- InferenceRequest -------->|
  |                              | [CU зарезервированы]
  |<-- TokenStreamMsg ---------- |
  |<-- TokenStreamMsg ---------- |
  |<-- TokenStreamMsg (финал) -- |
  |<-- TradeProposal ----------- | [провайдер подписывает]
  |--- TradeAccept ------------->| [потребитель подтверждает]
  |                              | [SignedTradeRecord записан]
  |                              | [TradeGossip разослан]
```

### Будущий многошаговый поток

```text
Координатор      Воркер А        Воркер Б        Финальная стадия
    |                |               |                |
    |-- AssignShard->|               |                |
    |-- AssignShard----------------->|                |
    |-- AssignShard---------------------------------->|
    |<-- ShardReady--|               |                |
    |<---------------- ShardReady ---|                |
    |<-------------------------------- ShardReady ---|
    |-- Рассылка PipelineTopology всем -------------->|
    |-- Forward ---->|-- Forward ---->|-- TokenResult->|
```

## Версионирование

Текущая версия: `1`

- Пиры объявляют свою версию через `Hello` и `Welcome`.
- Эталонная реализация в настоящее время предполагает совместимость пиров и игнорирует неизвестные будущие нагрузки.
- Критические изменения в протоколе должны увеличивать `version` и явно определять поведение при понижении версии.
