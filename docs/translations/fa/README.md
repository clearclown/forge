<div align="center">

# Tirami

**محاسبات همان پول است. هر وات به جای اتلاف، هوش تولید می‌کند.**

[![Crates.io](https://img.shields.io/crates/v/tirami-core?label=crates.io&color=e6522c)](https://crates.io/crates/tirami-core)
[![License: MIT](https://img.shields.io/badge/License-MIT-brightgreen.svg)](../../../LICENSE)

---

[English](../../../README.md) · [日本語](../ja/README.md) · [简体中文](../zh-CN/README.md) · [繁體中文](../zh-TW/README.md) · [Español](../es/README.md) · [Français](../fr/README.md) · [Русский](../ru/README.md) · [Українська](../uk/README.md) · [हिन्दी](../hi/README.md) · [العربية](../ar/README.md) · **فارسی** · [עברית](../he/README.md)

</div>

**Tirami یک پروتکل استنتاج توزیع‌شده است که در آن محاسبات حکم پول را دارد.** نودها با انجام استنتاج‌های مفید LLM برای دیگران، TRM (Tirami Resource Merit) (TRM) کسب می‌کنند. برخلاف بیت‌کوین — که در آن برق برای هش‌های بی‌معنی سوزانده می‌شود — هر ژول انرژی مصرف شده در یک نود Tirami، هوش واقعی تولید می‌کند که واقعاً مورد نیاز کسی است.

موتور استنتاج توزیع‌شده بر پایه [mesh-llm](https://github.com/michaelneale/mesh-llm) اثر مایکل نیل ساخته شده است. Tirami یک اقتصاد محاسباتی را به آن اضافه می‌کند: حسابداری TRM، اثبات کار مفید (Proof of Useful Work)، قیمت‌گذاری پویا، بودجه‌بندی عوامل خودکار و کنترل‌های ایمنی. به [CREDITS.md](../../../CREDITS.md) مراجعه کنید.

**فورک یکپارچه:** [tirami-mesh](https://github.com/nm-arealnormalman/mesh-llm) — همان mesh-llm با لایه اقتصادی Tirami که به صورت داخلی ساخته شده است.

## دمو زنده

این خروجی واقعی از یک نود در حال اجرای Tirami است. هر استنتاج هزینه TRM دارد. هر TRM از طریق محاسبات مفید به دست می‌آید.

```
$ tirami node -m "qwen2.5:0.5b" --ledger tirami-ledger.json
  Model loaded: Qwen2.5-0.5B (Metal-accelerated, 491MB)
  API server listening on 127.0.0.1:3000
```

**بررسی موجودی — هر نود جدید ۱,۰۰۰ TRM در سطح رایگان دریافت می‌کند:**
```
$ curl localhost:3000/v1/tirami/balance
{
  "effective_balance": 1000,
  "contributed": 0,
  "consumed": 0,
  "reputation": 0.5
}
```

**پرسیدن سوال — استنتاج هزینه TRM دارد:**
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

هر پاسخ شامل `x_tirami` است — **هزینه آن محاسبات به واحد TRM** و موجودی باقی‌مانده. ارائه‌دهنده ۹ TRM کسب کرد. مصرف‌کننده ۹ TRM خرج کرد. فیزیک پشتوانه هر واحد است.

**سه استنتاج بعد — تراکنش‌های واقعی در دفتر کل:**
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

**هر تراکنش دارای یک ریشه مرکل است — قابل لنگر انداختن به بیت‌کوین برای اثبات تغییرناپذیر:**
```
$ curl localhost:3000/v1/tirami/network
{
  "total_trades": 3,
  "total_contributed_trm": 19,
  "merkle_root": "aac8db9f62dd9ff23926195a70ed8fcfc188fc867d9f2adabd8e694beb338748"
}
```

**عوامل هوش مصنوعی از کنترل خارج شده‌اند؟ سوئیچ قطع همه چیز را در چند میلی‌ثانیه متوقف می‌کند:**
```
$ curl -X POST localhost:3000/v1/tirami/kill \
    -d '{"activate":true, "reason":"anomaly detected", "operator":"admin"}'
→ KILL SWITCH ACTIVATED
→ All TRM transactions frozen. No agent can spend.
```

**کنترل‌های ایمنی همیشه روشن هستند:**
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

## چرا Tirami وجود دارد؟

```
Bitcoin:  electricity  →  meaningless SHA-256  →  BTC
Tirami:    electricity  →  useful LLM inference →  TRM
```

بیت‌کوین ثابت کرد که `برق → محاسبات → پول`. اما محاسبات بیت‌کوین بی‌هدف است. Tirami آن را معکوس می‌کند: هر TRM نشان‌دهنده هوش واقعی است که مشکل واقعی کسی را حل کرده است.

**چهار موردی که هیچ پروژه دیگری انجام نمی‌دهد:**

### ۱. محاسبات = ارز

هر استنتاج یک معامله است. ارائه‌دهنده TRM کسب می‌کند، مصرف‌کننده TRM خرج می‌کند. بدون بلاک‌چین، بدون توکن، بدون ICO. واحد TRM توسط فیزیک پشتیبانی می‌شود — برقی که برای کار مفید مصرف شده است. برخلاف Bittensor (TAO)، Akash (AKT) یا Golem (GLM)، نمی‌توان روی TRM سفته‌بازی کرد — با انجام محاسبات مفید به دست می‌آید.

### ۲. مقاوم در برابر دستکاری بدون بلاک‌چین

هر معامله توسط هر دو طرف به صورت دوگانه امضا (Ed25519) می‌شود و در سراسر شبکه همگام‌سازی می‌گردد. ریشه مرکل تمام معاملات می‌تواند برای بازرسی تغییرناپذیر به بیت‌کوین لنگر شود. نیازی به اجماع جهانی نیست — اثبات رمزنگاری دوجانبه کافی است.

### ۳. عوامل هوش مصنوعی محاسبات خود را مدیریت می‌کنند

یک عامل در تلفن همراه، محاسبات بیکار را در طول شب قرض می‌دهد → TRM کسب می‌کند → دسترسی به مدل 70B می‌خرد → هوشمندتر می‌شود → بیشتر کسب می‌کند. عامل به طور خودکار `/v1/tirami/balance` و `/v1/tirami/pricing` را بررسی می‌کند. سیاست‌های بودجه و قطع‌کننده‌های مدار از هزینه‌های افسارگسیخته جلوگیری می‌کنند.

```
عامل (1.5B در گوشی)
  → با ارائه استنتاج در طول شب TRM کسب می‌کند
  → برای مدل 70B واحد TRM خرج می‌کند → پاسخ‌های هوشمندتر
  → تصمیمات بهتر → کسب TRM بیشتر
  → چرخه تکرار می‌شود → عامل رشد می‌کند
```

### ۴. تامین مالی خرد محاسبات

نودها می‌توانند TRM بیکار خود را با بهره به نودهای دیگر قرض دهند. یک نود کوچک TRM قرض می‌گیرد، به مدل بزرگتری دسترسی پیدا می‌کند، TRM بیشتری کسب می‌کند و با بهره بازپرداخت می‌کند. هیچ پروژه استنتاج توزیع‌شده دیگری وام محاسباتی ارائه نمی‌دهد. این همان موتوری است که حلقه خودبهبودی را برای همه از نظر اقتصادی قابل اجرا می‌کند.

## معماری

<div dir="ltr">

```
┌─────────────────────────────────────────────────┐
│  L4: Discovery (tirami-agora) ✅ v0.1            │
│  Agent marketplace, reputation aggregation,     │
│  Nostr NIP-90, Google A2A payment extension     │
├─────────────────────────────────────────────────┤
│  L3: Intelligence (tirami-mind) ✅ v0.1          │
│  AutoAgent self-improvement loops,              │
│  harness marketplace, meta-optimization         │
├─────────────────────────────────────────────────┤
│  L2: Finance (tirami-bank) ✅ v0.1               │
│  Strategies, portfolios, futures, insurance,    │
│  risk model, yield optimizer                    │
├─────────────────────────────────────────────────┤
│  L1: Economy (tirami — this repo) ✅ Phase 1-13   │
│  TRM ledger, dual-signed trades, dynamic pricing,│
│  lending primitives, safety controls            │
├─────────────────────────────────────────────────┤
│  L0: Inference (tirami-mesh / mesh-llm) ✅       │
│  Pipeline parallelism, MoE sharding,            │
│  iroh mesh, Nostr discovery, MLX/llama.cpp      │
└─────────────────────────────────────────────────┘
```

</div>

همه ۵ لایه وجود دارند. ۷۸۵ تست در سراسر اکوسیستم قبول شده‌اند.

## شروع سریع

### گزینه ۱: دمو کامل با یک دستور (Rust، ~۳۰ ثانیه از صفر)

```bash
git clone https://github.com/clearclown/tirami && cd tirami
bash scripts/demo-e2e.sh
```

این اسکریپت SmolLM2-135M (~۱۰۰ مگابایت) را از HuggingFace دانلود می‌کند، یک نود واقعی Tirami با شتاب‌دهنده Metal/CUDA راه‌اندازی می‌کند، سه تکمیل چت واقعی اجرا می‌کند، از تمام endpoint‌های فازهای ۱-۱۲ عبور می‌کند و یک خلاصه رنگی چاپ می‌کند. تأیید شده در ۲۰۲۶-۰۴-۰۹ روی Apple Silicon Metal GPU.

پس از اتمام، همان نود به موارد زیر نیز پاسخ می‌دهد:

```bash
# کلاینت سازگار با OpenAI
export OPENAI_BASE_URL=http://127.0.0.1:3001/v1
export OPENAI_API_KEY=$(cat ~/.tirami/api_token 2>/dev/null || echo "$TOKEN")

# استریمینگ واقعی توکن به توکن (فاز ۱۱)
curl -N $OPENAI_BASE_URL/chat/completions \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"model":"smollm2:135m","messages":[{"role":"user","content":"hi"}],"stream":true}'

# اقتصاد فاز ۸ / اعتبار ۹ / متریک‌ها ۱۰ / لنگرگذاری
curl $OPENAI_BASE_URL/tirami/balance -H "Authorization: Bearer $OPENAI_API_KEY"
curl $OPENAI_BASE_URL/tirami/anchor?network=mainnet -H "Authorization: Bearer $OPENAI_API_KEY"
curl http://127.0.0.1:3001/metrics  # Prometheus، بدون احراز هویت
```

برای ماتریس کامل ویژگی‌ها در مقابل llama.cpp / mesh-llm / Ollama / Bittensor / Akash به [`docs/compatibility.md`](../../../docs/compatibility.md) مراجعه کنید.

### گزینه ۲: دستورات دستی Rust

**پیش‌نیاز**: [نصب Rust](https://rustup.rs/) (حدود ۲ دقیقه)

```bash
cargo build --release

# اجرای یک نود — مدل را به صورت خودکار از HuggingFace دانلود می‌کند
./target/release/tirami node -m "qwen2.5:0.5b" --ledger tirami-ledger.json

# یا هر کدام از موارد زیر:
./target/release/tirami chat -m "smollm2:135m" "گرانش چیست؟"
./target/release/tirami seed -m "qwen2.5:1.5b"               # کسب TRM به عنوان ارائه‌دهنده P2P
./target/release/tirami worker --seed <public_key>           # خرج TRM به عنوان مصرف‌کننده P2P
./target/release/tirami models                                # لیست کاتالوگ
```

**[Crates.io: tirami-core](https://crates.io/crates/tirami-core)** ·
**[سند سازگاری](../../../docs/compatibility.md)** ·
**[اسکریپت دمو](../../../scripts/demo-e2e.sh)**

### گزینه ۳: فایل‌های باینری از پیش ساخته شده / Docker

فایل‌های باینری از پیش ساخته شده و تصویر Docker ‏`clearclown/tirami:latest` در
[releases](../../../releases) ردیابی می‌شوند. تا آن زمان، گزینه ۱ در کمتر از دو دقیقه از سورس می‌سازد.

## مرجع API

### استنتاج (سازگار با OpenAI)

| نقطه انتهایی | توضیحات |
|----------|-------------|
| `POST /v1/chat/completions` | چت با استریمینگ. هر پاسخ شامل `x_tirami.cu_cost` است |
| `GET /v1/models` | لیست مدل‌های بارگذاری شده |

### اقتصاد

| نقطه انتهایی | توضیحات |
|----------|-------------|
| `GET /v1/tirami/balance` | موجودی TRM، اعتبار، سابقه مشارکت |
| `GET /v1/tirami/pricing` | قیمت بازار (صاف شده با EMA)، تخمین هزینه |
| `GET /v1/tirami/trades` | معاملات اخیر با مقادیر TRM |
| `GET /v1/tirami/network` | جریان کل TRM + ریشه مرکل |
| `GET /v1/tirami/providers` | رتبه‌بندی ارائه‌دهندگان بر اساس اعتبار و هزینه |
| `POST /v1/tirami/invoice` | ایجاد فاکتور لایتنینگ از موجودی TRM |
| `GET /v1/tirami/route` | انتخاب بهینه ارائه‌دهنده (هزینه/کیفیت/متوازن) |
| `GET /settlement` | صورت‌حساب تسویه قابل خروجی |

### وام‌دهی

| نقطه انتهایی | توضیحات |
|----------|-------------|
| `POST /v1/tirami/lend` | ارائه TRM به استخر وام‌دهی |
| `POST /v1/tirami/borrow` | درخواست وام TRM |
| `POST /v1/tirami/repay` | بازپرداخت وام معوق |
| `GET /v1/tirami/credit` | امتیاز اعتباری و تاریخچه |
| `GET /v1/tirami/pool` | وضعیت استخر وام‌دهی |
| `GET /v1/tirami/loans` | وام‌های فعال |

### ایمنی

| نقطه انتهایی | توضیحات |
|----------|-------------|
| `GET /v1/tirami/safety` | وضعیت سوئیچ قطع، قطع‌کننده مدار، سیاست بودجه |
| `POST /v1/tirami/kill` | توقف اضطراری — مسدود کردن تمام تراکنش‌های TRM |
| `POST /v1/tirami/policy` | تنظیم محدودیت‌های بودجه برای هر عامل |

## طراحی ایمنی

خرج کردن خودکار محاسبات توسط عوامل هوش مصنوعی قدرتمند اما خطرناک است. Tirami دارای پنج لایه ایمنی است:

| لایه | مکانیزم | حفاظت |
|-------|-----------|------------|
| **سوئیچ قطع** | اپراتور انسانی تمام معاملات را فوراً مسدود می‌کند | متوقف کردن عوامل فراری |
| **سیاست بودجه** | محدودیت برای هر عامل: در هر درخواست، ساعتی، مادام‌العمر | سقف کل قرار گرفتن در معرض خطر |
| **قطع‌کننده مدار** | قطع خودکار با ۵ خطا یا بیش از ۳۰ تراکنش در دقیقه | شناسایی ناهنجاری‌ها |
| **تشخیص سرعت** | پنجره لغزان ۱ دقیقه‌ای روی نرخ هزینه | جلوگیری از جهش‌های ناگهانی |
| **تایید انسانی** | تراکنش‌های بالاتر از آستانه نیاز به تایید انسان دارند | محافظت از هزینه‌های کلان |

اصل طراحی: **شکست-ایمن (Fail-safe)**. اگر هر بررسی نتواند ایمنی را تعیین کند، عملیات را **رد** می‌کند.

## ایده

| دوران | استاندارد | پشتوانه |
|-----|----------|---------|
| باستان | طلا | کمیابی زمین‌شناسی |
| ۱۹۴۴–۱۹۷۱ | برتون وودز | دلار وابسته به طلا |
| ۱۹۷۱–تاکنون | پترودلار | تقاضای نفت + قدرت نظامی |
| ۲۰۰۹–تاکنون | بیت‌کوین | انرژی روی SHA-256 (کار بی‌فایده) |
| **اکنون** | **استاندارد محاسباتی** | **انرژی روی استنتاج LLM (کار مفید)** |

اتاقی پر از مک‌مینی که Tirami را اجرا می‌کنند، مانند یک ساختمان آپارتمانی است — که در حین خواب صاحبش، با انجام کارهای مفید، سود تولید می‌کند.

## ساختار پروژه

```
tirami/  (این مخزن — لایه ۱)
├── crates/
│   ├── tirami-ledger/      # حسابداری TRM، وام‌دهی، agora (NIP-90)، ایمنی
│   ├── tirami-node/        # دیمون نود، API HTTP (وام‌دهی + مسیریابی)، پایپ‌لاین
│   ├── tirami-cli/         # رابط کاربری متنی: چت، seed، worker، تسویه، کیف پول
│   ├── tirami-lightning/   # پل ارتباطی TRM ↔ بیت‌کوین لایتنینگ (دوطرفه)
│   ├── tirami-net/         # شبکه P2P: iroh QUIC + Noise + gossip (معاملات + وام‌ها)
│   ├── tirami-proto/       # پروتکل ارتباطی: ۲۷+ نوع پیام، شامل Loan*
│   ├── tirami-infer/       # استنتاج: llama.cpp, GGUF, Metal/CPU
│   ├── tirami-core/        # انواع داده: NodeId, TRM, Config
│   └── tirami-shard/       # توپولوژی: تخصیص لایه
├── scripts/verify-impl.sh         # تست رگرسیون TDD (۲۴ اثبات)
└── docs/                  # مشخصات، استراتژی، مدل تهدید، نقشه راه
```

~۲۰,۰۰۰ خط کد Rust. **۷۸۵ تست قبول شده.** فازهای ۱-۶ کامل.

## مخازن خواهری (اکوسیستم کامل)

| مخزن | لایه | تست‌ها | وضعیت |
|------|-------|-------|--------|
| [clearclown/tirami](https://github.com/clearclown/tirami) (این) | L1 اقتصاد | ۷۸۵ | فاز ۱-۱۳ ✅ |
| [clearclown/tirami-bank](https://github.com/clearclown/tirami-bank) | L2 مالی | — | archived |
| [clearclown/tirami-mind](https://github.com/clearclown/tirami-mind) | L3 هوش | — | archived |
| [clearclown/tirami-agora](https://github.com/clearclown/tirami-agora) | L4 کشف | — | archived |
| [clearclown/tirami-economics](https://github.com/clearclown/tirami-economics) | نظریه | ۱۶/۱۶ GREEN | ✅ |
| [nm-arealnormalman/mesh-llm](https://github.com/nm-arealnormalman/mesh-llm) | L0 استنتاج | ۴۳ (tirami-economy) | ✅ |

## مستندات

- [استراتژی](../../../docs/strategy.md) — موقعیت‌یابی رقابتی، مشخصات وام‌دهی، معماری ۵ لایه
- [نظریه پولی](../../../docs/monetary-theory.md) — چرا TRM کار می‌کند: Soddy، بیت‌کوین، PoUW، ارز مختص AI
- [مفهوم و چشم‌انداز](../../../docs/concept.md) — چرا محاسبات همان پول است
- [مدل اقتصادی](../../../docs/economy.md) — اقتصاد TRM، اثبات کار مفید
- [معماری](../../../docs/architecture.md) — طراحی دو لایه
- [یکپارچه‌سازی عامل](../../../docs/agent-integration.md) — SDK، MCP، جریان کار وام‌گیری
- [پروتکل ارتباطی](../../../docs/protocol-spec.md) — ۱۷ نوع پیام
- [نقشه راه](../../../docs/roadmap.md) — مراحل توسعه
- [مدل تهدید](../../../docs/threat-model.md) — حملات امنیتی و اقتصادی
- [راه‌اندازی](../../../docs/bootstrap.md) — راه‌اندازی اولیه، کاهش عملکرد، بازیابی
- [پرداخت A2A](../../../docs/a2a-payment.md) — افزونه پرداخت TRM برای پروتکل‌های عامل
- [سازگاری](../../../docs/compatibility.md) — ماتریس ویژگی‌ها در مقابل llama.cpp / Ollama / Bittensor

## مجوز

MIT

## سپاسگزاری

استنتاج توزیع‌شده Tirami بر پایه [mesh-llm](https://github.com/michaelneale/mesh-llm) اثر مایکل نیل ساخته شده است. به [CREDITS.md](../../../CREDITS.md) مراجعه کنید.
