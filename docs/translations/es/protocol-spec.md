# Forge — Especificación del Protocolo de Red (Wire Protocol)

## Resumen

Los nodos de Forge intercambian mensajes de control serializados con bincode a través de conexiones QUIC cifradas establecidas por Iroh. Los tensores de activación se transportan como bytes puros dentro de los mensajes `Forward`. La implementación actual v1 utiliza el mismo sobre (envelope) para la inferencia local de seed/requester y para futuros mensajes de pipeline multi-salto (multi-hop).

## Sobre del Mensaje (Message Envelope)

Cada mensaje está envuelto en un sobre:

```rust
pub struct Envelope {
    pub msg_id: u64,
    pub sender: NodeId,
    pub timestamp: u64, // milisegundos unix
    pub payload: Payload,
}
```

Reglas de validación aplicadas por el tiempo de ejecución actual:
- `Envelope.sender` debe coincidir con la identidad del par remoto autenticado de la conexión QUIC.
- Los valores de `msg_id` duplicados del mismo par se descartan dentro de una ventana de repetición (replay window) acotada.
- `Hello.capability.node_id` y `Welcome.capability.node_id` deben coincidir con `Envelope.sender`.
- Los rangos de capas mal formados y las longitudes de tensor que no coinciden se rechazan antes de que los manejadores de nivel superior vean el mensaje.
- Los campos de prompt y token están acotados (`prompt_text` y `max_tokens`) para que un par no pueda pedirle a otro que asigne trabajo ilimitado.

## Enum Payload

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

## Descubrimiento y Handshake

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

- `version` es la versión del protocolo anunciada por el remitente.
- `capability` describe la CPU, memoria, ancho de banda y región para las decisiones de programación (scheduling).
- `known_peers` es una lista de pares oportunista, no un registro globalmente autoritativo.

## Asignación de Fragmentos (Shard Assignment)

Estos mensajes definen el futuro pipeline de capas multi-salto. Son parte de la v1 aunque la implementación de referencia actual ejecuta principalmente la inferencia del modelo completo en el seed.

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

## Mensajes de Inferencia

### Forward

`Forward` transporta un tensor de activación entre las etapas del pipeline.

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

- `tensor_data` son los bytes de activación puros.
- `dtype` es uno de `F16`, `F32` o `I8`.
- Se espera que el transporte WAN prefiera representaciones compactas como `I8`.

### TokenResult

`TokenResult` está reservado para los IDs de tokens muestreados en la etapa final en la inferencia multi-salto.

```rust
pub struct TokenResult {
    pub request_id: u64,
    pub tokens: Vec<u32>,
}
```

### InferenceRequest

El flujo actual de seed/requester envía el texto del prompt directamente. El seed realiza la tokenización localmente.

```rust
pub struct InferenceRequest {
    pub request_id: u64,
    pub prompt_text: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub top_p: f32,
}
```

- `prompt_text` reemplaza el hack anterior de prompt por ID de token.
- `max_tokens` es tanto un límite de generación como la base para las comprobaciones de asequibilidad de CU pre-vuelo.

### TokenStreamMsg

La respuesta de streaming actual envía fragmentos de texto decodificados en lugar de IDs de tokens.

```rust
pub struct TokenStreamMsg {
    pub request_id: u64,
    pub text: String,
    pub is_final: bool,
}
```

- `text` es un fragmento de texto decodificado adecuado para renderizado inmediato.
- `is_final = true` cierra el flujo para la solicitud.

### ErrorMsg

Los fallos de alcance de solicitud se devuelven como errores tipados en lugar de fragmentos de texto sobrecargados.

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

- `request_id` vincula el error con la solicitud de inferencia en curso.
- `retryable` indica al remitente si tiene sentido reintentar más tarde.
- El seed/runtime actual utiliza esto para solicitudes inválidas, rechazo de CU, saturación de concurrencia y fallos de generación.

## Salud y Vitalidad (Health and Liveness)

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

## Gestión del Clúster

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

## Firma de Intercambios (Prueba de Trabajo Útil)

Forge utiliza intercambios firmados dualmente para demostrar que el cálculo fue realizado y recibido. Tanto el proveedor como el consumidor deben firmar los mismos bytes canónicos del intercambio.

### TradeProposal

Enviado por el proveedor después de que se completa la inferencia. Contiene los detalles del intercambio y la firma Ed25519 del proveedor.

```rust
pub struct TradeProposal {
    pub request_id: u64,
    pub provider: NodeId,
    pub consumer: NodeId,
    pub cu_amount: u64,
    pub tokens_processed: u64,
    pub timestamp: u64,
    pub model_id: String,
    pub provider_sig: Vec<u8>,  // firma Ed25519 de 64 bytes
}
```

### TradeAccept

Enviado por el consumidor para contra-firmar el intercambio.

```rust
pub struct TradeAccept {
    pub request_id: u64,
    pub consumer_sig: Vec<u8>,  // firma Ed25519 de 64 bytes
}
```

### TradeGossip

Transmitido a todos los pares conectados después de que se registra un intercambio firmado dualmente. Cualquier nodo puede verificar ambas firmas.

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

### Bytes Canónicos para la Firma

Ambas partes firman la misma representación binaria determinista:

```
provider_id (32 bytes) + consumer_id (32 bytes) +
cu_amount (8 bytes LE) + tokens_processed (8 bytes LE) +
timestamp (8 bytes LE) + model_id (bytes variables)
```

### Flujo de Firma Dual

```text
Proveedor (Seed)                         Consumidor (Worker)
    |                                         |
    |--- TokenStream (inferencia) ----------->|
    |--- TokenStream (final) ---------------->|
    |                                         |
    |--- TradeProposal (provider_sig) ------->|
    |                                         |
    |    [consumidor verifica provider_sig]   |
    |    [consumidor contra-firma]            |
    |                                         |
    |<--- TradeAccept (consumer_sig) ---------|
    |                                         |
    [proveedor verifica ambas firmas]         |
    [registra SignedTradeRecord en el libro]  |
    [difunde TradeGossip a la red mesh]       |
```

Si el consumidor no responde en 5 segundos, el proveedor recurre a registrar un intercambio sin firma (compatible con versiones anteriores).

### Propagación de Gossip

Cuando un nodo recibe un mensaje `TradeGossip`:
1. Verifica ambas firmas Ed25519.
2. Comprueba la deduplicación SHA-256 (rechaza si ya se ha visto).
3. Registra el intercambio en el libro (ledger) local.
4. El intercambio NO se vuelve a difundir (gossip de un solo salto para evitar tormentas).

## Reglas de Serialización

- Los mensajes de control utilizan bincode.
- `Forward.tensor_data` se transmite como bytes contiguos puros.
- El sobre permanece uniforme en todos los tipos de mensajes para que los transportes puedan ser genéricos.
- El tiempo de ejecución rechaza tramas de protocolo de más de 64 MiB.
- El protocolo no incluye campos de liquidación fiduciaria, blockchain o intercambio. Esos pertenecen a integraciones fuera del protocolo.

## Ciclo de Vida de la Conexión

### Flujo actual de seed/requester

```text
Requester                       Seed
  |                              |
  |--- QUIC + cifrado ---------->|
  |--- Hello ------------------->|
  |<-- Welcome ------------------|
  |--- InferenceRequest -------->|
  |                              | [CU reservada]
  |<-- TokenStreamMsg ---------- |
  |<-- TokenStreamMsg ---------- |
  |<-- TokenStreamMsg (final) -- |
  |<-- TradeProposal ----------- | [proveedor firma]
  |--- TradeAccept ------------->| [consumidor contra-firma]
  |                              | [SignedTradeRecord registrado]
  |                              | [TradeGossip difundido]
```

### Futuro flujo multi-salto

```text
Coordinador       Worker A        Worker B        Etapa Final
    |                |               |                |
    |-- AssignShard->|               |                |
    |-- AssignShard----------------->|                |
    |-- AssignShard---------------------------------->|
    |<-- ShardReady--|               |                |
    |<---------------- ShardReady ---|                |
    |<-------------------------------- ShardReady ---|
    |-- PipelineTopology difundido a todos ---------->|
    |-- Forward ---->|-- Forward ---->|-- TokenResult->|
```

## Versiones

Versión actual: `1`

- Los pares anuncian su versión a través de `Hello` y `Welcome`.
- La implementación de referencia actualmente asume pares compatibles e ignora cargas útiles futuras desconocidas.
- Los cambios estructurales en el protocolo deben incrementar la `version` y definir el comportamiento de degradación explícitamente.
