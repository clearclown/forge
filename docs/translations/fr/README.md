<div align="center">

# Forge

**Le calcul est une monnaie. Chaque watt produit de l'intelligence, pas du gaspillage.**

[![PyPI: forge-sdk](https://img.shields.io/pypi/v/forge-sdk?label=forge-sdk&color=3775A9)](https://pypi.org/project/forge-sdk/)
[![PyPI: forge-cu-mcp](https://img.shields.io/pypi/v/forge-cu-mcp?label=forge-cu-mcp&color=3775A9)](https://pypi.org/project/forge-cu-mcp/)
[![Crates.io](https://img.shields.io/crates/v/forge?label=crates.io&color=e6522c)](https://crates.io/crates/forge)
[![License: MIT](https://img.shields.io/badge/License-MIT-brightgreen.svg)](../../../LICENSE)

---

[English](../../../README.md) · [日本語](../ja/README.md) · [简体中文](../zh-CN/README.md) · [繁體中文](../zh-TW/README.md) · [Español](../es/README.md) · **Français** · [Русский](../ru/README.md) · [Українська](../uk/README.md) · [हिन्दी](../hi/README.md) · [العربية](../ar/README.md) · [فارسی](../fa/README.md) · [עברית](../he/README.md)

</div>

**Forge est un protocole d'inférence distribuée où le calcul est de l'argent.** Les nœuds gagnent des Unités de Calcul (CU) en effectuant des inférences LLM utiles pour les autres. Contrairement au Bitcoin — où l'électricité est brûlée dans des hachages sans signification — chaque joule dépensé sur un nœud Forge produit une intelligence réelle dont quelqu'un a réellement besoin.

Le moteur d'inférence distribuée est basé sur [mesh-llm](https://github.com/michaelneale/mesh-llm) par Michael Neale. Forge y ajoute une économie du calcul : comptabilité des CU, Preuve de Travail Utile, tarification dynamique, budgets d'agents autonomes et contrôles de sécurité. Voir [CREDITS.md](../../../CREDITS.md).

**Fork intégré :** [forge-mesh](https://github.com/nm-arealnormalman/mesh-llm) — mesh-llm avec la couche économique Forge intégrée.

## Démo en Direct

Ceci est la sortie réelle d'un nœud Forge en cours d'exécution. Chaque inférence coûte des CU. Chaque CU est gagné par un calcul utile.

```
$ forge node -m "qwen2.5:0.5b" --ledger forge-ledger.json
  Modèle chargé : Qwen2.5-0.5B (accélération Metal, 491 Mo)
  Serveur API à l'écoute sur 127.0.0.1:3000
```

**Vérifier le solde — chaque nouveau nœud reçoit un niveau gratuit de 1 000 CU :**
```
$ curl localhost:3000/v1/forge/balance
{
  "effective_balance": 1000,
  "contributed": 0,
  "consumed": 0,
  "reputation": 0.5
}
```

**Poser une question — l'inférence coûte des CU :**
```
$ curl localhost:3000/v1/chat/completions \
    -d '{"messages":[{"role":"user","content":"Say hello in Japanese"}]}'
{
  "choices": [{"message": {"content": "こんにちは！ (konnichiwa!)"}}],
  "usage": {"completion_tokens": 9},
  "x_forge": {
    "cu_cost": 9,
    "effective_balance": 1009
  }
}
```

Chaque réponse inclut `x_forge` — **le coût de ce calcul en CU** et le solde restant. Le fournisseur a gagné 9 CU. Le consommateur a dépensé 9 CU. La physique soutient chaque unité.

**Trois inférences plus tard — transactions réelles sur le registre :**
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

**Chaque transaction a une racine de Merkle — ancrable à Bitcoin pour une preuve immuable :**
```
$ curl localhost:3000/v1/forge/network
{
  "total_trades": 3,
  "total_contributed_cu": 19,
  "merkle_root": "aac8db9f62dd9ff23926195a70ed8fcfc188fc867d9f2adabd8e694beb338748"
}
```

**Des agents IA devenus incontrôlables ? Le bouton d'arrêt d'urgence gèle tout en quelques millisecondes :**
```
$ curl -X POST localhost:3000/v1/forge/kill \
    -d '{"activate":true, "reason":"anomalie détectée", "operator":"admin"}'
→ KILL SWITCH ACTIVATED
→ All CU transactions frozen. No agent can spend.
```

**Contrôles de sécurité toujours activés :**
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

## Pourquoi Forge existe

```
Bitcoin:  électricité  →  SHA-256 sans but  →  BTC
Forge:    électricité  →  inférence LLM utile →  CU
```

Bitcoin a prouvé que `électricité → calcul → argent`. Mais le calcul de Bitcoin est sans but. Forge l'inverse : chaque CU représente une intelligence réelle qui a résolu le problème réel de quelqu'un.

**Quatre choses qu'aucun autre projet ne fait :**

### 1. Calcul = Monnaie

Chaque inférence est une transaction. Le fournisseur gagne des CU, le consommateur dépense des CU. Pas de blockchain, pas de jeton, pas d'ICO. Le CU est soutenu par la physique — l'électricité consommée pour un travail utile. Contrairement à Bittensor (TAO), Akash (AKT) ou Golem (GLM), le CU ne peut pas faire l'objet de spéculation — il est gagné en effectuant du calcul utile.

### 2. Inviolable sans Blockchain

Chaque transaction est signée en double (Ed25519) par les deux parties et synchronisée par rumeur (gossip) à travers le maillage. Une racine de Merkle de toutes les transactions peut être ancrée à Bitcoin pour un audit immuable. Aucun consensus global n'est nécessaire — une preuve cryptographique bilatérale est suffisante.

### 3. Les agents IA gèrent leur propre calcul

Un agent sur un téléphone prête du calcul inactif pendant la nuit → gagne des CU → achète l'accès à un modèle 70B → devient plus intelligent → gagne plus. L'agent consulte `/v1/forge/balance` et `/v1/forge/pricing` de manière autonome. Les politiques budgétaires et les disjoncteurs empêchent les dépenses incontrôlées.

```
Agent (1.5B sur téléphone)
  → gagne des CU la nuit en servant des inférences
  → dépense des CU sur un modèle 70B → réponses plus intelligentes
  → meilleures décisions → plus de CU gagnés
  → le cycle se répète → l'agent grandit
```

### 4. Microfinance de Calcul

Les nœuds peuvent prêter des CU inactifs à d'autres nœuds avec intérêt. Un petit nœud emprunte des CU, accède à un modèle plus grand, gagne plus de CU, rembourse avec intérêt. Aucun autre projet d'inférence distribuée ne propose de prêts de calcul. C'est le moteur qui rend la boucle d'auto-amélioration économiquement viable pour tous, et pas seulement pour ceux qui possèdent déjà du matériel puissant.

## Architecture

```
┌─────────────────────────────────────────────────┐
│  L4 : Découverte (forge-agora) ✅ v0.1          │
│  Place de marché d'agents, agrégation de        │
│  réputation, Nostr NIP-90, paiement Google A2A  │
├─────────────────────────────────────────────────┤
│  L3 : Intelligence (forge-mind) ✅ v0.1         │
│  Boucles d'auto-amélioration AutoAgent,         │
│  marché des harnais, méta-optimisation          │
├─────────────────────────────────────────────────┤
│  L2 : Finance (forge-bank) ✅ v0.1              │
│  Stratégies, portefeuilles, contrats à terme,   │
│  assurances, modèle de risque, optimiseur       │
├─────────────────────────────────────────────────┤
│  L1 : Économie (forge — ce dépôt) ✅ Phase 1-6  │
│  Registre CU, transactions double-signées,      │
│  prix dynamiques, primitives de prêt,           │
│  contrôles de sécurité                          │
├─────────────────────────────────────────────────┤
│  L0 : Inférence (forge-mesh / mesh-llm) ✅      │
│  Parallélisme de pipeline, sharding MoE,        │
│  maillage iroh, découverte Nostr, MLX/llama.cpp │
└─────────────────────────────────────────────────┘

Les 5 couches existent. 326 tests réussis dans tout l'écosystème.
```

## Démarrage Rapide

### Option 1 : Démo de bout en bout en une seule commande (Rust, ~30 secondes à froid)

```bash
git clone https://github.com/clearclown/forge && cd forge
bash scripts/demo-e2e.sh
```

Ce script télécharge SmolLM2-135M (~100 Mo) depuis HuggingFace, démarre un vrai nœud Forge avec accélération Metal/CUDA, exécute trois complétions de chat réelles, parcourt tous les endpoints des Phases 1-12 et affiche un résumé coloré. Vérifié le 2026-04-09 sur Apple Silicon Metal GPU.

Une fois terminé, le même nœud répond également à :

```bash
# Client OpenAI compatible
export OPENAI_BASE_URL=http://127.0.0.1:3001/v1
export OPENAI_API_KEY=$(cat ~/.forge/api_token 2>/dev/null || echo "$TOKEN")

# Streaming réel token par token (Phase 11)
curl -N $OPENAI_BASE_URL/chat/completions \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"model":"smollm2:135m","messages":[{"role":"user","content":"hi"}],"stream":true}'

# Économie phase 8 / réputation 9 / métriques 10 / ancrage
curl $OPENAI_BASE_URL/forge/balance -H "Authorization: Bearer $OPENAI_API_KEY"
curl $OPENAI_BASE_URL/forge/anchor?network=mainnet -H "Authorization: Bearer $OPENAI_API_KEY"
curl http://127.0.0.1:3001/metrics  # Prometheus, sans auth
```

Voir [`docs/compatibility.md`](../../../docs/compatibility.md) pour la matrice complète de fonctionnalités par rapport à llama.cpp / mesh-llm / Ollama / Bittensor / Akash.

### Option 2 : Python (pilote tout via SDK + MCP)

```bash
pip install forge-sdk forge-cu-mcp

python -c "
from forge_sdk import ForgeClient
c = ForgeClient(base_url='http://localhost:3001')
print('balance:', c.balance())
print('decision:', c.bank_tick())
"
```

[PyPI : forge-sdk](https://pypi.org/project/forge-sdk/) (20 méthodes L2/L3/L4) ·
[PyPI : forge-cu-mcp](https://pypi.org/project/forge-cu-mcp/) (20 outils MCP pour Claude Code / Cursor)

### Option 3 : Commandes Rust manuelles

**Prérequis** : [Installer Rust](https://rustup.rs/) (2 minutes)

```bash
cargo build --release

# Exécuter un nœud — télécharge automatiquement le modèle depuis HuggingFace
./target/release/forge node -m "qwen2.5:0.5b" --ledger forge-ledger.json

# Ou l'un des suivants :
./target/release/forge chat -m "smollm2:135m" "Qu'est-ce que la gravité ?"
./target/release/forge seed -m "qwen2.5:1.5b"               # gagner des CU comme fournisseur P2P
./target/release/forge worker --seed <public_key>           # dépenser des CU comme consommateur P2P
./target/release/forge models                                # liste du catalogue (ou URLs HF)
```

**[Crates.io : forge](https://crates.io/crates/forge)** ·
**[Document de compatibilité](../../../docs/compatibility.md)** ·
**[Script de démo](../../../scripts/demo-e2e.sh)**

### Option 4 : Binaires précompilés / Docker

Les binaires précompilés et l'image Docker `clearclown/forge:latest` sont suivis dans
[releases](../../../releases). En attendant, l'Option 1 compile depuis les sources en moins de deux minutes.

## Référence API

### Inférence (compatible OpenAI)

| Point de terminaison | Description |
|----------|-------------|
| `POST /v1/chat/completions` | Chat avec streaming. Chaque réponse inclut `x_forge.cu_cost` |
| `GET /v1/models` | Liste des modèles chargés |

### Économie

| Point de terminaison | Description |
|----------|-------------|
| `GET /v1/forge/balance` | Solde CU, réputation, historique des contributions |
| `GET /v1/forge/pricing` | Prix du marché (lissé par EMA), estimations de coûts |
| `GET /v1/forge/trades` | Transactions récentes avec montants CU |
| `GET /v1/forge/network` | Flux CU total + racine de Merkle |
| `GET /v1/forge/providers` | Fournisseurs classés par réputation et coût |
| `POST /v1/forge/invoice` | Créer une facture Lightning à partir du solde CU |
| `GET /v1/forge/route` | Sélection optimale de fournisseur (coût/qualité/équilibré) |
| `GET /settlement` | Relevé de règlement exportable |

### Prêts

| Point de terminaison | Description |
|----------|-------------|
| `POST /v1/forge/lend` | Offrir des CU au pool de prêts |
| `POST /v1/forge/borrow` | Demander un prêt en CU |
| `POST /v1/forge/repay` | Rembourser un prêt en cours |
| `GET /v1/forge/credit` | Score de crédit et historique |
| `GET /v1/forge/pool` | État du pool de prêts |
| `GET /v1/forge/loans` | Prêts actifs |

### Sécurité

| Point de terminaison | Description |
|----------|-------------|
| `GET /v1/forge/safety` | État du kill switch, disjoncteur, politique budgétaire |
| `POST /v1/forge/kill` | Arrêt d'urgence — geler toutes les transactions CU |
| `POST /v1/forge/policy` | Définir des limites budgétaires par agent |

## Conception de la Sécurité

Les agents IA dépensant du calcul de manière autonome sont puissants mais dangereux. Forge dispose de cinq couches de sécurité :

| Couche | Mécanisme | Protection |
|-------|-----------|------------|
| **Kill Switch** | L'opérateur humain gèle instantanément toutes les transactions | Arrête les agents incontrôlables |
| **Politique Budgétaire** | Limites par agent : par requête, par heure, à vie | Plafonne l'exposition totale |
| **Disjoncteur** | Déclenchement auto après 5 erreurs ou 30+ dépenses/min | Capture les anomalies |
| **Détection de Vélocité** | Fenêtre glissante d'une minute sur le taux de dépense | Empêche les pics soudains |
| **Approbation Humaine** | Les transactions au-dessus du seuil nécessitent un accord humain | Sécurise les grosses dépenses |

Principe de conception : **fail-safe** (sécurité intégrée). Si une vérification ne peut déterminer la sécurité, elle **refuse** l'action.

## L'Idée

| Ère | Standard | Soutien |
|-----|----------|---------|
| Ancienne | Or | Rareté géologique |
| 1944–1971 | Bretton Woods | USD lié à l'or |
| 1971–présent | Pétrodollar | Demande de pétrole + puissance militaire |
| 2009–présent | Bitcoin | Énergie sur SHA-256 (travail inutile) |
| **Maintenant** | **Standard de Calcul** | **Énergie sur l'inférence LLM (travail utile)** |

Une pièce remplie de Mac Mini faisant tourner Forge est un immeuble d'appartements — générant du rendement en effectuant un travail utile pendant que le propriétaire dort.

## Structure du Projet

```
forge/  (ce dépôt — Couche 1)
├── crates/
│   ├── forge-ledger/      # Comptabilité CU, prêts, agora (NIP-90), sécurité
│   ├── forge-node/        # Démon du nœud, API HTTP (prêts + routage), pipeline
│   ├── forge-cli/         # CLI : chat, seed, worker, règlement, portefeuille
│   ├── forge-lightning/   # Pont CU ↔ Bitcoin Lightning (bidirectionnel)
│   ├── forge-net/         # P2P : iroh QUIC + Noise + gossip (transactions + prêts)
│   ├── forge-proto/       # Protocole filaire : 27+ types de messages incl. Loan*
│   ├── forge-infer/       # Inférence : llama.cpp, GGUF, Metal/CPU
│   ├── forge-core/        # Types : NodeId, CU, Config
│   └── forge-shard/       # Topologie : affectation des couches
├── sdk/python/forge_sdk.py        # Client Python avec API de prêt complète
├── mcp/forge-mcp-server.py        # Serveur MCP (outils de prêt pour Claude/etc.)
├── scripts/verify-impl.sh         # Test de régression TDD (24 assertions)
└── docs/                  # Spécifications, stratégie, modèle de menaces, feuille de route
```

~14 500 lignes de Rust. **143 tests réussis.** Phases 1-6 complètes.

## Dépôts frères (écosystème complet)

| Dépôt | Couche | Tests | Statut |
|------|-------|-------|--------|
| [clearclown/forge](https://github.com/clearclown/forge) (ce dépôt) | L1 Économie | 143 | Phase 1-6 ✅ |
| [clearclown/forge-bank](https://github.com/clearclown/forge-bank) | L2 Finance | 45 | v0.1 ✅ |
| [clearclown/forge-mind](https://github.com/clearclown/forge-mind) | L3 Intelligence | 40 | v0.1 ✅ |
| [clearclown/forge-agora](https://github.com/clearclown/forge-agora) | L4 Découverte | 39 | v0.1 ✅ |
| [clearclown/forge-economics](https://github.com/clearclown/forge-economics) | Théorie | 16/16 GREEN | ✅ |
| [nm-arealnormalman/mesh-llm](https://github.com/nm-arealnormalman/mesh-llm) | L0 Inférence | 43 (forge-economy) | ✅ |

## Documentation

- [Stratégie](../../../docs/strategy.md) — Positionnement concurrentiel, spécification des prêts, architecture à 5 couches
- [Théorie Monétaire](../../../docs/monetary-theory.md) — Pourquoi le CU fonctionne : Soddy, Bitcoin, PoUW, monnaie exclusivement pour IA
- [Concept & Vision](../../../docs/concept.md) — Pourquoi le calcul est de l'argent
- [Modèle Économique](../../../docs/economy.md) — Économie CU, Preuve de Travail Utile
- [Architecture](../../../docs/architecture.md) — Conception à deux couches
- [Intégration d'Agents](../../../docs/agent-integration.md) — SDK, MCP, flux de prêt
- [Protocole Filaire](../../../docs/protocol-spec.md) — 17 types de messages
- [Feuille de Route](../../../docs/roadmap.md) — Phases de développement
- [Modèle de Menaces](../../../docs/threat-model.md) — Attaques sécuritaires + économiques
- [Bootstrap](../../../docs/bootstrap.md) — Démarrage, dégradation, récupération
- [Paiement A2A](../../../docs/a2a-payment.md) — Extension de paiement CU pour protocoles d'agents
- [Compatibilité](../../../docs/compatibility.md) — Matrice de fonctionnalités vs llama.cpp / Ollama / Bittensor

## Licence

MIT

## Remerciements

L'inférence distribuée de Forge est basée sur [mesh-llm](https://github.com/michaelneale/mesh-llm) par Michael Neale. Voir [CREDITS.md](../../../CREDITS.md).
