# Module Dependencies / Dépendances des Modules

## English

### Module Dependency Graph

This diagram shows how different modules depend on each other in the project.

```
main.rs
  │
  ├──> core::game_flow (GameFlowPlugin)
  │      └──> entities::player::Player
  │
  ├──> core::camera (camera_fit_inside_current_level)
  │      └──> entities::player::Player
  │
  ├──> physics::walls (WallPlugin)
  │
  ├──> physics::ground_detection (GroundDetectionPlugin)
  │
  ├──> physics::climbing (ClimbingPlugin)
  │      └──> physics::colliders::SensorBundle
  │
  ├──> entities::player (PlayerPlugin)
  │      ├──> physics::climbing::Climber
  │      ├──> physics::colliders::ColliderBundle
  │      ├──> physics::ground_detection::GroundDetection
  │      └──> systems::inventory::Inventory
  │
  ├──> entities::enemy (EnemyPlugin)
  │      └──> physics::colliders::ColliderBundle
  │
  ├──> entities::misc_objects (MiscObjectsPlugin)
  │      └──> physics::colliders::ColliderBundle
  │
  └──> systems::inventory (dbg_print_inventory)
         └──> entities::player::Player
```

### Dependency Layers

The project is organized in dependency layers from low-level to high-level:

**Layer 1 - Foundation (No internal dependencies)**
- `physics::colliders` - Defines collision shapes for entities
- `physics::walls` - Wall collision generation (depends only on external libs)

**Layer 2 - Core Physics Systems**
- `physics::ground_detection` - Ground detection (no internal deps)
- `physics::climbing` - Climbing mechanics (depends on `physics::colliders`)

**Layer 3 - Game Systems**
- `systems::inventory` - Inventory management

**Layer 4 - Entities**
- `entities::player` - Player (depends on physics and systems modules)
- `entities::enemy` - Enemies (depends on `physics::colliders`)
- `entities::misc_objects` - Objects (depends on `physics::colliders`)

**Layer 5 - Core Game Logic**
- `core::game_flow` - Game flow (depends on `entities::player`)
- `core::camera` - Camera system (depends on `entities::player`)

**Layer 6 - Application**
- `main.rs` - Application entry point (depends on all modules)

### Import Guidelines

To maintain a clean architecture:

1. **Avoid circular dependencies**: Lower-level modules should never import from higher-level modules
2. **Physics modules** can import from each other, but should not import from entities or core
3. **Entity modules** can import from physics and systems, but not from core
4. **Core modules** can import from entities, but should be minimal
5. **Systems modules** should have minimal dependencies on other internal modules

### Adding New Modules

When adding new functionality, follow these guidelines:

**Adding a new entity:**
- Create file in `src/entities/`
- May depend on: physics, systems modules
- Add to `src/entities/mod.rs`
- Register in `main.rs`

**Adding a new physics system:**
- Create file in `src/physics/`
- May depend on: other physics modules
- Should NOT depend on: entities, core, systems
- Add to `src/physics/mod.rs`
- Register plugin in `main.rs`

**Adding a new game system:**
- Create file in `src/systems/`
- May depend on: physics modules (minimal)
- Add to `src/systems/mod.rs`
- Register in `main.rs`

**Adding core functionality:**
- Create file in `src/core/`
- May depend on: entities, physics, systems
- Should be minimal and focused on game flow
- Add to `src/core/mod.rs`
- Register in `main.rs`

---

## Français

### Graphe de Dépendances des Modules

Ce diagramme montre comment les différents modules dépendent les uns des autres dans le projet.

```
main.rs
  │
  ├──> core::game_flow (GameFlowPlugin)
  │      └──> entities::player::Player
  │
  ├──> core::camera (camera_fit_inside_current_level)
  │      └──> entities::player::Player
  │
  ├──> physics::walls (WallPlugin)
  │
  ├──> physics::ground_detection (GroundDetectionPlugin)
  │
  ├──> physics::climbing (ClimbingPlugin)
  │      └──> physics::colliders::SensorBundle
  │
  ├──> entities::player (PlayerPlugin)
  │      ├──> physics::climbing::Climber
  │      ├──> physics::colliders::ColliderBundle
  │      ├──> physics::ground_detection::GroundDetection
  │      └──> systems::inventory::Inventory
  │
  ├──> entities::enemy (EnemyPlugin)
  │      └──> physics::colliders::ColliderBundle
  │
  ├──> entities::misc_objects (MiscObjectsPlugin)
  │      └──> physics::colliders::ColliderBundle
  │
  └──> systems::inventory (dbg_print_inventory)
         └──> entities::player::Player
```

### Couches de Dépendance

Le projet est organisé en couches de dépendance du bas niveau au haut niveau :

**Couche 1 - Fondation (Pas de dépendances internes)**
- `physics::colliders` - Définit les formes de collision pour les entités
- `physics::walls` - Génération de collisions de murs (dépend uniquement des libs externes)

**Couche 2 - Systèmes Physiques de Base**
- `physics::ground_detection` - Détection du sol (pas de dépendances internes)
- `physics::climbing` - Mécanique d'escalade (dépend de `physics::colliders`)

**Couche 3 - Systèmes de Jeu**
- `systems::inventory` - Gestion de l'inventaire

**Couche 4 - Entités**
- `entities::player` - Joueur (dépend des modules physics et systems)
- `entities::enemy` - Ennemis (dépend de `physics::colliders`)
- `entities::misc_objects` - Objets (dépend de `physics::colliders`)

**Couche 5 - Logique de Jeu Principale**
- `core::game_flow` - Flux de jeu (dépend de `entities::player`)
- `core::camera` - Système de caméra (dépend de `entities::player`)

**Couche 6 - Application**
- `main.rs` - Point d'entrée de l'application (dépend de tous les modules)

### Directives d'Import

Pour maintenir une architecture propre :

1. **Éviter les dépendances circulaires** : Les modules de bas niveau ne doivent jamais importer depuis les modules de haut niveau
2. **Les modules physics** peuvent s'importer entre eux, mais ne doivent pas importer depuis entities ou core
3. **Les modules entity** peuvent importer depuis physics et systems, mais pas depuis core
4. **Les modules core** peuvent importer depuis entities, mais doivent être minimaux
5. **Les modules systems** doivent avoir un minimum de dépendances sur d'autres modules internes

### Ajout de Nouveaux Modules

Lors de l'ajout de nouvelles fonctionnalités, suivez ces directives :

**Ajout d'une nouvelle entité :**
- Créer un fichier dans `src/entities/`
- Peut dépendre de : modules physics, systems
- Ajouter à `src/entities/mod.rs`
- Enregistrer dans `main.rs`

**Ajout d'un nouveau système physique :**
- Créer un fichier dans `src/physics/`
- Peut dépendre de : autres modules physics
- Ne doit PAS dépendre de : entities, core, systems
- Ajouter à `src/physics/mod.rs`
- Enregistrer le plugin dans `main.rs`

**Ajout d'un nouveau système de jeu :**
- Créer un fichier dans `src/systems/`
- Peut dépendre de : modules physics (minimal)
- Ajouter à `src/systems/mod.rs`
- Enregistrer dans `main.rs`

**Ajout de fonctionnalité core :**
- Créer un fichier dans `src/core/`
- Peut dépendre de : entities, physics, systems
- Doit être minimal et concentré sur le flux de jeu
- Ajouter à `src/core/mod.rs`
- Enregistrer dans `main.rs`
