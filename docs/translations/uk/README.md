<div align="center">

# Tirami

**Обчислення — це валюта. Кожен ват створює інтелект, а не відходи.**

[![Crates.io](https://img.shields.io/crates/v/tirami-core?label=crates.io&color=e6522c)](https://crates.io/crates/tirami-core)
[![License: MIT](https://img.shields.io/badge/License-MIT-brightgreen.svg)](../../../LICENSE)

---

[English](../../../README.md) · [日本語](../ja/README.md) · [简体中文](../zh-CN/README.md) · [繁體中文](../zh-TW/README.md) · [Español](../es/README.md) · [Français](../fr/README.md) · [Русский](../ru/README.md) · **Українська** · [हिन्दी](../hi/README.md) · [العربية](../ar/README.md) · [فارسی](../fa/README.md) · [עברית](../he/README.md)

</div>

**Tirami — це децентралізований протокол інференсу, де обчислення є грошима.** Вузли заробляють TRM (Tirami Resource Merit) (TRM (Tirami Resource Merit), TRM), виконуючи корисний інференс LLM для інших. На відміну від Bitcoin, де електроенергія спалюється на безглузді хеші, кожен джоуль, витрачений на вузлі Tirami, створює реальний інтелект, який комусь справді потрібен.

Розподілений рушій інференсу побудований на [mesh-llm](https://github.com/michaelneale/mesh-llm) Майкла Ніла. Tirami додає зверху обчислювальну економіку: облік TRM, доказ корисної роботи (Proof of Useful Work), динамічне ціноутворення, бюджети автономних агентів та засоби безпеки. Дивіться [CREDITS.md](../../../CREDITS.md).

**Інтегрований форк:** [tirami-mesh](https://github.com/nm-arealnormalman/mesh-llm) — mesh-llm із вбудованим економічним шаром Tirami.

## Жива демо-версія

Це реальний вивід вузла Tirami, що працює. Кожен інференс коштує TRM. Кожна одиниця TRM заробляється корисною роботою.

```
$ tirami node -m "qwen2.5:0.5b" --ledger tirami-ledger.json
  Model loaded: Qwen2.5-0.5B (Metal-accelerated, 491MB)
  API server listening on 127.0.0.1:3000
```

**Перевірка балансу — кожен новий вузол отримує 1000 TRM безкоштовно:**
```
$ curl localhost:3000/v1/tirami/balance
{
  "effective_balance": 1000,
  "contributed": 0,
  "consumed": 0,
  "reputation": 0.5
}
```

**Задати питання — інференс коштує TRM:**
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

Кожна відповідь містить `x_tirami` — **вартість цього обчислення в TRM** та залишок балансу. Провайдер заробив 9 TRM. Споживач витратив 9 TRM. Фізика стоїть за кожною одиницею.

**Через три інференси — реальні операції в леджері:**
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

**Кожна операція має корінь Меркла — з можливістю фіксації в Bitcoin для незмінного доказу:**
```
$ curl localhost:3000/v1/tirami/network
{
  "total_trades": 3,
  "total_contributed_trm": 19,
  "merkle_root": "aac8db9f62dd9ff23926195a70ed8fcfc188fc867d9f2adabd8e694beb338748"
}
```

**ШІ-агенти вийшли з-під контролю? Аварійний вимикач заморожує все за мілісекунди:**
```
$ curl -X POST localhost:3000/v1/tirami/kill \
    -d '{"activate":true, "reason":"detected anomaly", "operator":"admin"}'
→ KILL SWITCH ACTIVATED
→ All TRM transactions frozen. No agent can spend.
```

**Контроль безпеки завжди активний:**
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

## Чому існує Tirami

```
Bitcoin:  електроенергія → безглуздий SHA-256 → BTC
Tirami:    електроенергія → корисний інференс LLM → TRM
```

Bitcoin довів: `електроенергія → обчислення → гроші`. Але обчислення Bitcoin не мають мети. Tirami перевертає це: кожна одиниця TRM представляє реальний інтелект, який вирішив чиюсь реальну проблему.

**Чотири речі, які не робить жоден інший проект:**

### 1. Обчислення = Валюта

Кожен інференс — це обмін. Провайдер заробляє TRM, споживач витрачає TRM. Без блокчейну, без токенів, без ICO. TRM підкріплений фізикою — електроенергією, спожитою для корисної роботи. На відміну від Bittensor (TAO), Akash (AKT) чи Golem (GLM), TRM не можна спекулювати — його заробляють виконанням корисних обчислень.

### 2. Захист від підробок без блокчейну

Кожна операція підписується обома сторонами (Ed25519) і синхронізується через gossip-протокол у мережі. Корінь Меркла всіх операцій може бути закріплений у Bitcoin для незмінного аудиту. Глобальний консенсус не потрібен — двостороннього криптографічного доказу достатньо.

### 3. ШІ-агенти самі керують своїми обчисленнями

Агент на телефоні позичає вільні потужності вночі → заробляє TRM → купує доступ до 70B моделі → стає розумнішим → заробляє більше. Агент автономно перевіряє `/v1/tirami/balance` та `/v1/tirami/pricing`. Політики бюджету та запобіжники запобігають неконтрольованим витратам.

```
Агент (1.5B на телефоні)
  → заробляє TRM вночі, обслуговуючи інференс
  → витрачає TRM на 70B модель → розумніші відповіді
  → кращі рішення → більше зароблених TRM
  → цикл повторюється → агент росте
```

### 4. Мікрофінансування обчислень

Вузли можуть позичати вільні TRM іншим вузлам під відсотки. Малий вузол позичає TRM, отримує доступ до більшої моделі, заробляє більше TRM і повертає позику з відсотками. Жоден інший проект розподіленого інференсу не пропонує кредитування обчислень. Це механізм, який робить цикл самовдосконалення економічно життєздатним для всіх, а не лише для тих, хто вже володіє потужним обладнанням.

## Архітектура

```
┌─────────────────────────────────────────────────┐
│  L4: Виявлення (tirami-agora) ✅ v0.1            │
│  Маркетплейс агентів, агрегація репутації,      │
│  Nostr NIP-90, розширення оплати Google A2A     │
├─────────────────────────────────────────────────┤
│  L3: Інтелект (tirami-mind) ✅ v0.1              │
│  Цикли самовдосконалення AutoAgent,             │
│  маркетплейс харнесів, мета-оптимізація         │
├─────────────────────────────────────────────────┤
│  L2: Фінанси (tirami-bank) ✅ v0.1               │
│  Стратегії, портфелі, ф'ючерси, страхування,   │
│  модель ризиків, оптимізатор дохідності         │
├─────────────────────────────────────────────────┤
│  L1: Економіка (tirami — цей репозиторій) ✅ Фаза 1-13 │
│  Леджер TRM, двосторонньо підписані операції,    │
│  динамічне ціноутворення, примітиви позик,      │
│  засоби безпеки                                 │
├─────────────────────────────────────────────────┤
│  L0: Інференс (tirami-mesh / mesh-llm) ✅        │
│  Паралелізм пайплайну, MoE шардинг,             │
│  iroh mesh, Nostr discovery, MLX/llama.cpp      │
└─────────────────────────────────────────────────┘

Усі 5 шарів існують. 785 тестів проходять по всій екосистемі.
```

## Швидкий старт

### Варіант 1: Демо від початку до кінця однією командою (Rust, ~30 секунд з нуля)

```bash
git clone https://github.com/clearclown/tirami && cd tirami
bash scripts/demo-e2e.sh
```

Скрипт завантажує SmolLM2-135M (~100 МБ) з HuggingFace, запускає справжній вузол Tirami з Metal/CUDA-прискоренням, виконує три реальних завершення чату, проходить усі ендпоінти Фаз 1-13 і виводить кольоровий звіт. Перевірено 2026-04-09 на Apple Silicon Metal GPU.

Після завершення той самий вузол також відповідає на:

```bash
# Сумісний з OpenAI клієнт
export OPENAI_BASE_URL=http://127.0.0.1:3001/v1
export OPENAI_API_KEY=$(cat ~/.tirami/api_token 2>/dev/null || echo "$TOKEN")

# Реальний потоковий вивід токен за токеном (Фаза 11)
curl -N $OPENAI_BASE_URL/chat/completions \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"model":"smollm2:135m","messages":[{"role":"user","content":"hi"}],"stream":true}'

# Економіка фаза 8 / репутація 9 / метрики 10 / якорення
curl $OPENAI_BASE_URL/tirami/balance -H "Authorization: Bearer $OPENAI_API_KEY"
curl $OPENAI_BASE_URL/tirami/anchor?network=mainnet -H "Authorization: Bearer $OPENAI_API_KEY"
curl http://127.0.0.1:3001/metrics  # Prometheus, без авторизації
```

Повна матриця функцій (проти llama.cpp / mesh-llm / Ollama / Bittensor / Akash) — у [`docs/compatibility.md`](../../../docs/compatibility.md).

### Варіант 2: Ручні команди Rust

**Передумови**: [Встановити Rust](https://rustup.rs/) (2 хвилини)

```bash
cargo build --release

# Запуск вузла — автоматично завантажує модель з HuggingFace
./target/release/tirami node -m "qwen2.5:0.5b" --ledger tirami-ledger.json

# Або будь-яка з таких команд:
./target/release/tirami chat -m "smollm2:135m" "Що таке гравітація?"
./target/release/tirami seed -m "qwen2.5:1.5b"               # заробляти TRM як P2P провайдер
./target/release/tirami worker --seed <public_key>           # витрачати TRM як P2P споживач
./target/release/tirami models                                # список каталогу (або HF URL)
```

**[Crates.io: tirami-core](https://crates.io/crates/tirami-core)** ·
**[Документ сумісності](../../../docs/compatibility.md)** ·
**[Демо-скрипт](../../../scripts/demo-e2e.sh)**

### Варіант 3: Готові бінарні файли / Docker

Готові бінарні файли та Docker-образ `clearclown/tirami:latest` відстежуються в
[releases](../../../releases). До цього Варіант 1 збирає з вихідного коду менш ніж за дві хвилини.

## API Довідка

### Інференс (сумісний з OpenAI)

| Ендпоінт | Опис |
|----------|-------------|
| `POST /v1/chat/completions` | Чат зі стрімінгом. Кожна відповідь містить `x_tirami.cu_cost` |
| `GET /v1/models` | Список завантажених моделей |

### Економіка

| Ендпоінт | Опис |
|----------|-------------|
| `GET /v1/tirami/balance` | Баланс TRM, репутація, історія внесків |
| `GET /v1/tirami/pricing` | Ринкова ціна (згладжена EMA), оцінка вартості |
| `GET /v1/tirami/trades` | Останні операції з сумами TRM |
| `GET /v1/tirami/network` | Загальний потік TRM + корінь Меркла |
| `GET /v1/tirami/providers` | Рейтинг провайдерів за репутацією та ціною |
| `POST /v1/tirami/invoice` | Створити Lightning інвойс із балансу TRM |
| `GET /v1/tirami/route` | Оптимальний вибір провайдера (вартість/якість/баланс) |
| `GET /settlement` | Звіт про розрахунки для експорту |

### Кредитування

| Ендпоінт | Опис |
|----------|-------------|
| `POST /v1/tirami/lend` | Запропонувати TRM до пулу кредитування |
| `POST /v1/tirami/borrow` | Запит на кредит у TRM |
| `POST /v1/tirami/repay` | Погасити непогашений кредит |
| `GET /v1/tirami/credit` | Кредитний рейтинг та історія |
| `GET /v1/tirami/pool` | Стан пулу кредитування |
| `GET /v1/tirami/loans` | Активні кредити |

### Безпека

| Ендпоінт | Опис |
|----------|-------------|
| `GET /v1/tirami/safety` | Стан аварійного стопу, запобіжники, політика бюджету |
| `POST /v1/tirami/kill` | Екстрена зупинка — заморозити всі транзакції TRM |
| `POST /v1/tirami/policy` | Встановити ліміти бюджету для агентів |

## Дизайн безпеки

ШІ-агенти, що автономно витрачають обчислення — це потужно, але небезпечно. Tirami має п'ять шарів безпеки:

| Шар | Механізм | Захист |
|-------|-----------|------------|
| **Kill Switch** | Оператор миттєво заморожує всі торги | Зупиняє агентів, що вийшли з-під контролю |
| **Budget Policy** | Ліміти на агента: за запит, погодинно, за весь час | Обмежує загальні ризики |
| **Circuit Breaker** | Автоматично спрацьовує при 5 помилках або 30+ витратах/хв | Виявляє аномалії |
| **Velocity Detection** | Ковзне вікно в 1 хвилину для швидкості витрат | Запобігає сплескам |
| **Human Approval** | Транзакції вище порогу потребують згоди людини | Охороняє великі витрати |

Принцип дизайну: **fail-safe**. Якщо будь-яка перевірка не може визначити безпеку, вона **відхиляє** дію.

## Ідея

| Епоха | Стандарт | Забезпечення |
|-----|----------|---------|
| Античність | Золото | Геологічна рідкість |
| 1944–1971 | Бреттон-Вудс | USD, прив'язаний до золота |
| 1971–сьогодні | Петродолар | Попит на нафту + військова міць |
| 2009–сьогодні | Bitcoin | Енергія на SHA-256 (марна робота) |
| **Зараз** | **Обчислювальний стандарт** | **Енергія на інференс LLM (корисна робота)** |

Кімната, повна Mac Mini з Tirami — це багатоквартирний будинок, який приносить дохід, виконуючи корисну роботу, поки власник спить.

## Структура проекту

```
tirami/  (цей репозиторій — Шар 1)
├── crates/
│   ├── tirami-ledger/      # Облік TRM, кредитування, agora (NIP-90), безпека
│   ├── tirami-node/        # Демон вузла, HTTP API (кредитування + роутинг), пайплайн
│   ├── tirami-cli/         # CLI: chat, seed, worker, settle, wallet
│   ├── tirami-lightning/   # Міст TRM ↔ Bitcoin Lightning (двонаправлений)
│   ├── tirami-net/         # P2P: iroh QUIC + Noise + gossip (операції + позики)
│   ├── tirami-proto/       # Протокол передачі: 27+ типів повідомлень, вкл. Loan*
│   ├── tirami-infer/       # Інференс: llama.cpp, GGUF, Metal/CPU
│   ├── tirami-core/        # Типи: NodeId, TRM, Config
│   └── tirami-shard/       # Топологія: призначення шарів
├── scripts/verify-impl.sh         # TDD регресійний тест (24 твердження)
└── docs/                  # Специфікації, стратегія, модель загроз, роадмап
```

~20,000 рядків Rust. **785 тести проходять.** Фази 1-13 завершено.

## Супутні репозиторії (повна екосистема)

| Репозиторій | Шар | Тести | Статус |
|------|-------|-------|--------|
| [clearclown/tirami](https://github.com/clearclown/tirami) (цей) | L1 Економіка | 785 | Фаза 1-13 ✅ |
| [clearclown/tirami-bank](https://github.com/clearclown/tirami-bank) | L2 Фінанси | — | archived |
| [clearclown/tirami-mind](https://github.com/clearclown/tirami-mind) | L3 Інтелект | — | archived |
| [clearclown/tirami-agora](https://github.com/clearclown/tirami-agora) | L4 Виявлення | — | archived |
| [clearclown/tirami-economics](https://github.com/clearclown/tirami-economics) | Теорія | 16/16 GREEN | ✅ |
| [nm-arealnormalman/mesh-llm](https://github.com/nm-arealnormalman/mesh-llm) | L0 Інференс | 43 (tirami-economy) | ✅ |

## Документація

- [Стратегія](../../../docs/strategy.md) — Конкурентне позиціонування, специфікація кредитування, 5-шарова архітектура
- [Монетарна теорія](../../../docs/monetary-theory.md) — Чому працює TRM: Содді, Bitcoin, PoUW, валюта лише для ШІ
- [Концепція та візія](../../../docs/concept.md) — Чому обчислення — це гроші
- [Економічна модель](../../../docs/economy.md) — Економіка TRM, доказ корисної роботи
- [Архітектура](../../../docs/architecture.md) — Двошаровий дизайн
- [Інтеграція агентів](../../../docs/agent-integration.md) — SDK, MCP, процес кредитування
- [Протокол передачі](../../../docs/protocol-spec.md) — 17 типів повідомлень
- [Роадмап](../../../docs/roadmap.md) — Фази розробки
- [Модель загроз](../../../docs/threat-model.md) — Безпека + економічні атаки
- [Бутстрап](../../../docs/bootstrap.md) — Запуск, деградація, відновлення
- [Платежі A2A](../../../docs/a2a-payment.md) — Розширення платежів TRM для протоколів агентів
- [Сумісність](../../../docs/compatibility.md) — Матриця функцій проти llama.cpp / Ollama / Bittensor

## Ліцензія

MIT

## Подяки

Розподілений інференс Tirami побудований на [mesh-llm](https://github.com/michaelneale/mesh-llm) Майкла Ніла. Дивіться [CREDITS.md](../../../CREDITS.md).
