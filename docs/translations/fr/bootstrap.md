# Forge — Séquence de Démarrage (Bootstrap)

## Présentation

Forge a deux chemins de démarrage :

- le **flux de référence actuel**, qui est explicite et dirigé par l'opérateur
- le **flux cible**, où un nœud basé sur mesh-llm rejoint un maillage et commence à gagner des CU automatiquement

## Flux de Référence Actuel

```text
1. Démarrer un hôte de modèle : forge seed -m "qwen2.5:0.5b"
2. Copier la clé publique affichée
3. Connecter un demandeur : forge worker --seed <seed_public_key>
4. Vérifier l'état : forge status --url http://127.0.0.1:3000
5. Vérifier le solde CU : curl http://127.0.0.1:3000/v1/forge/balance
```

L'API HTTP s'écoute sur `127.0.0.1` par défaut. Si elle est exposée, définissez `--api-token`.

## Démarrage Cible (fork mesh-llm)

Une fois que Forge sera intégré à mesh-llm :

```text
1. forge --auto                          # rejoindre le meilleur maillage public
2. forge --model Qwen2.5-32B --publish   # créer un maillage public, gagner des CU
3. forge --join <token>                  # rejoindre avec un GPU, gagner des CU
4. forge --client --join <token>         # rejoindre en tant que consommateur, dépenser des CU
```

Chaque inférence servie gagne des CU. Chaque inférence consommée dépense des CU. La couche économique est automatique — aucune configuration séparée n'est nécessaire.

## Démarrage Économique

### Nouveau Nœud (Solde Zéro)

```text
1. Le nœud rejoint le maillage
2. Niveau gratuit : 1 000 CU disponibles immédiatement
3. Le nœud sert sa première requête d'inférence → gagne des CU
4. Le solde CU augmente avec chaque requête servie
5. Le nœud peut maintenant dépenser des CU pour l'inférence d'autres nœuds
```

### Nœud Existant (A un Solde)

```text
1. Le nœud redémarre, charge le registre persistant (forge-ledger.json)
2. Intégrité HMAC-SHA256 vérifiée
3. Ancien solde, transactions et réputation restaurés
4. Le nœud reprend ses gains et ses dépenses de CU
```

## Dégradation & Récupération

| Événement | Impact Économique | Impact Inférence |
|---|---|---|
| 1 nœud distant se déconnecte | Les nœuds restants absorbent le travail, le flux CU continue | Brève pause, modèle rééquilibré |
| Tous les nœuds distants se déconnectent | L'économie CU est en pause, mode local uniquement | Retour au petit modèle local |
| Batterie faible du nœud (<20%) | Arrêt du service (les gains sont en pause), peut toujours consommer | Décharger les couches à distance |
| Le nœud retrouve le réseau | Reprise des gains de CU, la réputation se rétablit | Redécouverte des pairs, ré-expansion |

**Invariant clé** : Le solde CU d'un nœud persiste après les redémarrages et les déconnexions. Les CU gagnés ne sont jamais perdus.

## Modèle de Contribution des Nœuds

- **Contributeurs** : Les appareils servant l'inférence gagnent des CU
- **Consommateurs** : Les appareils demandant l'inférence dépensent des CU
- **Solde** : Plus de contribution → plus de CU → plus d'accès au calcul des autres
- **Niveau gratuit** : 1 000 CU pour les nouveaux nœuds, consommés dès la première requête
- **Rendement (Yield)** : Les nœuds en ligne gagnent 0,1 % de rendement par heure (pondéré par la réputation)
- **Pas de paiement obligatoire** : Le protocole fonctionne aux CU. Les ponts externes (Lightning, fiat) sont optionnels.

## Sécurité pendant le Démarrage

- Identité Ed25519 créée avant toute activité réseau
- Toutes les connexions chiffrées via QUIC + Noise
- Dans le flux actuel seed/worker, la graine (seed) voit le texte de l'invite (limite de confiance explicite)
- Les transactions CU sont enregistrées localement avec une intégrité HMAC-SHA256
- Cible : transactions double-signées synchronisées par rumeur (gossip) à travers le maillage
