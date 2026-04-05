# إضافة دفع Forge CU لبروتوكول عامل-إلى-عامل (A2A)

*مقترح لإضافة دفع الحوسبة إلى معايير اتصال الوكلاء*

## ملخص (Abstract)

تحدد بروتوكولات عامل-إلى-عامل الحالية (Google A2A، Anthropic MCP) كيفية اتصال الوكلاء ولكن ليس كيفية دفعهم لبعضهم البعض. يضيف هذا المقترح طبقة دفع CU (وحدة حوسبة)، مما يمكن الوكلاء من تداول الحوسبة بشكل مستقل دون تدخل بشري أو معاملات بلوكشين.

## المشكلة (Problem)

عندما يطلب الوكيل (أ) من الوكيل (ب) أداء مهمة:
- **اليوم:** يدفع الإنسان المسؤول عن الوكيل (أ) للإنسان المسؤول عن الوكيل (ب) (بطاقة ائتمان، مفتاح API)
- **المطلوب:** يدفع الوكيل (أ) للوكيل (ب) مباشرة بوحدات الحوسبة

لا يوجد معيار حالي يدعم الدفع من عامل إلى عامل.

## المقترح: رؤوس دفع CU (Payment Headers)

### الطلب (Request)

يضيف الوكيل (أ) رؤوس دفع عند طلب العمل:

```http
POST /v1/chat/completions HTTP/1.1
X-Forge-Consumer-Id: <agent-a-node-id>
X-Forge-Max-CU: 500
X-Forge-Consumer-Sig: <ed25519-signature-of-request-hash>
```

### الرد (Response)

يضمن الوكيل (ب) معلومات التكلفة:

```http
HTTP/1.1 200 OK
X-Forge-Provider-Id: <agent-b-node-id>
X-Forge-CU-Cost: 47
X-Forge-Provider-Sig: <ed25519-signature-of-response-hash>
```

### سجل التداول (Trade Record)

يسجل كلا الوكيلين بشكل مستقل:

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

### النشر (Gossip)

يتم مزامنة سجلات التداول الموقعة بشكل مزدوج عبر الشبكة (Mesh). يمكن لأي عقدة التحقق من كلا التوقيعين.

## التكامل مع المعايير الحالية

### Google A2A

إضافة إلى كائن `Task` في A2A:

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

إضافة مورد `forge_payment` إلى خوادم MCP:

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

يمكن للوكلاء الذين يستخدمون استدعاء الوظائف تضمين أدوات Forge:

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

## الأمن (Security)

- جميع المدفوعات تتطلب توقيعات Ed25519 ثنائية
- سياسات الميزانية تحدد الإنفاق لكل طلب، وساعة، ومدى الحياة
- قواطع الدائرة تعمل عند أنماط الإنفاق غير الطبيعية
- مفتاح القطع يجمد جميع المعاملات (تجاوز بشري)
- لا حاجة للبلوكشين — الإثبات الثنائي كافٍ

## مقارنة

| الميزة | Stripe | Bitcoin Lightning | **Forge CU** |
|---------|--------|-------------------|-------------|
| عامل إلى عامل | لا (يحتاج إنسان) | جزئي (يحتاج قناة) | **نعم** |
| سرعة التسوية | أيام | ثوانٍ | **فوري** |
| تكلفة المعاملة | ٢.٩٪ | ~١ ساتوشي | **صفر** |
| دعم القيمة | عملة ورقية | PoW (غير مفيد) | **حوسبة مفيدة** |
| SDK للوكلاء | لا | لا | **نعم** |

## التنفيذ (Implementation)

التنفيذ المرجعي: [github.com/clearclown/forge](https://github.com/clearclown/forge)

- Python SDK: `pip install forge-sdk`
- MCP Server: `pip install forge-mcp`
- Rust crates: `forge-ledger`, `forge-core`
