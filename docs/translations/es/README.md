# Forge

> El cómputo es moneda. Cada vatio produce inteligencia, no desperdicio.

**Forge es un protocolo de inferencia distribuida donde el cómputo es dinero.** Los nodos ganan Unidades de Cómputo (CU) al realizar inferencias útiles de LLM para otros. A diferencia de Bitcoin — donde la electricidad se quema en hashes sin sentido — cada julio gastado en un nodo de Forge produce inteligencia real que alguien realmente necesita.

El motor de inferencia distribuida está construido sobre [mesh-llm](https://github.com/michaelneale/mesh-llm) por Michael Neale. Forge añade una economía de cómputo por encima: contabilidad de CU, Prueba de Trabajo Útil, precios dinámicos, presupuestos de agentes autónomos y controles de seguridad. Ver [CREDITS.md](CREDITS.md).

**Fork integrado:** [forge-mesh](https://github.com/nm-arealnormalman/mesh-llm) — mesh-llm con la capa económica de Forge incorporada.

## Demo en Vivo

Esta es una salida real de un nodo de Forge en ejecución. Cada inferencia cuesta CU. Cada CU se gana mediante computación útil.

```
$ forge node -m "qwen2.5:0.5b" --ledger forge-ledger.json
  Modelo cargado: Qwen2.5-0.5B (acelerado por Metal, 491MB)
  Servidor API escuchando en 127.0.0.1:3000
```

**Verificar saldo — cada nuevo nodo recibe 1,000 CU de nivel gratuito:**
```
$ curl localhost:3000/v1/forge/balance
{
  "effective_balance": 1000,
  "contributed": 0,
  "consumed": 0,
  "reputation": 0.5
}
```

**Hacer una pregunta — la inferencia cuesta CU:**
```
$ curl localhost:3000/v1/chat/completions \
    -d '{"messages":[{"role":"user","content":"Di hola en japonés"}]}'
{
  "choices": [{"message": {"content": "こんにちは！ (konnichiwa!)"}}],
  "usage": {"completion_tokens": 9},
  "x_forge": {
    "cu_cost": 9,
    "effective_balance": 1009
  }
}
```

Cada respuesta incluye `x_forge` — **el costo de esa computación en CU** y el saldo restante. El proveedor ganó 9 CU. El consumidor gastó 9 CU. La física respaldó cada unidad.

**Tres inferencias después — transacciones reales en el libro contable:**
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

**Cada transacción tiene una raíz de Merkle — anclable a Bitcoin para una prueba inmutable:**
```
$ curl localhost:3000/v1/forge/network
{
  "total_trades": 3,
  "total_contributed_cu": 19,
  "merkle_root": "aac8db9f62dd9ff23926195a70ed8fcfc188fc867d9f2adabd8e694beb338748"
}
```

**¿Agentes de IA fuera de control? El interruptor de apagado (kill switch) congela todo en milisegundos:**
```
$ curl -X POST localhost:3000/v1/forge/kill \
    -d '{"activate":true, "reason":"anomalía detectada", "operator":"admin"}'
→ INTERRUPTOR DE APAGADO ACTIVADO
→ Todas las transacciones de CU congeladas. Ningún agente puede gastar.
```

**Controles de seguridad siempre activos:**
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

## Por qué existe Forge

```
Bitcoin:  electricidad  →  SHA-256 sin sentido  →  BTC
Forge:    electricidad  →  inferencia útil de LLM →  CU
```

Bitcoin demostró `electricidad → computación → dinero`. Pero la computación de Bitcoin no tiene propósito. Forge lo invierte: cada CU representa inteligencia real que resolvió el problema real de alguien.

**Tres cosas que ningún otro proyecto hace:**

### 1. Cómputo = Moneda

Cada inferencia es una transacción. El proveedor gana CU, el consumidor gasta CU. Sin blockchain, sin tokens, sin ICO. El CU está respaldado por la física — la electricidad consumida para un trabajo útil.

### 2. A prueba de manipulaciones sin una Blockchain

Cada transacción está firmada doblemente (Ed25519) por ambas partes y sincronizada por chismes (gossip) a través de la red. Una raíz de Merkle de todas las transacciones puede anclarse a Bitcoin para una auditoría inmutable. No se necesita un consenso global — la prueba criptográfica bilateral es suficiente.

### 3. Los agentes de IA gestionan su propio cómputo

Un agente en un teléfono presta cómputo inactivo durante la noche → gana CU → compra acceso a un modelo de 70B → se vuelve más inteligente → gana más. El agente consulta `/v1/forge/balance` y `/v1/forge/pricing` de forma autónoma. Las políticas presupuestarias y los disyuntores evitan el gasto descontrolado.

```
Agente (1.5B en el teléfono)
  → gana CU durante la noche sirviendo inferencias
  → gasta CU en un modelo de 70B → respuestas más inteligentes
  → mejores decisiones → más CU ganado
  → el ciclo se repite → el agente crece
```

## Arquitectura

```
┌─────────────────────────────────────────────────┐
│  Capa de Inferencia (mesh-llm)                  │
│  Paralelismo de pipeline, sharding MoE,         │
│  red iroh, descubrimiento Nostr, API de OpenAI  │
└──────────────────┬──────────────────────────────┘
                   │
┌──────────────────▼──────────────────────────────┐
│  Capa Económica (Forge)                         │
│  Libro contable de CU, transacciones firmadas,  │
│  gossip, precios dinámicos, raíz de Merkle,     │
│  controles de seguridad                         │
└──────────────────┬──────────────────────────────┘
                   │
┌──────────────────▼──────────────────────────────┐
│  Capa de Seguridad                              │
│  Kill switch, políticas presupuestarias,        │
│  disyuntores, detección de velocidad,           │
│  umbrales de aprobación humana                  │
└──────────────────┬──────────────────────────────┘
                   │ opcional
┌──────────────────▼──────────────────────────────┐
│  Puentes Externos                               │
│  CU ↔ BTC (Lightning), CU ↔ stablecoin          │
└─────────────────────────────────────────────────┘
```

## Inicio Rápido

```bash
# Construir
cargo build --release

# Ejecutar un nodo con modelo descargado automáticamente
forged node -m "qwen2.5:0.5b" --ledger forge-ledger.json

# Chatear localmente
forge chat -m "qwen2.5:0.5b" "¿Qué es la gravedad?"

# Iniciar una semilla (P2P, gana CU)
forge seed -m "qwen2.5:0.5b" --ledger forge-ledger.json

# Conectar como trabajador (P2P, gasta CU)
forge worker --seed <public_key>

# Listar modelos
forge models
```

## Referencia de la API

### Inferencia (compatible con OpenAI)

| Endpoint | Descripción |
|----------|-------------|
| `POST /v1/chat/completions` | Chat con streaming. Cada respuesta incluye `x_forge.cu_cost` |
| `GET /v1/models` | Listar modelos cargados |

### Economía

| Endpoint | Descripción |
|----------|-------------|
| `GET /v1/forge/balance` | Saldo de CU, reputación, historial de contribuciones |
| `GET /v1/forge/pricing` | Precio de mercado (suavizado EMA), estimaciones de costos |
| `GET /v1/forge/trades` | Transacciones recientes con montos de CU |
| `GET /v1/forge/network` | Flujo total de CU + raíz de Merkle |
| `GET /v1/forge/providers` | Proveedores clasificados por reputación y costo |
| `POST /v1/forge/invoice` | Crear factura de Lightning desde el saldo de CU |
| `GET /settlement` | Estado de liquidación exportable |

### Seguridad

| Endpoint | Descripción |
|----------|-------------|
| `GET /v1/forge/safety` | Estado del kill switch, disyuntor, política presupuestaria |
| `POST /v1/forge/kill` | Parada de emergencia — congelar todas las transacciones de CU |
| `POST /v1/forge/policy` | Establecer límites presupuestarios por agente |

## Diseño de Seguridad

Los agentes de IA que gastan cómputo de forma autónoma son poderosos pero peligrosos. Forge tiene cinco capas de seguridad:

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
| 1971–presente | Petroldólar | Demanda de petróleo + poder militar |
| 2009–presente | Bitcoin | Energía en SHA-256 (trabajo inútil) |
| **Ahora** | **Estándar de Cómputo** | **Energía en inferencia de LLM (trabajo útil)** |

Una habitación llena de Mac Minis ejecutando Forge es un edificio de apartamentos — generando rendimiento al realizar un trabajo útil mientras el propietario duerme.

## Estructura del Proyecto

```
forge/
├── crates/
│   ├── forge-ledger/      # Contabilidad de CU, transacciones, precios, seguridad, raíz de Merkle
│   ├── forge-node/        # Demonio del nodo, API HTTP, coordinador de pipeline
│   ├── forge-cli/         # CLI: chat, seed, worker, liquidar, billetera
│   ├── forge-lightning/   # Puente CU ↔ Bitcoin Lightning
│   ├── forge-net/         # P2P: iroh QUIC + Noise + gossip
│   ├── forge-proto/       # Protocolo de cable: 17 tipos de mensajes
│   ├── forge-infer/       # Inferencia: llama.cpp, GGUF, Metal/CPU
│   ├── forge-core/        # Tipos: NodeId, CU, Config
│   └── forge-shard/       # Topología: asignación de capas
└── docs/                  # Especificaciones, modelo de amenazas, hoja de ruta
```

~10,000 líneas de Rust. 76 pruebas. 2 auditorías de seguridad completadas.

## Documentación

- [Concepto y Visión](docs/concept.md) — Por qué el cómputo es dinero
- [Modelo Económico](docs/economy.md) — Economía de CU, Prueba de Trabajo Útil
- [Arquitectura](docs/architecture.md) — Diseño de dos capas
- [Protocolo de Cable](docs/protocol-spec.md) — 17 tipos de mensajes
- [Hoja de Ruta](docs/roadmap.md) — Fases de desarrollo
- [Modelo de Amenazas](docs/threat-model.md) — Ataques de seguridad y económicos
- [Arranque](docs/bootstrap.md) — Inicio, degradación, recuperación

## Licencia

MIT

## Agradecimientos

La inferencia distribuida de Forge está construida sobre [mesh-llm](https://github.com/michaelneale/mesh-llm) por Michael Neale. Ver [CREDITS.md](CREDITS.md).
