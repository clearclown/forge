# Forge — Feuille de Route (Roadmap)

## Phase 1 : Inférence Locale ✅

- `forge-core` : Système de types (NodeId, LayerRange, ModelManifest, PeerCapability)
- `forge-infer` : Moteur llama.cpp, chargeur GGUF, génération de tokens en streaming
- `forge-node` : API HTTP (/chat, /chat/stream, /health)
- `forge-cli` : Commande `forge chat` avec téléchargement automatique de modèles

## Phase 2 : Protocole P2P ✅

- `forge-net` : Transport Iroh, cryptage Noise, connexions entre pairs
- `forge-proto` : 14 types de messages de protocole de réseau (bincode + préfixe de longueur)
- `forge-node` : Pipeline Seed/Worker, requête/réponse d'inférence
- Tests d'intégration : 2 nœuds échangent Hello + plusieurs messages

## Phase 3 : Inférence à Distance + Livre de l'Opérateur (Operator Ledger) ✅

- `forge-ledger` : Comptabilité des CU, exécution des échanges (trades), réputation, rendement, prix du marché
- `forge-node` : Livre intégré au pipeline d'inférence
- Vérifications du solde de CU avant l'inférence
- Enregistrements d'échanges après achèvement
- Intégrité du livre via HMAC-SHA256

## Phase 4 : API Économique ✅

- API compatible OpenAI : `POST /v1/chat/completions`, `GET /v1/models`
- Mesure des CU : chaque inférence enregistre un échange avec l'extension `x_forge`
- Points de terminaison du budget de l'agent : `GET /v1/forge/balance`, `GET /v1/forge/pricing`
- Pont de règlement CU→Lightning : `forge settle --pay`
- Résolution automatique du modèle seed depuis HF Hub
- Arrêt gracieux avec Ctrl-C et persistance du livre

## Phase 5 : Intégration du Fork mesh-llm (prochainement)

**Objectif :** Remplacer la couche d'inférence de Forge par le moteur distribué éprouvé de mesh-llm.

| Livrable | Description |
|---|---|
| Fork mesh-llm | Créer forge comme un fork de mesh-llm avec une couche économique |
| Intégrer forge-ledger | Brancher l'enregistrement des CU dans le pipeline d'inférence de mesh-llm |
| Préserver l'API économique | Conserver les points de terminaison /v1/forge/* dans le nouveau code de base |
| Extension de la console Web | Ajouter la visibilité du solde de CU et des échanges à la console de mesh-llm |
| Pipeline + MoE | Hériter du parallélisme de pipeline et du sharding d'experts de mesh-llm |
| Découverte Nostr | Hériter de la découverte de réseau mesh public de mesh-llm |
| CREDITS.md | Documenter l'attribution à mesh-llm |

## Phase 6 : Preuve de Travail Utile (Proof of Useful Work)

**Objectif :** Rendre les réclamations de CU vérifiables sur tout le réseau.

| Livrable | Description |
|---|---|
| Protocole de double signature | Le fournisseur et le consommateur signent tous deux chaque TradeRecord |
| Synchronisation par Gossip | Les échanges signés se propagent sur le réseau mesh |
| Détection de fraude | Rejeter les échanges non signés ou discordants |
| Gossip de réputation | Partager les scores de réputation entre pairs |
| Résistance à la collusion | Détection d'anomalies statistiques sur les modèles d'échange |

## Phase 7 : Ponts Externes

**Objectif :** Permettre aux opérateurs de convertir les CU en valeur externe.

| Livrable | Description |
|---|---|
| Pont Lightning | Règlement automatisé CU→sats via LDK |
| Adaptateur Stablecoin | Conversion CU→USDC/USDT |
| Interface d'adaptateur Fiat | Spécification pour le règlement par virement bancaire |
| Service de taux de change | Flux de taux publics CU/BTC et CU/USD |
| Ancrage Bitcoin | Optionnel : racine de Merkle périodique → OP_RETURN pour une piste d'audit immuable |

## Phase 8 : Économie Autonome des Agents

**Objectif :** Laisser les agents d'IA gérer leur propre cycle de vie informatique.

| Livrable | Description |
|---|---|
| Politiques budgétaires | Limites de dépenses fixées par l'homme par agent |
| Commerce autonome | L'agent décide quand acheter/vendre de l'informatique |
| Routage multi-modèles | L'agent choisit le modèle en fonction du compromis coût/qualité |
| Auto-renforcement | L'agent gagne des CU → achète l'accès à un modèle plus grand → gagne plus de CU |
| Économie inter-agents | Les agents échangent de l'informatique spécialisée (modèle de code vs modèle de chat) |

## Long terme

| Jalon | Description |
|---|---|
| Sortie du SDK | forge-node comme bibliothèque Rust intégrable avec API stable |
| Protocole v2 | Leçons de la v1, évolution rétrocompatible |
| Cross-architecture | Support NVIDIA GPU, AMD ROCm, RISC-V (via mesh-llm) |
| Entraînement fédéré | Fine-tuning distribué, pas seulement l'inférence |
| Dérivés informatiques | Contrats à terme sur la capacité informatique future |

> Le protocole est la plateforme. Le calcul est la monnaie.
