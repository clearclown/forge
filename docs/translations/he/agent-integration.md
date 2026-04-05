# Forge — מדריך אינטגרציה לסוכנים (Agent Integration Guide)

## למפתחי סוכני בינה מלאכותית (AI Agents)

Forge מעניק לסוכן שלך תקציב חישוב. הסוכן יכול להרוויח CU על ידי מתן שירותי אינפרנס ולהוציא CU כדי לגשת למודלים גדולים יותר. ללא כרטיס אשראי, ללא מפתח API וללא מעורבות אנושית בתהליך.

## אינטגרציה מהירה

### כל לקוח HTTP

```python
import requests

FORGE = "http://127.0.0.1:3000"

# בדיקה האם לסוכן יש מספיק תקציב לבקשה
balance = requests.get(f"{FORGE}/v1/forge/balance").json()
if balance["effective_balance"] > 100:
    # הרצת אינפרנס (עולה CU)
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

# לקוח פשוט
forge = ForgeClient()
result = forge.chat("Explain quantum computing")
print(f"Answer: {result['content']}")
print(f"Cost: {result['cu_cost']} CU, Balance: {result['balance']} CU")

# סוכן אוטונומי עם ניהול תקציב
agent = ForgeAgent(max_cu_per_task=500)
while agent.has_budget():
    result = agent.think("What should I do next?")
    if result is None:
        break  # התקציב אזל
```

### MCP (Claude Code, Cursor)

הוסיפו להגדרות ה-MCP שלכם:
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

עוזר ה-AI יוכל להשתמש בכלים כמו `forge_balance`, `forge_pricing`, `forge_inference`.

### LangChain

```python
from langchain_openai import ChatOpenAI

llm = ChatOpenAI(
    base_url="http://127.0.0.1:3000/v1",
    api_key="not-needed",
    model="qwen2.5-0.5b-instruct-q4_k_m"
)
response = llm.invoke("Hello")
# מטא-דאטה של x_forge זמין בכותרות התגובה
```

### curl

```bash
# בדיקת יתרה
curl localhost:3000/v1/forge/balance

# הרצת אינפרנס
curl localhost:3000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"messages":[{"role":"user","content":"hello"}]}'

# בדיקת העלות
curl localhost:3000/v1/forge/trades
```

## הלופ הכלכלי של הסוכן (Agent Economic Loop)

הדפוס המומלץ עבור סוכן אוטונומי:

```python
from forge_sdk import ForgeClient

forge = ForgeClient()

def agent_loop():
    while True:
        # 1. בדיקת תקציב
        balance = forge.balance()
        if balance["effective_balance"] < 50:
            print("Low CU balance. Waiting to earn more...")
            time.sleep(60)
            continue

        # 2. בדיקת תמחור
        pricing = forge.pricing()
        cost_per_100 = pricing["estimated_cost_100_tokens"]

        # 3. החלטה האם המשימה שווה את העלות
        if cost_per_100 > 500:
            print("Market price too high. Waiting...")
            time.sleep(30)
            continue

        # 4. ביצוע
        result = forge.chat("Analyze this data...", max_tokens=200)
        print(f"Done. Cost: {result['cu_cost']} CU")

        # 5. בדיקת בטיחות
        safety = forge.safety()
        if safety["circuit_tripped"]:
            print("Circuit breaker tripped. Pausing...")
            time.sleep(300)
```

## בטיחות למפתחי סוכנים

### הגדרת מדיניות תקציב

```bash
# הגבלת סוכן ל-1000 CU בשעה
curl -X POST localhost:3000/v1/forge/policy \
  -H "Content-Type: application/json" \
  -d '{
    "node_id": "<agent_node_id>",
    "max_cu_per_hour": 1000,
    "max_cu_per_request": 100,
    "human_approval_threshold": 500
  }'
```

### ניטור קצב הוצאות (Spend Velocity)

```bash
curl localhost:3000/v1/forge/safety
# מחזיר: הוצאה שעתית, הוצאה לכל החיים, הוצאות בדקה האחרונה
```

### עצירת חירום

```bash
# הקפאת הכל
curl -X POST localhost:3000/v1/forge/kill \
  -d '{"activate": true, "reason": "agent anomaly"}'
```

## רפרנס API לסוכנים

| מה הסוכן צריך | נקודת קצה (Endpoint) | מתודה |
|-----------------|----------|--------|
| "כמה CU יש לי?" | `/v1/forge/balance` | GET |
| "כמה זה יעלה?" | `/v1/forge/pricing` | GET |
| "מי הספק הכי זול?" | `/v1/forge/providers` | GET |
| "הרצת אינפרנס" | `/v1/chat/completions` | POST |
| "מה הוצאתי?" | `/v1/forge/trades` | GET |
| "האם אני במצב בטוח?" | `/v1/forge/safety` | GET |
| "משיכת כספים לביטקוין" | `/v1/forge/invoice` | POST |
| "לעצור הכל" | `/v1/forge/kill` | POST |
