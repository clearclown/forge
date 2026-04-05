# Forge — Руководство по интеграции агентов

## Для разработчиков ШИ-агентов

Forge предоставляет вашему агенту бюджет на вычисления. Агент может зарабатывать CU, обслуживая инференс, и тратить CU для доступа к более крупным моделям. Никаких кредитных карт, API-ключей и участия человека.

## Быстрая интеграция

### Любой HTTP-клиент

```python
import requests

FORGE = "http://127.0.0.1:3000"

# Проверка, может ли агент позволить себе запрос
balance = requests.get(f"{FORGE}/v1/forge/balance").json()
if balance["effective_balance"] > 100:
    # Запуск инференса (стоит CU)
    r = requests.post(f"{FORGE}/v1/chat/completions", json={
        "messages": [{"role": "user", "content": "Что такое гравитация?"}],
        "max_tokens": 256
    }).json()
    print(r["choices"][0]["message"]["content"])
    print(f"Стоимость: {r['x_forge']['cu_cost']} CU")
```

### Python SDK

```python
from forge_sdk import ForgeClient, ForgeAgent

# Простой клиент
forge = ForgeClient()
result = forge.chat("Объясни квантовые вычисления")
print(f"Ответ: {result['content']}")
print(f"Стоимость: {result['cu_cost']} CU, Баланс: {result['balance']} CU")

# Автономный агент с управлением бюджетом
agent = ForgeAgent(max_cu_per_task=500)
while agent.has_budget():
    result = agent.think("Что мне делать дальше?")
    if result is None:
        break  # бюджет исчерпан
```

### MCP (Claude Code, Cursor)

Добавьте в настройки MCP:
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

После этого ИИ-ассистент сможет использовать инструменты типа `forge_balance`, `forge_pricing`, `forge_inference`.

### LangChain

```python
from langchain_openai import ChatOpenAI

llm = ChatOpenAI(
    base_url="http://127.0.0.1:3000/v1",
    api_key="not-needed",
    model="qwen2.5-0.5b-instruct-q4_k_m"
)
response = llm.invoke("Привет")
# метаданные x_forge доступны в заголовках ответа
```

### curl

```bash
# Проверить баланс
curl localhost:3000/v1/forge/balance

# Запустить инференс
curl localhost:3000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"messages":[{"role":"user","content":"привет"}]}'

# Проверить стоимость
curl localhost:3000/v1/forge/trades
```

## Экономический цикл агента

Рекомендуемый паттерн для автономного агента:

```python
from forge_sdk import ForgeClient

forge = ForgeClient()

def agent_loop():
    while True:
        # 1. Проверить бюджет
        balance = forge.balance()
        if balance["effective_balance"] < 50:
            print("Низкий баланс CU. Ожидание заработка...")
            time.sleep(60)
            continue

        # 2. Проверить цены
        pricing = forge.pricing()
        cost_per_100 = pricing["estimated_cost_100_tokens"]

        # 3. Решить, стоит ли задача своих денег
        if cost_per_100 > 500:
            print("Рыночная цена слишком высока. Ожидание...")
            time.sleep(30)
            continue

        # 4. Выполнить
        result = forge.chat("Проанализируй эти данные...", max_tokens=200)
        print(f"Готово. Стоимость: {result['cu_cost']} CU")

        # 5. Проверить безопасность
        safety = forge.safety()
        if safety["circuit_tripped"]:
            print("Предохранитель сработал. Пауза...")
            time.sleep(300)
```

## Безопасность для разработчиков агентов

### Установка бюджетных политик

```bash
# Ограничить агента до 1000 CU в час
curl -X POST localhost:3000/v1/forge/policy \
  -H "Content-Type: application/json" \
  -d '{
    "node_id": "<agent_node_id>",
    "max_cu_per_hour": 1000,
    "max_cu_per_request": 100,
    "human_approval_threshold": 500
  }'
```

### Мониторинг скорости трат

```bash
curl localhost:3000/v1/forge/safety
# Возвращает: hourly_spend, lifetime_spend, spends_last_minute
```

### Экстренная остановка

```bash
# Заморозить всё
curl -X POST localhost:3000/v1/forge/kill \
  -d '{"activate": true, "reason": "аномалия агента"}'
```

## API Справочник для агентов

| Что нужно агенту | Эндпоинт | Метод |
|-----------------|----------|--------|
| "Сколько у меня CU?" | `/v1/forge/balance` | GET |
| "Сколько это будет стоить?" | `/v1/forge/pricing` | GET |
| "Кто самый дешевый провайдер?" | `/v1/forge/providers` | GET |
| "Запустить инференс" | `/v1/chat/completions` | POST |
| "На что я потратил?" | `/v1/forge/trades` | GET |
| "Я в безопасности?" | `/v1/forge/safety` | GET |
| "Вывести в Bitcoin" | `/v1/forge/invoice` | POST |
| "ОСТАНОВИТЬ ВСЁ" | `/v1/forge/kill` | POST |
