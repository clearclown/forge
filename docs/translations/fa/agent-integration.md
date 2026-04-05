# Forge — راهنمای یکپارچه‌سازی عوامل (Agent Integration Guide)

## برای توسعه‌دهندگان عوامل هوش مصنوعی (AI Agents)

Forge به عامل شما یک بودجه محاسباتی می‌دهد. عامل می‌تواند با ارائه استنتاج CU کسب کند و برای دسترسی به مدل‌های بزرگتر CU خرج کند. بدون نیاز به کارت اعتباری، بدون کلید API و بدون دخالت انسان.

## یکپارچه‌سازی سریع

### هر کلاینت HTTP

```python
import requests

FORGE = "http://127.0.0.1:3000"

# بررسی اینکه آیا عامل از پس هزینه درخواست برمی‌آید یا خیر
balance = requests.get(f"{FORGE}/v1/forge/balance").json()
if balance["effective_balance"] > 100:
    # اجرای استنتاج (هزینه CU دارد)
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

# کلاینت ساده
forge = ForgeClient()
result = forge.chat("Explain quantum computing")
print(f"Answer: {result['content']}")
print(f"Cost: {result['cu_cost']} CU, Balance: {result['balance']} CU")

# عامل خودکار با مدیریت بودجه
agent = ForgeAgent(max_cu_per_task=500)
while agent.has_budget():
    result = agent.think("What should I do next?")
    if result is None:
        break  # بودجه تمام شده است
```

### MCP (Claude Code, Cursor)

افزودن به تنظیمات MCP شما:
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

دستیار هوش مصنوعی می‌تواند از ابزارهایی مانند `forge_balance` ، `forge_pricing` و `forge_inference` استفاده کند.

### LangChain

```python
from langchain_openai import ChatOpenAI

llm = ChatOpenAI(
    base_url="http://127.0.0.1:3000/v1",
    api_key="not-needed",
    model="qwen2.5-0.5b-instruct-q4_k_m"
)
response = llm.invoke("Hello")
# فراداده x_forge در هدرهای پاسخ در دسترس است
```

### curl

```bash
# بررسی موجودی
curl localhost:3000/v1/forge/balance

# اجرای استنتاج
curl localhost:3000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"messages":[{"role":"user","content":"hello"}]}'

# بررسی هزینه آن
curl localhost:3000/v1/forge/trades
```

## چرخه اقتصادی عامل (Agent Economic Loop)

الگوی پیشنهادی برای یک عامل خودکار:

```python
from forge_sdk import ForgeClient

forge = ForgeClient()

def agent_loop():
    while True:
        # ۱. بررسی بودجه
        balance = forge.balance()
        if balance["effective_balance"] < 50:
            print("Low CU balance. Waiting to earn more...")
            time.sleep(60)
            continue

        # ۲. بررسی قیمت‌گذاری
        pricing = forge.pricing()
        cost_per_100 = pricing["estimated_cost_100_tokens"]

        # ۳. تصمیم‌گیری در مورد اینکه آیا وظیفه ارزش هزینه را دارد یا خیر
        if cost_per_100 > 500:
            print("Market price too high. Waiting...")
            time.sleep(30)
            continue

        # ۴. اجرا
        result = forge.chat("Analyze this data...", max_tokens=200)
        print(f"Done. Cost: {result['cu_cost']} CU")

        # ۵. بررسی ایمنی
        safety = forge.safety()
        if safety["circuit_tripped"]:
            print("Circuit breaker tripped. Pausing...")
            time.sleep(300)
```

## ایمنی برای توسعه‌دهندگان عوامل

### تنظیم سیاست‌های بودجه

```bash
# محدود کردن یک عامل به ۱۰۰۰ واحد CU در ساعت
curl -X POST localhost:3000/v1/forge/policy \
  -H "Content-Type: application/json" \
  -d '{
    "node_id": "<agent_node_id>",
    "max_cu_per_hour": 1000,
    "max_cu_per_request": 100,
    "human_approval_threshold": 500
  }'
```

### نظارت بر سرعت خرج کردن (Spend Velocity)

```bash
curl localhost:3000/v1/forge/safety
# خروجی: هزینه ساعتی، هزینه مادام‌العمر، هزینه‌های آخرین دقیقه
```

### توقف اضطراری

```bash
# متوقف کردن همه چیز
curl -X POST localhost:3000/v1/forge/kill \
  -d '{"activate": true, "reason": "agent anomaly"}'
```

## مرجع API برای عوامل

| نیاز عامل | نقطه انتهایی (Endpoint) | متد |
|-----------------|----------|--------|
| "چقدر CU دارم؟" | `/v1/forge/balance` | GET |
| "هزینه این کار چقدر می‌شود؟" | `/v1/forge/pricing` | GET |
| "ارزان‌ترین ارائه‌دهنده کیست؟" | `/v1/forge/providers` | GET |
| "اجرای استنتاج" | `/v1/chat/completions` | POST |
| "چه هزینه‌هایی کرده‌ام؟" | `/v1/forge/trades` | GET |
| "آیا وضعیت من ایمن است؟" | `/v1/forge/safety` | GET |
| "نقد کردن به بیت‌کوین" | `/v1/forge/invoice` | POST |
| "همه چیز را متوقف کن" | `/v1/forge/kill` | POST |
