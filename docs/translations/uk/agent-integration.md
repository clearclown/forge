# Forge — Посібник з інтеграції агентів

## Для розробників ШІ-агентів

Forge надає вашому агенту бюджет на обчислення. Агент може заробляти CU, обслуговуючи інференс, і витрачати CU для доступу до більших моделей. Жодних кредитних карток, API-ключів або втручання людини.

## Швидка інтеграція

### Будь-який HTTP-клієнт

```python
import requests

FORGE = "http://127.0.0.1:3000"

# Перевірка, чи може агент дозволити собі запит
balance = requests.get(f"{FORGE}/v1/forge/balance").json()
if balance["effective_balance"] > 100:
    # Запуск інференсу (коштує CU)
    r = requests.post(f"{FORGE}/v1/chat/completions", json={
        "messages": [{"role": "user", "content": "Що таке гравітація?"}],
        "max_tokens": 256
    }).json()
    print(r["choices"][0]["message"]["content"])
    print(f"Вартість: {r['x_forge']['cu_cost']} CU")
```

### Python SDK

```python
from forge_sdk import ForgeClient, ForgeAgent

# Простий клієнт
forge = ForgeClient()
result = forge.chat("Поясни квантові обчислення")
print(f"Відповідь: {result['content']}")
print(f"Вартість: {result['cu_cost']} CU, Баланс: {result['balance']} CU")

# Автономний агент з управлінням бюджетом
agent = ForgeAgent(max_cu_per_task=500)
while agent.has_budget():
    result = agent.think("Що мені робити далі?")
    if result is None:
        break  # бюджет вичерпано
```

### MCP (Claude Code, Cursor)

Додайте до налаштувань MCP:
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

Після цього ШІ-асистент зможе використовувати інструменти на кшталт `forge_balance`, `forge_pricing`, `forge_inference`.

### LangChain

```python
from langchain_openai import ChatOpenAI

llm = ChatOpenAI(
    base_url="http://127.0.0.1:3000/v1",
    api_key="not-needed",
    model="qwen2.5-0.5b-instruct-q4_k_m"
)
response = llm.invoke("Привіт")
# метадані x_forge доступні в заголовках відповіді
```

### curl

```bash
# Перевірка балансу
curl localhost:3000/v1/forge/balance

# Запуск інференсу
curl localhost:3000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"messages":[{"role":"user","content":"привіт"}]}'

# Перевірка вартості
curl localhost:3000/v1/forge/trades
```

## Економічний цикл агента

Рекомендований патерн для автономного агента:

```python
from forge_sdk import ForgeClient

forge = ForgeClient()

def agent_loop():
    while True:
        # 1. Перевірка бюджету
        balance = forge.balance()
        if balance["effective_balance"] < 50:
            print("Низький баланс CU. Чекаю, щоб заробити більше...")
            time.sleep(60)
            continue

        # 2. Перевірка ціни
        pricing = forge.pricing()
        cost_per_100 = pricing["estimated_cost_100_tokens"]

        # 3. Рішення, чи варте завдання своєї вартості
        if cost_per_100 > 500:
            print("Ринкова ціна занадто висока. Чекаю...")
            time.sleep(30)
            continue

        # 4. Виконання
        result = forge.chat("Проаналізуй ці дані...", max_tokens=200)
        print(f"Готово. Вартість: {result['cu_cost']} CU")

        # 5. Перевірка безпеки
        safety = forge.safety()
        if safety["circuit_tripped"]:
            print("Запобіжник спрацював. Пауза...")
            time.sleep(300)
```

## Безпека для розробників агентів

### Встановлення політик бюджету

```bash
# Обмежити агента 1000 CU на годину
curl -X POST localhost:3000/v1/forge/policy \
  -H "Content-Type: application/json" \
  -d '{
    "node_id": "<agent_node_id>",
    "max_cu_per_hour": 1000,
    "max_cu_per_request": 100,
    "human_approval_threshold": 500
  }'
```

### Моніторинг швидкості витрат

```bash
curl localhost:3000/v1/forge/safety
# Повертає: hourly_spend, lifetime_spend, spends_last_minute
```

### Екстрена зупинка

```bash
# Заморозити все
curl -X POST localhost:3000/v1/forge/kill \
  -d '{"activate": true, "reason": "аномалія агента"}'
```

## API Довідка для агентів

| Що потрібно агенту | Ендпоінт | Метод |
|-----------------|----------|--------|
| "Скільки в мене CU?" | `/v1/forge/balance` | GET |
| "Скільки це буде коштувати?" | `/v1/forge/pricing` | GET |
| "Хто найдешевший провайдер?" | `/v1/forge/providers` | GET |
| "Запустити інференс" | `/v1/chat/completions` | POST |
| "Скільки я витратив?" | `/v1/forge/trades` | GET |
| "Чи я в безпеці?" | `/v1/forge/safety` | GET |
| "Вивести в Bitcoin" | `/v1/forge/invoice` | POST |
| "ЗУПИНИТИ ВСЕ" | `/v1/forge/kill` | POST |
