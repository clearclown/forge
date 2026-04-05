# Forge — Secuencia de Arranque (Bootstrap)

## Descripción General

Forge tiene dos rutas de arranque:

- el **flujo de referencia actual**, que es explícito y dirigido por el operador.
- el **flujo objetivo**, donde un nodo basado en mesh-llm se une a una red mesh y comienza a ganar CU automáticamente.

## Flujo de Referencia Actual

```text
1. Iniciar un anfitrión de modelo: forge seed -m "qwen2.5:0.5b"
2. Copiar la clave pública impresa
3. Conectar un solicitante: forge worker --seed <seed_public_key>
4. Verificar el estado: forge status --url http://127.0.0.1:3000
5. Verificar el saldo de CU: curl http://127.0.0.1:3000/v1/forge/balance
```

La API HTTP se vincula a `127.0.0.1` por defecto. Si se expone, establezca `--api-token`.

## Arranque Objetivo (fork de mesh-llm)

Una vez que Forge se integre con mesh-llm:

```text
1. forge --auto                          # unirse a la mejor red mesh pública
2. forge --model Qwen2.5-32B --publish   # crear una red mesh pública, ganar CU
3. forge --join <token>                  # unirse con GPU, ganar CU
4. forge --client --join <token>         # unirse como consumidor, gastar CU
```

Cada inferencia servida gana CU. Cada inferencia consumida gasta CU. La capa económica es automática — no se necesita una configuración separada.

## Arranque Económico

### Nuevo Nodo (Saldo Cero)

```text
1. El nodo se une a la red mesh
2. Nivel gratuito: 1,000 CU disponibles de inmediato
3. El nodo sirve la primera solicitud de inferencia → gana CU
4. El saldo de CU crece con cada solicitud servida
5. El nodo ahora puede gastar CU en la inferencia de otros nodos
```

### Nodo Existente (Tiene Saldo)

```text
1. El nodo se reinicia, carga el libro contable persistente (forge-ledger.json)
2. Integridad HMAC-SHA256 verificada
3. Saldo anterior, transacciones y reputación restaurados
4. El nodo reanuda la ganancia y el gasto de CU
```

## Degradación y Recuperación

| Evento | Impacto Económico | Impacto en la Inferencia |
|---|---|---|
| 1 nodo remoto se desconecta | Los nodos restantes absorben el trabajo, el flujo de CU continúa | Pausa breve, modelo reequilibrado |
| Todos los nodos remotos se desconectan | La economía de CU se pausa, modo solo local | Volver al modelo pequeño local |
| Batería baja del nodo (<20%) | Deja de servir (la ganancia se pausa), aún puede consumir | Descargar capas a remoto |
| El nodo recupera la red | Reanuda la ganancia de CU, la reputación se recupera | Redescubrir pares, reexpandirse |

**Invariante clave**: El saldo de CU de un nodo persiste a través de reinicios y desconexiones. El CU ganado nunca se pierde.

## Modelo de Contribución del Nodo

- **Contribuyentes**: Los dispositivos que sirven inferencias ganan CU.
- **Consumidores**: Los dispositivos que solicitan inferencias gastan CU.
- **Saldo**: Más contribución → más CU → más acceso al cómputo de otros.
- **Nivel gratuito**: 1,000 CU para nuevos nodos, consumidos desde la primera solicitud.
- **Rendimiento (Yield)**: Los nodos en línea ganan un rendimiento del 0.1% por hora (ponderado por reputación).
- **Sin pago obligatorio**: El protocolo funciona con CU. Los puentes externos (Lightning, fiat) son opcionales.

## Seguridad durante el Arranque

- Identidad Ed25519 creada antes de cualquier actividad de red.
- Todas las conexiones cifradas a través de QUIC + Noise.
- En el flujo actual de semilla/trabajador, la semilla ve el texto del prompt (límite de confianza explícito).
- Las transacciones de CU se registran localmente con integridad HMAC-SHA256.
- Objetivo: transacciones firmadas doblemente sincronizadas por chismes (gossip) a través de la red mesh.
