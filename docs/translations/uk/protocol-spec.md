# Forge — Специфікація протоколу передачі (Wire Protocol)

## Огляд

Вузли Forge обмінюються серіалізованими через bincode контрольними повідомленнями через зашифровані QUIC-з'єднання, встановлені Iroh. Тензори активації передаються як сирі байти всередині повідомлень `Forward`. Поточна реалізація v1 використовує той самий конверт (envelope) для локального інференсу seed/requester та для майбутніх багатоланкових повідомлень пайплайну.

## Конверт повідомлення (Message Envelope)

Кожне повідомлення загорнуте в конверт:

```rust
pub struct Envelope {
    pub msg_id: u64,
    pub sender: NodeId,
    pub timestamp: u64, // unix millis
    pub payload: Payload,
}
```

Правила валідації, що виконуються поточною середою виконання:
- `Envelope.sender` має відповідати автентифікованій ідентичності віддаленого піра з QUIC-з'єднання
- дублікати `msg_id` від одного й того самого піра відкидаються в межах обмеженого вікна повторів
- `Hello.capability.node_id` та `Welcome.capability.node_id` мають відповідати `Envelope.sender`
- некоректні діапазони шарів та невідповідні довжини тензорів відхиляються до того, як обробники вищого рівня побачать повідомлення
- поля промпту та токенів обмежені (`prompt_text` та `max_tokens`), тому один пір не може просити іншого виділити необмежений обсяг роботи

## Payload Enum

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

## Пошук та рукостискання (Discovery and Handshake)

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

- `version` — версія протоколу, яку заявляє відправник.
- `capability` описує CPU, пам'ять, пропускну здатність та регіон для прийняття рішень щодо планування.
- `known_peers` — це опортуністичний список пірів, а не глобально авторитетний реєстр.

## Призначення шарду (Shard Assignment)

Ці повідомлення визначають майбутній багатоланковий пайплайн шарів. Вони є частиною v1, хоча поточна референсна реалізація в основному запускає інференс повної моделі на сіді.

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

## Повідомлення інференсу

### Forward

`Forward` переносить тензор активації між стадіями пайплайну.

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

- `tensor_data` — це сирі байти активації.
- `dtype` — один із `F16`, `F32` або `I8`.
- Очікується, що транспорт через WAN надаватиме перевагу компактним представленням, таким як `I8`.

### TokenResult

`TokenResult` зарезервовано для згенерованих ID токенів на фінальній стадії багатоланкового інференсу.

```rust
pub struct TokenResult {
    pub request_id: u64,
    pub tokens: Vec<u32>,
}
```

### InferenceRequest

Поточний потік seed/requester надсилає текст промпту безпосередньо. Сід токенізує його локально.

```rust
pub struct InferenceRequest {
    pub request_id: u64,
    pub prompt_text: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub top_p: f32,
}
```

- `prompt_text` замінює попередній хак із промптом у вигляді ID токенів.
- `max_tokens` є як лімітом генерації, так і основою для попередньої перевірки платоспроможності в CU.

### TokenStreamMsg

Поточна стрімінгова відповідь надсилає декодовані фрагменти тексту замість ID токенів.

```rust
pub struct TokenStreamMsg {
    pub request_id: u64,
    pub text: String,
    pub is_final: bool,
}
```

- `text` — декодований фрагмент тексту, придатний для негайного відображення.
- `is_final = true` закриває стрім для запиту.

### ErrorMsg

Помилки рівня запиту повертаються як типізовані помилки, а не перевантажені текстові фрагменти.

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

- `request_id` пов'язує помилку з активним запитом інференсу.
- `retryable` повідомляє викликачу, чи є сенс повторити запит пізніше.
- Поточний сід/рантайм використовує це для невалідних запитів, відхилення через CU, перевантаження та збоїв генерації.

## Здоров'я та активність (Health and Liveness)

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

## Управління кластером

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

## Підпис операцій (Доказ корисної роботи)

Forge використовує операції з подвійним підписом, щоб довести, що обчислення були виконані та отримані. І провайдер, і споживач мають підписати ті самі канонічні байти операції.

### TradeProposal

Надсилається провайдером після завершення інференсу. Містить деталі операції та підпис Ed25519 провайдера.

```rust
pub struct TradeProposal {
    pub request_id: u64,
    pub provider: NodeId,
    pub consumer: NodeId,
    pub cu_amount: u64,
    pub tokens_processed: u64,
    pub timestamp: u64,
    pub model_id: String,
    pub provider_sig: Vec<u8>,  // 64-байтний підпис Ed25519
}
```

### TradeAccept

Надсилається споживачем для контр-підпису операції.

```rust
pub struct TradeAccept {
    pub request_id: u64,
    pub consumer_sig: Vec<u8>,  // 64-байтний підпис Ed25519
}
```

### TradeGossip

Транслюється всім підключеним пірам після запису операції з подвійним підписом. Будь-який вузол може перевірити обидва підписи.

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

### Канонічні байти для підпису

Обидві сторони підписують однакове детерміноване бінарне представлення:

```
provider_id (32 байти) + consumer_id (32 байти) +
cu_amount (8 байтів LE) + tokens_processed (8 байтів LE) +
timestamp (8 байтів LE) + model_id (змінна кількість байтів)
```

### Потік подвійного підпису

```text
Провайдер (Seed)                         Споживач (Worker)
    |                                         |
    |--- TokenStream (інференс) -------------->|
    |--- TokenStream (фінальний) ------------->|
    |                                         |
    |--- TradeProposal (provider_sig) ------->|
    |                                         |
    |    [споживач перевіряє provider_sig]    |
    |    [споживач контр-підписує]             |
    |                                         |
    |<--- TradeAccept (consumer_sig) ---------|
    |                                         |
    [провайдер перевіряє обидва підписи]      |
    [записує SignedTradeRecord у леджер]      |
    [транслює TradeGossip у мережу]           |
```

Якщо споживач не відповідає протягом 5 секунд, провайдер переходить до запису непідписаної операції (зворотна сумісність).

### Поширення через Gossip

Коли вузол отримує повідомлення `TradeGossip`:
1. Перевірити обидва підписи Ed25519
2. Перевірити дедуплікацію SHA-256 (відхилити, якщо вже бачили)
3. Записати операцію в локальний леджер
4. Операція НЕ пересилається далі (одноланковий gossip для запобігання штормам)

## Правила серіалізації

- Контрольні повідомлення використовують bincode.
- `Forward.tensor_data` передається як сирі безперервні байти.
- Конверт залишається однаковим для всіх типів повідомлень, тому транспорти можуть залишатися загальними.
- Рантайм відхиляє кадри протоколу, більші за 64 МіБ.
- Протокол не містить полів для фіату, блокчейну або біржових розрахунків. Вони належать до інтеграцій поза протоколом.

## Життєвий цикл з'єднання

### Поточний потік seed/requester

```text
Реквестер                       Сід
  |                              |
  |--- QUIC + шифрування -------->|
  |--- Hello ------------------->|
  |<-- Welcome ------------------|
  |--- InferenceRequest -------->|
  |                              | [CU зарезервовано]
  |<-- TokenStreamMsg ---------- |
  |<-- TokenStreamMsg ---------- |
  |<-- TokenStreamMsg (final) -- |
  |<-- TradeProposal ----------- | [провайдер підписує]
  |--- TradeAccept ------------->| [споживач підписує]
  |                              | [SignedTradeRecord записано]
  |                              | [TradeGossip транслюється]
```

### Майбутній багатоланковий потік

```text
Координатор       Воркер A        Воркер B        Фінальна стадія
    |                |               |                |
    |-- AssignShard->|               |                |
    |-- AssignShard----------------->|                |
    |-- AssignShard---------------------------------->|
    |<-- ShardReady--|               |                |
    |<---------------- ShardReady ---|                |
    |<-------------------------------- ShardReady ---|
    |-- PipelineTopology транслюється всім ---------->|
    |-- Forward ---->|-- Forward ---->|-- TokenResult->|
```

## Версіонування

Поточна версія: `1`

- Піри заявляють про свою версію через `Hello` та `Welcome`.
- Референсна реалізація наразі припускає сумісність пірів та ігнорує невідомі майбутні навантаження.
- Зміни в протоколі, що порушують сумісність, мають збільшувати `version` та явно визначати поведінку при відкаті версії.
