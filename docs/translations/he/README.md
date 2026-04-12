<div align="center">

# Forge

**חישוב הוא מטבע. כל וואט מייצר בינה, לא פסולת.**

[![PyPI: forge-sdk](https://img.shields.io/pypi/v/forge-sdk?label=forge-sdk&color=3775A9)](https://pypi.org/project/forge-sdk/)
[![PyPI: forge-cu-mcp](https://img.shields.io/pypi/v/forge-cu-mcp?label=forge-cu-mcp&color=3775A9)](https://pypi.org/project/forge-cu-mcp/)
[![Crates.io](https://img.shields.io/crates/v/forge?label=crates.io&color=e6522c)](https://crates.io/crates/forge)
[![License: MIT](https://img.shields.io/badge/License-MIT-brightgreen.svg)](../../../LICENSE)

---

[English](../../../README.md) · [日本語](../ja/README.md) · [简体中文](../zh-CN/README.md) · [繁體中文](../zh-TW/README.md) · [Español](../es/README.md) · [Français](../fr/README.md) · [Русский](../ru/README.md) · [Українська](../uk/README.md) · [हिन्दी](../hi/README.md) · [العربية](../ar/README.md) · [فارسی](../fa/README.md) · **עברית**

</div>

**Forge הוא פרוטוקול אינפרנס מבוזר שבו כוח חישוב הוא כסף.** צמתים (Nodes) מרוויחים יחידות חישוב (CU) על ידי ביצוע אינפרנס LLM מועיל עבור אחרים. בשונה מביטקוין — שבו חשמל נשרף על חישובי האש (Hashes) חסרי משמעות — כל ג'אול שמושקע בצומת Forge מייצר בינה אמיתית שמישהו באמת צריך.

מנוע האינפרנס המבוזר בנוי על [mesh-llm](https://github.com/michaelneale/mesh-llm) של מייקל ניל (Michael Neale). Forge מוסיף כלכלת חישוב מעל: חשבונאות CU, הוכחת עבודה מועילה (Proof of Useful Work), תמחור דינמי, תקציבי סוכנים אוטונומיים ובקרות בטיחות. ראו [CREDITS.md](../../../CREDITS.md).

**פורק משולב:** [forge-mesh](https://github.com/nm-arealnormalman/mesh-llm) — mesh-llm עם שכבה כלכלית של Forge מובנית בתוכו.

## דמו חי

זהו פלט אמיתי מצומת Forge פעיל. כל אינפרנס עולה CU. כל CU מורווח על ידי חישוב מועיל.

```
$ forge node -m "qwen2.5:0.5b" --ledger forge-ledger.json
  Model loaded: Qwen2.5-0.5B (Metal-accelerated, 491MB)
  API server listening on 127.0.0.1:3000
```

**בדיקת יתרה — כל צומת חדש מקבל 1,000 CU במסלול החינמי:**
```
$ curl localhost:3000/v1/forge/balance
{
  "effective_balance": 1000,
  "contributed": 0,
  "consumed": 0,
  "reputation": 0.5
}
```

**שאילת שאלה — אינפרנס עולה CU:**
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

כל תגובה כוללת את השדה `x_forge` — **עלות החישוב ביחידות CU** והיתרה שנותרה. הספק הרוויח 9 CU. הצרכן שילם 9 CU. הפיזיקה מגבה כל יחידה.

**שלושה אינפרנסים מאוחר יותר — עסקאות אמיתיות בספר החשבונות:**
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

**לכל עסקה יש שורש מרקל — ניתן לעיגון בביטקוין להוכחה בלתי ניתנת לשינוי:**
```
$ curl localhost:3000/v1/forge/network
{
  "total_trades": 3,
  "total_contributed_cu": 19,
  "merkle_root": "aac8db9f62dd9ff23926195a70ed8fcfc188fc867d9f2adabd8e694beb338748"
}
```

**סוכני בינה מלאכותית יצאו משליטה? כפתור השבתה מקפיא הכל בתוך מילישניות:**
```
$ curl -X POST localhost:3000/v1/forge/kill \
    -d '{"activate":true, "reason":"anomaly detected", "operator":"admin"}'
→ KILL SWITCH ACTIVATED
→ All CU transactions frozen. No agent can spend.
```

**בקרות בטיחות תמיד פועלות:**
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

## למה Forge קיים

```
Bitcoin:  electricity  →  meaningless SHA-256  →  BTC
Forge:    electricity  →  useful LLM inference →  CU
```

ביטקוין הוכיח ש`חשמל → חישוב → כסף`. אבל החישוב של ביטקוין הוא חסר תכלית. Forge הופך את זה: כל CU מייצג בינה אמיתית שפתרה בעיה אמיתית של מישהו.

**ארבעה דברים שאף פרויקט אחר לא עושה:**

### 1. חישוב = מטבע

כל אינפרנס הוא עסקה. הספק מרוויח CU, הצרכן מוציא CU. ללא בלוקצ'יין, ללא טוקן, ללא ICO. ה-CU מגובה בפיזיקה — החשמל שנצרך עבור עבודה מועילה. בשונה מ-Bittensor (TAO), Akash (AKT) או Golem (GLM), לא ניתן לספסר על CU — הוא מורווח על ידי ביצוע חישוב מועיל.

### 2. עמיד בפני שינויים ללא בלוקצ'יין

כל עסקה חתומה כפול (Ed25519) על ידי שני הצדדים ומסונכרנת ברשת. שורש מרקל של כל העסקאות יכול להיות מעוגן בביטקוין לביקורת בלתי ניתנת לשינוי. אין צורך בקונצנזוס עולמי — הוכחה קריפטוגרפית בילטרלית מספיקה.

### 3. סוכני AI מנהלים את החישוב שלהם בעצמם

סוכן בטלפון נייד משאיל כוח חישוב פנוי בלילה → מרוויח CU → קונה גישה למודל 70B → הופך לחכם יותר → מרוויח יותר. הסוכן בודק את `/v1/forge/balance` ו-`/v1/forge/pricing` באופן אוטונומי. מדיניות תקציב ומפסקי זרם מונעים הוצאות חסרות רסן.

```
סוכן (1.5B בטלפון)
  → מרוויח CU בלילה על ידי מתן שירותי אינפרנס
  → מוציא CU על מודל 70B → תשובות חכמות יותר
  → החלטות טובות יותר → יותר CU מורווח
  → המחזור חוזר על עצמו → הסוכן גדל
```

### 4. מיקרו-מימון חישוב

צמתים יכולים להלוות CU לא פעיל לצמתים אחרים בריבית. צומת קטן לווה CU, מקבל גישה למודל גדול יותר, מרוויח יותר CU, ומחזיר עם ריבית. אף פרויקט אינפרנס מבוזר אחר לא מציע הלוואת חישוב. זהו המנוע שהופך את לולאת השיפור העצמי לכלכלית עבור כולם.

## ארכיטקטורה

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

כל 5 השכבות קיימות. 326 בדיקות עוברות בכל האקוסיסטם.

## התחלה מהירה

### אפשרות 1: דמו מקצה לקצה בפקודה אחת (Rust, ~30 שניות מאפס)

```bash
git clone https://github.com/clearclown/forge && cd forge
bash scripts/demo-e2e.sh
```

הסקריפט מוריד SmolLM2-135M (~100 MB) מ-HuggingFace, מפעיל צומת Forge אמיתי עם האצת Metal/CUDA, מריץ שלוש השלמות צ'אט אמיתיות, עובר על כל ה-endpoints של שלבים 1-12 ומדפיס סיכום צבעוני. אומת ב-2026-04-09 על Apple Silicon Metal GPU.

לאחר שסיים, אותו צומת מגיב גם ל:

```bash
# לקוח תואם-OpenAI
export OPENAI_BASE_URL=http://127.0.0.1:3001/v1
export OPENAI_API_KEY=$(cat ~/.forge/api_token 2>/dev/null || echo "$TOKEN")

# סטרימינג אמיתי טוקן-אחר-טוקן (שלב 11)
curl -N $OPENAI_BASE_URL/chat/completions \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"model":"smollm2:135m","messages":[{"role":"user","content":"hi"}],"stream":true}'

# כלכלה שלב 8 / מוניטין 9 / מדדים 10 / עיגון
curl $OPENAI_BASE_URL/forge/balance -H "Authorization: Bearer $OPENAI_API_KEY"
curl $OPENAI_BASE_URL/forge/anchor?network=mainnet -H "Authorization: Bearer $OPENAI_API_KEY"
curl http://127.0.0.1:3001/metrics  # Prometheus, ללא אימות
```

ראו [`docs/compatibility.md`](../../../docs/compatibility.md) למטריצת הפיצ'רים המלאה מול llama.cpp / mesh-llm / Ollama / Bittensor / Akash.

### אפשרות 2: Python (שולט בהכל דרך SDK + MCP)

```bash
pip install forge-sdk forge-cu-mcp

python -c "
from forge_sdk import ForgeClient
c = ForgeClient(base_url='http://localhost:3001')
print('balance:', c.balance())
print('decision:', c.bank_tick())
"
```

[PyPI: forge-sdk](https://pypi.org/project/forge-sdk/) (20 מתודות L2/L3/L4) ·
[PyPI: forge-cu-mcp](https://pypi.org/project/forge-cu-mcp/) (20 כלי MCP ל-Claude Code / Cursor)

### אפשרות 3: פקודות Rust ידניות

**דרישות מוקדמות**: [התקנת Rust](https://rustup.rs/) (כ-2 דקות)

```bash
cargo build --release

# הרצת צומת — מוריד אוטומטית את המודל מ-HuggingFace
./target/release/forge node -m "qwen2.5:0.5b" --ledger forge-ledger.json

# או אחת מהבאות:
./target/release/forge chat -m "smollm2:135m" "מה זה כוח הכבידה?"
./target/release/forge seed -m "qwen2.5:1.5b"               # להרוויח CU כספק P2P
./target/release/forge worker --seed <public_key>           # להוציא CU כצרכן P2P
./target/release/forge models                                # רשימת קטלוג
```

**[Crates.io: forge](https://crates.io/crates/forge)** ·
**[מסמך תאימות](../../../docs/compatibility.md)** ·
**[סקריפט דמו](../../../scripts/demo-e2e.sh)**

### אפשרות 4: קבצים בינאריים מוכנים מראש / Docker

קבצים בינאריים מוכנים מראש ותמונת Docker ‏`clearclown/forge:latest` מתועדים ב-
[releases](../../../releases). עד אז, אפשרות 1 בונה מקוד מקור בפחות משתי דקות.

## רפרנס API

### אינפרנס (תואם OpenAI)

| נקודת קצה | תיאור |
|----------|-------------|
| `POST /v1/chat/completions` | צ'אט עם סטרימינג. כל תגובה כוללת את `x_forge.cu_cost` |
| `GET /v1/models` | רשימת מודלים טעונים |

### כלכלה

| נקודת קצה | תיאור |
|----------|-------------|
| `GET /v1/forge/balance` | יתרת CU, מוניטין, היסטוריית תרומה |
| `GET /v1/forge/pricing` | מחיר שוק (מוחלק EMA), הערכות עלות |
| `GET /v1/forge/trades` | עסקאות אחרונות עם סכומי CU |
| `GET /v1/forge/network` | זרימת CU כוללת + שורש מרקל |
| `GET /v1/forge/providers` | ספקים מדורגים לפי מוניטין ועלות |
| `POST /v1/forge/invoice` | יצירת חשבונית Lightning מיתרת CU |
| `GET /v1/forge/route` | בחירת ספק אופטימלית (עלות/איכות/מאוזן) |
| `GET /settlement` | דוח סליקה ניתן לייצוא |

### הלוואות

| נקודת קצה | תיאור |
|----------|-------------|
| `POST /v1/forge/lend` | הצעת CU לבריכת ההלוואות |
| `POST /v1/forge/borrow` | בקשת הלוואת CU |
| `POST /v1/forge/repay` | החזר הלוואה פתוחה |
| `GET /v1/forge/credit` | ציון אשראי והיסטוריה |
| `GET /v1/forge/pool` | מצב בריכת ההלוואות |
| `GET /v1/forge/loans` | הלוואות פעילות |

### בטיחות

| נקודת קצה | תיאור |
|----------|-------------|
| `GET /v1/forge/safety` | מצב כפתור השבתה, מפסק זרם, מדיניות תקציב |
| `POST /v1/forge/kill` | עצירת חירום — הקפאת כל עסקאות ה-CU |
| `POST /v1/forge/policy` | הגדרת מגבלות תקציב לכל סוכן |

## עיצוב בטיחותי

סוכני בינה מלאכותית שמוציאים כוח חישוב באופן אוטונומי הם עוצמתיים אך מסוכנים. ל-Forge יש חמש שכבות בטיחות:

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

חדר מלא במחשבי Mac Mini שמריצים את Forge הוא כמו בניין מגורים — מייצר תשואה על ידי ביצוע עבודה מועילה בזמן שהבעלים ישן.

## מבנה הפרויקט

```
forge/  (המאגר הזה — שכבה 1)
├── crates/
│   ├── forge-ledger/      # חשבונאות CU, הלוואות, agora (NIP-90), בטיחות
│   ├── forge-node/        # דמון הצומת, HTTP API (הלוואות + ניתוב), פייפליין
│   ├── forge-cli/         # CLI: צ'אט, seed, עובד, סליקה, ארנק
│   ├── forge-lightning/   # גשר CU ↔ ביטקוין Lightning (דו-כיווני)
│   ├── forge-net/         # P2P: iroh QUIC + Noise + gossip (עסקאות + הלוואות)
│   ├── forge-proto/       # פרוטוקול תקשורת: 27+ סוגי הודעות, כולל Loan*
│   ├── forge-infer/       # אינפרנס: llama.cpp, GGUF, Metal/CPU
│   ├── forge-core/        # טיפוסים: NodeId, CU, Config
│   └── forge-shard/       # טופולוגיה: הקצאת שכבות
├── sdk/python/forge_sdk.py        # לקוח Python עם API הלוואות מלא
├── mcp/forge-mcp-server.py        # שרת MCP (כלי הלוואות ל-Claude וכו')
├── scripts/verify-impl.sh         # בדיקת רגרסיה TDD (24 טענות)
└── docs/                  # מפרטים, אסטרטגיה, מודל איומים, מפת דרכים
```

~14,500 שורות קוד ב-Rust. **143 בדיקות עוברות.** שלבים 1-6 הושלמו.

## מאגרים אחים (האקוסיסטם המלא)

| מאגר | שכבה | בדיקות | סטטוס |
|------|-------|-------|--------|
| [clearclown/forge](https://github.com/clearclown/forge) (זה) | L1 כלכלה | 143 | שלב 1-6 ✅ |
| [clearclown/forge-bank](https://github.com/clearclown/forge-bank) | L2 פיננסים | 45 | v0.1 ✅ |
| [clearclown/forge-mind](https://github.com/clearclown/forge-mind) | L3 אינטליגנציה | 40 | v0.1 ✅ |
| [clearclown/forge-agora](https://github.com/clearclown/forge-agora) | L4 גילוי | 39 | v0.1 ✅ |
| [clearclown/forge-economics](https://github.com/clearclown/forge-economics) | תיאוריה | 16/16 GREEN | ✅ |
| [nm-arealnormalman/mesh-llm](https://github.com/nm-arealnormalman/mesh-llm) | L0 אינפרנס | 43 (forge-economy) | ✅ |

## תיעוד

- [אסטרטגיה](../../../docs/strategy.md) — מיצוב תחרותי, מפרט הלוואות, ארכיטקטורת 5 שכבות
- [תאוריה מוניטרית](../../../docs/monetary-theory.md) — למה CU עובד: Soddy, ביטקוין, PoUW, מטבע AI בלבד
- [קונספט וחזון](../../../docs/concept.md) — למה חישוב הוא כסף
- [מודל כלכלי](../../../docs/economy.md) — כלכלת CU, הוכחת עבודה מועילה
- [ארכיטקטורה](../../../docs/architecture.md) — עיצוב דו-שכבתי
- [אינטגרציית סוכן](../../../docs/agent-integration.md) — SDK, MCP, זרימת עבודת הלוואה
- [פרוטוקול תקשורת](../../../docs/protocol-spec.md) — 17 סוגי הודעות
- [מפת דרכים](../../../docs/roadmap.md) — שלבי פיתוח
- [מודל איומים](../../../docs/threat-model.md) — מתקפות אבטחה וכלכליות
- [Bootstrap](../../../docs/bootstrap.md) — הפעלה, הידרדרות, התאוששות
- [תשלום A2A](../../../docs/a2a-payment.md) — הרחבת תשלום CU לפרוטוקולי סוכנים
- [תאימות](../../../docs/compatibility.md) — מטריצת פיצ'רים מול llama.cpp / Ollama / Bittensor

## רישיון

MIT

## תודות

האינפרנס המבוזר של Forge בנוי על [mesh-llm](https://github.com/michaelneale/mesh-llm) של מייקל ניל. ראו [CREDITS.md](../../../CREDITS.md).
