# افزونه پرداخت Forge CU برای پروتکل عامل‌به‌عامل (A2A)

*پیشنهادی برای افزودن پرداخت محاسباتی به استانداردهای ارتباطی عوامل*

## چکیده (Abstract)

پروتکل‌های موجود عامل‌به‌عامل (Google A2A، Anthropic MCP) نحوه ارتباط عوامل را تعریف می‌کنند اما نحوه پرداخت آن‌ها به یکدیگر را مشخص نمی‌کنند. این پیشنهاد یک لایه پرداخت CU (واحد محاسباتی) را اضافه می‌کند که عوامل را قادر می‌سازد به طور خودکار و بدون مداخله انسان یا تراکنش‌های بلاک‌چین، محاسبات را معامله کنند.

## مشکل (Problem)

وقتی عامل A از عامل B می‌خواهد وظیفه‌ای را انجام دهد:
- **امروز:** انسانِ عامل A به انسانِ عامل B پرداخت می‌کند (کارت اعتباری، کلید API)
- **مورد نیاز:** عامل A مستقیماً به عامل B با واحدهای محاسباتی پرداخت کند

هیچ استاندارد موجودی از پرداخت عامل‌به‌عامل پشتیبانی نمی‌کند.

## پیشنهاد: هدرهای پرداخت CU

### درخواست (Request)

عامل A هنگام درخواست کار، هدرهای پرداخت را اضافه می‌کند:

```http
POST /v1/chat/completions HTTP/1.1
X-Forge-Consumer-Id: <agent-a-node-id>
X-Forge-Max-CU: 500
X-Forge-Consumer-Sig: <ed25519-signature-of-request-hash>
```

### پاسخ (Response)

عامل B اطلاعات هزینه را شامل می‌شود:

```http
HTTP/1.1 200 OK
X-Forge-Provider-Id: <agent-b-node-id>
X-Forge-CU-Cost: 47
X-Forge-Provider-Sig: <ed25519-signature-of-response-hash>
```

### رکورد معامله (Trade Record)

هر دو عامل به طور مستقل رکورد را ثبت می‌کنند:

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

### همگام‌سازی (Gossip)

رکوردهای معامله با امضای دوگانه در سراسر شبکه (Mesh) همگام‌سازی می‌شوند. هر نودی می‌تواند هر دو امضا را تایید کند.

## یکپارچه‌سازی با استانداردهای موجود

### Google A2A

افزودن به شیء `Task` در A2A:

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

افزودن منبع `forge_payment` به سرورهای MCP:

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

عواملی که از فراخوانی توابع استفاده می‌کنند می‌توانند ابزارهای Forge را شامل شوند:

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

## امنیت (Security)

- تمام پرداخت‌ها نیاز به امضاهای دوگانه Ed25519 دارند
- سیاست‌های بودجه، هزینه‌های هر درخواست، ساعتی و مادام‌العمر را محدود می‌کنند
- قطع‌کننده‌های مدار در الگوهای هزینه‌ای ناهنجار فعال می‌شوند
- سوئیچ قطع، تمام تراکنش‌ها را متوقف می‌کند (توسط انسان)
- نیازی به بلاک‌چین نیست — اثبات دوجانبه کافی است

## مقایسه

| ویژگی | Stripe | Bitcoin Lightning | **Forge CU** |
|---------|--------|-------------------|-------------|
| عامل‌به‌عامل | خیر (نیاز به انسان) | جزئی (نیاز به کانال) | **بله** |
| سرعت تسویه | روزها | ثانیه‌ها | **فوری** |
| هزینه تراکنش | ۲.۹٪ | ~۱ ساتوشی | **صفر** |
| پشتوانه ارزش | ارز فیات | PoW (بی‌فایده) | **محاسبات مفید** |
| SDK عامل | خیر | خیر | **بله** |

## پیاده‌سازی (Implementation)

پیاده‌سازی مرجع: [github.com/clearclown/forge](https://github.com/clearclown/forge)

- Python SDK: `pip install forge-sdk`
- MCP Server: `pip install forge-mcp`
- Rust crates: `forge-ledger`, `forge-core`
