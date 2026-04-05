# Forge — वायर प्रोटोकॉल स्पेसिफिकेशन (Wire Protocol Specification)

## अवलोकन (Overview)

Forge नोड्स Iroh द्वारा स्थापित एन्क्रिप्टेड QUIC कनेक्शन पर bincode-serialized कंट्रोल संदेशों का आदान-प्रदान करते हैं। एक्टिवेशन टेंसर्स (Activation tensors) को `Forward` संदेशों के भीतर कच्चे बाइट्स (raw bytes) के रूप में ले जाया जाता है। वर्तमान v1 कार्यान्वयन स्थानीय सीड/अनुरोधकर्ता इन्फरेंस और भविष्य के मल्टी-हॉप पाइपलाइन संदेशों के लिए एक ही लिफाफे (envelope) का उपयोग करता है।

## संदेश लिफाफा (Message Envelope)

प्रत्येक संदेश एक लिफाफे में लिपटा होता है:

```rust
pub struct Envelope {
    pub msg_id: u64,
    pub sender: NodeId,
    pub timestamp: u64, // unix millis
    pub payload: Payload,
}
```

वर्तमान रनटाइम द्वारा लागू सत्यापन नियम:
- `Envelope.sender` QUIC कनेक्शन से प्रमाणित रिमोट पीयर पहचान (peer identity) से मेल खाना चाहिए।
- एक ही पीयर से डुप्लिकेट `msg_id` मानों को एक निश्चित रीप्ले विंडो के भीतर छोड़ दिया जाता है।
- `Hello.capability.node_id` और `Welcome.capability.node_id` `Envelope.sender` से मेल खाने चाहिए।
- खराब लेयर रेंज और बेमेल टेंसर लंबाई को उच्च-स्तरीय हैंडलर द्वारा संदेश देखने से पहले ही अस्वीकार कर दिया जाता है।
- प्रॉम्ट और टोकन फ़ील्ड सीमित हैं (`prompt_text` और `max_tokens`), ताकि एक पीयर दूसरे से असीमित कार्य के आवंटन के लिए न कह सके।

## पेलोड एनम (Payload Enum)

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

## डिस्कवरी और हैंडशेक (Discovery and Handshake)

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

- `version` भेजने वाले द्वारा विज्ञापित प्रोटोकॉल वर्जन है।
- `capability` शेड्यूलिंग निर्णयों के लिए CPU, मेमोरी, बैंडविड्थ और क्षेत्र का वर्णन करता है।
- `known_peers` एक अवसरवादी पीयर सूची है, न कि वैश्विक रूप से आधिकारिक रजिस्ट्री।

## शार्ड असाइनमेंट (Shard Assignment)

ये संदेश भविष्य की मल्टी-हॉप लेयर पाइपलाइन को परिभाषित करते हैं। वे v1 का हिस्सा हैं, भले ही वर्तमान संदर्भ कार्यान्वयन मुख्य रूप से सीड पर पूरे-मॉडल का इन्फरेंस चलाता है।

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

## इन्फरेंस संदेश (Inference Messages)

### फॉरवर्ड (Forward)

`Forward` पाइपलाइन चरणों के बीच एक एक्टिवेशन टेंसर ले जाता है।

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

- `tensor_data` कच्चे एक्टिवेशन बाइट्स हैं।
- `dtype` `F16`, `F32`, या `I8` में से एक है।
- WAN ट्रांसपोर्ट से `I8` जैसे कॉम्पैक्ट प्रतिनिधित्व को प्राथमिकता देने की उम्मीद की जाती है।

### टोकन रिज़ल्ट (TokenResult)

`TokenResult` मल्टी-हॉप इन्फरेंस में अंतिम-चरण के सैंपल किए गए टोकन ID के लिए आरक्षित है।

```rust
pub struct TokenResult {
    pub request_id: u64,
    pub tokens: Vec<u32>,
}
```

### इन्फरेंस रिक्वेस्ट (InferenceRequest)

वर्तमान सीड/अनुरोधकर्ता प्रवाह सीधे प्रॉम्ट टेक्स्ट भेजता है। सीड स्थानीय रूप से टोकनाइज़ (tokenize) करता है।

```rust
pub struct InferenceRequest {
    pub request_id: u64,
    pub prompt_text: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub top_p: f32,
}
```

- `prompt_text` पहले के टोकन-ID प्रॉम्ट हैक की जगह लेता है।
- `max_tokens` एक जनरेशन सीमा है और प्री-फ़्लाइट CU खर्च वहन करने की क्षमता जांचने का आधार भी है।

### टोकन स्ट्रीम संदेश (TokenStreamMsg)

वर्तमान स्ट्रीमिंग रिस्पॉन्स टोकन ID के बजाय डिकोड किए गए टेक्स्ट के टुकड़े भेजता है।

```rust
pub struct TokenStreamMsg {
    pub request_id: u64,
    pub text: String,
    pub is_final: bool,
}
```

- `text` एक डिकोड किया गया टेक्स्ट टुकड़ा है जो तुरंत रेंडर करने के लिए उपयुक्त है।
- `is_final = true` अनुरोध के लिए स्ट्रीम को बंद कर देता है।

### एरर मैसेज (ErrorMsg)

अनुरोध-स्तरीय विफलताओं को टेक्स्ट टुकड़ों के बजाय टाइप्ड एरर के रूप में लौटाया जाता है।

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

- `request_id` त्रुटि को चल रहे इन्फरेंस अनुरोध से जोड़ता है।
- `retryable` कॉल करने वाले को बताता है कि क्या बाद में पुनः प्रयास करना समझदारी है।
- वर्तमान सीड/रनटाइम इसका उपयोग अमान्य अनुरोधों, CU अस्वीकृति, कंकेंसी सैचुरेशन और जनरेशन विफलताओं के लिए करता है।

## स्वास्थ्य और जीवंतता (Health and Liveness)

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

## क्लस्टर प्रबंधन (Cluster Management)

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

## ट्रेड साइनिंग (Trade Signing - उपयोगी कार्य का प्रमाण)

Forge यह साबित करने के लिए द्विपक्षीय हस्ताक्षरित ट्रेडों का उपयोग करता है कि गणना की गई थी और प्राप्त हुई थी। प्रदाता और उपभोक्ता दोनों को एक ही कैनोनिकल (canonical) ट्रेड बाइट्स पर हस्ताक्षर करने चाहिए।

### ट्रेड प्रस्ताव (TradeProposal)

इन्फरेंस पूरा होने के बाद प्रदाता द्वारा भेजा जाता है। इसमें ट्रेड विवरण और प्रदाता का Ed25519 हस्ताक्षर होता है।

```rust
pub struct TradeProposal {
    pub request_id: u64,
    pub provider: NodeId,
    pub consumer: NodeId,
    pub cu_amount: u64,
    pub tokens_processed: u64,
    pub timestamp: u64,
    pub model_id: String,
    pub provider_sig: Vec<u8>,  // 64-बाइट Ed25519 हस्ताक्षर
}
```

### ट्रेड स्वीकृति (TradeAccept)

ट्रेड पर प्रति-हस्ताक्षर करने के लिए उपभोक्ता द्वारा भेजा जाता है।

```rust
pub struct TradeAccept {
    pub request_id: u64,
    pub consumer_sig: Vec<u8>,  // 64-बाइट Ed25519 हस्ताक्षर
}
```

### ट्रेड गॉसिप (TradeGossip)

द्विपक्षीय हस्ताक्षरित ट्रेड रिकॉर्ड होने के बाद सभी जुड़े हुए पीयर्स को प्रसारित (broadcast) किया जाता है। कोई भी नोड दोनों हस्ताक्षरों को सत्यापित कर सकता है।

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

### हस्ताक्षर के लिए कैनोनिकल बाइट्स (Canonical Bytes for Signing)

दोनों पक्ष एक ही नियतात्मक बाइनरी प्रतिनिधित्व (deterministic binary representation) पर हस्ताक्षर करते हैं:

```
provider_id (32 बाइट्स) + consumer_id (32 बाइट्स) +
cu_amount (8 बाइट्स LE) + tokens_processed (8 बाइट्स LE) +
timestamp (8 बाइट्स LE) + model_id (वैरिएबल बाइट्स)
```

### द्विपक्षीय हस्ताक्षर प्रवाह (Dual-Sign Flow)

```text
प्रदाता (Seed)                          उपभोक्ता (Worker)
    |                                         |
    |--- TokenStream (इन्फरेंस) -------------->|
    |--- TokenStream (अंतिम) ---------------->|
    |                                         |
    |--- TradeProposal (provider_sig) ------->|
    |                                         |
    |    [उपभोक्ता provider_sig सत्यापित करता है] |
    |    [उपभोक्ता प्रति-हस्ताक्षर करता है]        |
    |                                         |
    |<--- TradeAccept (consumer_sig) ---------|
    |                                         |
    [प्रदाता दोनों हस्ताक्षरों को सत्यापित करता है] |
    [लेज़र में SignedTradeRecord रिकॉर्ड करता है]   |
    [मेश को TradeGossip प्रसारित करता है]         |
```

यदि उपभोक्ता 5 सेकंड के भीतर जवाब नहीं देता है, तो प्रदाता एक बिना हस्ताक्षरित ट्रेड रिकॉर्ड करने पर वापस आ जाता है (पिछली संगतता - backward compatible)।

### गॉसिप प्रसार (Gossip Propagation)

जब किसी नोड को `TradeGossip` संदेश प्राप्त होता है:
1. दोनों Ed25519 हस्ताक्षरों को सत्यापित करें।
2. SHA-256 डिडुप्लीकेशन (deduplication) की जांच करें (यदि पहले देख चुके हैं तो अस्वीकार करें)।
3. स्थानीय लेज़र में ट्रेड रिकॉर्ड करें।
4. ट्रेड को दोबारा प्रसारित नहीं किया जाता है (नेटवर्क तूफानों को रोकने के लिए सिंगल-हॉप गॉसिप)।

## सीरियलाइजेशन नियम (Serialization Rules)

- कंट्रोल संदेश bincode का उपयोग करते हैं।
- `Forward.tensor_data` कच्चे निरंतर बाइट्स (contiguous bytes) के रूप में प्रेषित किया जाता है।
- लिफाफा सभी संदेश प्रकारों में समान रहता है ताकि ट्रांसपोर्ट जेनेरिक (generic) रह सकें।
- रनटाइम 64 MiB से बड़े प्रोटोकॉल फ्रेम को अस्वीकार कर देता है।
- प्रोटोकॉल में फिएट, ब्लॉकचेन या एक्सचेंज सेटलमेंट फ़ील्ड शामिल नहीं हैं। वे ऑफ-प्रोटोकॉल एकीकरण का हिस्सा हैं।

## कनेक्शन लाइफसाइकिल (Connection Lifecycle)

### वर्तमान सीड/अनुरोधकर्ता प्रवाह

```text
अनुरोधकर्ता (Requester)           सीड (Seed)
  |                              |
  |--- QUIC + एन्क्रिप्शन -------->|
  |--- Hello ------------------->|
  |<-- Welcome ------------------|
  |--- InferenceRequest -------->|
  |                              | [CU आरक्षित]
  |<-- TokenStreamMsg ---------- |
  |<-- TokenStreamMsg ---------- |
  |<-- TokenStreamMsg (अंतिम) --- |
  |<-- TradeProposal ----------- | [प्रदाता हस्ताक्षर करता है]
  |--- TradeAccept ------------->| [उपभोक्ता प्रति-हस्ताक्षर करता है]
  |                              | [SignedTradeRecord रिकॉर्ड किया गया]
  |                              | [TradeGossip प्रसारित]
```

### भविष्य का मल्टी-हॉप प्रवाह

```text
कोऑर्डिनेटर       वर्कर A        वर्कर B        अंतिम चरण
    |                |               |                |
    |-- AssignShard->|               |                |
    |-- AssignShard----------------->|               |                |
    |-- AssignShard---------------------------------->|
    |<-- ShardReady--|               |                |
    |<---------------- ShardReady ---|                |
    |<-------------------------------- ShardReady ---|
    |-- PipelineTopology सभी को प्रसारित ----------->|
    |-- Forward ---->|-- Forward ---->|-- TokenResult->|
```

## वर्जनिंग (Versioning)

वर्तमान वर्जन: `1`

- पीयर्स `Hello` और `Welcome` के माध्यम से अपने वर्जन का विज्ञापन करते हैं।
- संदर्भ कार्यान्वयन वर्तमान में संगत पीयर्स मानता है और अज्ञात पेलोड को अनदेखा करता है।
- प्रोटोकॉल में बड़े बदलावों पर `version` को बढ़ाया जाना चाहिए और डाउनग्रेड व्यवहार को स्पष्ट रूप से परिभाषित करना चाहिए।
