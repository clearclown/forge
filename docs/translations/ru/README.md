<div align="center">

# Tirami

**Вычисления — это валюта. Каждый ватт создает интеллект, а не отходы.**

[![Crates.io](https://img.shields.io/crates/v/tirami-core?label=crates.io&color=e6522c)](https://crates.io/crates/tirami-core)
[![License: MIT](https://img.shields.io/badge/License-MIT-brightgreen.svg)](../../../LICENSE)

---

[English](../../../README.md) · [日本語](../ja/README.md) · [简体中文](../zh-CN/README.md) · [繁體中文](../zh-TW/README.md) · [Español](../es/README.md) · [Français](../fr/README.md) · **Русский** · [Українська](../uk/README.md) · [हिन्दी](../hi/README.md) · [العربية](../ar/README.md) · [فارسی](../fa/README.md) · [עברית](../he/README.md)

</div>

**Tirami — это протокол децентрализованного инференса, в котором вычисления являются деньгами.** Узлы зарабатывают TRM (Tirami Resource Merit) (TRM (Tirami Resource Merit), TRM), выполняя полезный инференс LLM для других. В отличие от Bitcoin, где электричество сжигается на бессмысленные хеши, каждый джоуль, потраченный на узле Tirami, создает реальный интеллект, который кому-то действительно нужен.

Распределенный движок инференса построен на [mesh-llm](https://github.com/michaelneale/mesh-llm) Майкла Нила. Tirami добавляет сверху вычислительную экономику: учет TRM, доказательство полезной работы (Proof of Useful Work), динамическое ценообразование, бюджеты автономных агентов и средства безопасности. См. [CREDITS.md](../../../CREDITS.md).

**Интегрированный форк:** [tirami-mesh](https://github.com/nm-arealnormalman/mesh-llm) — mesh-llm со встроенным экономическим слоем Tirami.

## Живая демо-версия

Это реальный вывод работающего узла Tirami. Каждый инференс стоит TRM. Каждая единица TRM зарабатывается полезными вычислениями.

```
$ tirami node -m "qwen2.5:0.5b" --ledger tirami-ledger.json
  Model loaded: Qwen2.5-0.5B (Metal-accelerated, 491MB)
  API server listening on 127.0.0.1:3000
```

**Проверка баланса — каждый новый узел получает 1000 TRM бесплатно:**
```
$ curl localhost:3000/v1/tirami/balance
{
  "effective_balance": 1000,
  "contributed": 0,
  "consumed": 0,
  "reputation": 0.5
}
```

**Задать вопрос — инференс стоит TRM:**
```
$ curl localhost:3000/v1/chat/completions \
    -d '{"messages":[{"role":"user","content":"Say hello in Japanese"}]}'
{
  "choices": [{"message": {"content": "こんにちは！ (konnichiwa!)"}}],
  "usage": {"completion_tokens": 9},
  "x_tirami": {
    "trm_cost": 9,
    "effective_balance": 1009
  }
}
```

Каждый ответ содержит `x_tirami` — **стоимость этого вычисления в TRM** и остаток баланса. Провайдер заработал 9 TRM. Потребитель потратил 9 TRM. Физика стоит за каждой единицей.

**Через три инференса — реальные сделки в леджере:**
```
$ curl localhost:3000/v1/tirami/trades
{
  "count": 3,
  "trades": [
    {"trm_amount": 5, "tokens_processed": 5, "model_id": "qwen2.5-0.5b-instruct-q4_k_m"},
    {"trm_amount": 5, "tokens_processed": 5, "model_id": "qwen2.5-0.5b-instruct-q4_k_m"},
    {"trm_amount": 9, "tokens_processed": 9, "model_id": "qwen2.5-0.5b-instruct-q4_k_m"}
  ]
}
```

**Каждая сделка имеет корень Меркла — с возможностью фиксации в Bitcoin для неизменяемого доказательства:**
```
$ curl localhost:3000/v1/tirami/network
{
  "total_trades": 3,
  "total_contributed_trm": 19,
  "merkle_root": "aac8db9f62dd9ff23926195a70ed8fcfc188fc867d9f2adabd8e694beb338748"
}
```

**ШИ-агенты вышли из-под контроля? Аварийный выключатель замораживает всё за миллисекунды:**
```
$ curl -X POST localhost:3000/v1/tirami/kill \
    -d '{"activate":true, "reason":"detected anomaly", "operator":"admin"}'
→ KILL SWITCH ACTIVATED
→ All TRM transactions frozen. No agent can spend.
```

**Контроль безопасности всегда активен:**
```
$ curl localhost:3000/v1/tirami/safety
{
  "kill_switch_active": false,
  "circuit_tripped": false,
  "policy": {
    "max_trm_per_hour": 10000,
    "max_trm_per_request": 1000,
    "max_trm_lifetime": 1000000,
    "human_approval_threshold": 5000
  }
}
```

## Почему существует Tirami

```
Bitcoin:  электричество  →  бессмысленный SHA-256  →  BTC
Tirami:    электричество  →  полезный инференс LLM  →  TRM
```

Bitcoin доказал: `электричество → вычисления → деньги`. Но вычисления Bitcoin бесцельны. Tirami переворачивает это: каждая единица TRM представляет реальный интеллект, который решил чью-то реальную проблему.

**Четыре вещи, которые не делает ни один другой проект:**

### 1. Вычисления = Валюта

Каждый инференс — это обмен. Провайдер зарабатывает TRM, потребитель тратит TRM. Без блокчейна, без токенов, без ICO. TRM подкреплен физикой — электричеством, потребленным для полезной работы. В отличие от Bittensor (TAO), Akash (AKT) или Golem (GLM), TRM нельзя спекулировать — он зарабатывается выполнением полезных вычислений.

### 2. Защита от подделок без блокчейна

Каждая сделка подписывается обеими сторонами (Ed25519) и синхронизируется через gossip-протокол в сети. Корень Меркла всех сделок может быть закреплен в Bitcoin для неизменяемого аудита. Глобальный консенсус не нужен — двустороннего криптографического доказательства достаточно.

### 3. ШИ-агенты сами управляют своими вычислениями

Агент на телефоне одалживает свободные мощности ночью → зарабатывает TRM → покупает доступ к 70B модели → становится умнее → зарабатывает больше. Агент автономно проверяет `/v1/tirami/balance` и `/v1/tirami/pricing`. Политики бюджета и предохранители предотвращают неконтролируемые траты.

```
Агент (1.5B на телефоне)
  → зарабатывает TRM ночью, обслуживая инференс
  → тратит TRM на 70B модель → более умные ответы
  → лучшие решения → больше заработанных TRM
  → цикл повторяется → агент растет
```

### 4. Микрофинансирование вычислений

Узлы могут кредитовать простаивающие TRM другим узлам под проценты. Маленький узел занимает TRM, получает доступ к большей модели, зарабатывает больше TRM и возвращает заем с процентами. Ни один другой проект распределенного инференса не предлагает кредитование вычислений. Это механизм, который делает цикл самосовершенствования экономически жизнеспособным для всех, а не только для тех, кто уже владеет мощным оборудованием.

## Архитектура

```
┌─────────────────────────────────────────────────┐
│  L4: Обнаружение (tirami-agora) ✅ v0.1          │
│  Маркетплейс агентов, агрегация репутации,      │
│  Nostr NIP-90, расширение оплаты Google A2A     │
├─────────────────────────────────────────────────┤
│  L3: Интеллект (tirami-mind) ✅ v0.1             │
│  Циклы самосовершенствования AutoAgent,         │
│  маркетплейс харнессов, мета-оптимизация        │
├─────────────────────────────────────────────────┤
│  L2: Финансы (tirami-bank) ✅ v0.1               │
│  Стратегии, портфели, фьючерсы, страхование,   │
│  модель рисков, оптимизатор доходности          │
├─────────────────────────────────────────────────┤
│  L1: Экономика (tirami — этот репозиторий) ✅ Фаза 1-13 │
│  Леджер TRM, двусторонне подписанные сделки,     │
│  динамическое ценообразование, примитивы займа, │
│  средства безопасности                          │
├─────────────────────────────────────────────────┤
│  L0: Инференс (tirami-mesh / mesh-llm) ✅        │
│  Параллелизм пайплайна, MoE шардинг,            │
│  iroh mesh, Nostr discovery, MLX/llama.cpp      │
└─────────────────────────────────────────────────┘

Все 5 слоев существуют. 785 тестов проходят по всей экосистеме.
```

## Быстрый старт

### Вариант 1: Демо от начала до конца одной командой (Rust, ~30 секунд с нуля)

```bash
git clone https://github.com/clearclown/tirami && cd tirami
bash scripts/demo-e2e.sh
```

Скрипт скачивает SmolLM2-135M (~100 МБ) с HuggingFace, запускает настоящий узел Tirami с Metal/CUDA-ускорением, выполняет три реальных завершения чата, проходит все эндпоинты Фаз 1-13 и выводит цветной отчет. Проверено 2026-04-09 на Apple Silicon Metal GPU.

После завершения тот же узел отвечает на:

```bash
# Совместимый с OpenAI клиент
export OPENAI_BASE_URL=http://127.0.0.1:3001/v1
export OPENAI_API_KEY=$(cat ~/.tirami/api_token 2>/dev/null || echo "$TOKEN")

# Реальный потоковый вывод токен за токеном (Фаза 11)
curl -N $OPENAI_BASE_URL/chat/completions \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"model":"smollm2:135m","messages":[{"role":"user","content":"hi"}],"stream":true}'

# Экономика фаза 8 / репутация 9 / метрики 10 / якорение
curl $OPENAI_BASE_URL/tirami/balance -H "Authorization: Bearer $OPENAI_API_KEY"
curl $OPENAI_BASE_URL/tirami/anchor?network=mainnet -H "Authorization: Bearer $OPENAI_API_KEY"
curl http://127.0.0.1:3001/metrics  # Prometheus, без авторизации
```

Полная матрица функций (против llama.cpp / mesh-llm / Ollama / Bittensor / Akash) — в [`docs/compatibility.md`](../../../docs/compatibility.md).

### Вариант 2: Ручные команды Rust

**Предварительные требования**: [Установить Rust](https://rustup.rs/) (2 минуты)

```bash
cargo build --release

# Запуск узла — автоматически скачивает модель с HuggingFace
./target/release/tirami node -m "qwen2.5:0.5b" --ledger tirami-ledger.json

# Или любая из следующих команд:
./target/release/tirami chat -m "smollm2:135m" "Что такое гравитация?"
./target/release/tirami seed -m "qwen2.5:1.5b"               # зарабатывать TRM как P2P провайдер
./target/release/tirami worker --seed <public_key>           # тратить TRM как P2P потребитель
./target/release/tirami models                                # список каталога (или HF URL)
```

**[Crates.io: tirami-core](https://crates.io/crates/tirami-core)** ·
**[Документ совместимости](../../../docs/compatibility.md)** ·
**[Демо-скрипт](../../../scripts/demo-e2e.sh)**

### Вариант 3: Готовые бинарные файлы / Docker

Готовые бинарные файлы и Docker-образ `clearclown/tirami:latest` отслеживаются в
[releases](../../../releases). До этого Вариант 1 собирает из исходников менее чем за две минуты.

## API Справочник

### Инференс (совместим с OpenAI)

| Эндпоинт | Описание |
|----------|-------------|
| `POST /v1/chat/completions` | Чат со стримингом. Каждый ответ содержит `x_tirami.cu_cost` |
| `GET /v1/models` | Список загруженных моделей |

### Экономика

| Эндпоинт | Описание |
|----------|-------------|
| `GET /v1/tirami/balance` | Баланс TRM, репутация, история вклада |
| `GET /v1/tirami/pricing` | Рыночная цена (сглаженная EMA), оценка стоимости |
| `GET /v1/tirami/trades` | Последние сделки с суммами TRM |
| `GET /v1/tirami/network` | Общий поток TRM + корень Меркла |
| `GET /v1/tirami/providers` | Рейтинг провайдеров по репутации и цене |
| `POST /v1/tirami/invoice` | Создать Lightning инвойс из баланса TRM |
| `GET /v1/tirami/route` | Оптимальный выбор провайдера (стоимость/качество/баланс) |
| `GET /settlement` | Отчет о расчетах для экспорта |

### Кредитование

| Эндпоинт | Описание |
|----------|-------------|
| `POST /v1/tirami/lend` | Предложить TRM в пул кредитования |
| `POST /v1/tirami/borrow` | Запросить кредит в TRM |
| `POST /v1/tirami/repay` | Погасить непогашенный кредит |
| `GET /v1/tirami/credit` | Кредитный рейтинг и история |
| `GET /v1/tirami/pool` | Состояние пула кредитования |
| `GET /v1/tirami/loans` | Активные кредиты |

### Безопасность

| Эндпоинт | Описание |
|----------|-------------|
| `GET /v1/tirami/safety` | Состояние аварийного стопа, предохранители, политика бюджета |
| `POST /v1/tirami/kill` | Экстренная остановка — заморозить все транзакции TRM |
| `POST /v1/tirami/policy` | Установить лимиты бюджета для агентов |

## Дизайн безопасности

ШИ-агенты, автономно тратящие вычисления — это мощно, но опасно. Tirami имеет пять слоев безопасности:

| Слой | Механизм | Защита |
|-------|-----------|------------|
| **Kill Switch** | Оператор мгновенно замораживает все торги | Останавливает вышедших из-под контроля агентов |
| **Budget Policy** | Лимиты на агента: за запрос, почасово, за все время | Ограничивает общие риски |
| **Circuit Breaker** | Автоматически срабатывает при 5 ошибках или 30+ тратах/мин | Выявляет аномалии |
| **Velocity Detection** | Скользящее окно в 1 минуту для скорости трат | Предотвращает всплески |
| **Human Approval** | Транзакции выше порога требуют согласия человека | Охраняет крупные траты |

Принцип дизайна: **fail-safe**. Если любая проверка не может определить безопасность, она **отклоняет** действие.

## Идея

| Эпоха | Стандарт | Обеспечение |
|-----|----------|---------|
| Античность | Золото | Геологическая редкость |
| 1944–1971 | Бреттон-Вудс | USD, привязанный к золоту |
| 1971–сегодня | Петродоллар | Спрос на нефть + военная мощь |
| 2009–сегодня | Bitcoin | Энергия на SHA-256 (бесполезная работа) |
| **Сейчас** | **Вычислительный стандарт** | **Энергия на инференс LLM (полезная работа)** |

Комната, полная Mac Mini с Tirami — это многоквартирный дом, который приносит доход, выполняя полезную работу, пока владелец спит.

## Структура проекта

```
tirami/  (этот репозиторий — Слой 1)
├── crates/
│   ├── tirami-ledger/      # Учет TRM, кредитование, agora (NIP-90), безопасность
│   ├── tirami-node/        # Демон узла, HTTP API (кредитование + роутинг), пайплайн
│   ├── tirami-cli/         # CLI: chat, seed, worker, settle, wallet
│   ├── tirami-lightning/   # Мост TRM ↔ Bitcoin Lightning (двунаправленный)
│   ├── tirami-net/         # P2P: iroh QUIC + Noise + gossip (сделки + займы)
│   ├── tirami-proto/       # Протокол передачи: 27+ типов сообщений, вкл. Loan*
│   ├── tirami-infer/       # Инференс: llama.cpp, GGUF, Metal/CPU
│   ├── tirami-core/        # Типы: NodeId, TRM, Config
│   └── tirami-shard/       # Топология: назначение слоев
├── scripts/verify-impl.sh         # TDD регрессионный тест (24 утверждения)
└── docs/                  # Спецификации, стратегия, модель угроз, роадмап
```

~20,000 строк Rust. **785 теста проходят.** Фазы 1-6 завершены.

## Сопутствующие репозитории (полная экосистема)

| Репозиторий | Слой | Тесты | Статус |
|------|-------|-------|--------|
| [clearclown/tirami](https://github.com/clearclown/tirami) (этот) | L1 Экономика | 785 | Фаза 1-13 ✅ |
| [clearclown/tirami-bank](https://github.com/clearclown/tirami-bank) | L2 Финансы | — | archived |
| [clearclown/tirami-mind](https://github.com/clearclown/tirami-mind) | L3 Интеллект | — | archived |
| [clearclown/tirami-agora](https://github.com/clearclown/tirami-agora) | L4 Обнаружение | — | archived |
| [clearclown/tirami-economics](https://github.com/clearclown/tirami-economics) | Теория | 16/16 GREEN | ✅ |
| [nm-arealnormalman/mesh-llm](https://github.com/nm-arealnormalman/mesh-llm) | L0 Инференс | 43 (tirami-economy) | ✅ |

## Документация

- [Стратегия](../../../docs/strategy.md) — Конкурентное позиционирование, спецификация кредитования, 5-слойная архитектура
- [Монетарная теория](../../../docs/monetary-theory.md) — Почему работает TRM: Содди, Bitcoin, PoUW, валюта только для ИИ
- [Концепция и видение](../../../docs/concept.md) — Почему вычисления — это деньги
- [Экономическая модель](../../../docs/economy.md) — Экономика TRM, доказательство полезной работы
- [Архитектура](../../../docs/architecture.md) — Двухслойный дизайн
- [Интеграция агентов](../../../docs/agent-integration.md) — SDK, MCP, процесс кредитования
- [Протокол передачи](../../../docs/protocol-spec.md) — 17 типов сообщений
- [Роадмап](../../../docs/roadmap.md) — Фазы разработки
- [Модель угроз](../../../docs/threat-model.md) — Безопасность + экономические атаки
- [Бутстрап](../../../docs/bootstrap.md) — Запуск, деградация, восстановление
- [Платежи A2A](../../../docs/a2a-payment.md) — Расширение платежей TRM для протоколов агентов
- [Совместимость](../../../docs/compatibility.md) — Матрица функций против llama.cpp / Ollama / Bittensor

## Лицензия

MIT

## Благодарности

Распределенный инференс Tirami построен на [mesh-llm](https://github.com/michaelneale/mesh-llm) Майкла Нила. См. [CREDITS.md](../../../CREDITS.md).
