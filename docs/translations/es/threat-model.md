# Forge — Modelo de Amenazas (Threat Model)

## Objetivos de Seguridad

1. **Confidencialidad en tránsito**: los observadores pasivos y los repetidores (relays) no deben poder leer prompts ni respuestas.
2. **Pares autenticados**: cada conexión directa debe vincularse a una identidad de nodo criptográfica.
3. **Confianza acotada**: el sistema debe hacer explícito qué nodos ven el texto plano y cuáles solo ven el estado intermedio.
4. **Disponibilidad**: los fallos de un solo par deben degradar el servicio en lugar de corromperlo silenciosamente.

## Cifrado

### Capa de Transporte
- Todas las conexiones utilizan QUIC con TLS 1.3 (vía Iroh).
- Handshake adicional del Protocolo Noise (patrón XX) para la autenticación de pares.
- Resultado: cifrado simétrico ChaCha20-Poly1305 con secreto hacia adelante (forward secrecy).
- Claves efímeras por sesión: la vulneración de una sesión no afecta a las demás.

### Identidad
- Cada nodo tiene un par de claves Ed25519 persistente.
- Generado en el primer lanzamiento, almacenado en el llavero de la plataforma.
- ID de nodo = hash de la clave pública.
- Sin autoridad de certificación central — modelo de red de confianza (web-of-trust).

### Qué se Cifra
| Datos | Cifrado | Notas |
|---|---|---|
| Texto del prompt | Sí | Cifrado en tránsito; visible para el seed en el flujo de referencia actual |
| Salida de texto en streaming | Sí | Cifrado en tránsito; visible para el seed que lo generó |
| Tensores de activación | Planificado | Relevante una vez que la inferencia dividida esté activa en el runtime |
| Mensajes de control | Sí | Todos los mensajes del protocolo dentro de QUIC |
| Capacidades de los pares | Sí | Intercambiados a través de un canal cifrado |

## Análisis de Amenazas

### T1: Nodo Seed Malicioso (flujo de referencia actual)
**Amenaza**: El operador del seed lee el prompt o la respuesta porque el worker envía el texto del prompt al seed y el seed ejecuta el modelo completo.

**Estado actual**: Este es un límite de confianza explícito, no una propiedad de seguridad resuelta.

**Mitigación**:
- Conecte solo workers a seeds en los que confíe con prompts en texto plano.
- Mantenga el transporte cifrado para que los repetidores y observadores pasivos no puedan leer los contenidos.
- Avanzar hacia la inferencia dividida (split inference) para que los pares de etapas intermedias no reciban prompts en texto plano.

### T2: Nodo de Pipeline Malicioso (objetivo del flujo de inferencia dividida)
**Amenaza**: Un nodo en el futuro pipeline intenta extraer el prompt o la respuesta a partir de las activaciones intermedias.

**Objetivo de mitigación**: Un nodo en la etapa k del pipeline solo debe ver el tensor de activación de salida de la capa k-1 y producir el tensor de activación para la capa k. No debe recibir el texto original del prompt.

**Riesgo residual**: Los tensores de activación pueden filtrar información sobre la entrada. La privacidad diferencial, la redundancia y el atestiguamiento (attestation) siguen siendo trabajo futuro.

### T3: Ataque Sybil
**Amenaza**: Un atacante crea muchos nodos falsos para dominar el pipeline.

**Mitigación**:
- Sistema de reputación basado en el comportamiento observado (tiempo de actividad, cálculo correcto).
- Los nodos nuevos comienzan con baja reputación y posiciones de pipeline limitadas.
- Las capas críticas (primera y última) se asignan preferentemente a nodos de alta reputación.
- Limitación de tasa (rate limiting) en las uniones de nuevos nodos desde el mismo rango de IP.

### T4: Inferencia Bizantina
**Amenaza**: Un nodo malicioso devuelve tensores de activación incorrectos.

**Mitigación (MVP)**: Aceptar el riesgo. Para la mayoría de los casos de uso, un resultado de inferencia sutilmente incorrecto es detectable por el usuario.

**Mitigación (futuro)**:
- Cálculo redundante en capas críticas (2 nodos calculan las mismas capas, se comparan).
- Cálculo verificable mediante atestiguamiento TEE (Apple Silicon Secure Enclave).
- Detección de anomalías estadísticas en las distribuciones de los tensores de activación.

### T5: Análisis de Tráfico
**Amenaza**: Un observador monitorea los patrones de tráfico cifrado para inferir el uso.

**Mitigación**:
- QUIC multiplexa toda la comunicación sobre una sola conexión.
- El tráfico actual de seed/worker todavía filtra metadatos gruesos de sincronización de solicitudes y longitud de respuestas.
- Relleno (padding) en los mensajes de control a un tamaño constante (opcional, no en MVP).

### T6: Vulneración del Servidor de Repetidor (Relay)
**Amenaza**: Los servidores de repetidores de arranque (bootstrap) se ven comprometidos.

**Impacto**: Mínimo. Los servidores de repetidores solo facilitan el establecimiento de la conexión. Ven:
- Qué IDs de nodo se están conectando (metadatos).
- Paquetes QUIC cifrados (no pueden descifrar).
- NO ven prompts o respuestas descifrados.

**Mitigación**: Múltiples operadores de repetidores independientes. La red continúa sin repetidores una vez que se puebla la DHT.

### T7: Envenenamiento del Modelo
**Amenaza**: Un nodo sirve un modelo GGUF modificado con pesos con puertas traseras (backdoors).

**Mitigación**:
- Archivos de modelo verificados por hash SHA-256 contra manifiestos conocidos.
- Manifiestos de modelo distribuidos vía DHT con firmas de los publicadores de modelos.
- Los nodos solo cargan modelos de fuentes verificadas (hashes de HuggingFace).

### T8: Denegación de Servicio (DoS)
**Amenaza**: Los nodos se unen y luego dejan de responder, interrumpiendo la inferencia.

**Mitigación**:
- El tiempo de espera del latido (heartbeat) y el reequilibrio son propiedades de ejecución objetivo, no garantías completas de la implementación actual.
- El fallback local es un objetivo de diseño para futuros clientes de inferencia dividida.
- Penalización de reputación para los nodos que se desconectan frecuentemente.
- La degradación ordenada es un principio de diseño central.
- Las solicitudes de inferencia entrantes están acotadas por la validación del runtime y un límite fijo de ejecución concurrente en el seed.
- Los valores de `msg_id` de protocolo duplicados del mismo par se descartan dentro de una ventana de repetición acotada.

### T9: Exposición de la API Administrativa
**Amenaza**: Un operador vincula la API HTTP local a una interfaz pública sin protección, exponiendo `/status`, `/topology`, `/settlement` o `/chat`.

**Mitigación en la implementación actual**:
- El demonio vincula la API HTTP a `127.0.0.1` por defecto.
- Los operadores aún pueden exponerla intencionalmente con `--bind 0.0.0.0`.
- Las rutas administrativas expuestas pueden protegerse con un token de portador (bearer token) a través de `--api-token`.
- Los cuerpos de las solicitudes JSON tienen un tamaño limitado antes de la deserialización para reducir el abuso de asignación en `/chat`.

**Riesgo residual**: La autenticación por token de portador es un control del operador, no TLS mutuo. Si el token se filtra, la API debe considerarse comprometida hasta que se rote.

## Jerarquía de Confianza

```
Más confiable:   Tu propio dispositivo (teléfono, portátil)
                 ↓
Confiable:       Tus propios dispositivos en LAN (Mac Mini en casa)
                 ↓
Semi-confiable:  Pares WAN de alta reputación (meses de tiempo de actividad)
                 ↓
No confiable:    Nuevos pares WAN (recién unidos, sin historial)
```

La asignación de capas debe seguir esta jerarquía una vez que exista la inferencia dividida:
- Primeras y últimas capas (más sensibles — ven los embeddings de entrada y los logits de salida) → tus propios dispositivos.
- Capas intermedias (solo ven activaciones intermedias) → pueden asignarse a pares semi-confiables o no confiables.

## Garantías de Privacidad

**Lo que Forge garantiza hoy:**
- Los prompts y las respuestas se cifran en tránsito entre pares directamente conectados.
- Los repetidores y observadores pasivos de la red no ven el contenido descifrado de los prompts o respuestas.
- No hay un servidor central obligatorio en la ruta de datos.
- El límite de confianza actual entre seed/worker es explícito.

**Lo que Forge no garantiza hoy:**
- Que el seed no pueda leer el prompt o la respuesta.
- Que la inferencia dividida oculte el texto plano de todos los proveedores de cómputo remoto.
- Que la inferencia remota incorrecta se detecte automáticamente.

**Lo que Forge pretende garantizar más adelante:**
- Los pares de etapas intermedias no reciben prompts en texto plano.
- Los tensores de activación se cifran en tránsito entre las etapas del pipeline.
- La visibilidad del prompt se reduce al conjunto mínimo de nodos de frontera confiables.

Esas garantías posteriores dependen de enviar primero la inferencia dividida real. Hasta entonces, Forge debe describirse como inferencia remota cifrada con un límite de confianza honesto.

## Amenazas Económicas

### T10: Falsificación de CU

**Amenaza**: Un nodo reclama CU que no ganó fabricando TradeRecords.

**Mitigación actual**: El libro local con integridad HMAC-SHA256 evita la manipulación a nivel de archivo. Sin embargo, el operador del nodo todavía puede escribir intercambios arbitrarios en su propio libro.

**Mitigación objetivo**: Protocolo de firma dual. Cada TradeRecord debe ser firmado tanto por el proveedor como por el consumidor. Un nodo no puede acreditarse CU a sí mismo sin la firma de una contraparte. La sincronización por gossip significa que otros nodos pueden verificar ambas firmas de forma independiente.

**Riesgo residual**: Colusión entre el proveedor y el consumidor para crear intercambios falsos. Esto es económicamente irracional: el consumidor coludido no gana nada. La detección de anomalías estadísticas en el volumen y la frecuencia de los intercambios puede señalar patrones sospechosos.

### T11: Abuso del Nivel Gratuito (Sybil)

**Amenaza**: Un atacante crea muchos NodeIds para explotar repetidamente el nivel gratuito de 1,000 CU.

**Mitigación actual**: Si existen más de 100 nodos desconocidos (contribuido = 0, consumido > 0) en el libro, se rechazan las nuevas solicitudes de nivel gratuito. Cada NodeId es un par de claves Ed25519 — barato de crear pero rastreable.

**Mitigación objetivo**: Prueba de Trabajo (PoW) en el registro del nodo (pequeño coste computacional para crear una nueva identidad), o entrada basada en participación (stake) (los nuevos nodos deben contribuir al cómputo antes de consumir).

### T12: Divergencia del Libro

**Amenaza**: Diferentes nodos tienen vistas incompatibles de los mismos intercambios, lo que lleva a la inconsistencia económica.

**Mitigación actual**: Cada nodo mantiene su propia vista local. No hay garantía de consistencia entre nodos.

**Mitigación objetivo**: TradeRecords firmados dualmente y sincronizados por gossip. Ambas partes producen registros firmados idénticos. Cualquier nodo que reciba una actualización por gossip puede verificar las firmas y rechazar inconsistencias. El anclaje periódico de resúmenes a Bitcoin (OP_RETURN) proporciona un rastro de auditoría inmutable opcional.

### T13: Manipulación del Mercado

**Amenaza**: Un nodo infla artificialmente los factores de oferta o demanda para manipular los precios.

**Mitigación actual**: El precio de mercado se calcula localmente a partir de las propias observaciones de cada nodo. Ningún nodo puede obligar a otro nodo a adoptar su precio.

**Mitigación objetivo**: Señales de precios basadas en gossip ponderadas por reputación. Las observaciones de los nodos de alta reputación tienen más peso. Los nodos nuevos o de baja reputación no pueden influir significativamente en los precios de toda la red.

### T14: Ataque a la Calidad de la Inferencia

**Amenaza**: Un proveedor devuelve una inferencia de baja calidad o truncada para ganar CU sin realizar el cálculo completo.

**Mitigación actual**: Aceptar el riesgo. Para la mayoría de los casos de uso, las salidas obviamente incorrectas son detectables por el consumidor.

**Mitigación objetivo**: Verificación de calidad por parte del consumidor. El consumidor puede volver a ejecutar una pequeña muestra de tokens localmente para verificar que la salida del proveedor sea consistente. Penalización de reputación para los proveedores cuyas salidas fallen las comprobaciones puntuales.
