# Forge — Guía de Integración de Agentes

## Para Desarrolladores de Agentes de IA

Forge le da a su agente un presupuesto de cómputo. El agente puede ganar CU al servir inferencias y gastar CU para acceder a modelos más grandes. Sin tarjeta de crédito, sin clave API, sin humanos en el proceso.

## Integración Rápida

### Cualquier Cliente HTTP

```python
import requests

FORGE = "http://127.0.0.1:3000"

# Verificar si el agente puede costear una solicitud
balance = requests.get(f"{FORGE}/v1/forge/balance").json()
if balance["effective_balance"] > 100:
    # Ejecutar inferencia (cuesta CU)
    r = requests.post(f"{FORGE}/v1/chat/completions", json={
        "messages": [{"role": "user", "content": "¿Qué es la gravedad?"}],
        "max_tokens": 256
    }).json()
    print(r["choices"][0]["message"]["content"])
    print(f"Costo: {r['x_forge']['cu_cost']} CU")
```

### SDK de Python

```python
from forge_sdk import ForgeClient, ForgeAgent

# Cliente simple
forge = ForgeClient()
result = forge.chat("Explica la computación cuántica")
print(f"Respuesta: {result['content']}")
print(f"Costo: {result['cu_cost']} CU, Saldo: {result['balance']} CU")

# Agente autónomo con gestión de presupuesto
agent = ForgeAgent(max_cu_per_task=500)
while agent.has_budget():
    result = agent.think("¿Qué debería hacer ahora?")
    if result is None:
        break  # presupuesto agotado
```

### MCP (Claude Code, Cursor)

Añada a su configuración de MCP:
```json
{
  "mcpServers": {
    "forge": {
      "command": "python",
      "args": ["ruta/a/forge/mcp/forge-mcp-server.py"]
    }
  }
}
```

El asistente de IA puede entonces usar herramientas como `forge_balance`, `forge_pricing`, `forge_inference`.

### LangChain

```python
from langchain_openai import ChatOpenAI

llm = ChatOpenAI(
    base_url="http://127.0.0.1:3000/v1",
    api_key="no-es-necesaria",
    model="qwen2.5-0.5b-instruct-q4_k_m"
)
response = llm.invoke("Hola")
# metadatos x_forge disponibles en los encabezados de respuesta
```

### curl

```bash
# Verificar saldo
curl localhost:3000/v1/forge/balance

# Ejecutar inferencia
curl localhost:3000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"messages":[{"role":"user","content":"hola"}]}'

# Verificar qué costó
curl localhost:3000/v1/forge/trades
```

## Bucle Económico del Agente

El patrón recomendado para un agente autónomo:

```python
from forge_sdk import ForgeClient

forge = ForgeClient()

def agent_loop():
    while True:
        # 1. Verificar presupuesto
        balance = forge.balance()
        if balance["effective_balance"] < 50:
            print("Saldo de CU bajo. Esperando para ganar más...")
            time.sleep(60)
            continue

        # 2. Verificar precios
        pricing = forge.pricing()
        cost_per_100 = pricing["estimated_cost_100_tokens"]

        # 3. Decidir si la tarea vale el costo
        if cost_per_100 > 500:
            print("Precio de mercado demasiado alto. Esperando...")
            time.sleep(30)
            continue

        # 4. Ejecutar
        result = forge.chat("Analiza estos datos...", max_tokens=200)
        print(f"Hecho. Costo: {result['cu_cost']} CU")

        # 5. Verificar seguridad
        safety = forge.safety()
        if safety["circuit_tripped"]:
            print("Disyuntor activado. Pausando...")
            time.sleep(300)
```

## Seguridad para Desarrolladores de Agentes

### Establecer Políticas de Presupuesto

```bash
# Limitar un agente a 1000 CU por hora
curl -X POST localhost:3000/v1/forge/policy \
  -H "Content-Type: application/json" \
  -d '{
    "node_id": "<agent_node_id>",
    "max_cu_per_hour": 1000,
    "max_cu_per_request": 100,
    "human_approval_threshold": 500
  }'
```

### Monitorear Velocidad de Gasto

```bash
curl localhost:3000/v1/forge/safety
# Devuelve: hourly_spend, lifetime_spend, spends_last_minute
```

### Parada de Emergencia

```bash
# Congelar todo
curl -X POST localhost:3000/v1/forge/kill \
  -d '{"activate": true, "reason": "anomalía del agente"}'
```

## Referencia de la API para Agentes

| Qué necesita el agente | Endpoint | Método |
|-----------------|----------|--------|
| "¿Cuánto CU tengo?" | `/v1/forge/balance` | GET |
| "¿Cuánto costará esto?" | `/v1/forge/pricing` | GET |
| "¿Quién es el proveedor más barato?" | `/v1/forge/providers` | GET |
| "Ejecutar inferencia" | `/v1/chat/completions` | POST |
| "¿En qué gasté?" | `/v1/forge/trades` | GET |
| "¿Estoy seguro?" | `/v1/forge/safety` | GET |
| "Cobrar en Bitcoin" | `/v1/forge/invoice` | POST |
| "DETENER TODO" | `/v1/forge/kill` | POST |
