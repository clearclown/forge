<div align="center">

# Tirami

**חישוב הוא מטבע. כל וואט מייצר בינה, לא פסולת.**

[![Crates.io](https://img.shields.io/crates/v/tirami-core?label=crates.io&color=e6522c)](https://crates.io/crates/tirami-core)
[![License: MIT](https://img.shields.io/badge/License-MIT-brightgreen.svg)](../../../LICENSE)

---

[English](../../../README.md) · [日本語](../ja/README.md) · [简体中文](../zh-CN/README.md) · [繁體中文](../zh-TW/README.md) · [Español](../es/README.md) · [Français](../fr/README.md) · [Русский](../ru/README.md) · [Українська](../uk/README.md) · [हिन्दी](../hi/README.md) · [العربية](../ar/README.md) · [فارسی](../fa/README.md) · **עברית**

</div>

**Tirami הוא פרוטוקול אינפרנס מבוזר שבו כוח חישוב הוא כסף.** צמתים (Nodes) מרוויחים TRM (Tirami Resource Merit) (TRM) על ידי ביצוע אינפרנס LLM מועיל עבור אחרים. בשונה מביטקוין — שבו חשמל נשרף על חישובי האש (Hashes) חסרי משמעות — כל ג'אול שמושקע בצומת Tirami מייצר בינה אמיתית שמישהו באמת צריך.

מנוע האינפרנס המבוזר בנוי על [mesh-llm](https://github.com/michaelneale/mesh-llm) של מייקל ניל (Michael Neale). Tirami מוסיף כלכלת חישוב מעל: חשבונאות TRM, הוכחת עבודה מועילה (Proof of Useful Work), תמחור דינמי, תקציבי סוכנים אוטונומיים ובקרות בטיחות. ראו [CREDITS.md](../../../CREDITS.md).

**פורק משולב:** [tirami-mesh](https://github.com/nm-arealnormalman/mesh-llm) — mesh-llm עם שכבה כלכלית של Tirami מובנית בתוכו.

## דמו חי

זהו פלט אמיתי מצומת Tirami פעיל. כל אינפרנס עולה TRM. כל TRM מורווח על ידי חישוב מועיל.

```
$ tirami node -m "qwen2.5:0.5b" --ledger tirami-ledger.json
  Model loaded: Qwen2.5-0.5B (Metal-accelerated, 491MB)
  API server listening on 127.0.0.1:3000
```

**בדיקת יתרה — כל צומת חדש מקבל 1,000 TRM במסלול החינמי:**
```
$ curl localhost:3000/v1/tirami/balance
{
  "effective_balance": 1000,
  "contributed": 0,
  "consumed": 0,
  "reputation": 0.5
}
```

**שאילת שאלה — אינפרנס עולה TRM:**
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

כל תגובה כוללת את השדה `x_tirami` — **עלות החישוב ביחידות TRM** והיתרה שנותרה. הספק הרוויח 9 TRM. הצרכן שילם 9 TRM. הפיזיקה מגבה כל יחידה.

**שלושה אינפרנסים מאוחר יותר — עסקאות אמיתיות בספר החשבונות:**
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

**לכל עסקה יש שורש מרקל — ניתן לעיגון בביטקוין להוכחה בלתי ניתנת לשינוי:**
```
$ curl localhost:3000/v1/tirami/network
{
  "total_trades": 3,
  "total_contributed_trm": 19,
  "merkle_root": "aac8db9f62dd9ff23926195a70ed8fcfc188fc867d9f2adabd8e694beb338748"
}
```

**סוכני בינה מלאכותית יצאו משליטה? כפתור השבתה מקפיא הכל בתוך מילישניות:**
```
$ curl -X POST localhost:3000/v1/tirami/kill \
    -d '{"activate":true, "reason":"anomaly detected", "operator":"admin"}'
→ KILL SWITCH ACTIVATED
→ All TRM transactions frozen. No agent can spend.
```

**בקרות בטיחות תמיד פועלות:**
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

## למה Tirami קיים

```
Bitcoin:  electricity  →  meaningless SHA-256  →  BTC
Tirami:    electricity  →  useful LLM inference →  TRM
```

ביטקוין הוכיח ש`חשמל → חישוב → כסף`. אבל החישוב של ביטקוין הוא חסר תכלית. Tirami הופך את זה: כל TRM מייצג בינה אמיתית שפתרה בעיה אמיתית של מישהו.

**ארבעה דברים שאף פרויקט אחר לא עושה:**

### 1. חישוב = מטבע

כל אינפרנס הוא עסקה. הספק מרוויח TRM, הצרכן מוציא TRM. ללא בלוקצ'יין, ללא טוקן, ללא ICO. ה-TRM מגובה בפיזיקה — החשמל שנצרך עבור עבודה מועילה. בשונה מ-Bittensor (TAO), Akash (AKT) או Golem (GLM), לא ניתן לספסר על TRM — הוא מורווח על ידי ביצוע חישוב מועיל.

### 2. עמיד בפני שינויים ללא בלוקצ'יין

כל עסקה חתומה כפול (Ed25519) על ידי שני הצדדים ומסונכרנת ברשת. שורש מרקל של כל העסקאות יכול להיות מעוגן בביטקוין לביקורת בלתי ניתנת לשינוי. אין צורך בקונצנזוס עולמי — הוכחה קריפטוגרפית בילטרלית מספיקה.

### 3. סוכני AI מנהלים את החישוב שלהם בעצמם

סוכן בטלפון נייד משאיל כוח חישוב פנוי בלילה → מרוויח TRM → קונה גישה למודל 70B → הופך לחכם יותר → מרוויח יותר. הסוכן בודק את `/v1/tirami/balance` ו-`/v1/tirami/pricing` באופן אוטונומי. מדיניות תקציב ומפסקי זרם מונעים הוצאות חסרות רסן.

```
סוכן (1.5B בטלפון)
  → מרוויח TRM בלילה על ידי מתן שירותי אינפרנס
  → מוציא TRM על מודל 70B → תשובות חכמות יותר
  → החלטות טובות יותר → יותר TRM מורווח
  → המחזור חוזר על עצמו → הסוכן גדל
```

### 4. מיקרו-מימון חישוב

צמתים יכולים להלוות TRM לא פעיל לצמתים אחרים בריבית. צומת קטן לווה TRM, מקבל גישה למודל גדול יותר, מרוויח יותר TRM, ומחזיר עם ריבית. אף פרויקט אינפרנס מבוזר אחר לא מציע הלוואת חישוב. זהו המנוע שהופך את לולאת השיפור העצמי לכלכלית עבור כולם.

## ארכיטקטורה

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

כל 5 השכבות קיימות. 785 בדיקות עוברות בכל האקוסיסטם.

## התחלה מהירה

### אפשרות 1: דמו מקצה לקצה בפקודה אחת (Rust, ~30 שניות מאפס)

```bash
git clone https://github.com/clearclown/tirami && cd tirami
bash scripts/demo-e2e.sh
```

הסקריפט מוריד SmolLM2-135M (~100 MB) מ-HuggingFace, מפעיל צומת Tirami אמיתי עם האצת Metal/CUDA, מריץ שלוש השלמות צ'אט אמיתיות, עובר על כל ה-endpoints של שלבים 1-13 ומדפיס סיכום צבעוני. אומת ב-2026-04-09 על Apple Silicon Metal GPU.

לאחר שסיים, אותו צומת מגיב גם ל:

```bash
# לקוח תואם-OpenAI
export OPENAI_BASE_URL=http://127.0.0.1:3001/v1
export OPENAI_API_KEY=$(cat ~/.tirami/api_token 2>/dev/null || echo "$TOKEN")

# סטרימינג אמיתי טוקן-אחר-טוקן (שלב 11)
curl -N $OPENAI_BASE_URL/chat/completions \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"model":"smollm2:135m","messages":[{"role":"user","content":"hi"}],"stream":true}'

# כלכלה שלב 8 / מוניטין 9 / מדדים 10 / עיגון
curl $OPENAI_BASE_URL/tirami/balance -H "Authorization: Bearer $OPENAI_API_KEY"
curl $OPENAI_BASE_URL/tirami/anchor?network=mainnet -H "Authorization: Bearer $OPENAI_API_KEY"
curl http://127.0.0.1:3001/metrics  # Prometheus, ללא אימות
```

ראו [`docs/compatibility.md`](../../../docs/compatibility.md) למטריצת הפיצ'רים המלאה מול llama.cpp / mesh-llm / Ollama / Bittensor / Akash.

### אפשרות 2: פקודות Rust ידניות

**דרישות מוקדמות**: [התקנת Rust](https://rustup.rs/) (כ-2 דקות)

```bash
cargo build --release

# הרצת צומת — מוריד אוטומטית את המודל מ-HuggingFace
./target/release/tirami node -m "qwen2.5:0.5b" --ledger tirami-ledger.json

# או אחת מהבאות:
./target/release/tirami chat -m "smollm2:135m" "מה זה כוח הכבידה?"
./target/release/tirami seed -m "qwen2.5:1.5b"               # להרוויח TRM כספק P2P
./target/release/tirami worker --seed <public_key>           # להוציא TRM כצרכן P2P
./target/release/tirami models                                # רשימת קטלוג
```

**[Crates.io: tirami-core](https://crates.io/crates/tirami-core)** ·
**[מסמך תאימות](../../../docs/compatibility.md)** ·
**[סקריפט דמו](../../../scripts/demo-e2e.sh)**

### אפשרות 3: קבצים בינאריים מוכנים מראש / Docker

קבצים בינאריים מוכנים מראש ותמונת Docker ‏`clearclown/tirami:latest` מתועדים ב-
[releases](../../../releases). עד אז, אפשרות 1 בונה מקוד מקור בפחות משתי דקות.

## רפרנס API

### אינפרנס (תואם OpenAI)

| נקודת קצה | תיאור |
|----------|-------------|
| `POST /v1/chat/completions` | צ'אט עם סטרימינג. כל תגובה כוללת את `x_tirami.cu_cost` |
| `GET /v1/models` | רשימת מודלים טעונים |

### כלכלה

| נקודת קצה | תיאור |
|----------|-------------|
| `GET /v1/tirami/balance` | יתרת TRM, מוניטין, היסטוריית תרומה |
| `GET /v1/tirami/pricing` | מחיר שוק (מוחלק EMA), הערכות עלות |
| `GET /v1/tirami/trades` | עסקאות אחרונות עם סכומי TRM |
| `GET /v1/tirami/network` | זרימת TRM כוללת + שורש מרקל |
| `GET /v1/tirami/providers` | ספקים מדורגים לפי מוניטין ועלות |
| `POST /v1/tirami/invoice` | יצירת חשבונית Lightning מיתרת TRM |
| `GET /v1/tirami/route` | בחירת ספק אופטימלית (עלות/איכות/מאוזן) |
| `GET /settlement` | דוח סליקה ניתן לייצוא |

### הלוואות

| נקודת קצה | תיאור |
|----------|-------------|
| `POST /v1/tirami/lend` | הצעת TRM לבריכת ההלוואות |
| `POST /v1/tirami/borrow` | בקשת הלוואת TRM |
| `POST /v1/tirami/repay` | החזר הלוואה פתוחה |
| `GET /v1/tirami/credit` | ציון אשראי והיסטוריה |
| `GET /v1/tirami/pool` | מצב בריכת ההלוואות |
| `GET /v1/tirami/loans` | הלוואות פעילות |

### בטיחות

| נקודת קצה | תיאור |
|----------|-------------|
| `GET /v1/tirami/safety` | מצב כפתור השבתה, מפסק זרם, מדיניות תקציב |
| `POST /v1/tirami/kill` | עצירת חירום — הקפאת כל עסקאות ה-TRM |
| `POST /v1/tirami/policy` | הגדרת מגבלות תקציב לכל סוכן |

## עיצוב בטיחותי

סוכני בינה מלאכותית שמוציאים כוח חישוב באופן אוטונומי הם עוצמתיים אך מסוכנים. ל-Tirami יש חמש שכבות בטיחות:

| שכבה | מנגנון | הגנה |
|-------|-----------|------------|
| **כפתור השבתה** | מפעיל אנושי מקפיא את כל העסקאות באופן מיידי | עוצר סוכנים שיצאו משליטה |
| **מדיניות תקציב** | מגבלות לכל סוכן: לבקשה, לשעה, לכל החיים | מגביל חשיפה כוללת |
| **מפסק זרם** | ניתוק אוטומטי לאחר 5 שגיאות או 30+ הוצאות לדקה | תופס אנומליות |
| **זיהוי מהירות** | חלון זמן של דקה אחת על קצב ההוצאות | מונע התפרצויות |
| **אישור אנושי** | עסקאות מעל סף מסוים דורשות אישור אנושי | מגן על הוצאות גדולות |

עיקרון עיצובי: **Fail-safe**. אם בדיקה כלשהי אינה יכולה לקבוע בטיחות, היא **דוחה** את הפעולה.

## הרעיון

| עידן | סטנדרט | גיבוי |
|-----|----------|---------|
| עת עתיקה | זהב | מחסור גיאולוגי |
| 1944–1971 | ברטון וודז | דולר אמריקאי צמוד לזהב |
| 1971–היום | פטרודולר | ביקוש לנפט + כוח צבאי |
| 2009–היום | ביטקוין | אנרגיה על SHA-256 (עבודה חסרת תועלת) |
| **עכשיו** | **סטנדרט החישוב** | **אנרגיה על אינפרנס LLM (עבודה מועילה)** |

חדר מלא במחשבי Mac Mini שמריצים את Tirami הוא כמו בניין מגורים — מייצר תשואה על ידי ביצוע עבודה מועילה בזמן שהבעלים ישן.

## מבנה הפרויקט

```
tirami/  (המאגר הזה — שכבה 1)
├── crates/
│   ├── tirami-ledger/      # חשבונאות TRM, הלוואות, agora (NIP-90), בטיחות
│   ├── tirami-node/        # דמון הצומת, HTTP API (הלוואות + ניתוב), פייפליין
│   ├── tirami-cli/         # CLI: צ'אט, seed, עובד, סליקה, ארנק
│   ├── tirami-lightning/   # גשר TRM ↔ ביטקוין Lightning (דו-כיווני)
│   ├── tirami-net/         # P2P: iroh QUIC + Noise + gossip (עסקאות + הלוואות)
│   ├── tirami-proto/       # פרוטוקול תקשורת: 27+ סוגי הודעות, כולל Loan*
│   ├── tirami-infer/       # אינפרנס: llama.cpp, GGUF, Metal/CPU
│   ├── tirami-core/        # טיפוסים: NodeId, TRM, Config
│   └── tirami-shard/       # טופולוגיה: הקצאת שכבות
├── scripts/verify-impl.sh         # בדיקת רגרסיה TDD (24 טענות)
└── docs/                  # מפרטים, אסטרטגיה, מודל איומים, מפת דרכים
```

~20,000 שורות קוד ב-Rust. **785 בדיקות עוברות.** שלבים 1-6 הושלמו.

## מאגרים אחים (האקוסיסטם המלא)

| מאגר | שכבה | בדיקות | סטטוס |
|------|-------|-------|--------|
| [clearclown/tirami](https://github.com/clearclown/tirami) (זה) | L1 כלכלה | 785 | שלב 1-13 ✅ |
| [clearclown/tirami-bank](https://github.com/clearclown/tirami-bank) | L2 פיננסים | — | archived |
| [clearclown/tirami-mind](https://github.com/clearclown/tirami-mind) | L3 אינטליגנציה | — | archived |
| [clearclown/tirami-agora](https://github.com/clearclown/tirami-agora) | L4 גילוי | — | archived |
| [clearclown/tirami-economics](https://github.com/clearclown/tirami-economics) | תיאוריה | 16/16 GREEN | ✅ |
| [nm-arealnormalman/mesh-llm](https://github.com/nm-arealnormalman/mesh-llm) | L0 אינפרנס | 43 (tirami-economy) | ✅ |

## תיעוד

- [אסטרטגיה](../../../docs/strategy.md) — מיצוב תחרותי, מפרט הלוואות, ארכיטקטורת 5 שכבות
- [תאוריה מוניטרית](../../../docs/monetary-theory.md) — למה TRM עובד: Soddy, ביטקוין, PoUW, מטבע AI בלבד
- [קונספט וחזון](../../../docs/concept.md) — למה חישוב הוא כסף
- [מודל כלכלי](../../../docs/economy.md) — כלכלת TRM, הוכחת עבודה מועילה
- [ארכיטקטורה](../../../docs/architecture.md) — עיצוב דו-שכבתי
- [אינטגרציית סוכן](../../../docs/agent-integration.md) — SDK, MCP, זרימת עבודת הלוואה
- [פרוטוקול תקשורת](../../../docs/protocol-spec.md) — 17 סוגי הודעות
- [מפת דרכים](../../../docs/roadmap.md) — שלבי פיתוח
- [מודל איומים](../../../docs/threat-model.md) — מתקפות אבטחה וכלכליות
- [Bootstrap](../../../docs/bootstrap.md) — הפעלה, הידרדרות, התאוששות
- [תשלום A2A](../../../docs/a2a-payment.md) — הרחבת תשלום TRM לפרוטוקולי סוכנים
- [תאימות](../../../docs/compatibility.md) — מטריצת פיצ'רים מול llama.cpp / Ollama / Bittensor

## רישיון

MIT

## תודות

האינפרנס המבוזר של Tirami בנוי על [mesh-llm](https://github.com/michaelneale/mesh-llm) של מייקל ניל. ראו [CREDITS.md](../../../CREDITS.md).
