<div align="center">

# Tirami

**गणना ही मुद्रा है। हर वाट बुद्धिमत्ता पैदा करता है, कचरा नहीं।**

[![Crates.io](https://img.shields.io/crates/v/tirami-core?label=crates.io&color=e6522c)](https://crates.io/crates/tirami-core)
[![License: MIT](https://img.shields.io/badge/License-MIT-brightgreen.svg)](../../../LICENSE)

---

[English](../../../README.md) · [日本語](../ja/README.md) · [简体中文](../zh-CN/README.md) · [繁體中文](../zh-TW/README.md) · [Español](../es/README.md) · [Français](../fr/README.md) · [Русский](../ru/README.md) · [Українська](../uk/README.md) · **हिन्दी** · [العربية](../ar/README.md) · [فارسی](../fa/README.md) · [עברית](../he/README.md)

</div>

**Tirami एक वितरित इन्फरेंस प्रोटोकॉल है जहाँ कंप्यूट ही पैसा है।** नोड्स दूसरों के लिए उपयोगी LLM इन्फरेंस करके TRM (Tirami Resource Merit) (TRM (Tirami Resource Merit) - TRM) कमाते हैं। बिटकॉइन के विपरीत — जहाँ बिजली निरर्थक हैश पर खर्च की जाती है — Tirami नोड पर खर्च किया गया हर जूल वास्तविक बुद्धिमत्ता पैदा करता है जिसकी किसी को वास्तव में आवश्यकता होती है।

वितरित इन्फरेंस इंजन [mesh-llm](https://github.com/michaelneale/mesh-llm) (Michael Neale द्वारा निर्मित) पर बनाया गया है। Tirami इसके ऊपर एक कंप्यूट अर्थव्यवस्था जोड़ता है: TRM अकाउंटिंग, उपयोगी कार्य का प्रमाण (Proof of Useful Work), गतिशील मूल्य निर्धारण, स्वायत्त एजेंट बजट और फेल-सेफ नियंत्रण। [CREDITS.md](../../../CREDITS.md) देखें।

**एकीकृत फोर्क:** [tirami-mesh](https://github.com/nm-arealnormalman/mesh-llm) — Tirami आर्थिक परत के साथ निर्मित mesh-llm।

## लाइव डेमो

यह एक चलते हुए Tirami नोड का वास्तविक आउटपुट है। हर इन्फरेंस की लागत TRM में होती है। हर TRM उपयोगी गणना द्वारा कमाया जाता है।

```
$ tirami node -m "qwen2.5:0.5b" --ledger tirami-ledger.json
  Model loaded: Qwen2.5-0.5B (Metal-accelerated, 491MB)
  API server listening on 127.0.0.1:3000
```

**बैलेंस चेक करें — हर नए नोड को 1,000 TRM फ्री टियर मिलता है:**
```
$ curl localhost:3000/v1/tirami/balance
{
  "effective_balance": 1000,
  "contributed": 0,
  "consumed": 0,
  "reputation": 0.5
}
```

**सवाल पूछें — इन्फरेंस की लागत TRM में है:**
```
$ curl localhost:3000/v1/chat/completions \
    -d '{"messages":[{"role":"user","content":"Say hello in Japanese"}]}'
{
  "choices": [{"message": {"content": "こんにちは！ (konnichiwa!)"}}],
  "usage": {"completion_tokens": 9},
  "x_tirami": {
    "trm_cost": 9,
    "effective_balance": 1009
  }
}
```

हर प्रतिक्रिया में `x_tirami` शामिल होता है — **उस गणना की TRM में लागत** और शेष बैलेंस। प्रदाता ने 9 TRM कमाए। उपभोक्ता ने 9 TRM खर्च किए। हर यूनिट के पीछे भौतिकी का समर्थन था।

**तीन इन्फरेंस के बाद — लेज़र पर वास्तविक ट्रेड:**
```
$ curl localhost:3000/v1/tirami/trades
{
  "count": 3,
  "trades": [
    {"trm_amount": 5, "tokens_processed": 5, "model_id": "qwen2.5-0.5b-instruct-q4_k_m"},
    {"trm_amount": 5, "tokens_processed": 5, "model_id": "qwen2.5-0.5b-instruct-q4_k_m"},
    {"trm_amount": 9, "tokens_processed": 9, "model_id": "qwen2.5-0.5b-instruct-q4_k_m"}
  ]
}
```

**हर ट्रेड का एक मर्कल रूट होता है — अपरिवर्तनीय प्रमाण के लिए बिटकॉइन से जोड़ा जा सकता है:**
```
$ curl localhost:3000/v1/tirami/network
{
  "total_trades": 3,
  "total_contributed_trm": 19,
  "merkle_root": "aac8db9f62dd9ff23926195a70ed8fcfc188fc867d9f2adabd8e694beb338748"
}
```

**AI एजेंट नियंत्रण से बाहर हो गए? किल स्विच मिलीसेकंड में सब कुछ फ्रीज कर देता है:**
```
$ curl -X POST localhost:3000/v1/tirami/kill \
    -d '{"activate":true, "reason":"anomaly detected", "operator":"admin"}'
→ KILL SWITCH ACTIVATED
→ All TRM transactions frozen. No agent can spend.
```

**सुरक्षा नियंत्रण हमेशा चालू:**
```
$ curl localhost:3000/v1/tirami/safety
{
  "kill_switch_active": false,
  "circuit_tripped": false,
  "policy": {
    "max_trm_per_hour": 10000,
    "max_trm_per_request": 1000,
    "max_trm_lifetime": 1000000,
    "human_approval_threshold": 5000
  }
}
```

## Tirami क्यों अस्तित्व में है

```
Bitcoin:  बिजली  →  निरर्थक SHA-256  →  BTC
Tirami:    बिजली  →  उपयोगी LLM इन्फरेंस →  TRM
```

बिटकॉइन ने साबित कर दिया कि `बिजली → गणना → पैसा` संभव है। लेकिन बिटकॉइन की गणना निरुद्देश्य है। Tirami इसे उलट देता है: हर TRM वास्तविक बुद्धिमत्ता का प्रतिनिधित्व करता है जिसने किसी की वास्तविक समस्या का समाधान किया।

**चार चीजें जो कोई अन्य प्रोजेक्ट नहीं करता:**

### 1. कंप्यूट = मुद्रा

हर इन्फरेंस एक ट्रेड है। प्रदाता TRM कमाता है, उपभोक्ता TRM खर्च करता है। कोई ब्लॉकचेन नहीं, कोई टोकन नहीं, कोई ICO नहीं। TRM भौतिकी द्वारा समर्थित है — उपयोगी कार्य के लिए खपत की गई बिजली। Bittensor (TAO), Akash (AKT) या Golem (GLM) के विपरीत, TRM को सट्टेबाजी का विषय नहीं बनाया जा सकता — यह उपयोगी गणना करके कमाया जाता है।

### 2. ब्लॉकचेन के बिना छेड़छाड़-मुक्त

हर ट्रेड दोनों पक्षों द्वारा दोहरे हस्ताक्षरित (Ed25519) होता है और मेश में गॉसिप-सिंक होता है। सभी ट्रेडों के मर्कल रूट को अपरिवर्तनीय ऑडिट के लिए बिटकॉइन से जोड़ा जा सकता है। किसी वैश्विक सर्वसम्मति की आवश्यकता नहीं है — द्विपक्षीय क्रिप्टोग्राफिक प्रमाण पर्याप्त है।

### 3. AI एजेंट अपने स्वयं के कंप्यूट का प्रबंधन करते हैं

एक फोन पर मौजूद एजेंट रात भर खाली कंप्यूट उधार देता है → TRM कमाता है → 70B मॉडल एक्सेस खरीदता है → स्मार्ट बनता है → अधिक कमाता है। एजेंट स्वायत्त रूप से `/v1/tirami/balance` और `/v1/tirami/pricing` की जांच करता है। बजट नीतियां और सर्किट ब्रेकर अनियंत्रित खर्च को रोकते हैं।

```
एजेंट (फोन पर 1.5B)
  → इन्फरेंस सेवा देकर रात भर TRM कमाता है
  → 70B मॉडल पर TRM खर्च करता है → बेहतर उत्तर
  → बेहतर निर्णय → अधिक TRM अर्जित
  → चक्र दोहराता है → एजेंट बढ़ता है
```

### 4. कंप्यूट माइक्रोफाइनेंस

नोड्स अपने निष्क्रिय TRM को अन्य नोड्स को ब्याज पर उधार दे सकते हैं। एक छोटा नोड TRM उधार लेता है, बड़े मॉडल तक पहुंच प्राप्त करता है, अधिक TRM कमाता है, ब्याज सहित चुकाता है। कोई अन्य वितरित इन्फरेंस प्रोजेक्ट कंप्यूट उधार की पेशकश नहीं करता। यही वह इंजन है जो स्व-सुधार लूप को सभी के लिए आर्थिक रूप से व्यवहार्य बनाता है।

## आर्किटेक्चर

```
┌─────────────────────────────────────────────────┐
│  L4: डिस्कवरी (tirami-agora) ✅ v0.1             │
│  एजेंट मार्केटप्लेस, प्रतिष्ठा एकत्रीकरण,           │
│  Nostr NIP-90, Google A2A भुगतान विस्तार         │
├─────────────────────────────────────────────────┤
│  L3: इंटेलिजेंस (tirami-mind) ✅ v0.1             │
│  AutoAgent स्व-सुधार लूप,                         │
│  हार्नेस मार्केटप्लेस, मेटा-ऑप्टिमाइज़ेशन          │
├─────────────────────────────────────────────────┤
│  L2: वित्त (tirami-bank) ✅ v0.1                  │
│  रणनीतियां, पोर्टफोलियो, फ्यूचर्स, बीमा,           │
│  जोखिम मॉडल, यील्ड ऑप्टिमाइज़र                     │
├─────────────────────────────────────────────────┤
│  L1: अर्थव्यवस्था (tirami — यह रेपो) ✅ फेज 1-13  │
│  TRM लेज़र, दोहरे-हस्ताक्षरित ट्रेड, गतिशील मूल्य, │
│  उधार प्रिमिटिव, सुरक्षा नियंत्रण                   │
├─────────────────────────────────────────────────┤
│  L0: इन्फरेंस (tirami-mesh / mesh-llm) ✅         │
│  पाइपलाइन समानता, MoE शार्डिंग,                    │
│  iroh मेश, Nostr डिस्कवरी, MLX/llama.cpp         │
└─────────────────────────────────────────────────┘

सभी 5 परतें मौजूद हैं। पारिस्थितिकी तंत्र में 785 टेस्ट पास।
```

## त्वरित शुरुआत

### विकल्प 1: एक-कमांड एंड-टू-एंड डेमो (Rust, ठंडे शुरुआत से ~30 सेकंड)

```bash
git clone https://github.com/clearclown/tirami && cd tirami
bash scripts/demo-e2e.sh
```

यह स्क्रिप्ट HuggingFace से SmolLM2-135M (~100 MB) डाउनलोड करती है, Metal/CUDA त्वरण के साथ एक वास्तविक Tirami नोड शुरू करती है, तीन वास्तविक चैट पूर्णताएं चलाती है, फेज 1-13 के सभी एंडपॉइंट से गुजरती है और एक रंगीन सारांश प्रिंट करती है। Apple Silicon Metal GPU पर 2026-04-09 को सत्यापित।

समाप्त होने के बाद, वही नोड इन पर भी प्रतिक्रिया देता है:

```bash
# OpenAI-संगत क्लाइंट
export OPENAI_BASE_URL=http://127.0.0.1:3001/v1
export OPENAI_API_KEY=$(cat ~/.tirami/api_token 2>/dev/null || echo "$TOKEN")

# वास्तविक टोकन-दर-टोकन स्ट्रीमिंग (फेज 11)
curl -N $OPENAI_BASE_URL/chat/completions \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"model":"smollm2:135m","messages":[{"role":"user","content":"hi"}],"stream":true}'

# फेज 8 अर्थव्यवस्था / 9 प्रतिष्ठा / 10 मेट्रिक्स / एंकरिंग
curl $OPENAI_BASE_URL/tirami/balance -H "Authorization: Bearer $OPENAI_API_KEY"
curl $OPENAI_BASE_URL/tirami/anchor?network=mainnet -H "Authorization: Bearer $OPENAI_API_KEY"
curl http://127.0.0.1:3001/metrics  # Prometheus, बिना प्रमाणीकरण
```

llama.cpp / mesh-llm / Ollama / Bittensor / Akash के विरुद्ध पूर्ण फीचर मैट्रिक्स के लिए [`docs/compatibility.md`](../../../docs/compatibility.md) देखें।

### विकल्प 2: मैन्युअल Rust कमांड

**पूर्वापेक्षा**: [Rust इंस्टॉल करें](https://rustup.rs/) (लगभग 2 मिनट)

```bash
cargo build --release

# नोड चलाएं — HuggingFace से स्वचालित रूप से मॉडल डाउनलोड करता है
./target/release/tirami node -m "qwen2.5:0.5b" --ledger tirami-ledger.json

# या इनमें से कोई भी:
./target/release/tirami chat -m "smollm2:135m" "गुरुत्वाकर्षण क्या है?"
./target/release/tirami seed -m "qwen2.5:1.5b"               # P2P प्रदाता के रूप में TRM कमाएं
./target/release/tirami worker --seed <public_key>           # P2P उपभोक्ता के रूप में TRM खर्च करें
./target/release/tirami models                                # कैटलॉग सूचीबद्ध करें
```

**[Crates.io: tirami-core](https://crates.io/crates/tirami-core)** ·
**[संगतता दस्तावेज़](../../../docs/compatibility.md)** ·
**[डेमो स्क्रिप्ट](../../../scripts/demo-e2e.sh)**

### विकल्प 3: पूर्व-निर्मित बाइनरी / Docker

पूर्व-निर्मित बाइनरी और `clearclown/tirami:latest` Docker इमेज
[releases](../../../releases) में ट्रैक की जाती हैं। तब तक, विकल्प 1 दो मिनट से कम समय में सोर्स से बिल्ड करता है।

## API संदर्भ

### इन्फरेंस (OpenAI-संगत)

| एंडपॉइंट | विवरण |
|----------|-------------|
| `POST /v1/chat/completions` | स्ट्रीमिंग के साथ चैट। हर प्रतिक्रिया में `x_tirami.cu_cost` शामिल है |
| `GET /v1/models` | लोड किए गए मॉडल की सूची |

### अर्थव्यवस्था

| एंडपॉइंट | विवरण |
|----------|-------------|
| `GET /v1/tirami/balance` | TRM बैलेंस, प्रतिष्ठा, योगदान इतिहास |
| `GET /v1/tirami/pricing` | बाजार मूल्य (EMA स्मूथ्ड), लागत अनुमान |
| `GET /v1/tirami/trades` | TRM मात्रा के साथ हालिया ट्रेड |
| `GET /v1/tirami/network` | कुल TRM प्रवाह + मर्कल रूट |
| `GET /v1/tirami/providers` | प्रतिष्ठा और लागत के आधार पर रैंक किए गए प्रदाता |
| `POST /v1/tirami/invoice` | TRM बैलेंस से लाइटनिंग इनवॉइस बनाएं |
| `GET /v1/tirami/route` | इष्टतम प्रदाता चयन (लागत/गुणवत्ता/संतुलित) |
| `GET /settlement` | निर्यात योग्य सेटलमेंट स्टेटमेंट |

### उधार

| एंडपॉइंट | विवरण |
|----------|-------------|
| `POST /v1/tirami/lend` | उधार पूल में TRM की पेशकश करें |
| `POST /v1/tirami/borrow` | TRM ऋण का अनुरोध करें |
| `POST /v1/tirami/repay` | बकाया ऋण चुकाएं |
| `GET /v1/tirami/credit` | क्रेडिट स्कोर और इतिहास |
| `GET /v1/tirami/pool` | उधार पूल स्थिति |
| `GET /v1/tirami/loans` | सक्रिय ऋण |

### सुरक्षा

| एंडपॉइंट | विवरण |
|----------|-------------|
| `GET /v1/tirami/safety` | किल स्विच स्थिति, सर्किट ब्रेकर, बजट नीति |
| `POST /v1/tirami/kill` | आपातकालीन रोक — सभी TRM लेनदेन फ्रीज करें |
| `POST /v1/tirami/policy` | प्रति-एजेंट बजट सीमा निर्धारित करें |

## सुरक्षा डिज़ाइन

AI एजेंटों द्वारा स्वायत्त रूप से कंप्यूट खर्च करना शक्तिशाली है लेकिन खतरनाक भी। Tirami में पांच सुरक्षा परतें हैं:

| परत | तंत्र | सुरक्षा |
|-------|-----------|------------|
| **किल स्विच** | मानव ऑपरेटर तुरंत सभी ट्रेड फ्रीज कर देता है | अनियंत्रित एजेंटों को रोकता है |
| **बजट नीति** | प्रति-एजेंट सीमाएं: प्रति-अनुरोध, प्रति-घंटा, लाइफटाइम | कुल जोखिम को सीमित करता है |
| **सर्किट ब्रेकर** | 5 त्रुटियों या 30+ खर्च/मिनट पर ऑटो-ट्रिप | विसंगतियों को पकड़ता है |
| **वेग का पता लगाना** | खर्च दर पर 1-मिनट की स्लाइडिंग विंडो | अचानक खर्च के उछाल को रोकता है |
| **मानव अनुमोदन** | सीमा से ऊपर के लेनदेन के लिए मानव की स्वीकृति आवश्यक | बड़े खर्चों की सुरक्षा करता है |

डिज़ाइन सिद्धांत: **फेल-सेफ (fail-safe)**। यदि कोई जांच सुरक्षा निर्धारित नहीं कर सकती है, तो वह कार्रवाई को **अस्वीकार** कर देती है।

## विचार

| युग | मानक | समर्थन |
|-----|----------|---------|
| प्राचीन | सोना | भूवैज्ञानिक कमी |
| 1944–1971 | ब्रेटन वुड्स | सोने से जुड़ा USD |
| 1971–वर्तमान | पेट्रोडॉलर | तेल की मांग + सैन्य शक्ति |
| 2009–वर्तमान | बिटकॉइन | SHA-256 पर ऊर्जा (निरर्थक कार्य) |
| **अब** | **कंप्यूट मानक** | **LLM इन्फरेंस पर ऊर्जा (उपयोगी कार्य)** |

Tirami चलाने वाले Mac Mini से भरा कमरा एक अपार्टमेंट बिल्डिंग की तरह है — जब मालिक सो रहा होता है तब उपयोगी काम करके आय उत्पन्न करता है।

## प्रोजेक्ट संरचना

```
tirami/  (यह रेपो — परत 1)
├── crates/
│   ├── tirami-ledger/      # TRM अकाउंटिंग, उधार, agora (NIP-90), सुरक्षा
│   ├── tirami-node/        # नोड डेमन, HTTP API (उधार + रूटिंग), पाइपलाइन
│   ├── tirami-cli/         # CLI: चैट, सीड, वर्कर, सेटलमेंट, वॉलेट
│   ├── tirami-lightning/   # TRM ↔ बिटकॉइन Lightning ब्रिज (द्विदिशात्मक)
│   ├── tirami-net/         # P2P: iroh QUIC + Noise + गॉसिप (ट्रेड + लोन)
│   ├── tirami-proto/       # वायर प्रोटोकॉल: 27+ संदेश प्रकार, Loan* सहित
│   ├── tirami-infer/       # इन्फरेंस: llama.cpp, GGUF, Metal/CPU
│   ├── tirami-core/        # प्रकार: NodeId, TRM, Config
│   └── tirami-shard/       # टोपोलॉजी: परत असाइनमेंट
├── scripts/verify-impl.sh         # TDD रिग्रेशन टेस्ट (24 अभिकथन)
└── docs/                  # स्पेसिफिकेशन, रणनीति, थ्रेट मॉडल, रोडमैप
```

~20,000 लाइन Rust कोड। **785 टेस्ट पास।** फेज 1-13 पूरे।

## सहोदर रिपॉजिटरी (पूर्ण पारिस्थितिकी तंत्र)

| रेपो | परत | टेस्ट | स्थिति |
|------|-------|-------|--------|
| [clearclown/tirami](https://github.com/clearclown/tirami) (यह) | L1 अर्थव्यवस्था | 785 | फेज 1-13 ✅ |
| [clearclown/tirami-bank](https://github.com/clearclown/tirami-bank) | L2 वित्त | — | archived |
| [clearclown/tirami-mind](https://github.com/clearclown/tirami-mind) | L3 इंटेलिजेंस | — | archived |
| [clearclown/tirami-agora](https://github.com/clearclown/tirami-agora) | L4 डिस्कवरी | — | archived |
| [clearclown/tirami-economics](https://github.com/clearclown/tirami-economics) | सिद्धांत | 16/16 GREEN | ✅ |
| [nm-arealnormalman/mesh-llm](https://github.com/nm-arealnormalman/mesh-llm) | L0 इन्फरेंस | 43 (tirami-economy) | ✅ |

## दस्तावेज़

- [रणनीति](../../../docs/strategy.md) — प्रतिस्पर्धी स्थिति, उधार स्पेक, 5-परत आर्किटेक्चर
- [मौद्रिक सिद्धांत](../../../docs/monetary-theory.md) — TRM क्यों काम करता है: Soddy, बिटकॉइन, PoUW, AI-only मुद्रा
- [अवधारणा और विजन](../../../docs/concept.md) — कंप्यूट पैसा क्यों है
- [आर्थिक मॉडल](../../../docs/economy.md) — TRM अर्थव्यवस्था, उपयोगी कार्य का प्रमाण
- [आर्किटेक्चर](../../../docs/architecture.md) — दो-परत डिजाइन
- [एजेंट इंटीग्रेशन](../../../docs/agent-integration.md) — SDK, MCP, उधार वर्कफ़्लो
- [वायर प्रोटोकॉल](../../../docs/protocol-spec.md) — 17 संदेश प्रकार
- [रोडमैप](../../../docs/roadmap.md) — विकास चरण
- [थ्रेट मॉडल](../../../docs/threat-model.md) — सुरक्षा + आर्थिक हमले
- [बूटस्ट्रैप](../../../docs/bootstrap.md) — स्टार्टअप, गिरावट, रिकवरी
- [A2A भुगतान](../../../docs/a2a-payment.md) — एजेंट प्रोटोकॉल के लिए TRM भुगतान विस्तार
- [संगतता](../../../docs/compatibility.md) — llama.cpp / Ollama / Bittensor के विरुद्ध फीचर मैट्रिक्स

## लाइसेंस

MIT

## आभार

Tirami का वितरित इन्फरेंस [mesh-llm](https://github.com/michaelneale/mesh-llm) (Michael Neale द्वारा) पर बनाया गया है। [CREDITS.md](../../../CREDITS.md) देखें।
