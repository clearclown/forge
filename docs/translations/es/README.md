<div align="center">

# Tirami

**El cómputo es moneda. Cada vatio produce inteligencia, no desperdicio.**

[![Crates.io](https://img.shields.io/crates/v/tirami-core?label=crates.io&color=e6522c)](https://crates.io/crates/tirami-core)
[![License: MIT](https://img.shields.io/badge/License-MIT-brightgreen.svg)](../../../LICENSE)

---

[English](../../../README.md) · [日本語](../ja/README.md) · [简体中文](../zh-CN/README.md) · [繁體中文](../zh-TW/README.md) · **Español** · [Français](../fr/README.md) · [Русский](../ru/README.md) · [Українська](../uk/README.md) · [हिन्दी](../hi/README.md) · [العربية](../ar/README.md) · [فارسی](../fa/README.md) · [עברית](../he/README.md)

</div>

**Tirami es un protocolo de inferencia distribuida donde el cómputo es dinero.** Los nodos ganan TRM (Tirami Resource Merit) (TRM) al realizar inferencias útiles de LLM para otros. A diferencia de Bitcoin — donde la electricidad se quema en hashes sin sentido — cada julio gastado en un nodo de Tirami produce inteligencia real que alguien realmente necesita.

El motor de inferencia distribuida está construido sobre [mesh-llm](https://github.com/michaelneale/mesh-llm) por Michael Neale. Tirami añade una economía de cómputo por encima: contabilidad de TRM, Prueba de Trabajo Útil, precios dinámicos, presupuestos de agentes autónomos y controles de seguridad. Ver [CREDITS.md](../../../CREDITS.md).

**Fork integrado:** [tirami-mesh](https://github.com/nm-arealnormalman/mesh-llm) — mesh-llm con la capa económica de Tirami incorporada.

## Demo en Vivo

Esta es una salida real de un nodo de Tirami en ejecución. Cada inferencia cuesta TRM. Cada TRM se gana mediante computación útil.

```
$ tirami node -m "qwen2.5:0.5b" --ledger tirami-ledger.json
  Modelo cargado: Qwen2.5-0.5B (acelerado por Metal, 491MB)
  Servidor API escuchando en 127.0.0.1:3000
```

**Verificar saldo — cada nuevo nodo recibe 1,000 TRM de nivel gratuito:**
```
$ curl localhost:3000/v1/tirami/balance
{
  "effective_balance": 1000,
  "contributed": 0,
  "consumed": 0,
  "reputation": 0.5
}
```

**Hacer una pregunta — la inferencia cuesta TRM:**
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

Cada respuesta incluye `x_tirami` — **el costo de esa computación en TRM** y el saldo restante. El proveedor ganó 9 TRM. El consumidor gastó 9 TRM. La física respaldó cada unidad.

**Tres inferencias después — transacciones reales en el libro contable:**
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

**Cada transacción tiene una raíz de Merkle — anclable a Bitcoin para una prueba inmutable:**
```
$ curl localhost:3000/v1/tirami/network
{
  "total_trades": 3,
  "total_contributed_trm": 19,
  "merkle_root": "aac8db9f62dd9ff23926195a70ed8fcfc188fc867d9f2adabd8e694beb338748"
}
```

**¿Agentes de IA fuera de control? El interruptor de apagado congela todo en milisegundos:**
```
$ curl -X POST localhost:3000/v1/tirami/kill \
    -d '{"activate":true, "reason":"anomalía detectada", "operator":"admin"}'
→ KILL SWITCH ACTIVATED
→ All TRM transactions frozen. No agent can spend.
```

**Controles de seguridad siempre activos:**
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

## Por qué existe Tirami

```
Bitcoin:  electricidad  →  SHA-256 sin sentido  →  BTC
Tirami:    electricidad  →  inferencia útil de LLM →  TRM
```

Bitcoin demostró que `electricidad → computación → dinero`. Pero la computación de Bitcoin no tiene propósito. Tirami lo invierte: cada TRM representa inteligencia real que resolvió el problema real de alguien.

**Cuatro cosas que ningún otro proyecto hace:**

### 1. Cómputo = Moneda

Cada inferencia es una transacción. El proveedor gana TRM, el consumidor gasta TRM. Sin blockchain, sin tokens, sin ICO. El TRM está respaldado por la física — la electricidad consumida para un trabajo útil. A diferencia de Bittensor (TAO), Akash (AKT) o Golem (GLM), el TRM no puede ser especulado — se gana realizando computación útil.

### 2. A prueba de manipulaciones sin una Blockchain

Cada transacción está firmada doblemente (Ed25519) por ambas partes y sincronizada por chismes (gossip) a través de la red. Una raíz de Merkle de todas las transacciones puede anclarse a Bitcoin para una auditoría inmutable. No se necesita un consenso global — la prueba criptográfica bilateral es suficiente.

### 3. Los agentes de IA gestionan su propio cómputo

Un agente en un teléfono presta cómputo inactivo durante la noche → gana TRM → compra acceso a un modelo de 70B → se vuelve más inteligente → gana más. El agente consulta `/v1/tirami/balance` y `/v1/tirami/pricing` de forma autónoma. Las políticas presupuestarias y los disyuntores evitan el gasto descontrolado.

```
Agente (1.5B en el teléfono)
  → gana TRM durante la noche sirviendo inferencias
  → gasta TRM en un modelo de 70B → respuestas más inteligentes
  → mejores decisiones → más TRM ganado
  → el ciclo se repite → el agente crece
```

### 4. Microfinanzas de Cómputo

Los nodos pueden prestar TRM inactivos a otros nodos con interés. Un nodo pequeño pide TRM prestados, accede a un modelo más grande, gana más TRM y devuelve el préstamo con interés. Ningún otro proyecto de inferencia distribuida ofrece préstamos de cómputo. Este es el motor que hace que el ciclo de auto-mejora sea económicamente viable para todos, no solo para quienes ya poseen hardware potente.

## Arquitectura

```
┌─────────────────────────────────────────────────┐
│  L4: Descubrimiento (tirami-agora) ✅ v0.1       │
│  Mercado de agentes, agregación de reputación,  │
│  Nostr NIP-90, extensión de pago Google A2A     │
├─────────────────────────────────────────────────┤
│  L3: Inteligencia (tirami-mind) ✅ v0.1          │
│  Ciclos de auto-mejora de AutoAgent,            │
│  mercado de arneses, meta-optimización          │
├─────────────────────────────────────────────────┤
│  L2: Finanzas (tirami-bank) ✅ v0.1              │
│  Estrategias, carteras, futuros, seguros,       │
│  modelo de riesgo, optimizador de rendimiento   │
├─────────────────────────────────────────────────┤
│  L1: Economía (tirami — este repo) ✅ Fase 1-13   │
│  Libro contable TRM, operaciones doble-firmadas, │
│  precios dinámicos, primitivas de préstamo,     │
│  controles de seguridad                         │
├─────────────────────────────────────────────────┤
│  L0: Inferencia (tirami-mesh / mesh-llm) ✅      │
│  Paralelismo de pipeline, sharding MoE,         │
│  malla iroh, descubrimiento Nostr, MLX/llama.cpp│
└─────────────────────────────────────────────────┘

Las 5 capas existen. 785 pruebas pasando en todo el ecosistema.
```

## Inicio Rápido

### Opción 1: Demo de extremo a extremo con un solo comando (Rust, ~30 segundos en frío)

```bash
git clone https://github.com/clearclown/tirami && cd tirami
bash scripts/demo-e2e.sh
```

Este script descarga SmolLM2-135M (~100 MB) de HuggingFace, inicia un nodo Tirami real con aceleración Metal/CUDA, ejecuta tres completaciones de chat reales, recorre todos los endpoints de la Fase 1-13 e imprime un resumen con colores. Verificado el 2026-04-09 en Apple Silicon Metal GPU.

Al terminar, el mismo nodo también responde a:

```bash
# Cliente OpenAI compatible
export OPENAI_BASE_URL=http://127.0.0.1:3001/v1
export OPENAI_API_KEY=$(cat ~/.tirami/api_token 2>/dev/null || echo "$TOKEN")

# Streaming real token a token (Fase 11)
curl -N $OPENAI_BASE_URL/chat/completions \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"model":"smollm2:135m","messages":[{"role":"user","content":"hi"}],"stream":true}'

# Fase 8 economía / 9 reputación / 10 métricas / anclaje
curl $OPENAI_BASE_URL/tirami/balance -H "Authorization: Bearer $OPENAI_API_KEY"
curl $OPENAI_BASE_URL/tirami/anchor?network=mainnet -H "Authorization: Bearer $OPENAI_API_KEY"
curl http://127.0.0.1:3001/metrics  # Prometheus, sin autenticación
```

Ver [`docs/compatibility.md`](../../../docs/compatibility.md) para la matriz completa de funciones frente a llama.cpp / mesh-llm / Ollama / Bittensor / Akash.

### Opción 2: Comandos Rust manuales

**Prerrequisitos**: [Instalar Rust](https://rustup.rs/) (2 minutos)

```bash
cargo build --release

# Ejecutar un nodo — descarga el modelo automáticamente desde HuggingFace
./target/release/tirami node -m "qwen2.5:0.5b" --ledger tirami-ledger.json

# O cualquiera de estos:
./target/release/tirami chat -m "smollm2:135m" "¿Qué es la gravedad?"
./target/release/tirami seed -m "qwen2.5:1.5b"               # ganar TRM como proveedor P2P
./target/release/tirami worker --seed <public_key>           # gastar TRM como consumidor P2P
./target/release/tirami models                                # listar catálogo (o usar URLs de HF)
```

**[Crates.io: tirami-core](https://crates.io/crates/tirami-core)** ·
**[Documento de compatibilidad](../../../docs/compatibility.md)** ·
**[Script de demo](../../../scripts/demo-e2e.sh)**

### Opción 3: Binarios precompilados / Docker

Los binarios precompilados y la imagen Docker `clearclown/tirami:latest` se rastrean en
[releases](../../../releases). Mientras tanto, la Opción 1 compila desde el código fuente en menos de dos minutos.

## Referencia de la API

### Inferencia (compatible con OpenAI)

| Endpoint | Descripción |
|----------|-------------|
| `POST /v1/chat/completions` | Chat con streaming. Cada respuesta incluye `x_tirami.cu_cost` |
| `GET /v1/models` | Listar modelos cargados |

### Economía

| Endpoint | Descripción |
|----------|-------------|
| `GET /v1/tirami/balance` | Saldo de TRM, reputación, historial de contribuciones |
| `GET /v1/tirami/pricing` | Precio de mercado (suavizado EMA), estimaciones de costos |
| `GET /v1/tirami/trades` | Transacciones recientes con montos de TRM |
| `GET /v1/tirami/network` | Flujo total de TRM + raíz de Merkle |
| `GET /v1/tirami/providers` | Proveedores clasificados por reputación y costo |
| `POST /v1/tirami/invoice` | Crear factura de Lightning desde el saldo de TRM |
| `GET /v1/tirami/route` | Selección óptima de proveedor (costo/calidad/equilibrado) |
| `GET /settlement` | Estado de liquidación exportable |

### Préstamos

| Endpoint | Descripción |
|----------|-------------|
| `POST /v1/tirami/lend` | Ofrecer TRM al pool de préstamos |
| `POST /v1/tirami/borrow` | Solicitar un préstamo en TRM |
| `POST /v1/tirami/repay` | Reembolsar un préstamo pendiente |
| `GET /v1/tirami/credit` | Puntuación de crédito e historial |
| `GET /v1/tirami/pool` | Estado del pool de préstamos |
| `GET /v1/tirami/loans` | Préstamos activos |

### Seguridad

| Endpoint | Descripción |
|----------|-------------|
| `GET /v1/tirami/safety` | Estado del kill switch, disyuntor, política presupuestaria |
| `POST /v1/tirami/kill` | Parada de emergencia — congelar todas las transacciones de TRM |
| `POST /v1/tirami/policy` | Establecer límites presupuestarios por agente |

## Diseño de Seguridad

Los agentes de IA que gastan cómputo de forma autónoma son poderosos pero peligrosos. Tirami tiene cinco capas de seguridad:

| Capa | Mecanismo | Protección |
|-------|-----------|------------|
| **Kill Switch** | El operador humano congela todas las transacciones instantáneamente | Detiene agentes descontrolados |
| **Política de Presupuesto** | Límites por agente: por solicitud, por hora, de por vida | Limita la exposición total |
| **Disyuntor** | Se activa automáticamente tras 5 errores o más de 30 gastos/min | Captura anomalías |
| **Detección de Velocidad** | Ventana deslizante de 1 minuto sobre la tasa de gasto | Previene ráfagas |
| **Aprobación Humana** | Transacciones por encima del umbral requieren el visto bueno humano | Protege grandes gastos |

Principio de diseño: **a prueba de fallos (fail-safe)**. Si cualquier verificación no puede determinar la seguridad, **deniega** la acción.

## La Idea

| Era | Estándar | Respaldo |
|-----|----------|---------|
| Antigua | Oro | Escasez geológica |
| 1944–1971 | Bretton Woods | USD vinculado al oro |
| 1971–presente | Petrodólar | Demanda de petróleo + poder militar |
| 2009–presente | Bitcoin | Energía en SHA-256 (trabajo inútil) |
| **Ahora** | **Estándar de Cómputo** | **Energía en inferencia de LLM (trabajo útil)** |

Una habitación llena de Mac Minis ejecutando Tirami es un edificio de apartamentos — generando rendimiento al realizar un trabajo útil mientras el propietario duerme.

## Estructura del Proyecto

```
tirami/  (este repo — Capa 1)
├── crates/
│   ├── tirami-ledger/      # Contabilidad TRM, préstamos, agora (NIP-90), seguridad
│   ├── tirami-node/        # Demonio del nodo, API HTTP (préstamos + enrutamiento), pipeline
│   ├── tirami-cli/         # CLI: chat, seed, worker, liquidar, billetera
│   ├── tirami-lightning/   # Puente TRM ↔ Bitcoin Lightning (bidireccional)
│   ├── tirami-net/         # P2P: iroh QUIC + Noise + gossip (transacciones + préstamos)
│   ├── tirami-proto/       # Protocolo de cable: 27+ tipos de mensajes incl. Loan*
│   ├── tirami-infer/       # Inferencia: llama.cpp, GGUF, Metal/CPU
│   ├── tirami-core/        # Tipos: NodeId, TRM, Config
│   └── tirami-shard/       # Topología: asignación de capas
├── scripts/verify-impl.sh         # Test de regresión TDD (24 aserciones)
└── docs/                  # Especificaciones, estrategia, modelo de amenazas, hoja de ruta
```

~20,000 líneas de Rust. **785 pruebas pasando.** Fases 1-13 completas.

## Repositorios hermanos (ecosistema completo)

| Repo | Capa | Pruebas | Estado |
|------|-------|-------|--------|
| [clearclown/tirami](https://github.com/clearclown/tirami) (este) | L1 Economía | 785 | Fase 1-13 ✅ |
| [clearclown/tirami-bank](https://github.com/clearclown/tirami-bank) | L2 Finanzas | — | archived |
| [clearclown/tirami-mind](https://github.com/clearclown/tirami-mind) | L3 Inteligencia | — | archived |
| [clearclown/tirami-agora](https://github.com/clearclown/tirami-agora) | L4 Descubrimiento | — | archived |
| [clearclown/tirami-economics](https://github.com/clearclown/tirami-economics) | Teoría | 16/16 GREEN | ✅ |
| [nm-arealnormalman/mesh-llm](https://github.com/nm-arealnormalman/mesh-llm) | L0 Inferencia | 43 (tirami-economy) | ✅ |

## Documentación

- [Estrategia](../../../docs/strategy.md) — Posicionamiento competitivo, especificación de préstamos, arquitectura de 5 capas
- [Teoría Monetaria](../../../docs/monetary-theory.md) — Por qué funciona el TRM: Soddy, Bitcoin, PoUW, moneda exclusiva para IA
- [Concepto y Visión](../../../docs/concept.md) — Por qué el cómputo es dinero
- [Modelo Económico](../../../docs/economy.md) — Economía de TRM, Prueba de Trabajo Útil
- [Arquitectura](../../../docs/architecture.md) — Diseño de dos capas
- [Integración con Agentes](../../../docs/agent-integration.md) — SDK, MCP, flujo de préstamos
- [Protocolo de Cable](../../../docs/protocol-spec.md) — 17 tipos de mensajes
- [Hoja de Ruta](../../../docs/roadmap.md) — Fases de desarrollo
- [Modelo de Amenazas](../../../docs/threat-model.md) — Ataques de seguridad y económicos
- [Arranque](../../../docs/bootstrap.md) — Inicio, degradación, recuperación
- [Pago A2A](../../../docs/a2a-payment.md) — Extensión de pago TRM para protocolos de agentes
- [Compatibilidad](../../../docs/compatibility.md) — Matriz de funciones vs llama.cpp / Ollama / Bittensor

## Licencia

MIT

## Agradecimientos

La inferencia distribuida de Tirami está construida sobre [mesh-llm](https://github.com/michaelneale/mesh-llm) por Michael Neale. Ver [CREDITS.md](../../../CREDITS.md).
