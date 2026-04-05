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
    -d '{"messages":[{"role":"user","content":"Dis bonjour en japonais"}]}'
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

**Des agents IA devenus incontrôlables ? Le bouton d'arrêt d'urgence (kill switch) gèle tout en quelques millisecondes :**
```
$ curl -X POST localhost:3000/v1/forge/kill \
    -d '{"activate":true, "reason":"anomalie détectée", "operator":"admin"}'
→ BOUTON D'ARRÊT D'URGENCE ACTIVÉ
→ Toutes les transactions CU sont gelées. Aucun agent ne peut dépenser.
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

**Trois choses qu'aucun autre projet ne fait :**

### 1. Calcul = Monnaie

Chaque inférence est une transaction. Le fournisseur gagne des CU, le consommateur dépense des CU. Pas de blockchain, pas de jeton, pas d'ICO. Le CU est soutenu par la physique — l'électricité consommée pour un travail utile.

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

## Architecture

```
┌─────────────────────────────────────────────────┐
│  Couche d'inférence (mesh-llm)                  │
│  Parallélisme de pipeline, sharding MoE,        │
│  réseau iroh, découverte Nostr, API OpenAI      │
└──────────────────┬──────────────────────────────┘
                   │
┌──────────────────▼──────────────────────────────┐
│  Couche économique (Forge)                       │
│  Registre CU, transactions double-signées,      │
│  gossip, prix dynamiques, racine Merkle,        │
│  contrôles de sécurité                          │
└──────────────────┬──────────────────────────────┘
                   │
┌──────────────────▼──────────────────────────────┐
│  Couche de sécurité                             │
│  Kill switch, politiques budgétaires,           │
│  disjoncteurs, détection de vélocité,           │
│  seuils d'approbation humaine                   │
└──────────────────┬──────────────────────────────┘
                   │ optionnel
┌──────────────────▼──────────────────────────────┐
│  Ponts externes                                 │
│  CU ↔ BTC (Lightning), CU ↔ stablecoin          │
└─────────────────────────────────────────────────┘
```

## Démarrage rapide

### Option 1 : Python (le plus rapide)

```bash
pip install forge-sdk
```

```python
from forge_sdk import ForgeNode

node = ForgeNode(model="qwen2.5:0.5b")
response = node.chat("Qu'est-ce que la gravité ?")
print(f"Coût : {response.cu_cost} CU")
```

> [PyPI : forge-sdk](https://pypi.org/project/forge-sdk/) · [PyPI : forge-cu-mcp](https://pypi.org/project/forge-cu-mcp/)

### Option 2 : Rust (contrôle total)

> **Prérequis** : [Installer Rust](https://rustup.rs/) (2 minutes)

```bash
# Compiler depuis les sources
cargo build --release

# Exécuter un nœud avec un modèle téléchargé automatiquement
./target/release/forged node -m "qwen2.5:0.5b" --ledger forge-ledger.json

# Discuter localement
./target/release/forge chat -m "qwen2.5:0.5b" "Qu'est-ce que la gravité ?"

# Démarrer un seed (P2P, gagne des CU)
./target/release/forge seed -m "qwen2.5:0.5b" --ledger forge-ledger.json

# Se connecter en tant que worker (P2P, dépense des CU)
./target/release/forge worker --seed <public_key>

# Lister les modèles
./target/release/forge models
```

> [Crates.io : forge](https://crates.io/crates/forge) · [Guide d'installation de Rust](https://rustup.rs/)

### Option 3 : Binaires précompilés

Les binaires précompilés arrivent bientôt. Voir les [releases](../../../releases).

### Option 4 : Docker

```bash
# Bientôt disponible
docker run -p 3000:3000 clearclown/forge:latest
```

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
| `GET /settlement` | Relevé de règlement exportable |

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
forge/
├── crates/
│   ├── forge-ledger/      # Comptabilité CU, transactions, prix, sécurité, racine Merkle
│   ├── forge-node/        # Démon du nœud, API HTTP, coordinateur de pipeline
│   ├── forge-cli/         # CLI : chat, seed, worker, règlement, portefeuille
│   ├── forge-lightning/   # Pont CU ↔ Bitcoin Lightning
│   ├── forge-net/         # P2P : iroh QUIC + Noise + gossip
│   ├── forge-proto/       # Protocole filaire : 17 types de messages
│   ├── forge-infer/       # Inférence : llama.cpp, GGUF, Metal/CPU
│   ├── forge-core/        # Types : NodeId, CU, Config
│   └── forge-shard/       # Topologie : affectation des couches
└── docs/                  # Spécifications, modèle de menaces, feuille de route
```

~10 000 lignes de Rust. 76 tests. 2 audits de sécurité terminés.

## Documentation

- [Concept & Vision](concept.md) — Pourquoi le calcul est de l'argent
- [Modèle Économique](economy.md) — Économie CU, Preuve de Travail Utile
- [Architecture](architecture.md) — Conception à deux couches
- [Protocole Filaire](protocol-spec.md) — 17 types de messages
- [Feuille de Route](roadmap.md) — Phases de développement
- [Modèle de Menaces](threat-model.md) — Attaques sécuritaires + économiques
- [Bootstrap](bootstrap.md) — Démarrage, dégradation, récupération

## Licence

MIT

## Remerciements

L'inférence distribuée de Forge est basée sur [mesh-llm](https://github.com/michaelneale/mesh-llm) par Michael Neale. Voir [CREDITS.md](../../../CREDITS.md).
