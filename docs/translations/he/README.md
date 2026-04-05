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

**Forge הוא פרוטוקול אינפרנס (Inference) מבוזר שבו כוח חישוב הוא כסף.** צמתים (Nodes) מרוויחים יחידות חישוב (CU) על ידי ביצוע אינפרנס LLM מועיל עבור אחרים. בשונה מביטקוין — שבו חשמל נשרף על חישובי האש (Hashes) חסרי משמעות — כל ג'אול שמושקע בצומת Forge מייצר בינה אמיתית שמישהו באמת צריך.

מנוע האינפרנס המבוזר בנוי על [mesh-llm](https://github.com/michaelneale/mesh-llm) של מייקל ניל (Michael Neale). Forge מוסיף כלכלת חישוב מעל: חשבונאות CU, הוכחת עבודה מועילה (Proof of Useful Work), תמחור דינמי, תקציבי סוכנים אוטונומיים ובקרות בטיחות. ראו [CREDITS.md](../../../CREDITS.md).

**פורק (Fork) משולב:** [forge-mesh](https://github.com/nm-arealnormalman/mesh-llm) — mesh-llm עם שכבה כלכלית של Forge מובנית בתוכו.

## דמו חי (Live Demo)

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

**לכל עסקה יש שורש מרקל (Merkle root) — ניתן לעיגון בביטקוין להוכחה בלתי ניתנת לשינוי:**
```
$ curl localhost:3000/v1/forge/network
{
  "total_trades": 3,
  "total_contributed_cu": 19,
  "merkle_root": "aac8db9f62dd9ff23926195a70ed8fcfc188fc867d9f2adabd8e694beb338748"
}
```

**סוכני בינה מלאכותית יצאו משליטה? כפתור השבתה (Kill switch) מקפיא הכל בתוך מילישניות:**
```
$ curl -X POST localhost:3000/v1/forge/kill \
    -d '{"activate":true, "reason":"anomaly detected", "operator":"admin"}'
← KILL SWITCH ACTIVATED
← כל עסקאות ה-CU הוקפאו. אף סוכן לא יכול להוציא כסף.
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

ביטקוין הוכיח ש`חשמל ← חישוב ← כסף`. אבל החישוב של ביטקוין הוא חסר תכלית. Forge הופך את זה: כל CU מייצג בינה אמיתית שפתרה בעיה אמיתית של מישהו.

**שלושה דברים שאף פרויקט אחר לא עושה:**

### 1. חישוב = מטבע

כל אינפרנס הוא עסקה. הספק מרוויח CU, הצרכן מוציא CU. ללא בלוקצ'יין, ללא טוקן (Token), ללא ICO. ה-CU מגובה בפיזיקה — החשמל שנצרך עבור עבודה מועילה.

### 2. עמיד בפני שינויים ללא בלוקצ'יין

כל עסקה חתומה כפול (Ed25519) על ידי שני הצדדים ומסונכרנת ברשת (Gossip-synced). שורש מרקל של כל העסקאות יכול להיות מעוגן בביטקוין לביקורת בלתי ניתנת לשינוי. אין צורך בקונצנזוס עולמי — הוכחה קריפטוגרפית בילטרלית מספיקה.

### 3. סוכני AI מנהלים את החישוב שלהם בעצמם

סוכן בטלפון נייד משאיל כוח חישוב פנוי בלילה ← מרוויח CU ← קונה גישה למודל 70B ← הופך לחכם יותר ← מרוויח יותר. הסוכן בודק את `/v1/forge/balance` ו-`/v1/forge/pricing` באופן אוטונומי. מדיניות תקציב ומפסקי זרם מונעים הוצאות חסרות רסן.

```
סוכן (1.5B בטלפון)
  ← מרוויח CU בלילה על ידי מתן שירותי אינפרנס
  ← מוציא CU על מודל 70B ← תשובות חכמות יותר
  ← החלטות טובות יותר ← יותר CU מורווח
  ← המחזור חוזר על עצמו ← הסוכן גדל
```

## ארכיטקטורה (Architecture)

```
┌─────────────────────────────────────────────────┐
│  Inference Layer (mesh-llm)                     │
│  Pipeline parallelism, MoE expert sharding,     │
│  iroh mesh, Nostr discovery, OpenAI API         │
└──────────────────┬──────────────────────────────┘
                   │
┌──────────────────▼──────────────────────────────┐
│  Economic Layer (Forge)                         │
│  CU ledger, dual-signed trades, gossip,         │
│  dynamic pricing, Merkle root, safety controls  │
└──────────────────┬──────────────────────────────┘
                   │
┌──────────────────▼──────────────────────────────┐
│  Safety Layer                                   │
│  Kill switch, budget policies, circuit breakers,│
│  velocity detection, human approval thresholds  │
└──────────────────┬──────────────────────────────┘
                   │ אופציונלי
┌──────────────────▼──────────────────────────────┐
│  External Bridges                               │
│  CU ↔ BTC (Lightning), CU ↔ stablecoin        │
└─────────────────────────────────────────────────┘
```

## התחלה מהירה (Quick Start)

### אפשרות 1: Python (הכי מהיר)

```bash
pip install forge-sdk
```

```python
from forge_sdk import ForgeNode

node = ForgeNode(model="qwen2.5:0.5b")
response = node.chat("מה זה כוח הכבידה?")
print(f"עלות: {response.cu_cost} CU")
```

> [PyPI: forge-sdk](https://pypi.org/project/forge-sdk/) · [PyPI: forge-cu-mcp](https://pypi.org/project/forge-cu-mcp/)

### אפשרות 2: Rust (שליטה מלאה)

> **דרישות מוקדמות**: [התקנת Rust](https://rustup.rs/) (כ-2 דקות)

```bash
# בנייה מקוד מקור
cargo build --release

# הרצת צומת עם מודל שמורד אוטומטית
./target/release/forged node -m "qwen2.5:0.5b" --ledger forge-ledger.json

# צ'אט מקומי
./target/release/forge chat -m "qwen2.5:0.5b" "מה זה כוח הכבידה?"

# התחלת Seed (רשת P2P, מרוויח CU)
./target/release/forge seed -m "qwen2.5:0.5b" --ledger forge-ledger.json

# התחברות כעובד (Worker) (רשת P2P, מוציא CU)
./target/release/forge worker --seed <public_key>

# רשימת מודלים
./target/release/forge models
```

> [Crates.io: forge](https://crates.io/crates/forge) · [מדריך התקנת Rust](https://rustup.rs/)

### אפשרות 3: קבצים בינאריים מוכנים מראש

קבצים בינאריים מוכנים מראש יהיו זמינים בקרוב. בדקו את [דף השחרורים](../../../releases).

### אפשרות 4: Docker

```bash
# בקרוב
docker run -p 3000:3000 clearclown/forge:latest
```

## רפרנס API

### אינפרנס (תואם OpenAI)

| נקודת קצה (Endpoint) | תיאור |
|----------|-------------|
| `POST /v1/chat/completions` | צ'אט עם סטרימינג. כל תגובה כוללת את `x_forge.cu_cost` |
| `GET /v1/models` | רשימת מודלים טעונים |

### כלכלה (Economy)

| נקודת קצה | תיאור |
|----------|-------------|
| `GET /v1/forge/balance` | יתרת CU, מוניטין, היסטוריית תרומה |
| `GET /v1/forge/pricing` | מחיר שוק (מוחלק EMA), הערכות עלות |
| `GET /v1/forge/trades` | עסקאות אחרונות עם סכומי CU |
| `GET /v1/forge/network` | זרימת CU כוללת + שורש מרקל |
| `GET /v1/forge/providers` | ספקים מדורגים לפי מוניטין ועלות |
| `POST /v1/forge/invoice` | יצירת חשבונית Lightning מיתרת CU |
| `GET /settlement` | דוח סליקה ניתן לייצוא |

### בטיחות (Safety)

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

## הרעיון (The Idea)

| עידן | סטנדרט | גיבוי |
|-----|----------|---------|
| עת עתיקה | זהב | מחסור גיאולוגי |
| 1944–1971 | ברטון וודס | דולר אמריקאי צמוד לזהב |
| 1971–היום | פטרודולר | ביקוש לנפט + כוח צבאי |
| 2009–היום | ביטקוין | אנרגיה על SHA-256 (עבודה חסרת תועלת) |
| **עכשיו** | **סטנדרט החישוב** | **אנרגיה על אינפרנס LLM (עבודה מועילה)** |

חדר מלא במחשבי Mac Mini שמריצים את Forge הוא כמו בניין מגורים — מייצר תשואה על ידי ביצוע עבודה מועילה בזמן שהבעלים ישן.

## מבנה הפרויקט

```
forge/
├── crates/
│   ├── forge-ledger/      # חשבונאות CU, עסקאות, תמחור, בטיחות, שורש מרקל
│   ├── forge-node/        # דמון הצומת, HTTP API, מתאם פייפליין
│   ├── forge-cli/         # CLI: צ'אט, seed, עובד, סליקה, ארנק
│   ├── forge-lightning/   # גשר CU ↔ ביטקוין Lightning
│   ├── forge-net/         # P2P: iroh QUIC + Noise + gossip
│   ├── forge-proto/       # פרוטוקול תקשורת: 17 סוגי הודעות
│   ├── forge-infer/       # אינפרנס: llama.cpp, GGUF, Metal/CPU
│   ├── forge-core/        # טיפוסים: NodeId, CU, Config
│   └── forge-shard/       # טופולוגיה: הקצאת שכבות
└── docs/                  # מפרטים, מודל איומים, מפת דרכים
```

~10,000 שורות קוד ב-Rust. 76 טסטים. הושלמו 2 ביקורות אבטחה.

## תיעוד (Docs)

- [קונספט וחזון](concept.md) — למה חישוב הוא כסף
- [מודל כלכלי](economy.md) — כלכלת CU, הוכחת עבודה מועילה
- [ארכיטקטורה](architecture.md) — עיצוב דו-שכבתי
- [פרוטוקול תקשורת](protocol-spec.md) — 17 סוגי הודעות
- [מפת דרכים](roadmap.md) — שלבי פיתוח
- [מודל איומים](threat-model.md) — מתקפות אבטחה וכלכליות
- [Bootstrap](bootstrap.md) — הפעלה, הידרדרות, התאוששות

## רישיון (License)

MIT

## תודות (Acknowledgements)

האינפרנס המבוזר של Forge בנוי על [mesh-llm](https://github.com/michaelneale/mesh-llm) של מייקל ניל. ראו [CREDITS.md](../../../CREDITS.md).
