# Forge — Guide d'Intégration des Agents

## Pour les Développeurs d'Agents IA

Forge donne à votre agent un budget de calcul. L'agent peut gagner des CU en servant des inférences et dépenser des CU pour accéder à des modèles plus grands. Pas de carte de crédit, pas de clé API, pas d'humain dans la boucle.

## Intégration Rapide

### N'importe quel Client HTTP

```python
import requests

FORGE = "http://127.0.0.1:3000"

# Vérifier si l'agent peut se permettre une requête
balance = requests.get(f"{FORGE}/v1/forge/balance").json()
if balance["effective_balance"] > 100:
    # Exécuter l'inférence (coûte des CU)
    r = requests.post(f"{FORGE}/v1/chat/completions", json={
        "messages": [{"role": "user", "content": "Qu'est-ce que la gravité ?"}],
        "max_tokens": 256
    }).json()
    print(r["choices"][0]["message"]["content"])
    print(f"Coût : {r['x_forge']['cu_cost']} CU")
```

### SDK Python

```python
from forge_sdk import ForgeClient, ForgeAgent

# Client simple
forge = ForgeClient()
result = forge.chat("Explique l'informatique quantique")
print(f"Réponse : {result['content']}")
print(f"Coût : {result['cu_cost']} CU, Solde : {result['balance']} CU")

# Agent autonome avec gestion budgétaire
agent = ForgeAgent(max_cu_per_task=500)
while agent.has_budget():
    result = agent.think("Que dois-je faire ensuite ?")
    if result is None:
        break  # budget épuisé
```

### MCP (Claude Code, Cursor)

Ajouter à vos paramètres MCP :
```json
{
  "mcpServers": {
    "forge": {
      "command": "python",
      "args": ["chemin/vers/forge/mcp/forge-mcp-server.py"]
    }
  }
}
```

L'assistant IA peut alors utiliser des outils comme `forge_balance`, `forge_pricing`, `forge_inference`.

### LangChain

```python
from langchain_openai import ChatOpenAI

llm = ChatOpenAI(
    base_url="http://127.0.0.1:3000/v1",
    api_key="pas-besoin",
    model="qwen2.5-0.5b-instruct-q4_k_m"
)
response = llm.invoke("Bonjour")
# métadonnées x_forge disponibles dans les en-têtes de réponse
```

### curl

```bash
# Vérifier le solde
curl localhost:3000/v1/forge/balance

# Exécuter l'inférence
curl localhost:3000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{"messages":[{"role":"user","content":"bonjour"}]}'

# Vérifier le coût
curl localhost:3000/v1/forge/trades
```

## Boucle Économique de l'Agent

Le modèle recommandé pour un agent autonome :

```python
from forge_sdk import ForgeClient

forge = ForgeClient()

def agent_loop():
    while True:
        # 1. Vérifier le budget
        balance = forge.balance()
        if balance["effective_balance"] < 50:
            print("Solde CU bas. En attente d'en gagner plus...")
            time.sleep(60)
            continue

        # 2. Vérifier les prix
        pricing = forge.pricing()
        cost_per_100 = pricing["estimated_cost_100_tokens"]

        # 3. Décider si la tâche vaut le coût
        if cost_per_100 > 500:
            print("Prix du marché trop élevé. En attente...")
            time.sleep(30)
            continue

        # 4. Exécuter
        result = forge.chat("Analyser ces données...", max_tokens=200)
        print(f"Terminé. Coût : {result['cu_cost']} CU")

        # 5. Vérifier la sécurité
        safety = forge.safety()
        if safety["circuit_tripped"]:
            print("Disjoncteur déclenché. Pause...")
            time.sleep(300)
```

## Sécurité pour les Développeurs d'Agents

### Définir des Politiques Budgétaires

```bash
# Limiter un agent à 1 000 CU par heure
curl -X POST localhost:3000/v1/forge/policy \
  -H "Content-Type: application/json" \
  -d '{
    "node_id": "<agent_node_id>",
    "max_cu_per_hour": 1000,
    "max_cu_per_request": 100,
    "human_approval_threshold": 500
  }'
```

### Surveiller la Vélocité des Dépenses

```bash
curl localhost:3000/v1/forge/safety
# Retourne : hourly_spend, lifetime_spend, spends_last_minute
```

### Arrêt d'Urgence

```bash
# Tout geler
curl -X POST localhost:3000/v1/forge/kill \
  -d '{"activate": true, "reason": "anomalie agent"}'
```

## Référence API pour les Agents

| Besoins de l'agent | Point de terminaison | Méthode |
|-----------------|----------|--------|
| "Combien de CU ai-je ?" | `/v1/forge/balance` | GET |
| "Combien cela va-t-il coûter ?" | `/v1/forge/pricing` | GET |
| "Qui est le fournisseur le moins cher ?" | `/v1/forge/providers` | GET |
| "Exécuter l'inférence" | `/v1/chat/completions` | POST |
| "Qu'ai-je dépensé ?" | `/v1/forge/trades` | GET |
| "Suis-je en sécurité ?" | `/v1/forge/safety` | GET |
| "Retirer en Bitcoin" | `/v1/forge/invoice` | POST |
| "TOUT ARRÊTER" | `/v1/forge/kill` | POST |
