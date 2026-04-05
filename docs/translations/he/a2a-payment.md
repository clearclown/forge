# הרחבת תשלום Forge CU עבור פרוטוקול סוכן-לסוכן (A2A)

*הצעה להוספת תשלום עבור חישוב לסטנדרטים של תקשורת בין סוכנים*

## תקציר (Abstract)

פרוטוקולי סוכן-לסוכן קיימים (Google A2A, Anthropic MCP) מגדירים כיצד סוכנים מתקשרים אך לא כיצד הם משלמים זה לזה. הצעה זו מוסיפה שכבת תשלום ב-CU (יחידות חישוב), המאפשרת לסוכנים לסחור בכוח חישוב באופן אוטונומי ללא התערבות אנושית או עסקאות בלוקצ'יין.

## הבעיה (Problem)

כאשר סוכן א' מבקש מסוכן ב' לבצע משימה:
- **היום:** האדם של סוכן א' משלם לאדם של סוכן ב' (כרטיס אשראי, מפתח API)
- **הצורך:** סוכן א' משלם לסוכן ב' ישירות ביחידות חישוב

אף סטנדרט קיים אינו תומך בתשלום סוכן-לסוכן.

## הצעה: כותרות תשלום CU

### בקשה (Request)

סוכן א' מוסיף כותרות תשלום בעת בקשת עבודה:

```http
POST /v1/chat/completions HTTP/1.1
X-Forge-Consumer-Id: <agent-a-node-id>
X-Forge-Max-CU: 500
X-Forge-Consumer-Sig: <ed25519-signature-of-request-hash>
```

### תגובה (Response)

סוכן ב' כולל מידע על העלות:

```http
HTTP/1.1 200 OK
X-Forge-Provider-Id: <agent-b-node-id>
X-Forge-CU-Cost: 47
X-Forge-Provider-Sig: <ed25519-signature-of-response-hash>
```

### רישום עסקה (Trade Record)

שני הסוכנים רושמים באופן עצמאי:

```json
{
  "provider": "<agent-b>",
  "consumer": "<agent-a>",
  "cu_amount": 47,
  "tokens_processed": 47,
  "timestamp": 1775289254032,
  "provider_sig": "<sig>",
  "consumer_sig": "<sig>"
}
```

### הפצה (Gossip)

רישומי עסקאות חתומים כפול מסונכרנים ברשת (Mesh). כל צומת יכול לאמת את שתי החתימות.

## אינטגרציה עם סטנדרטים קיימים

### Google A2A

הוספה לאובייקט ה-A2A `Task`:

```json
{
  "id": "task-123",
  "status": "completed",
  "payment": {
    "protocol": "forge-cu",
    "consumer": "<node-id>",
    "provider": "<node-id>",
    "cu_amount": 47,
    "consumer_sig": "<sig>",
    "provider_sig": "<sig>"
  }
}
```

### Anthropic MCP

הוספת משאב `forge_payment` לשרתי MCP:

```json
{
  "resources": [{
    "uri": "forge://payment/balance",
    "name": "CU Balance",
    "mimeType": "application/json"
  }]
}
```

### OpenAI Function Calling

סוכנים המשתמשים ב-Function Calling יכולים לכלול כלי Forge:

```json
{
  "tools": [{
    "type": "function",
    "function": {
      "name": "forge_pay",
      "description": "Pay CU for a compute task",
      "parameters": {
        "provider": "string",
        "cu_amount": "integer"
      }
    }
  }]
}
```

## אבטחה (Security)

- כל התשלומים דורשים חתימות Ed25519 בילטרליות
- מדיניות תקציב מגבילה הוצאות לבקשה, לשעה ולכל החיים
- מפסקי זרם קופצים בדפוסי הוצאה חריגים
- כפתור השבתה מקפיא את כל העסקאות (עקיפה אנושית)
- אין צורך בבלוקצ'יין — הוכחה בילטרלית מספיקה

## השוואה

| תכונה | Stripe | Bitcoin Lightning | **Forge CU** |
|---------|--------|-------------------|-------------|
| סוכן-לסוכן | לא (דורש אדם) | חלקי (דורש ערוץ) | **כן** |
| מהירות סליקה | ימים | שניות | **מיידי** |
| עלות עסקה | 2.9% | ~1 sat | **אפס** |
| גיבוי ערך | פיאט | PoW (חסר תועלת) | **חישוב מועיל** |
| SDK לסוכנים | לא | לא | **כן** |

## מימוש (Implementation)

מימוש ייחוס: [github.com/clearclown/forge](https://github.com/clearclown/forge)

- Python SDK: `pip install forge-sdk`
- MCP Server: `pip install forge-mcp`
- Rust crates: `forge-ledger`, `forge-core`
