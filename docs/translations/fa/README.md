<div align="center">

# Forge

**محاسبات همان پول است. هر وات به جای اتلاف، هوش تولید می‌کند.**

[![PyPI: forge-sdk](https://img.shields.io/pypi/v/forge-sdk?label=forge-sdk&color=3775A9)](https://pypi.org/project/forge-sdk/)
[![PyPI: forge-cu-mcp](https://img.shields.io/pypi/v/forge-cu-mcp?label=forge-cu-mcp&color=3775A9)](https://pypi.org/project/forge-cu-mcp/)
[![Crates.io](https://img.shields.io/crates/v/forge?label=crates.io&color=e6522c)](https://crates.io/crates/forge)
[![License: MIT](https://img.shields.io/badge/License-MIT-brightgreen.svg)](../../../LICENSE)

---

[English](../../../README.md) · [日本語](../ja/README.md) · [简体中文](../zh-CN/README.md) · [繁體中文](../zh-TW/README.md) · [Español](../es/README.md) · [Français](../fr/README.md) · [Русский](../ru/README.md) · [Українська](../uk/README.md) · [हिन्दी](../hi/README.md) · [العربية](../ar/README.md) · **فارسی** · [עברית](../he/README.md)

</div>

**Forge یک پروتکل استنتاج توزیع‌شده است که در آن محاسبات حکم پول را دارد.** نودها (Nodes) با انجام استنتاج‌های مفید LLM برای دیگران، واحدهای محاسباتی (CU) کسب می‌کنند. برخلاف بیت‌کوین — که در آن برق برای هش‌های بی‌معنی سوزانده می‌شود — هر ژول انرژی مصرف شده در یک نود Forge، هوش واقعی تولید می‌کند که واقعاً مورد نیاز کسی است.

موتور استنتاج توزیع‌شده بر پایه [mesh-llm](https://github.com/michaelneale/mesh-llm) اثر مایکل نیل (Michael Neale) ساخته شده است. Forge یک اقتصاد محاسباتی را به آن اضافه می‌کند: حسابداری CU، اثبات کار مفید (Proof of Useful Work)، قیمت‌گذاری پویا، بودجه‌بندی عوامل خودکار و کنترل‌های ایمنی. به [CREDITS.md](../../../CREDITS.md) مراجعه کنید.

**فورک یکپارچه:** [forge-mesh](https://github.com/nm-arealnormalman/mesh-llm) — همان mesh-llm با لایه اقتصادی Forge که به صورت داخلی ساخته شده است.

## دمو زنده (Live Demo)

این خروجی واقعی از یک نود در حال اجرای Forge است. هر استنتاج هزینه CU دارد. هر CU از طریق محاسبات مفید به دست می‌آید.

```
$ forge node -m "qwen2.5:0.5b" --ledger forge-ledger.json
  Model loaded: Qwen2.5-0.5B (Metal-accelerated, 491MB)
  API server listening on 127.0.0.1:3000
```

**بررسی موجودی — هر نود جدید ۱,۰۰۰ CU در سطح رایگان دریافت می‌کند:**
```
$ curl localhost:3000/v1/forge/balance
{
  "effective_balance": 1000,
  "contributed": 0,
  "consumed": 0,
  "reputation": 0.5
}
```

**پرسیدن سوال — استنتاج هزینه CU دارد:**
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

هر پاسخ شامل `x_forge` است — **هزینه آن محاسبات به واحد CU** و موجودی باقی‌مانده. ارائه‌دهنده ۹ CU کسب کرد. مصرف‌کننده ۹ CU خرج کرد. فیزیک پشتوانه هر واحد است.

**سه استنتاج بعد — تراکنش‌های واقعی در دفتر کل:**
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

**هر تراکنش دارای یک ریشه مرکل (Merkle root) است — قابل لنگر انداختن به بیت‌کوین برای اثبات تغییرناپذیر:**
```
$ curl localhost:3000/v1/forge/network
{
  "total_trades": 3,
  "total_contributed_cu": 19,
  "merkle_root": "aac8db9f62dd9ff23926195a70ed8fcfc188fc867d9f2adabd8e694beb338748"
}
```

**عوامل هوش مصنوعی از کنترل خارج شده‌اند؟ سوئیچ قطع (Kill switch) همه چیز را در چند میلی‌ثانیه متوقف می‌کند:**
```
$ curl -X POST localhost:3000/v1/forge/kill \
    -d '{"activate":true, "reason":"anomaly detected", "operator":"admin"}'
← KILL SWITCH ACTIVATED
← تمام تراکنش‌های CU متوقف شد. هیچ عاملی نمی‌تواند خرج کند.
```

**کنترل‌های ایمنی همیشه روشن هستند:**
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

## چرا Forge وجود دارد؟

```
Bitcoin:  electricity  →  meaningless SHA-256  →  BTC
Forge:    electricity  →  useful LLM inference →  CU
```

بیت‌کوین ثابت کرد که `برق ← محاسبات ← پول`. اما محاسبات بیت‌کوین بی‌هدف است. Forge آن را معکوس می‌کند: هر CU نشان‌دهنده هوش واقعی است که مشکل واقعی کسی را حل کرده است.

**چهار موردی که هیچ پروژه دیگری انجام نمی‌دهد:**

### ۱. محاسبات = ارز

هر استنتاج یک معامله است. ارائه‌دهنده CU کسب می‌کند، مصرف‌کننده CU خرج می‌کند. بدون بلاک‌چین، بدون توکن، بدون ICO. واحد CU توسط فیزیک پشتیبانی می‌شود — برقی که برای کار مفید مصرف شده است.

### ۲. مقاوم در برابر دستکاری بدون بلاک‌چین

هر معامله توسط هر دو طرف به صورت دوگانه امضا (Ed25519) می‌شود و در سراسر شبکه (Mesh) همگام‌سازی (Gossip-synced) می‌گردد. ریشه مرکل تمام معاملات می‌تواند برای بازرسی تغییرناپذیر به بیت‌کوین لنگر شود. نیازی به اجماع جهانی نیست — اثبات رمزنگاری دوجانبه کافی است.

### ۳. عوامل هوش مصنوعی محاسبات خود را مدیریت می‌کنند

یک عامل در تلفن همراه، محاسبات بیکار را در طول شب قرض می‌دهد ← CU کسب می‌کند ← دسترسی به مدل 70B می‌خرد ← هوشمندتر می‌شود ← بیشتر کسب می‌کند. عامل به طور خودکار `/v1/forge/balance` و `/v1/forge/pricing` را بررسی می‌کند. سیاست‌های بودجه و قطع‌کننده‌های مدار از هزینه‌های افسارگسیخته جلوگیری می‌کنند.

```
عامل (1.5B در گوشی)
  ← با ارائه استنتاج در طول شب CU کسب می‌کند
  ← برای مدل 70B واحد CU خرج می‌کند ← پاسخ‌های هوشمندتر
  ← تصمیمات بهتر ← کسب CU بیشتر
  ← چرخه تکرار می‌شود ← عامل رشد می‌کند
```

### ۴. تامین مالی خرد محاسبات

نودها می‌توانند CU بیکار خود را با بهره به نودهای دیگر قرض دهند. یک نود کوچک CU قرض می‌گیرد، به مدل بزرگتری دسترسی پیدا می‌کند، CU بیشتری کسب می‌کند و با بهره بازپرداخت می‌کند. هیچ پروژه استنتاج توزیع‌شده دیگری وام محاسباتی ارائه نمی‌دهد — این موضوع از طریق تحلیل رقابتی هر پروژه بزرگ در این حوزه تایید شده است. این همان موتوری است که حلقه خودبهبودی را برای همه از نظر اقتصادی قابل اجرا می‌کند، نه فقط برای کسانی که از قبل سخت‌افزار قدرتمندی دارند.

## معماری

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

## شروع سریع (Quick Start)

### گزینه ۱: Python (سریع‌ترین)

```bash
pip install forge-sdk
```

```python
from forge_sdk import ForgeNode

node = ForgeNode(model="qwen2.5:0.5b")
response = node.chat("گرانش چیست؟")
print(f"هزینه: {response.cu_cost} CU")
```

> [PyPI: forge-sdk](https://pypi.org/project/forge-sdk/) · [PyPI: forge-cu-mcp](https://pypi.org/project/forge-cu-mcp/)

### گزینه ۲: Rust (کنترل کامل)

> **پیش‌نیاز**: [نصب Rust](https://rustup.rs/) (حدود ۲ دقیقه)

```bash
# ساخت از سورس
cargo build --release

# اجرای یک نود با مدل دانلود شده خودکار
./target/release/forged node -m "qwen2.5:0.5b" --ledger forge-ledger.json

# چت محلی
./target/release/forge chat -m "qwen2.5:0.5b" "گرانش چیست؟"

# شروع یک Seed (در شبکه P2P، کسب CU)
./target/release/forge seed -m "qwen2.5:0.5b" --ledger forge-ledger.json

# اتصال به عنوان Worker (در شبکه P2P، خرج CU)
./target/release/forge worker --seed <public_key>

# لیست مدل‌ها
./target/release/forge models
```

> [Crates.io: forge](https://crates.io/crates/forge) · [راهنمای نصب Rust](https://rustup.rs/)

### گزینه ۳: فایل‌های باینری از پیش ساخته شده

فایل‌های باینری از پیش ساخته شده به زودی در دسترس خواهند بود. [صفحه انتشار](../../../releases) را بررسی کنید.

### گزینه ۴: Docker

```bash
# به زودی
docker run -p 3000:3000 clearclown/forge:latest
```

## مرجع API

### استنتاج (سازگار با OpenAI)

| نقطه انتهایی | توضیحات |
|----------|-------------|
| `POST /v1/chat/completions` | چت با استریمینگ. هر پاسخ شامل `x_forge.cu_cost` است |
| `GET /v1/models` | لیست مدل‌های بارگذاری شده |

### اقتصاد (Economy)

| نقطه انتهایی | توضیحات |
|----------|-------------|
| `GET /v1/forge/balance` | موجودی CU، اعتبار، سابقه مشارکت |
| `GET /v1/forge/pricing` | قیمت بازار (صاف شده با EMA)، تخمین هزینه |
| `GET /v1/forge/trades` | معاملات اخیر با مقادیر CU |
| `GET /v1/forge/network` | جریان کل CU + ریشه مرکل |
| `GET /v1/forge/providers` | رتبه‌بندی ارائه‌دهندگان بر اساس اعتبار و هزینه |
| `POST /v1/forge/invoice` | ایجاد فاکتور لایتنینگ از موجودی CU |
| `GET /v1/forge/route` | انتخاب بهینه ارائه‌دهنده (هزینه/کیفیت/متوازن) |
| `GET /settlement` | صورت‌حساب تسویه قابل خروجی |

### وام‌دهی (Lending)

| نقطه انتهایی | توضیحات |
|----------|-------------|
| `POST /v1/forge/lend` | ارائه CU به استخر وام‌دهی |
| `POST /v1/forge/borrow` | درخواست وام CU |
| `POST /v1/forge/repay` | بازپرداخت وام معوق |
| `GET /v1/forge/credit` | امتیاز اعتباری و تاریخچه |
| `GET /v1/forge/pool` | وضعیت استخر وام‌دهی |
| `GET /v1/forge/loans` | وام‌های فعال |

### ایمنی (Safety)

| نقطه انتهایی | توضیحات |
|----------|-------------|
| `GET /v1/forge/safety` | وضعیت سوئیچ قطع، قطع‌کننده مدار، سیاست بودجه |
| `POST /v1/forge/kill` | توقف اضطراری — مسدود کردن تمام تراکنش‌های CU |
| `POST /v1/forge/policy` | تنظیم محدودیت‌های بودجه برای هر عامل |

## طراحی ایمنی

خرج کردن خودکار محاسبات توسط عوامل هوش مصنوعی قدرتمند اما خطرناک است. Forge دارای پنج لایه ایمنی است:

| لایه | مکانیزم | حفاظت |
|-------|-----------|------------|
| **سوئیچ قطع** | اپراتور انسانی تمام معاملات را فوراً مسدود می‌کند | متوقف کردن عوامل فراری |
| **سیاست بودجه** | محدودیت برای هر عامل: در هر درخواست، ساعتی، مادام‌العمر | سقف کل قرار گرفتن در معرض خطر |
| **قطع‌کننده مدار** | قطع خودکار با ۵ خطا یا بیش از ۳۰ تراکنش در دقیقه | شناسایی ناهنجاری‌ها |
| **تشخیص سرعت** | پنجره لغزان ۱ دقیقه‌ای روی نرخ هزینه | جلوگیری از جهش‌های ناگهانی |
| **تایید انسانی** | تراکنش‌های بالاتر از آستانه نیاز به تایید انسان دارند | محافظت از هزینه‌های کلان |

اصل طراحی: **شکست-ایمن (Fail-safe)**. اگر هر بررسی نتواند ایمنی را تعیین کند، عملیات را **رد** می‌کند.

## ایده (The Idea)

| دوران | استاندارد | پشتوانه |
|-----|----------|---------|
| باستان | طلا | کمیابی زمین‌شناسی |
| ۱۹۴۴–۱۹۷۱ | برتون وودز | دلار وابسته به طلا |
| ۱۹۷۱–تاکنون | پترودلار | تقاضای نفت + قدرت نظامی |
| ۲۰۰۹–تاکنون | بیت‌کوین | انرژی روی SHA-256 (کار بی‌فایده) |
| **اکنون** | **استاندارد محاسباتی** | **انرژی روی استنتاج LLM (کار مفید)** |

اتاقی پر از مک‌مینی (Mac Mini) که Forge را اجرا می‌کنند، مانند یک ساختمان آپارتمانی است — که در حین خواب صاحبش، با انجام کارهای مفید، سود تولید می‌کند.

## ساختار پروژه

```
forge/
├── crates/
│   ├── forge-ledger/      # حسابداری CU، معاملات، قیمت‌گذاری، ایمنی، ریشه مرکل
│   ├── forge-node/        # دیمون نود، API HTTP، هماهنگ‌کننده پایپ‌لاین
│   ├── forge-cli/         # رابط کاربری متنی (CLI): چت، seed، worker، تسویه، کیف پول
│   ├── forge-lightning/   # پل ارتباطی CU ↔ بیت‌کوین لایتنینگ
│   ├── forge-net/         # شبکه P2P: iroh QUIC + Noise + gossip
│   ├── forge-proto/       # پروتکل ارتباطی: ۱۷ نوع پیام
│   ├── forge-infer/       # استنتاج: llama.cpp, GGUF, Metal/CPU
│   ├── forge-core/        # انواع داده: NodeId, CU, Config
│   └── forge-shard/       # توپولوژی: تخصیص لایه
└── docs/                  # مشخصات، مدل تهدید، نقشه راه
```

حدود ۱۰,۰۰۰ خط کد Rust. ۷۶ تست. ۲ ممیزی امنیتی تکمیل شده.

## مستندات (Docs)

- [استراتژی](strategy.md) — موقعیت‌یابی رقابتی، مشخصات وام‌دهی، معماری ۵ لایه
- [نظریه پولی](monetary-theory.md) — چرا CU کار می‌کند: Soddy، بیت‌کوین، PoUW، ارز مختص AI
- [مفهوم و چشم‌انداز](concept.md) — چرا محاسبات همان پول است
- [مدل اقتصادی](economy.md) — اقتصاد CU، اثبات کار مفید
- [معماری](architecture.md) — طراحی دو لایه
- [یکپارچه‌سازی عامل](agent-integration.md) — SDK، MCP، جریان کار وام‌گیری
- [پروتکل ارتباطی](protocol-spec.md) — ۱۷ نوع پیام
- [نقشه راه](roadmap.md) — مراحل توسعه
- [مدل تهدید](threat-model.md) — حملات امنیتی و اقتصادی
- [راه‌اندازی (Bootstrap)](bootstrap.md) — راه‌اندازی اولیه، کاهش عملکرد، بازیابی
- [پرداخت A2A](a2a-payment.md) — افزونه پرداخت CU برای پروتکل‌های عامل

## مجوز (License)

MIT

## سپاسگزاری (Acknowledgements)

استنتاج توزیع‌شده Forge بر پایه [mesh-llm](https://github.com/michaelneale/mesh-llm) اثر مایکل نیل ساخته شده است. به [CREDITS.md](../../../CREDITS.md) مراجعه کنید.
