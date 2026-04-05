# Forge — Modelo Económico

## Estándar de Cómputo (計算本位制)

Cada sistema monetario está respaldado por la escasez:

| Era | Estándar | Respaldo |
|-----|----------|---------|
| Antigua | Oro/Plata | Escasez geológica |
| 1944–1971 | Bretton Woods | USD vinculado al oro |
| 1971–presente | Petroldólar | Demanda de petróleo + poder militar |
| 2009–presente | Bitcoin | Electricidad quemada en SHA-256 |
| **Forge** | **Estándar de Cómputo** | **Electricidad gastada en inferencia útil** |

Forge introduce un Estándar de Cómputo: la unidad de valor está respaldada por el gasto real de energía que realiza una computación útil. A diferencia de la Prueba de Trabajo de Bitcoin, cada julio gastado en Forge produce inteligencia real.

## CU: La Moneda Nativa

### Qué es el CU

**1 CU = 1 mil millones de FLOPs de trabajo de inferencia verificado.**

El CU no es una criptomoneda. No es un token en una cadena de bloques. Es una unidad de cuenta que representa la computación real realizada. El CU tiene valor porque es un derecho sobre el cómputo futuro — si ganó CU sirviendo inferencias, puede gastarlo para recibir inferencias.

### Por qué CU y no Bitcoin

| Propiedad | CU | Bitcoin |
|----------|-----|---------|
| **Respaldo de valor** | Computación útil (intrínseca) | Computación de hash (artificial) |
| **Velocidad de liquidación** | Instantánea (libro contable local) | De segundos a minutos (Lightning/cadena) |
| **Costo de transacción** | Cero | Tarifas de canal, tarifas de la cadena |
| **Dependencia externa** | Ninguna | Salud de la red Bitcoin |
| **Riesgo cuántico** | Ninguno (sin acertijo criptográfico) | SHA-256 / ECDSA vulnerable |
| **Generación de rendimiento** | Sí (el hardware inactivo gana CU) | No (el BTC en la billetera no gana nada) |

El CU es la unidad de liquidación **principal**. Bitcoin, las stablecoins y el fiat son **rampas de salida** opcionales disponibles a través de adaptadores de puente fuera del protocolo.

### El CU como Activo Productivo

```
Edificio de apartamentos        Mac Mini en Forge
───────────────────         ──────────────────
Activo: edificio             Activo: hardware de cómputo
Costo: mantenimiento         Costo: electricidad
Ingresos: alquiler           Ingresos: CU de inferencia
Rendimiento: alq. - mant.    Rendimiento: CU ganado - costo elec.
Inactivo = ingresos perdidos Inactivo = potencial desperdiciado
```

Un dispositivo de computación en Forge no es como el Bitcoin en una billetera (valor estático, sin rendimiento). Es como una propiedad de alquiler — genera ingresos a través de un trabajo útil.

## Modelo de Transacción

### Ejecución de Transacciones

Cada inferencia crea una transacción entre dos partes:

```rust
pub struct TradeRecord {
    pub provider: NodeId,       // Quién realizó la inferencia
    pub consumer: NodeId,       // Quién la solicitó
    pub cu_amount: u64,         // CU transferidos
    pub tokens_processed: u64,  // Trabajo realizado
    pub timestamp: u64,
    pub model_id: String,
}
```

La transacción es registrada por ambas partes. En la implementación actual, cada nodo mantiene un libro contable local. La implementación objetivo añade firmas dobles y sincronización por chismes (gossip).

### Precios Dinámicos

Los precios de CU fluctúan según la oferta y demanda local:

```
precio_efectivo = cu_base_por_token × factor_demanda / factor_oferta
```

- **Más nodos inactivos** → aumenta factor_oferta → baja el precio.
- **Más solicitudes de inferencia** → aumenta factor_demanda → sube el precio.
- Cada nodo observa sus propias condiciones de mercado. No hay un libro de órdenes global.

### Nivel Gratuito

Los nuevos nodos sin historial de contribución reciben 1,000 CU. Esto permite que cualquiera use la red de inmediato. El nivel gratuito se consume desde la primera solicitud — no se restablece.

Mitigación de Sybil: si han aparecido más de 100 nodos desconocidos sin contribuir, se rechazan las nuevas solicitudes de nivel gratuito.

## Prueba de Trabajo Útil

### El Concepto

Prueba de Trabajo de Bitcoin: "Quemé electricidad computando hashes SHA-256. Aquí está el nonce que lo demuestra".

Prueba de Trabajo Útil de Forge: "Quemé electricidad ejecutando inferencias de LLM. Aquí está la respuesta, y aquí está la firma del consumidor confirmando que la recibió".

La diferencia clave: la prueba de Bitcoin se genera por uno mismo (cualquier minero puede producir un hash válido). La prueba de Forge requiere una **contraparte** — alguien que realmente quería la inferencia. No se puede falsificar la demanda.

### Protocolo de Verificación (objetivo)

```
1. El consumidor envía la solicitud de inferencia (InferenceRequest) al proveedor.
2. El proveedor ejecuta la inferencia, transmite los tokens de vuelta.
3. El consumidor recibe los tokens, calcula el hash de la respuesta.
4. Ambas partes firman el TradeRecord:
   - El proveedor firma: "Yo computé esto".
   - El consumidor firma: "Yo recibí esto".
5. El TradeRecord con firma doble se sincroniza por chismes (gossip) en la red.
6. Cualquier nodo puede verificar ambas firmas.
```

Un nodo no puede inflar su saldo de CU sin una contraparte colaboradora. La colusión es posible pero económicamente irracional — el consumidor coludido no gana nada al firmar transacciones falsas.

### Implementación Actual

La implementación de referencia actual utiliza libros contables locales con protección de integridad HMAC-SHA256. Las firmas dobles y el gossip son el siguiente paso.

## Rendimiento y Reputación

### Rendimiento (Yield)

Los nodos que permanecen en línea y contribuyen con cómputo ganan rendimiento:

```
yield_cu = cu_contribuido × 0.001 × reputación × horas_en_línea
```

Con una reputación de 1.0, un nodo con 10,000 CU contribuidos gana 80 CU por una noche de 8 horas. Esto no es inflación — es una recompensa por la disponibilidad. Los nodos que están en línea de manera confiable son más valiosos para la red.

### Reputación

Cada nodo tiene una puntuación de reputación entre 0.0 y 1.0:

- Los nuevos nodos comienzan en 0.5.
- El tiempo de actividad (uptime) y las transacciones exitosas aumentan la reputación.
- Las desconexiones y las transacciones fallidas disminuyen la reputación.
- Mayor reputación → mayor tasa de rendimiento, prioridad en la programación.

## Liquidación y Puentes Externos

### Regla Principal

**El protocolo liquida en CU.** La conversión a cualquier otra cosa es una cuestión de integración.

### Estados de Liquidación

Los operadores pueden exportar historiales de transacciones auditables para cualquier ventana de tiempo:

```
forge settle --hours 24 --price 0.05 --out settlement.json
```

El estado incluye: CU bruto ganado, CU bruto gastado, CU neto, recuento de transacciones y precio de referencia opcional por CU.

### Arquitectura del Puente

```
Capa 0: Protocolo Forge
  → Contabilidad de CU, transacciones, precios.

Capa 1: Estado de liquidación
  → Historial de transacciones exportable.
  → Tipo de cambio de referencia.

Capa 2: Puente externo (opcional)
  → CU ↔ BTC (Lightning)
  → CU ↔ stablecoin
  → CU ↔ fiat
```

La capa de puente está fuera del protocolo. Diferentes operadores pueden usar diferentes puentes. El protocolo sigue siendo útil con cero liquidez externa.

### Puente Lightning

Para los operadores que desean liquidación en Bitcoin:

```bash
forge settle --hours 24 --pay
```

Esto crea una factura de Lightning BOLT11 por el CU neto ganado, convertido al tipo de cambio configurado (por defecto: 10 msats por CU).

## Presupuestos Dirigidos por Agentes

### La Visión

Tradicional: Humano decide → Humano paga → IA ejecuta.
Forge: La política permite al agente → El agente verifica el presupuesto → El agente gasta CU → IA ejecuta.

### API

```
GET /v1/forge/balance   → Saldo de CU, contribución, consumo, reputación.
GET /v1/forge/pricing   → Precio de mercado, estimaciones de costos por 100/1000 tokens.
```

Un agente puede:
1. Verificar su saldo antes de realizar una solicitud.
2. Estimar el costo de la inferencia a los precios actuales del mercado.
3. Decidir si la solicitud vale el costo de CU.
4. Ejecutar y pagar automáticamente.

Los supervisores humanos establecen políticas presupuestarias. Los agentes operan dentro de esos límites de forma autónoma.

### Bucle de Autorrefuerzo

```
Agente (pequeño, teléfono)
  → gana CU prestando cómputo inactivo
  → gasta CU en el acceso a modelos más grandes
  → se vuelve más inteligente
  → toma mejores decisiones económicas
  → gana más CU
  → accede a modelos aún más grandes
  → ...
```

Este es un patrón de aplicación posible. El protocolo proporciona el mercado; los agentes proporcionan la estrategia.

## Por qué esto no es Web3

La mayoría de los proyectos Web3 crean escasez artificial (tokens) sobre bienes digitales abundantes. Forge hace lo contrario:

- **El cómputo es realmente escaso** — requiere electricidad real, silicio real, tiempo real.
- **El CU no es especulativo** — representa un trabajo verificado, no una apuesta por la adopción futura.
- **Sin ICO, sin venta de tokens, sin token de gobernanza** — el CU se gana trabajando.
- **No se requiere cadena de bloques** — las firmas bilaterales y el gossip son suficientes.
- **Sin contratos inteligentes** — el protocolo es el contrato.

El valor es fabricado por la física, no por el consenso.
