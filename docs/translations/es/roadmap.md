# Forge — Hoja de Ruta (Roadmap)

## Fase 1: Inferencia Local ✅

- `forge-core`: Sistema de tipos (NodeId, LayerRange, ModelManifest, PeerCapability)
- `forge-infer`: Motor llama.cpp, cargador GGUF, generación de tokens en streaming
- `forge-node`: API HTTP (/chat, /chat/stream, /health)
- `forge-cli`: Comando `forge chat` con descarga automática de modelos

## Fase 2: Protocolo P2P ✅

- `forge-net`: Transporte Iroh, cifrado Noise, conexiones entre pares
- `forge-proto`: 14 tipos de mensajes de protocolo de red (bincode + prefijo de longitud)
- `forge-node`: Pipeline Seed/Worker, solicitud/respuesta de inferencia
- Pruebas de integración: 2 nodos intercambian Hello + múltiples mensajes

## Fase 3: Inferencia Remota + Libro del Operador (Operator Ledger) ✅

- `forge-ledger`: Contabilidad de CU, ejecución de intercambios (trades), reputación, rendimiento, precios de mercado
- `forge-node`: Libro integrado en el pipeline de inferencia
- Comprobaciones de saldo de CU antes de la inferencia
- Registros de intercambio después de completar
- Integridad del libro mediante HMAC-SHA256

## Fase 4: API Económica ✅

- API compatible con OpenAI: `POST /v1/chat/completions`, `GET /v1/models`
- Medición de CU: cada inferencia registra un intercambio con la extensión `x_forge`
- Endpoints de presupuesto del agente: `GET /v1/forge/balance`, `GET /v1/forge/pricing`
- Puente de liquidación CU→Lightning: `forge settle --pay`
- Resolución automática del modelo seed desde HF Hub
- Apagado ordenado con Ctrl-C y persistencia del libro

## Fase 5: Integración del Fork de mesh-llm (próximo)

**Objetivo:** Reemplazar la capa de inferencia de Forge con el motor distribuido probado de mesh-llm.

| Entregable | Descripción |
|---|---|
| Fork de mesh-llm | Crear forge como un fork de mesh-llm con capa económica |
| Integrar forge-ledger | Vincular el registro de CU en el pipeline de inferencia de mesh-llm |
| Preservar API económica | Mantener los endpoints /v1/forge/* en el nuevo código base |
| Extensión de la consola web | Añadir visibilidad de saldo de CU e intercambios a la consola de mesh-llm |
| Pipeline + MoE | Heredar el paralelismo de pipeline y el sharding de expertos de mesh-llm |
| Descubrimiento Nostr | Heredar el descubrimiento de red mesh pública de mesh-llm |
| CREDITS.md | Documentar la atribución a mesh-llm |

## Fase 6: Prueba de Trabajo Útil (Proof of Useful Work)

**Objetivo:** Hacer que las reclamaciones de CU sean verificables en toda la red.

| Entregable | Descripción |
|---|---|
| Protocolo de firma dual | Tanto el proveedor como el consumidor firman cada TradeRecord |
| Sincronización por Gossip | Los intercambios firmados se propagan por la red mesh |
| Detección de fraude | Rechazar intercambios no firmados o que no coinciden |
| Gossip de reputación | Compartir puntuaciones de reputación entre pares |
| Resistencia a la colusión | Detección de anomalías estadísticas en los patrones de intercambio |

## Fase 7: Puentes Externos

**Objetivo:** Permitir que los operadores conviertan CU en valor externo.

| Entregable | Descripción |
|---|---|
| Puente Lightning | Liquidación automatizada de CU→sats a través de LDK |
| Adaptador de Stablecoin | Conversión de CU→USDC/USDT |
| Interfaz de adaptador Fiat | Especificación para liquidación mediante transferencia bancaria |
| Servicio de tasa de cambio | Fuentes públicas de tasas CU/BTC y CU/USD |
| Anclaje a Bitcoin | Opcional: raíz de Merkle periódica → OP_RETURN para rastro de auditoría inmutable |

## Fase 8: Economía Autónoma de Agentes

**Objetivo:** Permitir que los agentes de IA gestionen su propio ciclo de vida de cómputo.

| Entregable | Descripción |
|---|---|
| Políticas de presupuesto | Límites de gasto establecidos por humanos por agente |
| Comercio autónomo | El agente decide cuándo comprar/vender cómputo |
| Enrutamiento multi-modelo | El agente elige el modelo basándose en la relación coste/calidad |
| Autorrefuerzo | El agente gana CU → compra acceso a modelos más grandes → gana más CU |
| Economía entre agentes | Los agentes intercambian cómputo especializado (modelo de código vs modelo de chat) |

## Largo Plazo

| Hito | Descripción |
|---|---|
| Lanzamiento del SDK | forge-node como biblioteca de Rust integrable con API estable |
| Protocolo v2 | Lecciones de la v1, evolución compatible con versiones anteriores |
| Arquitectura cruzada | Soporte para NVIDIA GPU, AMD ROCm, RISC-V (vía mesh-llm) |
| Entrenamiento federado | Ajuste fino (fine-tuning) distribuido, no solo inferencia |
| Derivados de cómputo | Contratos a plazo sobre capacidad de cómputo futura |

> El protocolo es la plataforma. El cómputo es la moneda.
