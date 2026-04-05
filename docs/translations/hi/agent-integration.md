# Forge — एजेंट एकीकरण गाइड (Agent Integration Guide)

## AI एजेंट डेवलपर्स के लिए

Forge आपके एजेंट को एक कंप्यूट बजट (compute budget) देता है। एजेंट इन्फरेंस (inference) सेवा देकर CU कमा सकता है और बड़े मॉडल तक पहुँचने के लिए CU खर्च कर सकता है। कोई क्रेडिट कार्ड नहीं, कोई API कुंजी नहीं, और किसी मानव की आवश्यकता नहीं।

## त्वरित एकीकरण (Quick Integration)

### कोई भी HTTP क्लाइंट (Any HTTP Client)

```python
import requests

FORGE = "http://127.0.0.1:3000"

# जांचें कि क्या एजेंट अनुरोध का खर्च उठा सकता है
balance = requests.get(f"{FORGE}/v1/forge/balance").json()
if balance["effective_balance"] > 100:
    # इन्फरेंस चलाएं (इसकी लागत CU में होगी)
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

# साधारण क्लाइंट
forge = ForgeClient()
result = forge.chat("Explain quantum computing")
print(f"Answer: {result['content']}")
print(f"Cost: {result['cu_cost']} CU, Balance: {result['balance']} CU")

# बजट प्रबंधन के साथ स्वायत्त एजेंट
agent = ForgeAgent(max_cu_per_task=500)
while agent.has_budget():
    result = agent.think("What should I do next?")
    if result is None:
        break  # बजट खत्म हो गया
```

### MCP (Claude Code, Cursor)

अपने MCP सेटिंग्स में जोड़ें:
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

इसके बाद AI सहायक `forge_balance`, `forge_pricing`, `forge_inference` जैसे टूल का उपयोग कर सकता है।

### LangChain

```python
from langchain_openai import ChatOpenAI

llm = ChatOpenAI(
    base_url="http://127.0.0.1:3000/v1",
    api_key="not-needed",
    model="qwen2.5-0.5b-instruct-q4_k_m"
)
response = llm.invoke("Hello")
# x_forge मेटाडेटा रिस्पांस हेडर में उपलब्ध है
```

### curl

```bash
# बैलेंस चेक करें
curl localhost:3000/v1/forge/balance

# इन्फरेंस चलाएं
curl localhost:3000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"messages":[{"role":"user","content":"hello"}]}'

# इसकी लागत क्या थी, यह जांचें
curl localhost:3000/v1/forge/trades
```

## एजेंट आर्थिक लूप (Agent Economic Loop)

एक स्वायत्त एजेंट के लिए अनुशंसित पैटर्न:

```python
from forge_sdk import ForgeClient

forge = ForgeClient()

def agent_loop():
    while True:
        # 1. बजट जांचें
        balance = forge.balance()
        if balance["effective_balance"] < 50:
            print("CU बैलेंस कम है। और कमाने के लिए प्रतीक्षा कर रहा हूँ...")
            time.sleep(60)
            continue

        # 2. मूल्य निर्धारण जांचें
        pricing = forge.pricing()
        cost_per_100 = pricing["estimated_cost_100_tokens"]

        # 3. निर्णय लें कि क्या कार्य लागत के लायक है
        if cost_per_100 > 500:
            print("बाजार मूल्य बहुत अधिक है। प्रतीक्षा कर रहा हूँ...")
            time.sleep(30)
            continue

        # 4. निष्पादित करें
        result = forge.chat("Analyze this data...", max_tokens=200)
        print(f"हो गया। लागत: {result['cu_cost']} CU")

        # 5. सुरक्षा जांचें
        safety = forge.safety()
        if safety["circuit_tripped"]:
            print("सर्किट ब्रेकर ट्रिप हो गया है। रुक रहा हूँ...")
            time.sleep(300)
```

## एजेंट डेवलपर्स के लिए सुरक्षा (Safety)

### बजट नीतियां निर्धारित करें (Set Budget Policies)

```bash
# एक एजेंट को प्रति घंटे 1000 CU तक सीमित करें
curl -X POST localhost:3000/v1/forge/policy \
  -H "Content-Type: application/json" \
  -d '{
    "node_id": "<agent_node_id>",
    "max_cu_per_hour": 1000,
    "max_cu_per_request": 100,
    "human_approval_threshold": 500
  }'
```

### खर्च की गति की निगरानी करें (Monitor Spend Velocity)

```bash
curl localhost:3000/v1/forge/safety
# रिटर्न करता है: hourly_spend, lifetime_spend, spends_last_minute
```

### आपातकालीन रोक (Emergency Stop)

```bash
# सब कुछ फ्रीज करें
curl -X POST localhost:3000/v1/forge/kill \
  -d '{"activate": true, "reason": "agent anomaly"}'
```

## एजेंटों के लिए API संदर्भ (API Reference for Agents)

| एजेंट को क्या चाहिए | एंडपॉइंट (Endpoint) | विधि (Method) |
|-----------------|----------|--------|
| "मेरे पास कितना CU है?" | `/v1/forge/balance` | GET |
| "इसकी लागत कितनी होगी?" | `/v1/forge/pricing` | GET |
| "सबसे सस्ता प्रदाता कौन है?" | `/v1/forge/providers` | GET |
| "इन्फरेंस चलाएं" | `/v1/chat/completions` | POST |
| "मैंने क्या खर्च किया?" | `/v1/forge/trades` | GET |
| "क्या मैं सुरक्षित हूँ?" | `/v1/forge/safety` | GET |
| "बिटकॉइन में कैश आउट करें" | `/v1/forge/invoice` | POST |
| "सब कुछ रोक दें" | `/v1/forge/kill` | POST |
