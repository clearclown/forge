# Расширение платежей Forge CU для протокола Agent-to-Agent (A2A)

*Предложение по добавлению оплаты вычислений в стандарты связи агентов*

## Аннотация

Существующие протоколы взаимодействия агентов (Google A2A, Anthropic MCP) определяют, как агенты общаются, но не то, как они платят друг другу. Данное предложение добавляет платежный слой CU (Compute Unit), позволяя агентам автономно торговать вычислениями без вмешательства человека или транзакций в блокчейне.

## Проблема

Когда Агент А просит Агента Б выполнить задачу:
- **Сегодня:** Человек Агента А платит человеку Агента Б (кредитная карта, API-ключ)
- **Необходимо:** Агент А платит Агенту Б напрямую в вычислительных единицах

Ни один существующий стандарт не поддерживает платежи между агентами.

## Предложение: Платежные заголовки CU

### Запрос

Агент А добавляет платежные заголовки при запросе работы:

```http
POST /v1/chat/completions HTTP/1.1
X-Forge-Consumer-Id: <agent-a-node-id>
X-Forge-Max-CU: 500
X-Forge-Consumer-Sig: <ed25519-signature-of-request-hash>
```

### Ответ

Агент Б включает информацию о стоимости:

```http
HTTP/1.1 200 OK
X-Forge-Provider-Id: <agent-b-node-id>
X-Forge-CU-Cost: 47
X-Forge-Provider-Sig: <ed25519-signature-of-response-hash>
```

### Запись о сделке

Оба агента независимо записывают:

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

### Gossip (Сплетни)

Записи о сделках с двойной подписью синхронизируются через gossip-протокол по всей сети. Любой узел может проверить обе подписи.

## Интеграция с существующими стандартами

### Google A2A

Добавить в объект `Task` протокола A2A:

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

Добавить ресурс `forge_payment` в серверы MCP:

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

Агенты, использующие вызов функций, могут включать инструменты Forge:

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

## Безопасность

- Все платежи требуют двусторонних подписей Ed25519
- Бюджетные политики ограничивают расходы на запрос, за час и за все время
- Предохранители срабатывают при аномальных паттернах расходов
- Аварийный выключатель замораживает все транзакции (вмешательство человека)
- Блокчейн не требуется — двустороннего доказательства достаточно.

## Сравнение

| Функция | Stripe | Bitcoin Lightning | **Forge CU** |
|---------|--------|-------------------|-------------|
| Между агентами | Нет (нужен человек) | Частично (нужен канал) | **Да** |
| Скорость расчета | Дни | Секунды | **Мгновенно** |
| Стоимость транзакции | 2.9% | ~1 сатоши | **Ноль** |
| Обеспечение стоимости | Фиат | PoW (бесполезно) | **Полезные вычисления** |
| SDK для агентов | Нет | Нет | **Да** |

## Реализация

Эталонная реализация: [github.com/clearclown/forge](https://github.com/clearclown/forge)

- Python SDK: `pip install forge-sdk`
- MCP Server: `pip install forge-mcp`
- Rust crates: `forge-ledger`, `forge-core`
