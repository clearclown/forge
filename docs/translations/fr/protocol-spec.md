# Forge — Spécification du Protocole de Réseau (Wire Protocol)

## Aperçu

Les nœuds Forge échangent des messages de contrôle sérialisés avec bincode via des connexions QUIC cryptées établies par Iroh. Les tenseurs d'activation sont transportés sous forme d'octets bruts à l'intérieur des messages `Forward`. L'implémentation actuelle v1 utilise la même enveloppe pour l'inférence locale seed/requester et pour les futurs messages de pipeline multi-sauts (multi-hop).

## Enveloppe de Message (Message Envelope)

Chaque message est enveloppé dans une enveloppe :

```rust
pub struct Envelope {
    pub msg_id: u64,
    pub sender: NodeId,
    pub timestamp: u64, // millisecondes unix
    pub payload: Payload,
}
```

Règles de validation appliquées par le moteur d'exécution actuel :
- `Envelope.sender` doit correspondre à l'identité du pair distant authentifié de la connexion QUIC.
- Les valeurs `msg_id` en double provenant du même pair sont rejetées dans une fenêtre de rejeu (replay window) limitée.
- `Hello.capability.node_id` et `Welcome.capability.node_id` doivent correspondre à `Envelope.sender`.
- Les plages de couches mal formées et les longueurs de tenseurs discordantes sont rejetées avant que les gestionnaires de niveau supérieur ne voient le message.
- Les champs prompt et token sont limités (`prompt_text` et `max_tokens`) afin qu'un pair ne puisse pas demander à un autre d'allouer un travail illimité.

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

## Découverte et Handshake

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

- `version` est la version du protocole annoncée par l'expéditeur.
- `capability` décrit le CPU, la mémoire, la bande passante et la région pour les décisions d'ordonnancement (scheduling).
- `known_peers` est une liste de pairs opportuniste, pas un registre faisant autorité au niveau mondial.

## Assignation de Fragments (Shard Assignment)

Ces messages définissent le futur pipeline de couches multi-sauts. Ils font partie de la v1 même si l'implémentation de référence actuelle exécute principalement l'inférence du modèle complet sur le seed.

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

## Messages d'Inférence

### Forward

`Forward` transporte un tenseur d'activation entre les étapes du pipeline.

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

- `tensor_data` sont les octets d'activation bruts.
- `dtype` est l'un de `F16`, `F32` ou `I8`.
- Le transport WAN est censé préférer des représentations compactes telles que `I8`.

### TokenResult

`TokenResult` est réservé aux IDs de tokens échantillonnés à l'étape finale dans l'inférence multi-sauts.

```rust
pub struct TokenResult {
    pub request_id: u64,
    pub tokens: Vec<u32>,
}
```

### InferenceRequest

Le flux actuel seed/requester envoie directement le texte du prompt. Le seed effectue la tokenisation localement.

```rust
pub struct InferenceRequest {
    pub request_id: u64,
    pub prompt_text: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub top_p: f32,
}
```

- `prompt_text` remplace l'ancien hack de prompt par ID de token.
- `max_tokens` est à la fois une limite de génération et la base des vérifications d'abordabilité des CU avant le vol.

### TokenStreamMsg

La réponse en streaming actuelle envoie des fragments de texte décodés plutôt que des IDs de tokens.

```rust
pub struct TokenStreamMsg {
    pub request_id: u64,
    pub text: String,
    pub is_final: bool,
}
```

- `text` est un fragment de texte décodé adapté à un rendu immédiat.
- `is_final = true` ferme le flux pour la requête.

### ErrorMsg

Les échecs au niveau de la requête sont renvoyés sous forme d'erreurs typées plutôt que de fragments de texte surchargés.

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

- `request_id` lie l'erreur à la requête d'inférence en cours.
- `retryable` indique à l'appelant s'il est judicieux de réessayer plus tard.
- Le seed/runtime actuel utilise ceci pour les requêtes invalides, le rejet de CU, la saturation de la concurrence et les échecs de génération.

## Santé et Vitalité (Health and Liveness)

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

## Gestion du Cluster

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

## Signature d'Échanges (Preuve de Travail Utile)

Forge utilise des échanges à double signature pour prouver que le calcul a été effectué et reçu. Le fournisseur et le consommateur doivent tous deux signer les mêmes octets canoniques de l'échange.

### TradeProposal

Envoyé par le fournisseur une fois l'inférence terminée. Contient les détails de l'échange et la signature Ed25519 du fournisseur.

```rust
pub struct TradeProposal {
    pub request_id: u64,
    pub provider: NodeId,
    pub consumer: NodeId,
    pub cu_amount: u64,
    pub tokens_processed: u64,
    pub timestamp: u64,
    pub model_id: String,
    pub provider_sig: Vec<u8>,  // signature Ed25519 de 64 octets
}
```

### TradeAccept

Envoyé par le consommateur pour contre-signer l'échange.

```rust
pub struct TradeAccept {
    pub request_id: u64,
    pub consumer_sig: Vec<u8>,  // signature Ed25519 de 64 octets
}
```

### TradeGossip

Diffusé à tous les pairs connectés après l'enregistrement d'un échange à double signature. Tout nœud peut vérifier les deux signatures.

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

### Octets Canoniques pour la Signature

Les deux parties signent la même représentation binaire déterministe :

```
provider_id (32 octets) + consumer_id (32 octets) +
cu_amount (8 octets LE) + tokens_processed (8 octets LE) +
timestamp (8 octets LE) + model_id (octets variables)
```

### Flux de Double Signature

```text
Fournisseur (Seed)                      Consommateur (Worker)
    |                                         |
    |--- TokenStream (inférence) ------------>|
    |--- TokenStream (final) ---------------->|
    |                                         |
    |--- TradeProposal (provider_sig) ------->|
    |                                         |
    |    [le consommateur vérifie provider_sig]|
    |    [le consommateur contre-signe]       |
    |                                         |
    |<--- TradeAccept (consumer_sig) ---------|
    |                                         |
    [le fournisseur vérifie les deux sigs]    |
    [enregistre SignedTradeRecord dans le livre]|
    [diffuse TradeGossip au réseau mesh]      |
```

Si le consommateur ne répond pas dans les 5 secondes, le fournisseur se replie sur l'enregistrement d'un échange non signé (rétrocompatible).

### Propagation du Gossip

Lorsqu'un nœud reçoit un message `TradeGossip` :
1. Vérifier les deux signatures Ed25519.
2. Vérifier la déduplication SHA-256 (rejeter si déjà vu).
3. Enregistrer l'échange dans le livre (ledger) local.
4. L'échange n'est PAS rediffusé (gossip à saut unique pour éviter les tempêtes).

## Règles de Sérialisation

- Les messages de contrôle utilisent bincode.
- `Forward.tensor_data` est transmis sous forme d'octets contigus bruts.
- L'enveloppe reste uniforme pour tous les types de messages afin que les transports puissent rester génériques.
- Le moteur d'exécution rejette les trames de protocole supérieures à 64 MiB.
- Le protocole n'intègre pas de champs de règlement fiduciaire, de blockchain ou d'échange. Ceux-ci appartiennent à des intégrations hors protocole.

## Cycle de Vie de la Connexion

### Flux actuel seed/requester

```text
Demandeur                       Seed
  |                              |
  |--- QUIC + cryptage --------->|
  |--- Hello ------------------->|
  |<-- Welcome ------------------|
  |--- InferenceRequest -------->|
  |                              | [CU réservée]
  |<-- TokenStreamMsg ---------- |
  |<-- TokenStreamMsg ---------- |
  |<-- TokenStreamMsg (final) -- |
  |<-- TradeProposal ----------- | [le fournisseur signe]
  |--- TradeAccept ------------->| [le consommateur contre-signe]
  |                              | [SignedTradeRecord enregistré]
  |                              | [TradeGossip diffusé]
```

### Futur flux multi-sauts

```text
Coordinateur      Worker A        Worker B        Étape Finale
    |                |               |                |
    |-- AssignShard->|               |                |
    |-- AssignShard----------------->|                |
    |-- AssignShard---------------------------------->|
    |<-- ShardReady--|               |                |
    |<---------------- ShardReady ---|                |
    |<-------------------------------- ShardReady ---|
    |-- PipelineTopology diffusé à tous ------------->|
    |-- Forward ---->|-- Forward ---->|-- TokenResult->|
```

## Versionnage

Version actuelle : `1`

- Les pairs annoncent leur version via `Hello` et `Welcome`.
- L'implémentation de référence suppose actuellement des pairs compatibles et ignore les futures charges utiles inconnues.
- Les changements structurels du protocole doivent incrémenter la `version` et définir explicitement le comportement de dégradation.
