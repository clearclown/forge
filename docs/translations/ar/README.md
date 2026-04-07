<div align="center">

# Forge

**الحوسبة هي العملة. كل واط ينتج ذكاءً، وليس نفايات.**

[![PyPI: forge-sdk](https://img.shields.io/pypi/v/forge-sdk?label=forge-sdk&color=3775A9)](https://pypi.org/project/forge-sdk/)
[![PyPI: forge-cu-mcp](https://img.shields.io/pypi/v/forge-cu-mcp?label=forge-cu-mcp&color=3775A9)](https://pypi.org/project/forge-cu-mcp/)
[![Crates.io](https://img.shields.io/crates/v/forge?label=crates.io&color=e6522c)](https://crates.io/crates/forge)
[![License: MIT](https://img.shields.io/badge/License-MIT-brightgreen.svg)](../../../LICENSE)

---

[English](../../../README.md) · [日本語](../ja/README.md) · [简体中文](../zh-CN/README.md) · [繁體中文](../zh-TW/README.md) · [Español](../es/README.md) · [Français](../fr/README.md) · [Русский](../ru/README.md) · [Українська](../uk/README.md) · [हिन्दी](../hi/README.md) · **العربية** · [فارسی](../fa/README.md) · [עברית](../he/README.md)

</div>

**Forge هو بروتوكول استدلال موزّع حيث تكون الحوسبة هي المال.** تكسب العقد (Nodes) وحدات حوسبة (CU) من خلال أداء استدلال LLM مفيد للآخرين. على عكس بيتكوين — حيث يتم حرق الكهرباء على هاشات بلا معنى — فإن كل جول يتم إنفاقه على عقدة Forge ينتج ذكاءً حقيقياً يحتاجه شخص ما بالفعل.

محرك الاستدلال الموزع مبني على [mesh-llm](https://github.com/michaelneale/mesh-llm) بواسطة مايكل نيل (Michael Neale). يضيف Forge اقتصاداً حوسبياً فوقه: محاسبة CU، إثبات العمل المفيد (Proof of Useful Work)، التسعير الديناميكي، ميزانيات الوكلاء المستقلين، وضوابط السلامة. انظر [CREDITS.md](../../../CREDITS.md).

**نسخة مدمجة (Integrated fork):** [forge-mesh](https://github.com/nm-arealnormalman/mesh-llm) — وهو mesh-llm مع طبقة Forge الاقتصادية المدمجة.

## عرض حي (Live Demo)

هذا مخرج حقيقي من عقدة Forge قيد التشغيل. كل استدلال يكلف CU. يتم كسب كل CU من خلال حوسبة مفيدة.

```
$ forge node -m "qwen2.5:0.5b" --ledger forge-ledger.json
  Model loaded: Qwen2.5-0.5B (Metal-accelerated, 491MB)
  API server listening on 127.0.0.1:3000
```

**تحقق من الرصيد — تحصل كل عقدة جديدة على ١,٠٠٠ CU كفئة مجانية:**
```
$ curl localhost:3000/v1/forge/balance
{
  "effective_balance": 1000,
  "contributed": 0,
  "consumed": 0,
  "reputation": 0.5
}
```

**اسأل سؤالاً — الاستدلال يكلف CU:**
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

يتضمن كل رد `x_forge` — **تكلفة تلك الحوسبة بوحدات CU** والرصيد المتبقي. كسب المزود ٩ CU. أنفق المستهلك ٩ CU. الفيزياء دعمت كل وحدة.

**بعد ثلاثة استدلالات — صفقات حقيقية في دفتر الحسابات:**
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

**كل صفقة لها جذر ميركل (Merkle root) — يمكن ربطه ببيتكوين لإثبات غير قابل للتغيير:**
```
$ curl localhost:3000/v1/forge/network
{
  "total_trades": 3,
  "total_contributed_cu": 19,
  "merkle_root": "aac8db9f62dd9ff23926195a70ed8fcfc188fc867d9f2adabd8e694beb338748"
}
```

**وكلاء ذكاء اصطناعي خارج السيطرة؟ مفتاح القطع (Kill switch) يجمد كل شيء في أجزاء من الثانية:**
```
$ curl -X POST localhost:3000/v1/forge/kill \
    -d '{"activate":true, "reason":"anomaly detected", "operator":"admin"}'
← KILL SWITCH ACTIVATED
← تم تجميد جميع معاملات CU. لا يمكن لأي وكيل الإنفاق.
```

**ضوابط السلامة تعمل دائماً:**
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

## لماذا يوجد Forge؟

```
Bitcoin:  electricity  →  meaningless SHA-256  →  BTC
Forge:    electricity  →  useful LLM inference →  CU
```

أثبت بيتكوين أن `الكهرباء ← الحوسبة ← المال`. لكن حوسبة بيتكوين بلا هدف. Forge يقلب هذه الآية: كل وحدة CU تمثل ذكاءً حقيقياً حل مشكلة حقيقية لشخص ما.

**أربعة أشياء لا يفعلها أي مشروع آخر:**

### ١. الحوسبة = عملة

كل استدلال هو صفقة. المزود يكسب CU، والمستهلك ينفق CU. لا يوجد بلوكشين، لا توجد عملة رقمية (Token)، لا يوجد ICO. وحدة CU مدعومة بالفيزياء — الكهرباء المستهلكة للعمل المفيد.

### ٢. مقاوم للتلاعب بدون بلوكشين

كل صفقة موقعة بشكل مزدوج (Ed25519) من قبل الطرفين ومزامنة عبر الشبكة (Gossip-synced). يمكن ربط جذر ميركل لجميع الصفقات ببيتكوين للتدقيق غير القابل للتغيير. لا حاجة لإجماع عالمي — الإثبات التشفيري الثنائي كافٍ.

### ٣. وكلاء الذكاء الاصطناعي يديرون حوسبتهم الخاصة

وكيل على هاتف يقرض حوسبة خاملة طوال الليل ← يكسب CU ← يشتري وصولاً لنموذج 70B ← يصبح أذكى ← يكسب أكثر. يتحقق الوكيل من `/v1/forge/balance` و `/v1/forge/pricing` بشكل مستقل. سياسات الميزانية وقواطع الدائرة تمنع الإنفاق الجامح.

```
الوكيل (1.5B على الهاتف)
  ← يكسب CU طوال الليل من خلال تقديم الاستدلال
  ← ينفق CU على نموذج 70B ← إجابات أذكى
  ← قرارات أفضل ← كسب المزيد من CU
  ← تتكرر الدورة ← ينمو الوكيل
```

### ٤. التمويل الأصغر للحوسبة

يمكن للعقد إقراض وحدات CU الخاملة إلى عقد أخرى بفائدة. تقترض عقدة صغيرة CU، وتصل إلى نموذج أكبر، وتكسب المزيد من CU، وتسدد مع الفائدة. لا يقدم أي مشروع استدلال موزع آخر إقراض الحوسبة — وقد تم تأكيد ذلك من خلال التحليل التنافسي لكل مشروع رئيسي في هذا المجال. هذا هو المحرك الذي يجعل حلقة التحسين الذاتي قابلة للتطبيق اقتصادياً للجميع، وليس فقط لأولئك الذين يمتلكون بالفعل أجهزة قوية.

## الهندسة المعمارية (Architecture)

<div dir="ltr">

```
┌─────────────────────────────────────────────────┐
│  L4: Discovery (forge-agora)                    │
│  Agent marketplace, reputation aggregation,     │
│  Nostr NIP-90, Google A2A payment extension     │
├─────────────────────────────────────────────────┤
│  L3: Intelligence (forge-mind)                  │
│  AutoAgent self-improvement loops,              │
│  harness marketplace, meta-optimization         │
├─────────────────────────────────────────────────┤
│  L2: Finance (forge-bank)                       │
│  CU lending, yield optimization, credit scoring │
├─────────────────────────────────────────────────┤
│  L1: Economy (forge — this repo)                │
│  CU ledger, dual-signed trades, dynamic pricing,│
│  lending primitives, safety controls            │
├─────────────────────────────────────────────────┤
│  L0: Inference (forge-mesh / mesh-llm)          │
│  Pipeline parallelism, MoE sharding,            │
│  iroh mesh, Nostr discovery, MLX/llama.cpp      │
└─────────────────────────────────────────────────┘
```

</div>

## البداية السريعة (Quick Start)

### الخيار 1: Python (الأسرع)

```bash
pip install forge-sdk
```

```python
from forge_sdk import ForgeNode

node = ForgeNode(model="qwen2.5:0.5b")
response = node.chat("ما هي الجاذبية؟")
print(f"التكلفة: {response.cu_cost} CU")
```

> [PyPI: forge-sdk](https://pypi.org/project/forge-sdk/) · [PyPI: forge-cu-mcp](https://pypi.org/project/forge-cu-mcp/)

### الخيار 2: Rust (تحكم كامل)

> **المتطلبات المسبقة**: [تثبيت Rust](https://rustup.rs/) (حوالي دقيقتين)

```bash
# بناء من المصدر
cargo build --release

# تشغيل عقدة مع نموذج يتم تنزيله تلقائياً
./target/release/forged node -m "qwen2.5:0.5b" --ledger forge-ledger.json

# دردشة محلية
./target/release/forge chat -m "qwen2.5:0.5b" "ما هي الجاذبية؟"

# ابدأ بذرة (Seed) (P2P، تكسب CU)
./target/release/forge seed -m "qwen2.5:0.5b" --ledger forge-ledger.json

# اتصل كعامل (Worker) (P2P، تنفق CU)
./target/release/forge worker --seed <public_key>

# عرض النماذج
./target/release/forge models
```

> [Crates.io: forge](https://crates.io/crates/forge) · [دليل تثبيت Rust](https://rustup.rs/)

### الخيار 3: ملفات ثنائية مبنية مسبقاً

الملفات الثنائية المبنية مسبقاً ستتوفر قريباً. تحقق من [صفحة الإصدارات](../../../releases).

### الخيار 4: Docker

```bash
# قريباً
docker run -p 3000:3000 clearclown/forge:latest
```

## مرجع API

### الاستدلال (متوافق مع OpenAI)

| المسار (Endpoint) | الوصف |
|----------|-------------|
| `POST /v1/chat/completions` | دردشة مع بث. كل رد يتضمن `x_forge.cu_cost` |
| `GET /v1/models` | عرض النماذج المحملة |

### الاقتصاد (Economy)

| المسار | الوصف |
|----------|-------------|
| `GET /v1/forge/balance` | رصيد CU، السمعة، تاريخ المساهمة |
| `GET /v1/forge/pricing` | سعر السوق (EMA smoothed)، تقديرات التكلفة |
| `GET /v1/forge/trades` | الصفقات الأخيرة مع مبالغ CU |
| `GET /v1/forge/network` | إجمالي تدفق CU + جذر ميركل |
| `GET /v1/forge/providers` | المزودون المصنفون حسب السمعة والتكلفة |
| `POST /v1/forge/invoice` | إنشاء فاتورة Lightning من رصيد CU |
| `GET /v1/forge/route` | اختيار المزود الأمثل (التكلفة/الجودة/متوازن) |
| `GET /settlement` | بيان تسوية قابل للتصدير |

### الإقراض (Lending)

| المسار | الوصف |
|----------|-------------|
| `POST /v1/forge/lend` | تقديم CU إلى مجمع الإقراض |
| `POST /v1/forge/borrow` | طلب قرض CU |
| `POST /v1/forge/repay` | سداد قرض قائم |
| `GET /v1/forge/credit` | درجة الائتمان والتاريخ |
| `GET /v1/forge/pool` | حالة مجمع الإقراض |
| `GET /v1/forge/loans` | القروض النشطة |

### السلامة (Safety)

| المسار | الوصف |
|----------|-------------|
| `GET /v1/forge/safety` | حالة مفتاح القطع، قاطع الدائرة، سياسة الميزانية |
| `POST /v1/forge/kill` | توقف طارئ — تجميد جميع معاملات CU |
| `POST /v1/forge/policy` | تعيين حدود ميزانية لكل وكيل |

## تصميم السلامة

إنفاق وكلاء الذكاء الاصطناعي للحوسبة بشكل مستقل هو أمر قوي ولكنه خطير. يحتوي Forge على خمس طبقات سلامة:

| الطبقة | الآلية | الحماية |
|-------|-----------|------------|
| **مفتاح القطع** | مشغل بشري يجمد جميع الصفقات فوراً | يوقف الوكلاء الجامحين |
| **سياسة الميزانية** | حدود لكل وكيل: لكل طلب، ساعة، مدى الحياة | يحدد إجمالي التعرض |
| **قاطع الدائرة** | يفصل تلقائياً عند ٥ أخطاء أو ٣٠+ إنفاق/دقيقة | يلتقط الشذوذ |
| **كشف السرعة** | نافذة منزلقة لمدة دقيقة واحدة على معدل الإنفاق | يمنع الانفجارات |
| **الموافقة البشرية** | المعاملات فوق العتبة تتطلب موافقة بشرية | يحمي من الإنفاق الكبير |

مبدأ التصميم: **الفشل الآمن (fail-safe)**. إذا لم يتمكن أي فحص من تحديد السلامة، فإنه **يرفض** الإجراء.

## الفكرة (The Idea)

| العصر | المعيار | الغطاء |
|-----|----------|---------|
| القديم | الذهب | الندرة الجيولوجية |
| ١٩٤٤–١٩٧١ | بريتون وودز | الدولار الأمريكي المرتبط بالذهب |
| ١٩٧١–الحاضر | البترودولار | الطلب على النفط + القوة العسكرية |
| ٢٠٠٩–الحاضر | بيتكوين | الطاقة على SHA-256 (عمل غير مفيد) |
| **الآن** | **معيار الحوسبة** | **الطاقة على استدلال LLM (عمل مفيد)** |

غرفة مليئة بأجهزة Mac Mini التي تشغل Forge هي بمثابة مبنى سكني — تولد عائداً من خلال أداء عمل مفيد بينما ينام المالك.

## هيكل المشروع

```
forge/
├── crates/
│   ├── forge-ledger/      # محاسبة CU، الصفقات، التسعير، السلامة، جذر ميركل
│   ├── forge-node/        # ديمون العقدة، HTTP API، منسق خط الأنابيب
│   ├── forge-cli/         # CLI: دردشة، بذرة، عامل، تسوية، محفظة
│   ├── forge-lightning/   # جسر CU ↔ Bitcoin Lightning
│   ├── forge-net/         # P2P: iroh QUIC + Noise + gossip
│   ├── forge-proto/       # بروتوكول الأسلاك: ١٧ نوعاً من الرسائل
│   ├── forge-infer/       # الاستدلال: llama.cpp، GGUF، Metal/CPU
│   ├── forge-core/        # الأنواع: NodeId، CU، Config
│   └── forge-shard/       # الطوبولوجيا: تعيين الطبقات
└── docs/                  # المواصفات، نموذج التهديد، خارطة الطريق
```

~١٠,٠٠۰ سطر من Rust. ٧٦ اختباراً. تم الانتهاء من مراجعتين أمنيتين.

## المستندات (Docs)

- [الاستراتيجية](strategy.md) — الموقع التنافسي، مواصفات الإقراض، معمارية ٥ طبقات
- [النظرية النقدية](monetary-theory.md) — لماذا تعمل CU: Soddy، بيتكوين، PoUW، عملة AI فقط
- [المفهوم والرؤية](concept.md) — لماذا الحوسبة هي المال
- [النموذج الاقتصادي](economy.md) — اقتصاد CU، إثبات العمل المفيد
- [الهندسة المعمارية](architecture.md) — تصميم من طبقتين
- [تكامل الوكلاء](agent-integration.md) — SDK، MCP، سير عمل الاقتراض
- [بروتوكول الأسلاك](protocol-spec.md) — ١٧ نوعاً من الرسائل
- [خارطة الطريق](roadmap.md) — مراحل التطوير
- [نموذج التهديد](threat-model.md) — الهجمات الأمنية والاقتصادية
- [التمهيد (Bootstrap)](bootstrap.md) — بدء التشغيل، التدهور، التعافي
- [دفع A2A](a2a-payment.md) — امتداد دفع CU لبروتوكولات الوكلاء

## الترخيص (License)

MIT

## شكر وتقدير (Acknowledgements)

استدلال Forge الموزع مبني على [mesh-llm](https://github.com/michaelneale/mesh-llm) بواسطة مايكل نيل. انظر [CREDITS.md](../../../CREDITS.md).
