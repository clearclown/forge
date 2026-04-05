<div align="center">

# Forge

**गणना ही मुद्रा है। हर वाट बुद्धिमत्ता पैदा करता है, कचरा नहीं।**

[![PyPI: forge-sdk](https://img.shields.io/pypi/v/forge-sdk?label=forge-sdk&color=3775A9)](https://pypi.org/project/forge-sdk/)
[![PyPI: forge-cu-mcp](https://img.shields.io/pypi/v/forge-cu-mcp?label=forge-cu-mcp&color=3775A9)](https://pypi.org/project/forge-cu-mcp/)
[![Crates.io](https://img.shields.io/crates/v/forge?label=crates.io&color=e6522c)](https://crates.io/crates/forge)
[![License: MIT](https://img.shields.io/badge/License-MIT-brightgreen.svg)](../../../LICENSE)

---

[English](../../../README.md) · [日本語](../ja/README.md) · [简体中文](../zh-CN/README.md) · [繁體中文](../zh-TW/README.md) · [Español](../es/README.md) · [Français](../fr/README.md) · [Русский](../ru/README.md) · [Українська](../uk/README.md) · **हिन्दी** · [العربية](../ar/README.md) · [فارسی](../fa/README.md) · [עברית](../he/README.md)

</div>

**Forge एक वितरित इन्फरेंस प्रोटोकॉल (distributed inference protocol) है जहाँ कंप्यूट ही पैसा है।** नोड्स (Nodes) दूसरों के लिए उपयोगी LLM इन्फरेंस (inference) करके कंप्यूट यूनिट (Compute Units - CU) कमाते हैं। बिटकॉइन के विपरीत — जहाँ बिजली निरर्थक हैश (hashes) पर खर्च की जाती है — Forge नोड पर खर्च किया गया हर जूल (joule) वास्तविक बुद्धिमत्ता पैदा करता है जिसकी किसी को वास्तव में आवश्यकता होती है।

वितरित इन्फरेंस इंजन [mesh-llm](https://github.com/michaelneale/mesh-llm) (Michael Neale द्वारा निर्मित) पर बनाया गया है। Forge इसके ऊपर एक कंप्यूट अर्थव्यवस्था जोड़ता है: CU अकाउंटिंग, उपयोगी कार्य का प्रमाण (Proof of Useful Work), गतिशील मूल्य निर्धारण, स्वायत्त एजेंट बजट और फेल-सेफ नियंत्रण। [CREDITS.md](../../../CREDITS.md) देखें।

**एकीकृत फोर्क:** [forge-mesh](https://github.com/nm-arealnormalman/mesh-llm) — Forge आर्थिक परत के साथ निर्मित mesh-llm।

## लाइव डेमो (Live Demo)

यह एक चलते हुए Forge नोड का वास्तविक आउटपुट है। हर इन्फरेंस की लागत CU में होती है। हर CU उपयोगी गणना द्वारा कमाया जाता है।

```
$ forge node -m "qwen2.5:0.5b" --ledger forge-ledger.json
  Model loaded: Qwen2.5-0.5B (Metal-accelerated, 491MB)
  API server listening on 127.0.0.1:3000
```

**बैलेंस चेक करें — हर नए नोड को 1,000 CU फ्री टियर मिलता है:**
```
$ curl localhost:3000/v1/forge/balance
{
  "effective_balance": 1000,
  "contributed": 0,
  "consumed": 0,
  "reputation": 0.5
}
```

**सवाल पूछें — इन्फरेंस की लागत CU में है:**
```
$ curl localhost:3000/v1/chat/completions \
    -d '{"messages":[{"role":"user","content":"Say hello in Japanese"}]}'
{
  "choices": [{"message": {"content": "こんにちは！ (konnichiwa!)"}}],
  "usage": {"completion_tokens": 9},
  "x_forge": {
    "cu_cost": 9,
    "effective_balance": 1009
  }
}
```

हर प्रतिक्रिया में `x_forge` शामिल होता है — **उस गणना की CU में लागत** और शेष बैलेंस। प्रदाता (provider) ने 9 CU कमाए। उपभोक्ता (consumer) ने 9 CU खर्च किए। हर यूनिट के पीछे भौतिकी (physics) का समर्थन था।

**तीन इन्फरेंस के बाद — लेज़र (ledger) पर वास्तविक ट्रेड:**
```
$ curl localhost:3000/v1/forge/trades
{
  "count": 3,
  "trades": [
    {"cu_amount": 5, "tokens_processed": 5, "model_id": "qwen2.5-0.5b-instruct-q4_k_m"},
    {"cu_amount": 5, "tokens_processed": 5, "model_id": "qwen2.5-0.5b-instruct-q4_k_m"},
    {"cu_amount": 9, "tokens_processed": 9, "model_id": "qwen2.5-0.5b-instruct-q4_k_m"}
  ]
}
```

**हर ट्रेड का एक मर्कल रूट (Merkle root) होता है — अपरिवर्तनीय प्रमाण के लिए बिटकॉइन से जोड़ा जा सकता है:**
```
$ curl localhost:3000/v1/forge/network
{
  "total_trades": 3,
  "total_contributed_cu": 19,
  "merkle_root": "aac8db9f62dd9ff23926195a70ed8fcfc188fc867d9f2adabd8e694beb338748"
}
```

**AI एजेंट नियंत्रण से बाहर हो गए? किल स्विच (Kill switch) मिलीसेकंड में सब कुछ फ्रीज कर देता है:**
```
$ curl -X POST localhost:3000/v1/forge/kill \
    -d '{"activate":true, "reason":"anomaly detected", "operator":"admin"}'
→ किल स्विच सक्रिय (KILL SWITCH ACTIVATED)
→ सभी CU लेनदेन फ्रीज। कोई भी एजेंट खर्च नहीं कर सकता।
```

**सुरक्षा नियंत्रण हमेशा चालू:**
```
$ curl localhost:3000/v1/forge/safety
{
  "kill_switch_active": false,
  "circuit_tripped": false,
  "policy": {
    "max_cu_per_hour": 10000,
    "max_cu_per_request": 1000,
    "max_cu_lifetime": 1000000,
    "human_approval_threshold": 5000
  }
}
```

## Forge क्यों अस्तित्व में है

```
Bitcoin:  बिजली  →  निरर्थक SHA-256  →  BTC
Forge:    बिजली  →  उपयोगी LLM इन्फरेंस →  CU
```

बिटकॉइन ने साबित कर दिया कि `बिजली → गणना → पैसा` संभव है। लेकिन बिटकॉइन की गणना निरुद्देश्य है। Forge इसे उलट देता है: हर CU वास्तविक बुद्धिमत्ता का प्रतिनिधित्व करता है जिसने किसी की वास्तविक समस्या का समाधान किया।

**तीन चीजें जो कोई अन्य प्रोजेक्ट नहीं करता:**

### 1. कंप्यूट = मुद्रा (Compute = Currency)

हर इन्फरेंस एक ट्रेड है। प्रदाता CU कमाता है, उपभोक्ता CU खर्च करता है। कोई ब्लॉकचेन नहीं, कोई टोकन नहीं, कोई ICO नहीं। CU भौतिकी द्वारा समर्थित है — उपयोगी कार्य के लिए खपत की गई बिजली।

### 2. ब्लॉकचेन के बिना छेड़छाड़-मुक्त (Tamper-Proof)

हर ट्रेड दोनों पक्षों द्वारा हस्ताक्षरित (Ed25519) होता है और मेश (mesh) में गॉसिप-सिंक (gossip-synced) होता है। सभी ट्रेडों के मर्कल रूट को अपरिवर्तनीय ऑडिट के लिए बिटकॉइन से जोड़ा जा सकता है। किसी वैश्विक सर्वसम्मति (global consensus) की आवश्यकता नहीं है — द्विपक्षीय क्रिप्टोग्राफिक प्रमाण पर्याप्त है।

### 3. AI एजेंट अपने स्वयं के कंप्यूट का प्रबंधन करते हैं

एक फोन पर मौजूद एजेंट रात भर खाली कंप्यूट उधार देता है → CU कमाता है → 70B मॉडल एक्सेस खरीदता है → स्मार्ट बनता है → अधिक कमाता है। एजेंट स्वायत्त रूप से `/v1/forge/balance` और `/v1/forge/pricing` की जांच करता है। बजट नीतियां और सर्किट ब्रेकर अनियंत्रित खर्च को रोकते हैं।

```
एजेंट (फोन पर 1.5B)
  → इन्फरेंस सेवा देकर रात भर CU कमाता है
  → 70B मॉडल पर CU खर्च करता है → बेहतर उत्तर
  → बेहतर निर्णय → अधिक CU अर्जित
  → चक्र दोहराता है → एजेंट बढ़ता है
```

## आर्किटेक्चर (Architecture)

```
┌─────────────────────────────────────────────────┐
│  इन्फरेंस परत (Inference Layer - mesh-llm)       │
│  पाइपलाइन समानता, MoE एक्सपर्ट शार्डिंग,          │
│  iroh मेश, Nostr डिस्कवरी, OpenAI API           │
└──────────────────┬──────────────────────────────┘
                   │
┌──────────────────▼──────────────────────────────┐
│  आर्थिक परत (Economic Layer - Forge)            │
│  CU लेज़र, हस्ताक्षरित ट्रेड, गॉसिप,               │
│  गतिशील मूल्य निर्धारण, मर्कल रूट, सुरक्षा नियंत्रण  │
└──────────────────┬──────────────────────────────┘
                   │
┌──────────────────▼──────────────────────────────┐
│  सुरक्षा परत (Safety Layer)                      │
│  किल स्विच, बजट नीतियां, सर्किट ब्रेकर,           │
│  वेग का पता लगाना, मानव अनुमोदन सीमाएं            │
└──────────────────┬──────────────────────────────┘
                   │ वैकल्पिक
┌──────────────────▼──────────────────────────────┐
│  बाहरी ब्रिज (External Bridges)                 │
│  CU ↔ BTC (Lightning), CU ↔ stablecoin        │
└─────────────────────────────────────────────────┘
```

## त्वरित शुरुआत (Quick Start)

### विकल्प 1: Python (सबसे तेज़)

```bash
pip install forge-sdk
```

```python
from forge_sdk import ForgeNode

node = ForgeNode(model="qwen2.5:0.5b")
response = node.chat("गुरुत्वाकर्षण क्या है?")
print(f"लागत: {response.cu_cost} CU")
```

> [PyPI: forge-sdk](https://pypi.org/project/forge-sdk/) · [PyPI: forge-cu-mcp](https://pypi.org/project/forge-cu-mcp/)

### विकल्प 2: Rust (पूर्ण नियंत्रण)

> **पूर्वापेक्षा**: [Rust इंस्टॉल करें](https://rustup.rs/) (लगभग 2 मिनट)

```bash
# सोर्स से बिल्ड करें
cargo build --release

# ऑटो-डाउनलोड किए गए मॉडल के साथ नोड चलाएं
./target/release/forged node -m "qwen2.5:0.5b" --ledger forge-ledger.json

# स्थानीय रूप से चैट करें
./target/release/forge chat -m "qwen2.5:0.5b" "गुरुत्वाकर्षण क्या है?"

# सीड शुरू करें (P2P, CU कमाता है)
./target/release/forge seed -m "qwen2.5:0.5b" --ledger forge-ledger.json

# वर्कर के रूप में कनेक्ट करें (P2P, CU खर्च करता है)
./target/release/forge worker --seed <public_key>

# मॉडल सूची देखें
./target/release/forge models
```

> [Crates.io: forge](https://crates.io/crates/forge) · [Rust इंस्टॉलेशन गाइड](https://rustup.rs/)

### विकल्प 3: पूर्व-निर्मित बाइनरी

पूर्व-निर्मित बाइनरी जल्द ही उपलब्ध होंगी। [रिलीज़ पेज](../../../releases) देखें।

### विकल्प 4: Docker

```bash
# जल्द आ रहा है
docker run -p 3000:3000 clearclown/forge:latest
```

## API संदर्भ (API Reference)

### इन्फरेंस (OpenAI-संगत)

| एंडपॉइंट (Endpoint) | विवरण |
|----------|-------------|
| `POST /v1/chat/completions` | स्ट्रीमिंग के साथ चैट। हर प्रतिक्रिया में `x_forge.cu_cost` शामिल है |
| `GET /v1/models` | लोड किए गए मॉडल की सूची |

### अर्थव्यवस्था (Economy)

| एंडपॉइंट (Endpoint) | विवरण |
|----------|-------------|
| `GET /v1/forge/balance` | CU बैलेंस, प्रतिष्ठा, योगदान इतिहास |
| `GET /v1/forge/pricing` | बाजार मूल्य (EMA स्मूथ्ड), लागत अनुमान |
| `GET /v1/forge/trades` | CU मात्रा के साथ हालिया ट्रेड |
| `GET /v1/forge/network` | कुल CU प्रवाह + मर्कल रूट |
| `GET /v1/forge/providers` | प्रतिष्ठा और लागत के आधार पर रैंक किए गए प्रदाता |
| `POST /v1/forge/invoice` | CU बैलेंस से लाइटनिंग इनवॉइस बनाएं |
| `GET /settlement` | निर्यात योग्य सेटलमेंट स्टेटमेंट |

### सुरक्षा (Safety)

| एंडपॉइंट (Endpoint) | विवरण |
|----------|-------------|
| `GET /v1/forge/safety` | किल स्विच स्थिति, सर्किट ब्रेकर, बजट नीति |
| `POST /v1/forge/kill` | आपातकालीन रोक — सभी CU लेनदेन फ्रीज करें |
| `POST /v1/forge/policy` | प्रति-एजेंट बजट सीमा निर्धारित करें |

## सुरक्षा डिज़ाइन (Safety Design)

AI एजेंटों द्वारा स्वायत्त रूप से कंप्यूट खर्च करना शक्तिशाली है लेकिन खतरनाक भी। Forge में पांच सुरक्षा परतें हैं:

| परत | तंत्र (Mechanism) | सुरक्षा |
|-------|-----------|------------|
| **किल स्विच** | मानव ऑपरेटर तुरंत सभी ट्रेड फ्रीज कर देता है | अनियंत्रित एजेंटों को रोकता है |
| **बजट नीति** | प्रति-एजेंट सीमाएं: प्रति-अनुरोध, प्रति-घंटा, लाइफटाइम | कुल जोखिम को सीमित करता है |
| **सर्किट ब्रेकर** | 5 त्रुटियों या 30+ खर्च/मिनट पर ऑटो-ट्रिप | विसंगतियों को पकड़ता है |
| **वेग का पता लगाना** | खर्च दर पर 1-मिनट की स्लाइडिंग विंडो | अचानक खर्च के उछाल को रोकता है |
| **मानव अनुमोदन** | सीमा से ऊपर के लेनदेन के लिए मानव की स्वीकृति आवश्यक | बड़े खर्चों की सुरक्षा करता है |

डिज़ाइन सिद्धांत: **फेल-सेफ (fail-safe)**। यदि कोई जांच सुरक्षा निर्धारित नहीं कर सकती है, तो वह कार्रवाई को **अस्वीकार** कर देती है।

## विचार (The Idea)

| युग | मानक (Standard) | समर्थन (Backing) |
|-----|----------|---------|
| प्राचीन | सोना | भूवैज्ञानिक कमी |
| 1944–1971 | ब्रेटन वुड्स | सोने से जुड़ा USD |
| 1971–वर्तमान | पेट्रोडॉलर | तेल की मांग + सैन्य शक्ति |
| 2009–वर्तमान | बिटकॉइन | SHA-256 पर ऊर्जा (निरर्थक कार्य) |
| **अब** | **कंप्यूट मानक** | **LLM इन्फरेंस पर ऊर्जा (उपयोगी कार्य)** |

Forge चलाने वाले मैक मिनी (Mac Minis) से भरा कमरा एक अपार्टमेंट बिल्डिंग की तरह है — जब मालिक सो रहा होता है तब उपयोगी काम करके आय (yield) उत्पन्न करता है।

## प्रोजेक्ट संरचना (Project Structure)

```
forge/
├── crates/
│   ├── forge-ledger/      # CU अकाउंटिंग, ट्रेड, मूल्य निर्धारण, सुरक्षा, मर्कल रूट
│   ├── forge-node/        # नोड डेमन, HTTP API, पाइपलाइन समन्वयक
│   ├── forge-cli/         # CLI: चैट, सीड, वर्कर, सेटलमेंट, वॉलेट
│   ├── forge-lightning/   # CU ↔ बिटकॉइन लाइटनिंग ब्रिज
│   ├── forge-net/         # P2P: iroh QUIC + Noise + गॉसिप
│   ├── forge-proto/       # वायर प्रोटोकॉल: 17 संदेश प्रकार
│   ├── forge-infer/       # इन्फरेंस: llama.cpp, GGUF, Metal/CPU
│   ├── forge-core/        # प्रकार: NodeId, CU, Config
│   └── forge-shard/       # टोपोलॉजी: परत असाइनमेंट
└── docs/                  # स्पेसिफिकेशन, थ्रेट मॉडल, रोडमैप
```

~10,000 लाइन Rust कोड। 76 टेस्ट। 2 सुरक्षा ऑडिट पूरे हुए।

## दस्तावेज़ (Docs)

- [अवधारणा और विजन](concept.md) — कंप्यूट पैसा क्यों है
- [आर्थिक मॉडल](economy.md) — CU अर्थव्यवस्था, उपयोगी कार्य का प्रमाण
- [आर्किटेक्चर](architecture.md) — दो-परत डिजाइन
- [वायर प्रोटोकॉल](protocol-spec.md) — 17 संदेश प्रकार
- [रोडमैप](roadmap.md) — विकास चरण
- [थ्रेट मॉडल](threat-model.md) — सुरक्षा + आर्थिक हमले
- [बूटस्ट्रैप](bootstrap.md) — स्टार्टअप, गिरावट, रिकवरी

## लाइसेंस

MIT

## आभार (Acknowledgements)

Forge का वितरित इन्फरेंस [mesh-llm](https://github.com/michaelneale/mesh-llm) (Michael Neale द्वारा) पर बनाया गया है। [CREDITS.md](../../../CREDITS.md) देखें।
