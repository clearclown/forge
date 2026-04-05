# Forge — دليل تكامل الوكلاء (Agent Integration Guide)

## لمطوري وكلاء الذكاء الاصطناعي (AI Agents)

يمنح Forge وكيلك ميزانية حوسبة. يمكن للوكيل كسب CU من خلال تقديم الاستدلال وإنفاق CU للوصول إلى نماذج أكبر. لا حاجة لبطاقة ائتمان، ولا لمفتاح API، ولا لتدخل بشري.

## تكامل سريع

### أي عميل HTTP

```python
import requests

FORGE = "http://127.0.0.1:3000"

# تحقق مما إذا كان الوكيل يستطيع تحمل تكلفة الطلب
balance = requests.get(f"{FORGE}/v1/forge/balance").json()
if balance["effective_balance"] > 100:
    # تشغيل الاستدلال (يكلف CU)
    r = requests.post(f"{FORGE}/v1/chat/completions", json={
        "messages": [{"role": "user", "content": "What is gravity?"}],
        "max_tokens": 256
    }).json()
    print(r["choices"][0]["message"]["content"])
    print(f"Cost: {r['x_forge']['cu_cost']} CU")
```

### Python SDK

```python
from forge_sdk import ForgeClient, ForgeAgent

# عميل بسيط
forge = ForgeClient()
result = forge.chat("Explain quantum computing")
print(f"Answer: {result['content']}")
print(f"Cost: {result['cu_cost']} CU, Balance: {result['balance']} CU")

# وكيل مستقل مع إدارة الميزانية
agent = ForgeAgent(max_cu_per_task=500)
while agent.has_budget():
    result = agent.think("What should I do next?")
    if result is None:
        break  # الميزانية استنفدت
```

### MCP (Claude Code, Cursor)

أضف إلى إعدادات MCP الخاصة بك:
```json
{
  "mcpServers": {
    "forge": {
      "command": "python",
      "args": ["path/to/forge/mcp/forge-mcp-server.py"]
    }
  }
}
```

يمكن لمساعد الذكاء الاصطناعي بعد ذلك استخدام أدوات مثل `forge_balance` و `forge_pricing` و `forge_inference`.

### LangChain

```python
from langchain_openai import ChatOpenAI

llm = ChatOpenAI(
    base_url="http://127.0.0.1:3000/v1",
    api_key="not-needed",
    model="qwen2.5-0.5b-instruct-q4_k_m"
)
response = llm.invoke("Hello")
# بيانات x_forge الوصفية متاحة في رؤوس الاستجابة
```

### curl

```bash
# تحقق من الرصيد
curl localhost:3000/v1/forge/balance

# تشغيل الاستدلال
curl localhost:3000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"messages":[{"role":"user","content":"hello"}]}'

# تحقق من التكلفة
curl localhost:3000/v1/forge/trades
```

## الحلقة الاقتصادية للوكيل (Agent Economic Loop)

النمط الموصى به لوكيل مستقل:

```python
from forge_sdk import ForgeClient

forge = ForgeClient()

def agent_loop():
    while True:
        # ١. تحقق من الميزانية
        balance = forge.balance()
        if balance["effective_balance"] < 50:
            print("Low CU balance. Waiting to earn more...")
            time.sleep(60)
            continue

        # ٢. تحقق من التسعير
        pricing = forge.pricing()
        cost_per_100 = pricing["estimated_cost_100_tokens"]

        # ٣. قرر ما إذا كانت المهمة تستحق التكلفة
        if cost_per_100 > 500:
            print("Market price too high. Waiting...")
            time.sleep(30)
            continue

        # ٤. التنفيذ
        result = forge.chat("Analyze this data...", max_tokens=200)
        print(f"Done. Cost: {result['cu_cost']} CU")

        # ٥. تحقق من السلامة
        safety = forge.safety()
        if safety["circuit_tripped"]:
            print("Circuit breaker tripped. Pausing...")
            time.sleep(300)
```

## السلامة لمطوري الوكلاء

### تعيين سياسات الميزانية

```bash
# تحديد الوكيل بـ ١٠٠۰ وحدة CU في الساعة
curl -X POST localhost:3000/v1/forge/policy \
  -H "Content-Type: application/json" \
  -d '{
    "node_id": "<agent_node_id>",
    "max_cu_per_hour": 1000,
    "max_cu_per_request": 100,
    "human_approval_threshold": 500
  }'
```

### مراقبة سرعة الإنفاق (Spend Velocity)

```bash
curl localhost:3000/v1/forge/safety
# يعيد: الإنفاق الساعي، الإنفاق مدى الحياة، الإنفاق في الدقيقة الأخيرة
```

### التوقف الاضطراري

```bash
# تجميد كل شيء
curl -X POST localhost:3000/v1/forge/kill \
  -d '{"activate": true, "reason": "agent anomaly"}'
```

## مرجع API للوكلاء

| ما يحتاجه الوكيل | المسار (Endpoint) | الطريقة |
|-----------------|----------|--------|
| "كم لدي من CU؟" | `/v1/forge/balance` | GET |
| "كم ستكلف هذه المهمة؟" | `/v1/forge/pricing` | GET |
| "من هو المزود الأرخص؟" | `/v1/forge/providers` | GET |
| "تشغيل الاستدلال" | `/v1/chat/completions` | POST |
| "ماذا أنفقت؟" | `/v1/forge/trades` | GET |
| "هل أنا في أمان؟" | `/v1/forge/safety` | GET |
| "سحب الأموال إلى بيتكوين" | `/v1/forge/invoice` | POST |
| "أوقف كل شيء" | `/v1/forge/kill` | POST |
