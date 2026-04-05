# Forge — Modèle de Menaces (Threat Model)

## Objectifs de Sécurité

1. **Confidentialité en transit** : les observateurs passifs et les relais ne doivent pas pouvoir lire les prompts ou les réponses.
2. **Pairs authentifiés** : chaque connexion directe doit se lier à une identité de nœud cryptographique.
3. **Confiance limitée** : le système doit expliciter quels nœuds voient le texte brut et lesquels ne voient que l'état intermédiaire.
4. **Disponibilité** : les défaillances d'un seul pair doivent dégrader le service plutôt que de le corrompre silencieusement.

## Chiffrement

### Couche de Transport
- Toutes les connexions utilisent QUIC avec TLS 1.3 (via Iroh).
- Handshake supplémentaire du protocole Noise (modèle XX) pour l'authentification des pairs.
- Résultat : chiffrement symétrique ChaCha20-Poly1305 avec secret vers l'avant (forward secrecy).
- Clés éphémères par session — la compromission d'une session n'affecte pas les autres.

### Identité
- Chaque nœud possède une paire de clés Ed25519 persistante.
- Générée au premier lancement, stockée dans le trousseau de clés de la plateforme.
- Node ID = hash de la clé publique.
- Pas d'autorité de certification centrale — modèle de réseau de confiance (web-of-trust).

### Ce qui est Chiffré
| Données | Chiffré | Notes |
|---|---|---|
| Texte du prompt | Oui | Chiffré en transit ; visible par le seed dans le flux de référence actuel |
| Sortie texte en streaming | Oui | Chiffré en transit ; visible par le seed qui l'a générée |
| Tenseurs d'activation | Prévu | Pertinent une fois que l'inférence fractionnée sera active dans le runtime |
| Messages de contrôle | Oui | Tous les messages du protocole au sein de QUIC |
| Capacités des pairs | Oui | Échangées via un canal chiffré |

## Analyse des Menaces

### T1 : Nœud Seed Malveillant (flux de référence actuel)
**Menace** : L'opérateur du seed lit le prompt ou la réponse car le worker envoie le texte du prompt au seed et le seed exécute le modèle complet.

**État actuel** : Il s'agit d'une limite de confiance explicite, et non d'une propriété de sécurité résolue.

**Atténuation** :
- ne connectez les workers qu'à des seeds auxquels vous faites confiance avec des prompts en texte brut.
- maintenez le transport chiffré afin que les relais et les observateurs passifs ne puissent pas lire le contenu.
- passer à l'inférence fractionnée (split inference) pour que les pairs des étapes intermédiaires ne reçoivent pas de prompts en texte brut.

### T2 : Nœud de Pipeline Malveillant (cible du flux d'inférence fractionnée)
**Menace** : Un nœud dans le futur pipeline tente d'extraire le prompt ou la réponse à partir des activations intermédiaires.

**Cible d'atténuation** : Un nœud à l'étape k du pipeline ne devrait voir que le tenseur d'activation de sortie de la couche k-1 et produire le tenseur d'activation pour la couche k. Il ne doit pas recevoir le texte original du prompt.

**Risque résiduel** : Les tenseurs d'activation peuvent laisser échapper des informations sur l'entrée. La confidentialité différentielle, la redondance et l'attestation restent des travaux futurs.

### T3 : Attaque Sybil
**Menace** : Un attaquant crée de nombreux faux nœuds pour dominer le pipeline.

**Atténuation** :
- Système de réputation basé sur le comportement observé (temps de fonctionnement, calcul correct).
- Les nouveaux nœuds commencent avec une faible réputation et des positions de pipeline limitées.
- Les couches critiques (première et dernière) sont affectées de préférence aux nœuds de haute réputation.
- Limitation du débit sur les nouvelles adhésions de nœuds à partir de la même plage d'IP.

### T4 : Inférence Byzantine
**Menace** : Un nœud malveillant renvoie des tenseurs d'activation incorrects.

**Atténuation (MVP)** : Accepter le risque. Pour la plupart des cas d'utilisation, un résultat d'inférence subtilement erroné est détectable par l'utilisateur.

**Atténuation (futur)** :
- Calcul redondant sur les couches critiques (2 nœuds calculent les mêmes couches, comparaison).
- Calcul vérifiable utilisant l'attestation TEE (Apple Silicon Secure Enclave).
- Détection d'anomalies statistiques sur les distributions des tenseurs d'activation.

### T5 : Analyse du Trafic
**Menace** : L'observateur surveille les modèles de trafic chiffré pour en déduire l'utilisation.

**Atténuation** :
- QUIC multiplexe toutes les communications sur une seule connexion.
- Le trafic actuel seed/worker laisse toujours fuiter des métadonnées grossières sur le timing des requêtes et la longueur des réponses.
- Remplissage (padding) sur les messages de contrôle à une taille constante (optionnel, pas dans le MVP).

### T6 : Compromission du Serveur Relais
**Menace** : Les serveurs relais de bootstrap sont compromis.

**Impact** : Minimal. Les serveurs relais ne font que faciliter l'établissement de la connexion. Ils voient :
- Quels Node IDs se connectent (métadonnées).
- Les paquets QUIC chiffrés (ne peuvent pas déchiffrer).
- Ils ne voient PAS les prompts ou réponses déchiffrés.

**Atténuation** : Plusieurs opérateurs de relais indépendants. Le réseau continue sans relais une fois que la DHT est peuplée.

### T7 : Empoisonnement du Modèle
**Menace** : Un nœud sert un modèle GGUF modifié avec des poids piégés (backdoors).

**Atténuation** :
- Fichiers de modèles vérifiés par hash SHA-256 par rapport à des manifestes connus pour être bons.
- Manifestes de modèles distribués via DHT avec signatures des éditeurs de modèles.
- Les nœuds ne chargent que des modèles provenant de sources vérifiées (hashes HuggingFace).

### T8 : Déni de Service
**Menace** : Des nœuds rejoignent puis ne répondent plus, perturbant l'inférence.

**Atténuation** :
- Le délai d'expiration du heartbeat et le rééquilibrage sont des propriétés cibles du runtime, pas des garanties complètes de l'implémentation actuelle.
- le repli local est un objectif de conception pour les futurs clients d'inférence fractionnée.
- Pénalité de réputation pour les nœuds qui se déconnectent fréquemment.
- La dégradation gracieuse est un principe de conception fondamental.
- les requêtes d'inférence entrantes sont limitées par la validation du runtime et une limite d'exécution simultanée fixe sur le seed.
- les valeurs `msg_id` de protocole en double provenant du même pair sont rejetées dans une fenêtre de rejeu limitée.

### T9 : Exposition de l'API Administrative
**Menace** : Un opérateur lie l'API HTTP locale à une interface publique sans protection, exposant `/status`, `/topology`, `/settlement` ou `/chat`.

**Atténuation dans l'implémentation actuelle** :
- le démon lie l'API HTTP à `127.0.0.1` par défaut.
- les opérateurs peuvent toujours l'exposer intentionnellement avec `--bind 0.0.0.0`.
- les routes administratives exposées peuvent être protégées avec un jeton porteur via `--api-token`.
- les corps de requête JSON sont limités en taille avant la désérialisation pour réduire l'abus d'allocation sur `/chat`.

**Risque résiduel** : L'authentification par jeton porteur est un contrôle de l'opérateur, pas un TLS mutuel. Si le jeton fuite, l'API doit être traitée comme compromise jusqu'à sa rotation.

## Hiérarchie de Confiance

```
Le plus de confiance : Votre propre appareil (téléphone, ordinateur portable)
                     ↓
Confiance :          Vos propres appareils sur le LAN (Mac Mini à la maison)
                     ↓
Semi-confiance :     Pairs WAN à haute réputation (mois de fonctionnement)
                     ↓
Pas de confiance :   Nouveaux pairs WAN (nouvellement arrivés, pas d'historique)
```

L'affectation des couches devrait suivre cette hiérarchie une fois que l'inférence fractionnée existera :
- Premières et dernières couches (les plus sensibles — voient les embeddings d'entrée et les logits de sortie) → vos propres appareils.
- Couches intermédiaires (ne voient que les activations intermédiaires) → peuvent être affectées à des pairs semi-confiants ou non confiants.

## Garanties de Confidentialité

**Ce que Forge garantit aujourd'hui :**
- les prompts et les réponses sont chiffrés en transit entre les pairs directement connectés.
- les relais et les observateurs passifs du réseau ne voient pas le contenu déchiffré des prompts ou des réponses.
- il n'y a pas de serveur central obligatoire dans le chemin des données.
- la limite de confiance actuelle seed/worker est explicite.

**Ce que Forge ne garantit pas aujourd'hui :**
- que le seed ne puisse pas lire le prompt ou la réponse.
- que l'inférence fractionnée cache le texte brut à tous les fournisseurs de calcul à distance.
- que l'inférence à distance incorrecte soit détectée automatiquement.

**Ce que Forge vise à garantir plus tard :**
- les pairs des étapes intermédiaires ne reçoivent pas de prompts en texte brut.
- les tenseurs d'activation sont chiffrés en transit entre les étapes du pipeline.
- la visibilité du prompt est réduite à l'ensemble minimal de nœuds frontières de confiance.

Ces garanties ultérieures dépendent de l'expédition de l'inférence fractionnée réelle en premier. D'ici là, Forge doit être décrit comme une inférence à distance chiffrée avec une limite de confiance honnête.

## Menaces Économiques

### T10 : Contrefaçon de CU

**Menace** : Un nœud réclame des CU qu'il n'a pas gagnées en fabriquant des TradeRecords.

**Atténuation actuelle** : Le livre local avec intégrité HMAC-SHA256 empêche l'altération au niveau du fichier. Cependant, l'opérateur du nœud peut toujours écrire des échanges arbitraires dans son propre livre.

**Atténuation cible** : Protocole de double signature. Chaque TradeRecord doit être signé par le fournisseur et le consommateur. Un nœud ne peut pas s'attribuer des CU sans la signature d'une contrepartie. La synchronisation par gossip signifie que d'autres nœuds peuvent vérifier les deux signatures indépendamment.

**Risque résiduel** : Collusion entre fournisseur et consommateur pour créer de faux échanges. C'est économiquement irrationnel — le consommateur de connivence ne gagne rien. La détection d'anomalies statistiques sur le volume et la fréquence des échanges peut signaler des modèles suspects.

### T11 : Abus du Niveau Gratuit (Sybil)

**Menace** : Un attaquant crée de nombreux NodeIds pour exploiter de manière répétée le niveau gratuit de 1 000 CU.

**Atténuation actuelle** : Si plus de 100 nœuds inconnus (contribué = 0, consommé > 0) existent dans le livre, les nouvelles requêtes de niveau gratuit sont rejetées. Chaque NodeId est une paire de clés Ed25519 — bon marché à créer mais traçable.

**Atténuation cible** : Preuve de Travail (PoW) sur l'enregistrement du nœud (petit coût informatique pour créer une nouvelle identité), ou entrée basée sur la mise (stake) (les nouveaux nœuds doivent contribuer au calcul avant de consommer).

### T12 : Divergence du Livre

**Menace** : Différents nœuds ont des vues incompatibles des mêmes échanges, ce qui conduit à une incohérence économique.

**Atténuation actuelle** : Chaque nœud maintient sa propre vue locale. Aucune garantie de cohérence entre les nœuds.

**Atténuation cible :** TradeRecords à double signature synchronisés par gossip. Les deux parties produisent des enregistrements signés identiques. Tout nœud recevant une mise à jour de gossip peut vérifier les signatures et rejeter les incohérences. L'ancrage périodique du résumé à Bitcoin (OP_RETURN) fournit une piste d'audit immuable en option.

### T13 : Manipulation du Marché

**Menace** : Un nœud gonfle artificiellement les facteurs de demande ou d'offre pour manipuler les prix.

**Atténuation actuelle** : Le prix du marché est calculé localement à partir des propres observations de chaque nœud. Aucun nœud ne peut forcer un autre nœud à adopter son prix.

**Atténuation cible** : Signaux de prix basés sur le gossip pondérés par la réputation. Les observations des nœuds à haute réputation ont plus de poids. Les nouveaux nœuds ou ceux à faible réputation ne peuvent pas influencer de manière significative les prix à l'échelle du réseau.

### T14 : Attaque sur la Qualité de l'Inférence

**Menace** : Un fournisseur renvoie une inférence de mauvaise qualité ou tronquée pour gagner des CU sans effectuer le calcul complet.

**Atténuation actuelle** : Accepter le risque. Pour la plupart des cas d'utilisation, les sorties manifestement erronées sont détectables par le consommateur.

**Atténuation cible** : Vérification de la qualité côté consommateur. Le consommateur peut réexécuter un petit échantillon de tokens localement pour vérifier que la sortie du fournisseur est cohérente. Pénalité de réputation pour les fournisseurs dont les sorties échouent aux contrôles ponctuels.
