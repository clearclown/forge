<div align="center">

# Forge

**Обчислення — це валюта. Кожен ват створює інтелект, а не відходи.**

[![PyPI: forge-sdk](https://img.shields.io/pypi/v/forge-sdk?label=forge-sdk&color=3775A9)](https://pypi.org/project/forge-sdk/)
[![PyPI: forge-cu-mcp](https://img.shields.io/pypi/v/forge-cu-mcp?label=forge-cu-mcp&color=3775A9)](https://pypi.org/project/forge-cu-mcp/)
[![Crates.io](https://img.shields.io/crates/v/forge?label=crates.io&color=e6522c)](https://crates.io/crates/forge)
[![License: MIT](https://img.shields.io/badge/License-MIT-brightgreen.svg)](../../../LICENSE)

---

[English](../../../README.md) · [日本語](../ja/README.md) · [简体中文](../zh-CN/README.md) · [繁體中文](../zh-TW/README.md) · [Español](../es/README.md) · [Français](../fr/README.md) · [Русский](../ru/README.md) · **Українська** · [हिन्दी](../hi/README.md) · [العربية](../ar/README.md) · [فارسی](../fa/README.md) · [עברית](../he/README.md)

</div>

**Forge — це децентралізований протокол інференсу, де обчислення є грошима.** Вузли заробляють обчислювальні одиниці (Compute Units, CU), виконуючи корисний інференс LLM для інших. На відміну від Bitcoin, де електроенергія спалюється на безглузді хеші, кожен джоуль, витрачений на вузлі Forge, створює реальний інтелект, який комусь справді потрібен.

Розподілений рушій інференсу побудований на [mesh-llm](https://github.com/michaelneale/mesh-llm) Майкла Ніла. Forge додає зверху обчислювальну економіку: облік CU, доказ корисної роботи (Proof of Useful Work), динамічне ціноутворення, бюджети автономних агентів та засоби безпеки. Дивіться [CREDITS.md](../../../CREDITS.md).

**Інтегрований форк:** [forge-mesh](https://github.com/nm-arealnormalman/mesh-llm) — mesh-llm із вбудованим економічним шаром Forge.

## Жива демо-версія

Це реальний вивід вузла Forge, що працює. Кожен інференс коштує CU. Кожна одиниця CU заробляється корисною роботою.

```
$ forge node -m "qwen2.5:0.5b" --ledger forge-ledger.json
  Model loaded: Qwen2.5-0.5B (Metal-accelerated, 491MB)
  API server listening on 127.0.0.1:3000
```

**Перевірка балансу — кожен новий вузол отримує 1000 CU безкоштовно:**
```
$ curl localhost:3000/v1/forge/balance
{
  "effective_balance": 1000,
  "contributed": 0,
  "consumed": 0,
  "reputation": 0.5
}
```

**Задати питання — інференс коштує CU:**
```
$ curl localhost:3000/v1/chat/completions \
    -d '{"messages":[{"role":"user","content":"Say hello in Japanese"}]}'
{
  "choices": [{"message": {"content": "こんにちは！ (konnichiwa!)"}}],
  "usage": {"completion_tokens": 9},
  "x_forge": {
    "cu_cost": 9,
    "effective_balance": 1009
  }
}
```

Кожна відповідь містить `x_forge` — **вартість цього обчислення в CU** та залишок балансу. Провайдер заробив 9 CU. Споживач витратив 9 CU. Фізика стоїть за кожною одиницею.

**Через три інференси — реальні операції в леджері:**
```
$ curl localhost:3000/v1/forge/trades
{
  "count": 3,
  "trades": [
    {"cu_amount": 5, "tokens_processed": 5, "model_id": "qwen2.5-0.5b-instruct-q4_k_m"},
    {"cu_amount": 5, "tokens_processed": 5, "model_id": "qwen2.5-0.5b-instruct-q4_k_m"},
    {"cu_amount": 9, "tokens_processed": 9, "model_id": "qwen2.5-0.5b-instruct-q4_k_m"}
  ]
}
```

**Кожна операція має корінь Меркла — з можливістю фіксації в Bitcoin для незмінного доказу:**
```
$ curl localhost:3000/v1/forge/network
{
  "total_trades": 3,
  "total_contributed_cu": 19,
  "merkle_root": "aac8db9f62dd9ff23926195a70ed8fcfc188fc867d9f2adabd8e694beb338748"
}
```

**ШІ-агенти вийшли з-під контролю? Аварійний вимикач заморожує все за мілісекунди:**
```
$ curl -X POST localhost:3000/v1/forge/kill \
    -d '{"activate":true, "reason":"detected anomaly", "operator":"admin"}'
→ KILL SWITCH ACTIVATED
→ All CU transactions frozen. No agent can spend.
```

**Контроль безпеки завжди активний:**
```
$ curl localhost:3000/v1/forge/safety
{
  "kill_switch_active": false,
  "circuit_tripped": false,
  "policy": {
    "max_cu_per_hour": 10000,
    "max_cu_per_request": 1000,
    "max_cu_lifetime": 1000000,
    "human_approval_threshold": 5000
  }
}
```

## Чому існує Forge

```
Bitcoin:  електроенергія → безглуздий SHA-256 → BTC
Forge:    електроенергія → корисний інференс LLM → CU
```

Bitcoin довів: `електроенергія → обчислення → гроші`. Але обчислення Bitcoin не мають мети. Forge перевертає це: кожна одиниця CU представляє реальний інтелект, який вирішив чиюсь реальну проблему.

**Чотири речі, які не робить жоден інший проект:**

### 1. Обчислення = Валюта

Кожен інференс — це обмін. Провайдер заробляє CU, споживач витрачає CU. Без блокчейну, без токенів, без ICO. CU підкріплений фізикою — електроенергією, спожитою для корисної роботи. На відміну від Bittensor (TAO), Akash (AKT) чи Golem (GLM), CU не можна спекулювати — його заробляють виконанням корисних обчислень.

### 2. Захист від підробок без блокчейну

Кожна операція підписується обома сторонами (Ed25519) і синхронізується через gossip-протокол у мережі. Корінь Меркла всіх операцій може бути закріплений у Bitcoin для незмінного аудиту. Глобальний консенсус не потрібен — двостороннього криптографічного доказу достатньо.

### 3. ШІ-агенти самі керують своїми обчисленнями

Агент на телефоні позичає вільні потужності вночі → заробляє CU → купує доступ до 70B моделі → стає розумнішим → заробляє більше. Агент автономно перевіряє `/v1/forge/balance` та `/v1/forge/pricing`. Політики бюджету та запобіжники запобігають неконтрольованим витратам.

```
Агент (1.5B на телефоні)
  → заробляє CU вночі, обслуговуючи інференс
  → витрачає CU на 70B модель → розумніші відповіді
  → кращі рішення → більше зароблених CU
  → цикл повторюється → агент росте
```

### 4. Мікрофінансування обчислень

Вузли можуть позичати вільні CU іншим вузлам під відсотки. Малий вузол позичає CU, отримує доступ до більшої моделі, заробляє більше CU і повертає позику з відсотками. Жоден інший проект розподіленого інференсу не пропонує кредитування обчислень. Це механізм, який робить цикл самовдосконалення економічно життєздатним для всіх, а не лише для тих, хто вже володіє потужним обладнанням.

## Архітектура

```
┌─────────────────────────────────────────────────┐
│  L4: Виявлення (forge-agora) ✅ v0.1            │
│  Маркетплейс агентів, агрегація репутації,      │
│  Nostr NIP-90, розширення оплати Google A2A     │
├─────────────────────────────────────────────────┤
│  L3: Інтелект (forge-mind) ✅ v0.1              │
│  Цикли самовдосконалення AutoAgent,             │
│  маркетплейс харнесів, мета-оптимізація         │
├─────────────────────────────────────────────────┤
│  L2: Фінанси (forge-bank) ✅ v0.1               │
│  Стратегії, портфелі, ф'ючерси, страхування,   │
│  модель ризиків, оптимізатор дохідності         │
├─────────────────────────────────────────────────┤
│  L1: Економіка (forge — цей репозиторій) ✅ Фаза 1-6 │
│  Леджер CU, двосторонньо підписані операції,    │
│  динамічне ціноутворення, примітиви позик,      │
│  засоби безпеки                                 │
├─────────────────────────────────────────────────┤
│  L0: Інференс (forge-mesh / mesh-llm) ✅        │
│  Паралелізм пайплайну, MoE шардинг,             │
│  iroh mesh, Nostr discovery, MLX/llama.cpp      │
└─────────────────────────────────────────────────┘

Усі 5 шарів існують. 326 тестів проходять по всій екосистемі.
```

## Швидкий старт

### Варіант 1: Демо від початку до кінця однією командою (Rust, ~30 секунд з нуля)

```bash
git clone https://github.com/clearclown/forge && cd forge
bash scripts/demo-e2e.sh
```

Скрипт завантажує SmolLM2-135M (~100 МБ) з HuggingFace, запускає справжній вузол Forge з Metal/CUDA-прискоренням, виконує три реальних завершення чату, проходить усі ендпоінти Фаз 1-12 і виводить кольоровий звіт. Перевірено 2026-04-09 на Apple Silicon Metal GPU.

Після завершення той самий вузол також відповідає на:

```bash
# Сумісний з OpenAI клієнт
export OPENAI_BASE_URL=http://127.0.0.1:3001/v1
export OPENAI_API_KEY=$(cat ~/.forge/api_token 2>/dev/null || echo "$TOKEN")

# Реальний потоковий вивід токен за токеном (Фаза 11)
curl -N $OPENAI_BASE_URL/chat/completions \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"model":"smollm2:135m","messages":[{"role":"user","content":"hi"}],"stream":true}'

# Економіка фаза 8 / репутація 9 / метрики 10 / якорення
curl $OPENAI_BASE_URL/forge/balance -H "Authorization: Bearer $OPENAI_API_KEY"
curl $OPENAI_BASE_URL/forge/anchor?network=mainnet -H "Authorization: Bearer $OPENAI_API_KEY"
curl http://127.0.0.1:3001/metrics  # Prometheus, без авторизації
```

Повна матриця функцій (проти llama.cpp / mesh-llm / Ollama / Bittensor / Akash) — у [`docs/compatibility.md`](../../../docs/compatibility.md).

### Варіант 2: Python (керує всім через SDK + MCP)

```bash
pip install forge-sdk forge-cu-mcp

python -c "
from forge_sdk import ForgeClient
c = ForgeClient(base_url='http://localhost:3001')
print('balance:', c.balance())
print('decision:', c.bank_tick())
"
```

[PyPI: forge-sdk](https://pypi.org/project/forge-sdk/) (20 методів L2/L3/L4) ·
[PyPI: forge-cu-mcp](https://pypi.org/project/forge-cu-mcp/) (20 MCP-інструментів для Claude Code / Cursor)

### Варіант 3: Ручні команди Rust

**Передумови**: [Встановити Rust](https://rustup.rs/) (2 хвилини)

```bash
cargo build --release

# Запуск вузла — автоматично завантажує модель з HuggingFace
./target/release/forge node -m "qwen2.5:0.5b" --ledger forge-ledger.json

# Або будь-яка з таких команд:
./target/release/forge chat -m "smollm2:135m" "Що таке гравітація?"
./target/release/forge seed -m "qwen2.5:1.5b"               # заробляти CU як P2P провайдер
./target/release/forge worker --seed <public_key>           # витрачати CU як P2P споживач
./target/release/forge models                                # список каталогу (або HF URL)
```

**[Crates.io: forge](https://crates.io/crates/forge)** ·
**[Документ сумісності](../../../docs/compatibility.md)** ·
**[Демо-скрипт](../../../scripts/demo-e2e.sh)**

### Варіант 4: Готові бінарні файли / Docker

Готові бінарні файли та Docker-образ `clearclown/forge:latest` відстежуються в
[releases](../../../releases). До цього Варіант 1 збирає з вихідного коду менш ніж за дві хвилини.

## API Довідка

### Інференс (сумісний з OpenAI)

| Ендпоінт | Опис |
|----------|-------------|
| `POST /v1/chat/completions` | Чат зі стрімінгом. Кожна відповідь містить `x_forge.cu_cost` |
| `GET /v1/models` | Список завантажених моделей |

### Економіка

| Ендпоінт | Опис |
|----------|-------------|
| `GET /v1/forge/balance` | Баланс CU, репутація, історія внесків |
| `GET /v1/forge/pricing` | Ринкова ціна (згладжена EMA), оцінка вартості |
| `GET /v1/forge/trades` | Останні операції з сумами CU |
| `GET /v1/forge/network` | Загальний потік CU + корінь Меркла |
| `GET /v1/forge/providers` | Рейтинг провайдерів за репутацією та ціною |
| `POST /v1/forge/invoice` | Створити Lightning інвойс із балансу CU |
| `GET /v1/forge/route` | Оптимальний вибір провайдера (вартість/якість/баланс) |
| `GET /settlement` | Звіт про розрахунки для експорту |

### Кредитування

| Ендпоінт | Опис |
|----------|-------------|
| `POST /v1/forge/lend` | Запропонувати CU до пулу кредитування |
| `POST /v1/forge/borrow` | Запит на кредит у CU |
| `POST /v1/forge/repay` | Погасити непогашений кредит |
| `GET /v1/forge/credit` | Кредитний рейтинг та історія |
| `GET /v1/forge/pool` | Стан пулу кредитування |
| `GET /v1/forge/loans` | Активні кредити |

### Безпека

| Ендпоінт | Опис |
|----------|-------------|
| `GET /v1/forge/safety` | Стан аварійного стопу, запобіжники, політика бюджету |
| `POST /v1/forge/kill` | Екстрена зупинка — заморозити всі транзакції CU |
| `POST /v1/forge/policy` | Встановити ліміти бюджету для агентів |

## Дизайн безпеки

ШІ-агенти, що автономно витрачають обчислення — це потужно, але небезпечно. Forge має п'ять шарів безпеки:

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

Кімната, повна Mac Mini з Forge — це багатоквартирний будинок, який приносить дохід, виконуючи корисну роботу, поки власник спить.

## Структура проекту

```
forge/  (цей репозиторій — Шар 1)
├── crates/
│   ├── forge-ledger/      # Облік CU, кредитування, agora (NIP-90), безпека
│   ├── forge-node/        # Демон вузла, HTTP API (кредитування + роутинг), пайплайн
│   ├── forge-cli/         # CLI: chat, seed, worker, settle, wallet
│   ├── forge-lightning/   # Міст CU ↔ Bitcoin Lightning (двонаправлений)
│   ├── forge-net/         # P2P: iroh QUIC + Noise + gossip (операції + позики)
│   ├── forge-proto/       # Протокол передачі: 27+ типів повідомлень, вкл. Loan*
│   ├── forge-infer/       # Інференс: llama.cpp, GGUF, Metal/CPU
│   ├── forge-core/        # Типи: NodeId, CU, Config
│   └── forge-shard/       # Топологія: призначення шарів
├── sdk/python/forge_sdk.py        # Python-клієнт з повним API кредитування
├── mcp/forge-mcp-server.py        # MCP-сервер (інструменти кредитування для Claude/etc.)
├── scripts/verify-impl.sh         # TDD регресійний тест (24 твердження)
└── docs/                  # Специфікації, стратегія, модель загроз, роадмап
```

~14 500 рядків Rust. **143 тести проходять.** Фази 1-6 завершено.

## Супутні репозиторії (повна екосистема)

| Репозиторій | Шар | Тести | Статус |
|------|-------|-------|--------|
| [clearclown/forge](https://github.com/clearclown/forge) (цей) | L1 Економіка | 143 | Фаза 1-6 ✅ |
| [clearclown/forge-bank](https://github.com/clearclown/forge-bank) | L2 Фінанси | 45 | v0.1 ✅ |
| [clearclown/forge-mind](https://github.com/clearclown/forge-mind) | L3 Інтелект | 40 | v0.1 ✅ |
| [clearclown/forge-agora](https://github.com/clearclown/forge-agora) | L4 Виявлення | 39 | v0.1 ✅ |
| [clearclown/forge-economics](https://github.com/clearclown/forge-economics) | Теорія | 16/16 GREEN | ✅ |
| [nm-arealnormalman/mesh-llm](https://github.com/nm-arealnormalman/mesh-llm) | L0 Інференс | 43 (forge-economy) | ✅ |

## Документація

- [Стратегія](../../../docs/strategy.md) — Конкурентне позиціонування, специфікація кредитування, 5-шарова архітектура
- [Монетарна теорія](../../../docs/monetary-theory.md) — Чому працює CU: Содді, Bitcoin, PoUW, валюта лише для ШІ
- [Концепція та візія](../../../docs/concept.md) — Чому обчислення — це гроші
- [Економічна модель](../../../docs/economy.md) — Економіка CU, доказ корисної роботи
- [Архітектура](../../../docs/architecture.md) — Двошаровий дизайн
- [Інтеграція агентів](../../../docs/agent-integration.md) — SDK, MCP, процес кредитування
- [Протокол передачі](../../../docs/protocol-spec.md) — 17 типів повідомлень
- [Роадмап](../../../docs/roadmap.md) — Фази розробки
- [Модель загроз](../../../docs/threat-model.md) — Безпека + економічні атаки
- [Бутстрап](../../../docs/bootstrap.md) — Запуск, деградація, відновлення
- [Платежі A2A](../../../docs/a2a-payment.md) — Розширення платежів CU для протоколів агентів
- [Сумісність](../../../docs/compatibility.md) — Матриця функцій проти llama.cpp / Ollama / Bittensor

## Ліцензія

MIT

## Подяки

Розподілений інференс Forge побудований на [mesh-llm](https://github.com/michaelneale/mesh-llm) Майкла Ніла. Дивіться [CREDITS.md](../../../CREDITS.md).
