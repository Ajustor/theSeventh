# Project Architecture / Architecture du Projet

## English

### Project File Organization

This document describes the organization of the source code files in this Bevy game project. Files have been organized by their functional purpose to improve maintainability and code discoverability.

### Directory Structure

```
src/
├── main.rs              # Application entry point
├── core/                # Core game systems
│   ├── camera.rs        # Camera system and viewport management
│   └── game_flow.rs     # Game initialization, level management, restart logic
├── entities/            # Game entities
│   ├── player.rs        # Player character, controls, and movement
│   ├── enemy.rs         # Enemy entities and patrol AI
│   └── misc_objects.rs  # Miscellaneous objects (chests, pumpkins, etc.)
├── physics/             # Physics and collision systems
│   ├── colliders.rs     # Rapier collider configurations for entities
│   ├── ground_detection.rs  # Ground detection sensors and logic
│   ├── climbing.rs      # Climbing mechanics and ladder detection
│   └── walls.rs         # Optimized wall collision generation
└── systems/             # Game systems
    └── inventory.rs     # Inventory management system
```

### Module Categories

#### 1. **Core (`src/core/`)**
Contains fundamental game systems that manage the game's flow and presentation:
- **camera.rs**: Manages camera positioning, viewport scaling, and following the player within level bounds
- **game_flow.rs**: Handles game initialization (setup), level selection, level transitions, and restart functionality

#### 2. **Entities (`src/entities/`)**
Contains all entity definitions and their behaviors:
- **player.rs**: Player entity bundle, movement controls (WASD + Space), and player plugin
- **enemy.rs**: Enemy (Mob) entities with patrol path following behavior
- **misc_objects.rs**: Static and dynamic objects like chests and pumpkins

#### 3. **Physics (`src/physics/`)**
Contains all physics-related systems using Bevy Rapier2D:
- **colliders.rs**: Defines collider configurations for different entity types (player, enemies, chests)
- **ground_detection.rs**: Ground detection sensors that enable jumping mechanics
- **climbing.rs**: Ladder detection and climbing mechanics (disables gravity when climbing)
- **walls.rs**: Optimized algorithm to generate minimal wall colliders from tile data

#### 4. **Systems (`src/systems/`)**
Contains gameplay systems:
- **inventory.rs**: Player inventory management and debug printing

### Design Principles

This organization follows these principles:
- **Separation of Concerns**: Each module has a clear, single responsibility
- **Logical Grouping**: Related functionality is kept together
- **Scalability**: Easy to add new entities, systems, or physics components
- **Discoverability**: Clear naming and structure make it easy to find specific code

---

## Français

### Organisation des Fichiers du Projet

Ce document décrit l'organisation des fichiers de code source dans ce projet de jeu Bevy. Les fichiers ont été organisés selon leur objectif fonctionnel pour améliorer la maintenabilité et la découvrabilité du code.

### Structure des Répertoires

```
src/
├── main.rs              # Point d'entrée de l'application
├── core/                # Systèmes de jeu principaux
│   ├── camera.rs        # Système de caméra et gestion du viewport
│   └── game_flow.rs     # Initialisation du jeu, gestion des niveaux, logique de redémarrage
├── entities/            # Entités du jeu
│   ├── player.rs        # Personnage joueur, contrôles et déplacement
│   ├── enemy.rs         # Entités ennemies et IA de patrouille
│   └── misc_objects.rs  # Objets divers (coffres, citrouilles, etc.)
├── physics/             # Systèmes de physique et collision
│   ├── colliders.rs     # Configurations des colliders Rapier pour les entités
│   ├── ground_detection.rs  # Capteurs et logique de détection du sol
│   ├── climbing.rs      # Mécanique d'escalade et détection des échelles
│   └── walls.rs         # Génération optimisée des collisions de murs
└── systems/             # Systèmes de jeu
    └── inventory.rs     # Système de gestion d'inventaire
```

### Catégories de Modules

#### 1. **Core (`src/core/`)**
Contient les systèmes de jeu fondamentaux qui gèrent le flux et la présentation du jeu :
- **camera.rs** : Gère le positionnement de la caméra, la mise à l'échelle du viewport et le suivi du joueur dans les limites du niveau
- **game_flow.rs** : Gère l'initialisation du jeu (setup), la sélection de niveau, les transitions de niveau et la fonctionnalité de redémarrage

#### 2. **Entities (`src/entities/`)**
Contient toutes les définitions d'entités et leurs comportements :
- **player.rs** : Bundle d'entité joueur, contrôles de mouvement (WASD + Espace) et plugin joueur
- **enemy.rs** : Entités ennemies (Mob) avec comportement de suivi de chemin de patrouille
- **misc_objects.rs** : Objets statiques et dynamiques comme les coffres et les citrouilles

#### 3. **Physics (`src/physics/`)**
Contient tous les systèmes liés à la physique utilisant Bevy Rapier2D :
- **colliders.rs** : Définit les configurations de colliders pour différents types d'entités (joueur, ennemis, coffres)
- **ground_detection.rs** : Capteurs de détection du sol qui permettent les mécaniques de saut
- **climbing.rs** : Détection d'échelle et mécanique d'escalade (désactive la gravité pendant l'escalade)
- **walls.rs** : Algorithme optimisé pour générer des colliders de murs minimaux à partir des données de tuiles

#### 4. **Systems (`src/systems/`)**
Contient les systèmes de gameplay :
- **inventory.rs** : Gestion de l'inventaire du joueur et affichage de débogage

### Principes de Conception

Cette organisation suit ces principes :
- **Séparation des Préoccupations** : Chaque module a une responsabilité claire et unique
- **Regroupement Logique** : Les fonctionnalités liées sont conservées ensemble
- **Évolutivité** : Facile d'ajouter de nouvelles entités, systèmes ou composants physiques
- **Découvrabilité** : Une nomenclature et une structure claires facilitent la recherche de code spécifique

### Avantages de cette Organisation

1. **Clarté** : Chaque développeur peut rapidement comprendre où se trouve le code
2. **Maintenance** : Les modifications sont plus faciles car le code connexe est regroupé
3. **Collaboration** : Plusieurs développeurs peuvent travailler sur différents modules sans conflit
4. **Tests** : Plus facile de tester des modules individuels de manière isolée
5. **Extension** : Ajout facile de nouvelles fonctionnalités dans la catégorie appropriée
