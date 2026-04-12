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

**نسخة مدمجة:** [forge-mesh](https://github.com/nm-arealnormalman/mesh-llm) — وهو mesh-llm مع طبقة Forge الاقتصادية المدمجة.

## عرض حي

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

**كل صفقة لها جذر ميركل — يمكن ربطه ببيتكوين لإثبات غير قابل للتغيير:**
```
$ curl localhost:3000/v1/forge/network
{
  "total_trades": 3,
  "total_contributed_cu": 19,
  "merkle_root": "aac8db9f62dd9ff23926195a70ed8fcfc188fc867d9f2adabd8e694beb338748"
}
```

**وكلاء ذكاء اصطناعي خارج السيطرة؟ مفتاح القطع يجمد كل شيء في أجزاء من الثانية:**
```
$ curl -X POST localhost:3000/v1/forge/kill \
    -d '{"activate":true, "reason":"anomaly detected", "operator":"admin"}'
→ KILL SWITCH ACTIVATED
→ All CU transactions frozen. No agent can spend.
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

أثبت بيتكوين أن `الكهرباء → الحوسبة → المال`. لكن حوسبة بيتكوين بلا هدف. Forge يقلب هذه الآية: كل وحدة CU تمثل ذكاءً حقيقياً حل مشكلة حقيقية لشخص ما.

**أربعة أشياء لا يفعلها أي مشروع آخر:**

### ١. الحوسبة = عملة

كل استدلال هو صفقة. المزود يكسب CU، والمستهلك ينفق CU. لا يوجد بلوكشين، لا توجد عملة رقمية (Token)، لا يوجد ICO. وحدة CU مدعومة بالفيزياء — الكهرباء المستهلكة للعمل المفيد. على عكس Bittensor (TAO) وAkash (AKT) وGolem (GLM)، لا يمكن المضاربة على CU — يُكسب بأداء حوسبة مفيدة.

### ٢. مقاوم للتلاعب بدون بلوكشين

كل صفقة موقعة بشكل مزدوج (Ed25519) من قبل الطرفين ومزامنة عبر الشبكة. يمكن ربط جذر ميركل لجميع الصفقات ببيتكوين للتدقيق غير القابل للتغيير. لا حاجة لإجماع عالمي — الإثبات التشفيري الثنائي كافٍ.

### ٣. وكلاء الذكاء الاصطناعي يديرون حوسبتهم الخاصة

وكيل على هاتف يقرض حوسبة خاملة طوال الليل → يكسب CU → يشتري وصولاً لنموذج 70B → يصبح أذكى → يكسب أكثر. يتحقق الوكيل من `/v1/forge/balance` و`/v1/forge/pricing` بشكل مستقل. سياسات الميزانية وقواطع الدائرة تمنع الإنفاق الجامح.

```
الوكيل (1.5B على الهاتف)
  → يكسب CU طوال الليل من خلال تقديم الاستدلال
  → ينفق CU على نموذج 70B → إجابات أذكى
  → قرارات أفضل → كسب المزيد من CU
  → تتكرر الدورة → ينمو الوكيل
```

### ٤. التمويل الأصغر للحوسبة

يمكن للعقد إقراض وحدات CU الخاملة إلى عقد أخرى بفائدة. تقترض عقدة صغيرة CU، وتصل إلى نموذج أكبر، وتكسب المزيد من CU، وتسدد مع الفائدة. لا يقدم أي مشروع استدلال موزع آخر إقراض الحوسبة. هذا هو المحرك الذي يجعل حلقة التحسين الذاتي قابلة للتطبيق اقتصادياً للجميع.

## الهندسة المعمارية

<div dir="ltr">

```
┌─────────────────────────────────────────────────┐
│  L4: Discovery (forge-agora) ✅ v0.1            │
│  Agent marketplace, reputation aggregation,     │
│  Nostr NIP-90, Google A2A payment extension     │
├─────────────────────────────────────────────────┤
│  L3: Intelligence (forge-mind) ✅ v0.1          │
│  AutoAgent self-improvement loops,              │
│  harness marketplace, meta-optimization         │
├─────────────────────────────────────────────────┤
│  L2: Finance (forge-bank) ✅ v0.1               │
│  Strategies, portfolios, futures, insurance,    │
│  risk model, yield optimizer                    │
├─────────────────────────────────────────────────┤
│  L1: Economy (forge — this repo) ✅ Phase 1-6   │
│  CU ledger, dual-signed trades, dynamic pricing,│
│  lending primitives, safety controls            │
├─────────────────────────────────────────────────┤
│  L0: Inference (forge-mesh / mesh-llm) ✅       │
│  Pipeline parallelism, MoE sharding,            │
│  iroh mesh, Nostr discovery, MLX/llama.cpp      │
└─────────────────────────────────────────────────┘
```

</div>

جميع الطبقات الخمس موجودة. ٣٢٦ اختباراً ناجحاً عبر النظام البيئي بأكمله.

## البداية السريعة

### الخيار 1: عرض توضيحي شامل بأمر واحد (Rust، ~٣٠ ثانية من البداية)

```bash
git clone https://github.com/clearclown/forge && cd forge
bash scripts/demo-e2e.sh
```

يقوم هذا البرنامج النصي بتنزيل SmolLM2-135M (~١٠٠ ميغابايت) من HuggingFace، وتشغيل عقدة Forge حقيقية مع تسريع Metal/CUDA، وتنفيذ ثلاث إتمامات دردشة حقيقية، والمرور عبر جميع نقاط نهاية المراحل ١-١٢، وطباعة ملخص ملوّن. تم التحقق منه في ٢٠٢٦-٠٤-٠٩ على Apple Silicon Metal GPU.

بعد الانتهاء، تستجيب نفس العقدة أيضاً لـ:

```bash
# عميل متوافق مع OpenAI
export OPENAI_BASE_URL=http://127.0.0.1:3001/v1
export OPENAI_API_KEY=$(cat ~/.forge/api_token 2>/dev/null || echo "$TOKEN")

# بث حقيقي رمزاً بعد رمز (المرحلة ١١)
curl -N $OPENAI_BASE_URL/chat/completions \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"model":"smollm2:135m","messages":[{"role":"user","content":"hi"}],"stream":true}'

# اقتصاد المرحلة ٨ / سمعة ٩ / مقاييس ١٠ / التثبيت
curl $OPENAI_BASE_URL/forge/balance -H "Authorization: Bearer $OPENAI_API_KEY"
curl $OPENAI_BASE_URL/forge/anchor?network=mainnet -H "Authorization: Bearer $OPENAI_API_KEY"
curl http://127.0.0.1:3001/metrics  # Prometheus، بدون مصادقة
```

انظر [`docs/compatibility.md`](../../../docs/compatibility.md) للحصول على مصفوفة الميزات الكاملة مقابل llama.cpp / mesh-llm / Ollama / Bittensor / Akash.

### الخيار 2: Python (يتحكم في كل شيء عبر SDK + MCP)

```bash
pip install forge-sdk forge-cu-mcp

python -c "
from forge_sdk import ForgeClient
c = ForgeClient(base_url='http://localhost:3001')
print('balance:', c.balance())
print('decision:', c.bank_tick())
"
```

[PyPI: forge-sdk](https://pypi.org/project/forge-sdk/) (٢٠ طريقة L2/L3/L4) ·
[PyPI: forge-cu-mcp](https://pypi.org/project/forge-cu-mcp/) (٢٠ أداة MCP لـ Claude Code / Cursor)

### الخيار 3: أوامر Rust يدوية

**المتطلبات المسبقة**: [تثبيت Rust](https://rustup.rs/) (دقيقتان)

```bash
cargo build --release

# تشغيل عقدة — تنزيل النموذج تلقائياً من HuggingFace
./target/release/forge node -m "qwen2.5:0.5b" --ledger forge-ledger.json

# أو أي من التالي:
./target/release/forge chat -m "smollm2:135m" "ما هي الجاذبية؟"
./target/release/forge seed -m "qwen2.5:1.5b"               # كسب CU كمزود P2P
./target/release/forge worker --seed <public_key>           # إنفاق CU كمستهلك P2P
./target/release/forge models                                # قائمة الكتالوج
```

**[Crates.io: forge](https://crates.io/crates/forge)** ·
**[وثيقة التوافق](../../../docs/compatibility.md)** ·
**[برنامج نصي للعرض التوضيحي](../../../scripts/demo-e2e.sh)**

### الخيار 4: ملفات ثنائية مبنية مسبقاً / Docker

الملفات الثنائية المبنية مسبقاً وصورة Docker ‏`clearclown/forge:latest` يتم تتبعها في
[الإصدارات](../../../releases). حتى ذلك الحين، يبني الخيار ١ من المصدر في أقل من دقيقتين.

## مرجع API

### الاستدلال (متوافق مع OpenAI)

| المسار | الوصف |
|----------|-------------|
| `POST /v1/chat/completions` | دردشة مع بث. كل رد يتضمن `x_forge.cu_cost` |
| `GET /v1/models` | عرض النماذج المحملة |

### الاقتصاد

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

### الإقراض

| المسار | الوصف |
|----------|-------------|
| `POST /v1/forge/lend` | تقديم CU إلى مجمع الإقراض |
| `POST /v1/forge/borrow` | طلب قرض CU |
| `POST /v1/forge/repay` | سداد قرض قائم |
| `GET /v1/forge/credit` | درجة الائتمان والتاريخ |
| `GET /v1/forge/pool` | حالة مجمع الإقراض |
| `GET /v1/forge/loans` | القروض النشطة |

### السلامة

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

## الفكرة

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
forge/  (هذا المستودع — الطبقة 1)
├── crates/
│   ├── forge-ledger/      # محاسبة CU، الإقراض، agora (NIP-90)، السلامة
│   ├── forge-node/        # ديمون العقدة، HTTP API (إقراض + توجيه)، خط أنابيب
│   ├── forge-cli/         # CLI: دردشة، بذرة، عامل، تسوية، محفظة
│   ├── forge-lightning/   # جسر CU ↔ Bitcoin Lightning (ثنائي الاتجاه)
│   ├── forge-net/         # P2P: iroh QUIC + Noise + gossip (صفقات + قروض)
│   ├── forge-proto/       # بروتوكول الأسلاك: ٢٧+ نوعاً من الرسائل، Loan* مضمّن
│   ├── forge-infer/       # الاستدلال: llama.cpp، GGUF، Metal/CPU
│   ├── forge-core/        # الأنواع: NodeId، CU، Config
│   └── forge-shard/       # الطوبولوجيا: تعيين الطبقات
├── sdk/python/forge_sdk.py        # عميل Python مع واجهة API إقراض كاملة
├── mcp/forge-mcp-server.py        # خادم MCP (أدوات إقراض لـ Claude وغيره)
├── scripts/verify-impl.sh         # اختبار انحدار TDD (٢٤ تأكيداً)
└── docs/                  # المواصفات، الاستراتيجية، نموذج التهديد، خارطة الطريق
```

~١٤,٥٠٠ سطر من Rust. **١٤٣ اختباراً ناجحاً.** المراحل ١-٦ مكتملة.

## المستودعات الشقيقة (النظام البيئي الكامل)

| المستودع | الطبقة | الاختبارات | الحالة |
|------|-------|-------|--------|
| [clearclown/forge](https://github.com/clearclown/forge) (هذا) | L1 الاقتصاد | ١٤٣ | المرحلة ١-٦ ✅ |
| [clearclown/forge-bank](https://github.com/clearclown/forge-bank) | L2 المالية | ٤٥ | v0.1 ✅ |
| [clearclown/forge-mind](https://github.com/clearclown/forge-mind) | L3 الذكاء | ٤٠ | v0.1 ✅ |
| [clearclown/forge-agora](https://github.com/clearclown/forge-agora) | L4 الاكتشاف | ٣٩ | v0.1 ✅ |
| [clearclown/forge-economics](https://github.com/clearclown/forge-economics) | النظرية | ١٦/١٦ GREEN | ✅ |
| [nm-arealnormalman/mesh-llm](https://github.com/nm-arealnormalman/mesh-llm) | L0 الاستدلال | ٤٣ (forge-economy) | ✅ |

## المستندات

- [الاستراتيجية](../../../docs/strategy.md) — الموقع التنافسي، مواصفات الإقراض، معمارية ٥ طبقات
- [النظرية النقدية](../../../docs/monetary-theory.md) — لماذا تعمل CU: Soddy، بيتكوين، PoUW، عملة AI فقط
- [المفهوم والرؤية](../../../docs/concept.md) — لماذا الحوسبة هي المال
- [النموذج الاقتصادي](../../../docs/economy.md) — اقتصاد CU، إثبات العمل المفيد
- [الهندسة المعمارية](../../../docs/architecture.md) — تصميم من طبقتين
- [تكامل الوكلاء](../../../docs/agent-integration.md) — SDK، MCP، سير عمل الاقتراض
- [بروتوكول الأسلاك](../../../docs/protocol-spec.md) — ١٧ نوعاً من الرسائل
- [خارطة الطريق](../../../docs/roadmap.md) — مراحل التطوير
- [نموذج التهديد](../../../docs/threat-model.md) — الهجمات الأمنية والاقتصادية
- [التمهيد](../../../docs/bootstrap.md) — بدء التشغيل، التدهور، التعافي
- [دفع A2A](../../../docs/a2a-payment.md) — امتداد دفع CU لبروتوكولات الوكلاء
- [التوافق](../../../docs/compatibility.md) — مصفوفة الميزات مقابل llama.cpp / Ollama / Bittensor

## الترخيص

MIT

## شكر وتقدير

استدلال Forge الموزع مبني على [mesh-llm](https://github.com/michaelneale/mesh-llm) بواسطة مايكل نيل. انظر [CREDITS.md](../../../CREDITS.md).
