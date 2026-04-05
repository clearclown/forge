# Розширення оплати Forge CU для протоколу Agent-to-Agent (A2A)

*Пропозиція щодо додавання оплати обчислень до стандартів зв'язку агентів*

## Анотація

Існуючі протоколи взаємодії агентів (Google A2A, Anthropic MCP) визначають, як агенти спілкуються, але не те, як вони платять один одному. Ця пропозиція додає шар оплати CU (обчислювальних одиниць), дозволяючи агентам автономно торгувати обчисленнями без втручання людини або транзакцій у блокчейні.

## Проблема

Коли Агент А просить Агента Б виконати завдання:
- **Сьогодні:** людина Агента А платить людині Агента Б (кредитна картка, API-ключ)
- **Необхідно:** Агент А платить Агенту Б безпосередньо в обчислювальних одиницях

Жоден існуючий стандарт не підтримує оплату між агентами.

## Пропозиція: Заголовки оплати CU

### Запит (Request)

Агент А додає заголовки оплати при запиті роботи:

```http
POST /v1/chat/completions HTTP/1.1
X-Forge-Consumer-Id: <agent-a-node-id>
X-Forge-Max-CU: 500
X-Forge-Consumer-Sig: <ed25519-signature-of-request-hash>
```

### Відповідь (Response)

Агент Б включає інформацію про вартість:

```http
HTTP/1.1 200 OK
X-Forge-Provider-Id: <agent-b-node-id>
X-Forge-CU-Cost: 47
X-Forge-Provider-Sig: <ed25519-signature-of-response-hash>
```

### Запис операції (Trade Record)

Обидва агенти незалежно записують:

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

### Gossip

Записи операцій з двостороннім підписом синхронізуються через gossip-протокол по всій мережі. Будь-який вузол може перевірити обидва підписи.

## Інтеграція з існуючими стандартами

### Google A2A

Додати до об'єкта `Task` в A2A:

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

Додати ресурс `forge_payment` до MCP-серверів:

```json
{
  "resources": [{
    "uri": "forge://payment/balance",
    "name": "CU Balance",
    "mimeType": "application/json"
  }]
}
```

### Виклик функцій OpenAI (OpenAI Function Calling)

Агенти, що використовують виклик функцій, можуть включати інструменти Forge:

```json
{
  "tools": [{
    "type": "function",
    "function": {
      "name": "forge_pay",
      "description": "Оплатити CU за обчислювальне завдання",
      "parameters": {
        "provider": "string",
        "cu_amount": "integer"
      }
    }
  }]
}
```

## Безпека

- Усі платежі потребують двосторонніх підписів Ed25519
- Політики бюджету обмежують витрати на запит, за годину та за весь час
- Запобіжники спрацьовують при аномальних моделях витрат
- "Аварійний вимикач" заморожує всі транзакції (втручання людини)
- Блокчейн не потрібен — двостороннього доказу достатньо

## Порівняння

| Функція | Stripe | Bitcoin Lightning | **Forge CU** |
|---------|--------|-------------------|-------------|
| Міжагентська оплата | Ні (потрібна людина) | Частково (потрібен канал) | **Так** |
| Швидкість розрахунку | Дні | Секунди | **Миттєво** |
| Вартість транзакції | 2.9% | ~1 sat | **Нуль** |
| Забезпечення вартості | Фіат | PoW (марна робота) | **Корисні обчислення** |
| SDK для агентів | Ні | Ні | **Так** |

## Реалізація

Еталонна реалізація: [github.com/clearclown/forge](https://github.com/clearclown/forge)

- Python SDK: `pip install forge-sdk`
- MCP Server: `pip install forge-mcp`
- Rust crates: `forge-ledger`, `forge-core`
