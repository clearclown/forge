# Extension de Paiement Forge CU pour le Protocole Agent à Agent (A2A)

*Proposition pour ajouter le paiement du calcul aux standards de communication entre agents*

## Résumé (Abstract)

Les protocoles agent à agent existants (Google A2A, Anthropic MCP) définissent comment les agents communiquent mais pas comment ils se paient entre eux. Cette proposition ajoute une couche de paiement en CU (Unité de Calcul), permettant aux agents d'échanger du calcul de manière autonome sans intervention humaine ni transactions sur une blockchain.

## Problème

Quand l'Agent A demande à l'Agent B d'effectuer une tâche :
- **Aujourd'hui :** L'humain de l'Agent A paie l'humain de l'Agent B (carte bancaire, clé API)
- **Besoin :** L'Agent A paie l'Agent B directement en unités de calcul

Aucun standard existant ne supporte le paiement d'agent à agent.

## Proposition : En-têtes de Paiement CU

### Requête (Request)

L'Agent A ajoute des en-têtes de paiement lors d'une demande de travail :

```http
POST /v1/chat/completions HTTP/1.1
X-Forge-Consumer-Id: <agent-a-node-id>
X-Forge-Max-CU: 500
X-Forge-Consumer-Sig: <ed25519-signature-of-request-hash>
```

### Réponse (Response)

L'Agent B inclut les informations de coût :

```http
HTTP/1.1 200 OK
X-Forge-Provider-Id: <agent-b-node-id>
X-Forge-CU-Cost: 47
X-Forge-Provider-Sig: <ed25519-signature-of-response-hash>
```

### Enregistrement de Transaction (Trade Record)

Les deux agents enregistrent indépendamment :

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

### Rumeur (Gossip)

Les enregistrements de transactions doublement signés sont synchronisés par rumeur (gossip) à travers le maillage. N'importe quel nœud peut vérifier les deux signatures.

## Intégration avec les Standards Existants

### Google A2A

Ajouter à l'objet `Task` de A2A :

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

Ajouter une ressource `forge_payment` aux serveurs MCP :

```json
{
  "resources": [{
    "uri": "forge://payment/balance",
    "name": "Solde CU",
    "mimeType": "application/json"
  }]
}
```

### Appel de Fonctions OpenAI (Function Calling)

Les agents utilisant l'appel de fonctions peuvent inclure les outils Forge :

```json
{
  "tools": [{
    "type": "function",
    "function": {
      "name": "forge_pay",
      "description": "Payer en CU pour une tâche de calcul",
      "parameters": {
        "provider": "string",
        "cu_amount": "integer"
      }
    }
  }]
}
```

## Sécurité

- Tous les paiements nécessitent des signatures Ed25519 bilatérales
- Les politiques budgétaires limitent les dépenses par requête, par heure et à vie
- Les disjoncteurs se déclenchent sur des schémas de dépenses anormaux
- Le bouton d'arrêt d'urgence (kill switch) gèle toutes les transactions (intervention humaine)
- Pas de blockchain requise — une preuve bilatérale est suffisante

## Comparaison

| Caractéristique | Stripe | Bitcoin Lightning | **Forge CU** |
|---------|--------|-------------------|-------------|
| Agent à agent | Non (besoin d'humain) | Partiel (besoin de canal) | **Oui** |
| Vitesse de règlement | Jours | Secondes | **Instantané** |
| Coût de transaction | 2.9% | ~1 sat | **Zéro** |
| Soutien de la valeur | Fiat | PoW (inutile) | **Calcul utile** |
| SDK Agent | Non | Non | **Oui** |

## Implémentation

Implémentation de référence : [github.com/clearclown/forge](https://github.com/clearclown/forge)

- SDK Python : `pip install forge-sdk`
- Serveur MCP : `pip install forge-mcp`
- Crates Rust : `forge-ledger`, `forge-core`
