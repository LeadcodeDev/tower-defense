# Tower Defense avec Ratatui

## Structure du projet

Ce projet implémente un jeu de Tower Defense avec une interface utilisateur texte (TUI) en utilisant Ratatui.

## Intégration de Ratatui - Instructions

L'intégration de Ratatui dans notre jeu Tower Defense est en cours, mais nécessite encore quelques ajustements. Voici les étapes à suivre pour terminer l'implémentation :

### 1. Structure actuelle

Le code est organisé selon un modèle MVC (Modèle-Vue-Contrôleur) :
- **Modèle** : Les entités du domaine (Game, Tower, Monster, etc.)
- **Vue** : L'interface utilisateur Ratatui dans `infrastructure/ui`
- **Contrôleur** : La boucle principale et la gestion des événements

### 2. Problèmes à résoudre

Plusieurs erreurs de compilation doivent être résolues :

1. **Corriger l'implémentation du trait Tower** :
   - Il existe un conflit entre l'ancienne et la nouvelle définition du trait Tower
   - Pour chaque classe implémentant Tower, corriger la méthode `position` pour retourner une `Position` au lieu de `&Position`
   - Importer `TargetSelection` dans chaque fichier qui l'utilise

2. **Corriger les champs manquants dans TowerBase** :
   - Mettre à jour tous les constructeurs de `TowerBase` pour utiliser la nouvelle structure
   - Ajouter les champs `element`, `aoe` et `behavior` à TowerBase ou adapter le code existant
 
3. **Corriger Map::new dans forest_map()** :
   - Ajouter les paramètres manquants ou adapter la signature
   - Convertir les chaînes en String où nécessaire

### 3. Composants UI déjà implémentés

Les composants suivants sont déjà implémentés pour l'interface utilisateur :

- **App** : Gestion de l'état global de l'application
- **Event** : Gestion des événements utilisateur
- **Tui** : Configuration du terminal
- **UI** : Rendu des différentes vues (jeu, menu, pause, game over)

### 4. Exécution

Une fois les erreurs de compilation résolues, vous pourrez exécuter le jeu avec l'interface Ratatui :

```bash
cargo run
```

### 5. Architecture UI

L'architecture UI suit les meilleures pratiques de Ratatui :

- **Boucle événementielle** : Capture des entrées utilisateur et tick régulier
- **Rendu basé sur l'état** : Affichage différent selon l'état du jeu
- **Gestion propre du terminal** : Configuration et nettoyage corrects

## Avantages de cette intégration

- Séparation claire des responsabilités
- Interface utilisateur riche en console
- Structure extensible pour ajouter de nouvelles fonctionnalités
- Rendu optimisé avec mises à jour efficaces 