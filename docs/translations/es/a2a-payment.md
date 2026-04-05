# Extensión de Pago de Forge CU para el Protocolo Agente a Agente (A2A)

*Propuesta para añadir el pago por cómputo a los estándares de comunicación entre agentes*

## Resumen (Abstract)

Los protocolos existentes de agente a agente (Google A2A, Anthropic MCP) definen cómo se comunican los agentes, pero no cómo se pagan entre sí. Esta propuesta añade una capa de pago de CU (Unidad de Cómputo), permitiendo que los agentes intercambien cómputo de forma autónoma sin intervención humana ni transacciones en la cadena de bloques (blockchain).

## Problema

Cuando el Agente A le pide al Agente B que realice una tarea:
- **Hoy:** El humano del Agente A le paga al humano del Agente B (tarjeta de crédito, clave API).
- **Necesidad:** El Agente A le paga al Agente B directamente en unidades de cómputo.

Ningún estándar existente admite el pago de agente a agente.

## Propuesta: Encabezados de Pago de CU

### Solicitud (Request)

El Agente A añade encabezados de pago cuando solicita trabajo:

```http
POST /v1/chat/completions HTTP/1.1
X-Forge-Consumer-Id: <agent-a-node-id>
X-Forge-Max-CU: 500
X-Forge-Consumer-Sig: <ed25519-signature-of-request-hash>
```

### Respuesta (Response)

El Agente B incluye información de costos:

```http
HTTP/1.1 200 OK
X-Forge-Provider-Id: <agent-b-node-id>
X-Forge-CU-Cost: 47
X-Forge-Provider-Sig: <ed25519-signature-of-response-hash>
```

### Registro de Transacción (Trade Record)

Ambos agentes registran de forma independiente:

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

### Chismes (Gossip)

Los registros de transacciones con firma doble se sincronizan por chismes (gossip) a través de la red mesh. Cualquier nodo puede verificar ambas firmas.

## Integración con Estándares Existentes

### Google A2A

Añadir al objeto `Task` de A2A:

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

Añadir un recurso `forge_payment` a los servidores MCP:

```json
{
  "resources": [{
    "uri": "forge://payment/balance",
    "name": "Saldo de CU",
    "mimeType": "application/json"
  }]
}
```

### Llamada a Funciones de OpenAI (Function Calling)

Los agentes que utilizan la llamada a funciones pueden incluir herramientas de Forge:

```json
{
  "tools": [{
    "type": "function",
    "function": {
      "name": "forge_pay",
      "description": "Pagar CU por una tarea de cómputo",
      "parameters": {
        "provider": "string",
        "cu_amount": "integer"
      }
    }
  }]
}
```

## Seguridad

- Todos los pagos requieren firmas Ed25519 bilaterales.
- Las políticas presupuestarias limitan el gasto por solicitud, por hora y de por vida.
- Los disyuntores se activan ante patrones de gasto anómalos.
- El interruptor de apagado (kill switch) congela todas las transacciones (anulación humana).
- No se requiere blockchain — la prueba bilateral es suficiente.

## Comparación

| Característica | Stripe | Bitcoin Lightning | **Forge CU** |
|---------|--------|-------------------|-------------|
| Agente a agente | No (necesita humano) | Parcial (necesita canal) | **Sí** |
| Velocidad de liquidación | Días | Segundos | **Instantánea** |
| Costo de transacción | 2.9% | ~1 sat | **Cero** |
| Respaldo de valor | Fiat | PoW (inútil) | **Computación útil** |
| SDK para agentes | No | No | **Sí** |

## Implementación

Implementación de referencia: [github.com/clearclown/forge](https://github.com/clearclown/forge)

- SDK de Python: `pip install forge-sdk`
- Servidor MCP: `pip install forge-mcp`
- Crates de Rust: `forge-ledger`, `forge-core`
