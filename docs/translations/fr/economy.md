# Forge — Modèle Économique

## Standard de Calcul (Compute Standard — 計算本位制)

Chaque système monétaire repose sur la rareté :

| Ère | Standard | Soutien |
|-----|----------|---------|
| Ancienne | Or/Argent | Rareté géologique |
| 1944–1971 | Bretton Woods | USD lié à l'or |
| 1971–présent | Pétrodollar | Demande de pétrole + puissance militaire |
| 2009–présent | Bitcoin | Électricité brûlée sur SHA-256 |
| **Forge** | **Standard de Calcul** | **Électricité dépensée pour une inférence utile** |

Forge introduit un Standard de Calcul : l'unité de valeur est soutenue par une dépense énergétique réelle effectuant un calcul utile. Contrairement à la Preuve de Travail (Proof of Work) du Bitcoin, chaque joule dépensé dans Forge produit une intelligence réelle.

## CU : La Monnaie Native

### Qu'est-ce que le CU ?

**1 CU = 1 milliard de FLOPs de travail d'inférence vérifié.**

Le CU n'est pas une crypto-monnaie. Ce n'est pas un jeton sur une blockchain. C'est une unité de compte qui représente un calcul réel effectué. Le CU a de la valeur car il constitue un droit sur un calcul futur — si vous avez gagné des CU en servant une inférence, vous pouvez les dépenser pour recevoir une inférence.

### Pourquoi le CU, et non le Bitcoin ?

| Propriété | CU | Bitcoin |
|----------|-----|---------|
| **Soutien de valeur** | Calcul utile (intrinsèque) | Calcul de hachage (artificiel) |
| **Vitesse de règlement** | Instantanée (registre local) | Secondes à minutes (Lightning/chaîne) |
| **Coût de transaction** | Zéro | Frais de canal, frais on-chain |
| **Dépendance externe** | Aucune | Santé du réseau Bitcoin |
| **Risque quantique** | Aucun (pas de puzzle crypto) | SHA-256 / ECDSA vulnérables |
| **Génération de rendement**| Oui (matériel inactif gagne CU) | Non (le BTC dans le portefeuille ne gagne rien) |

Le CU est l'unité de règlement **principale**. Le Bitcoin, les stablecoins et les devises fiduciaires sont des **ponts de sortie** optionnels disponibles via des adaptateurs en dehors du protocole.

### Le CU comme Actif Productif

```
Immeuble d'appartements        Mac Mini sur Forge
───────────────────────        ──────────────────
Actif : bâtiment               Actif : matériel informatique
Coût : entretien               Coût : électricité
Revenu : loyer                 Revenu : CU provenant de l'inférence
Rendement : loyer - entretien  Rendement : CU gagnés - coût électricité
Inoccupé = perte de revenu     Inactif = potentiel gaspillé
```

Un appareil informatique sur Forge n'est pas comme du Bitcoin dans un portefeuille (valeur statique, pas de rendement). Il est comme une propriété locative — générant des revenus grâce à un travail utile.

## Modèle de Transaction

### Exécution des Échanges (Trades)

Chaque inférence crée un échange entre deux parties :

```rust
pub struct TradeRecord {
    pub provider: NodeId,       // Qui a exécuté l'inférence
    pub consumer: NodeId,       // Qui l'a demandée
    pub cu_amount: u64,         // CU transférés
    pub tokens_processed: u64,  // Travail effectué
    pub timestamp: u64,
    pub model_id: String,
}
```

L'échange est enregistré par les deux parties. Dans l'implémentation actuelle, chaque nœud maintient un registre local. L'implémentation cible ajoute des signatures doubles et une synchronisation par rumeur (gossip).

### Tarification Dynamique

Les prix des CU flottent en fonction de l'offre et de la demande locales :

```
prix_effectif = cu_base_par_token × facteur_demande / facteur_offre
```

- **Plus de nœuds inactifs** → facteur_offre augmente → le prix baisse
- **Plus de requêtes d'inférence** → facteur_demande augmente → le prix monte
- Chaque nœud observe ses propres conditions de marché. Pas de carnet d'ordres global.

### Niveau Gratuit

Les nouveaux nœuds sans historique de contribution reçoivent 1 000 CU. Cela permet à quiconque d'utiliser le réseau immédiatement. Le niveau gratuit est consommé dès la première requête — il ne se réinitialise pas.

Atténuation Sybil : si plus de 100 nœuds inconnus sont apparus sans contribuer, les nouvelles requêtes de niveau gratuit sont rejetées.

## Preuve de Travail Utile (Proof of Useful Work)

### Le Concept

Preuve de Travail du Bitcoin : "J'ai brûlé de l'électricité en calculant des hachages SHA-256. Voici le nonce qui le prouve."

Preuve de Travail Utile de Forge : "J'ai brûlé de l'électricité en exécutant une inférence LLM. Voici la réponse, et voici la signature du consommateur confirmant qu'il l'a reçue."

La différence clé : la preuve de Bitcoin est auto-générée (n'importe quel mineur peut produire un hachage valide). La preuve de Forge nécessite une **contrepartie** — quelqu'un qui voulait réellement l'inférence. On ne peut pas falsifier la demande.

### Protocole de Vérification (cible)

```
1. Le consommateur envoie une InferenceRequest au fournisseur
2. Le fournisseur exécute l'inférence, renvoie les tokens en streaming
3. Le consommateur reçoit les tokens, calcule le hachage de la réponse
4. Les deux parties signent le TradeRecord :
   - Le fournisseur signe : "J'ai calculé ceci"
   - Le consommateur signe : "J'ai reçu ceci"
5. Le TradeRecord double-signé est synchronisé par gossip sur le réseau
6. Tout nœud peut vérifier les deux signatures
```

Un nœud ne peut pas gonfler son solde CU sans une contrepartie coopérante. La collusion est possible mais économiquement irrationnelle — le consommateur de connivence ne gagne rien en signant de faux échanges.

### Implémentation Actuelle

L'implémentation de référence actuelle utilise des registres locaux avec une protection d'intégrité HMAC-SHA256. Les signatures doubles et le gossip sont l'étape suivante.

## Rendement et Réputation

### Rendement

Les nœuds qui restent en ligne et contribuent au calcul gagnent un rendement :

```
rendement_cu = cu_contribues × 0,001 × reputation × heures_activite
```

À une réputation de 1,0, un nœud ayant contribué à hauteur de 10 000 CU gagne 80 CU par nuit de 8 heures. Il ne s'agit pas d'inflation — c'est une récompense pour la disponibilité. Les nœuds qui sont connectés de manière fiable sont plus précieux pour le réseau.

### Réputation

Chaque nœud a un score de réputation compris entre 0,0 et 1,0 :

- Les nouveaux nœuds commencent à 0,5
- Le temps d'activité et les échanges réussis augmentent la réputation
- Les déconnexions et les échanges échoués diminuent la réputation
- Une réputation plus élevée → taux de rendement plus élevé, priorité dans l'ordonnancement

## Règlement et Ponts Externes

### Règle Fondamentale

**Le protocole se règle en CU.** La conversion vers toute autre chose est une question d'intégration.

### Relevés de Règlement

Les opérateurs peuvent exporter des historiques d'échanges auditables pour n'importe quelle fenêtre temporelle :

```
forge settle --hours 24 --price 0.05 --out settlement.json
```

Le relevé comprend : les CU bruts gagnés, les CU bruts dépensés, les CU nets, le nombre d'échanges et un prix de référence optionnel par CU.

### Architecture des Ponts

```
Couche 0 : Protocole Forge
  → Comptabilité CU, échanges, tarification

Couche 1 : Relevé de règlement
  → Historique d'échanges exportable
  → Taux de change de référence

Couche 2 : Pont externe (optionnel)
  → CU ↔ BTC (Lightning)
  → CU ↔ stablecoin
  → CU ↔ fiduciaire
```

La couche de pont est en dehors du protocole. Différents opérateurs peuvent utiliser différents ponts. Le protocole reste utile avec une liquidité externe nulle.

### Pont Lightning

Pour les opérateurs qui souhaitent un règlement en Bitcoin :

```bash
forge settle --hours 24 --pay
```

Cela crée une facture Lightning BOLT11 pour les CU nets gagnés, convertis au taux de change configuré (par défaut : 10 msats par CU).

## Budgets Dirigés par les Agents

### La Vision

Traditionnel : L'humain décide → L'humain paie → L'IA s'exécute
Forge : La politique autorise l'agent → L'agent vérifie le budget → L'agent dépense des CU → L'agent s'exécute

### API

```
GET /v1/forge/balance   → Solde CU, contribution, consommation, réputation
GET /v1/forge/pricing   → Prix du marché, estimations de coûts par 100/1000 tokens
```

Un agent peut :
1. Vérifier son solde avant de faire une requête
2. Estimer le coût de l'inférence aux prix actuels du marché
3. Décider si la requête en vaut le coût en CU
4. Exécuter et payer automatiquement

Les superviseurs humains définissent les politiques budgétaires. Les agents opèrent de manière autonome à l'intérieur de ces limites.

### Boucle d'Auto-Renforcement

```
Agent (petit, sur téléphone)
  → gagne des CU en prêtant du calcul inactif
  → dépense des CU pour l'accès à un modèle plus grand
  → devient plus intelligent
  → prend de meilleures décisions économiques
  → gagne plus de CU
  → accède à des modèles encore plus grands
  → ...
```

C'est un modèle d'application possible. Le protocole fournit le marché ; les agents fournissent la stratégie.

## Pourquoi ce n'est pas du Web3

La plupart des projets Web3 créent une rareté artificielle (jetons) par-dessus des biens numériques abondants. Forge fait le contraire :

- **Le calcul est réellement rare** — il nécessite de l'électricité réelle, du silicium réel, du temps réel
- **Le CU n'est pas spéculatif** — il représente un travail vérifié, pas un pari sur l'adoption future
- **Pas d'ICO, pas de vente de jetons, pas de jeton de gouvernance** — le CU est gagné en travaillant
- **Pas de blockchain requise** — les signatures bilatérales et le gossip sont suffisants
- **Pas de contrats intelligents (smart contracts)** — le protocole est le contrat

La valeur est fabriquée par la physique, pas par le consensus.
