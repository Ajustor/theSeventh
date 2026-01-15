# 📁 Structure du Projet - The Seventh

## Vue d'ensemble

Ce document décrit l'organisation des fichiers du projet "The Seventh", un jeu de plateforme 2D développé avec le moteur **Bevy** en Rust.

---

## 📂 Structure Actuelle

```
theSeventh/
├── .github/                    # Configuration GitHub (CI/CD, workflows)
├── assets/                     # Ressources graphiques et niveaux
│   ├── atlas/                  # Spritesheets et textures
│   │   ├── MV Icons Complete Sheet Free - ALL.png
│   │   ├── NuclearBlaze_by_deepnight.png
│   │   ├── SunnyLand-player.png
│   │   └── SunnyLand_by_Ansimuz-extended.png
│   ├── player.png
│   └── Typical_2D_platformer_example.ldtk
├── src/                        # Code source du jeu
│   ├── combat/                 # Système de combat
│   │   ├── mod.rs              # Module principal
│   │   ├── attack.rs           # Système d'attaque (hitbox, slash animation)
│   │   ├── damage.rs           # Système de dégâts (knockback, invincibilité)
│   │   └── health.rs           # Système de santé
│   ├── core/                   # Systèmes fondamentaux
│   │   ├── mod.rs              # Module principal
│   │   ├── camera.rs           # Caméra qui suit le joueur
│   │   └── game_flow.rs        # Gestion des niveaux et transitions
│   ├── entities/               # Entités du jeu
│   │   ├── mod.rs              # Module principal
│   │   ├── player.rs           # Logique du joueur (mouvement, actions)
│   │   ├── player_animation.rs # Animations du joueur
│   │   ├── enemy.rs            # Logique des ennemis (patrouille)
│   │   └── stats.rs            # Statistiques (vie, dégâts)
│   ├── physics/                # Physique et collisions
│   │   ├── mod.rs              # Module principal
│   │   ├── colliders.rs        # Bundles de collisions (Player, Mob, Chest, Door)
│   │   ├── climbing.rs         # Système d'escalade
│   │   ├── ground_detection.rs # Détection du sol
│   │   └── walls.rs            # Génération des murs
│   ├── ui/                     # Interface utilisateur
│   │   ├── mod.rs              # Module principal
│   │   ├── hud.rs              # HUD en jeu (barre de vie)
│   │   ├── menu.rs             # Menu principal
│   │   └── game_over.rs        # Écran Game Over
│   ├── world/                  # Objets du monde
│   │   ├── mod.rs              # Module principal
│   │   ├── inventory.rs        # Système d'inventaire
│   │   └── objects.rs          # Coffres, portes, citrouilles
│   └── main.rs                 # Point d'entrée
├── Cargo.toml                  # Configuration du projet Rust
├── Cargo.lock                  # Verrouillage des dépendances
└── STRUCTURE.md                # Ce fichier
```

---

## 🎯 Classification par Catégorie

### 1. ⚙️ **Configuration & Build**
| Fichier | Description |
|---------|-------------|
| `Cargo.toml` | Dépendances et configuration du projet |
| `Cargo.lock` | Versions exactes des dépendances |
| `.gitignore` | Fichiers ignorés par Git |

---

### 2. 🎮 **Core** (`src/core/`)
Systèmes fondamentaux du jeu.

| Fichier | Description |
|---------|-------------|
| `camera.rs` | Caméra qui suit le joueur et s'adapte au niveau |
| `game_flow.rs` | Setup des niveaux, transitions entre zones |

---

### 3. 👤 **Entities** (`src/entities/`)
Entités du jeu (joueur, ennemis).

| Fichier | Description |
|---------|-------------|
| `player.rs` | Mouvement, actions, knockback, invincibilité |
| `player_animation.rs` | États d'animation (idle, run, jump, attack...) |
| `enemy.rs` | Comportement IA, patrouille |
| `stats.rs` | Points de vie, dégâts (partagé) |

---

### 4. ⚔️ **Combat** (`src/combat/`)
Tout ce qui concerne le combat.

| Fichier | Description |
|---------|-------------|
| `attack.rs` | Hitbox d'attaque, animation de slash |
| `damage.rs` | Gestion des dégâts, knockback, invincibilité |
| `health.rs` | Points de vie, mort |

---

### 5. 🎯 **Physics** (`src/physics/`)
Physique et détection de collisions.

| Fichier | Description |
|---------|-------------|
| `colliders.rs` | Bundles pour Player, Mob, Chest, Door |
| `ground_detection.rs` | Capteur pour le saut |
| `walls.rs` | Génération optimisée des murs |
| `climbing.rs` | Système d'échelles |

---

### 6. 🖥️ **UI** (`src/ui/`)
Interface utilisateur.

| Fichier | Description |
|---------|-------------|
| `hud.rs` | Barre de vie en jeu |
| `menu.rs` | Menu principal (Jouer/Quitter) |
| `game_over.rs` | Écran de mort |

---

### 7. 🌍 **World** (`src/world/`)
Objets et éléments du monde.

| Fichier | Description |
|---------|-------------|
| `inventory.rs` | Système d'inventaire |
| `objects.rs` | Coffres, portes, citrouilles |

---

## 🔧 Dépendances Principales

| Dépendance | Version | Utilité |
|------------|---------|---------|
| `bevy` | 0.15.0 | Moteur de jeu ECS |
| `bevy_ecs_ldtk` | 0.11.0 | Import de niveaux LDtk |
| `bevy_rapier2d` | 0.28.0 | Physique 2D |
| `bevy-inspector-egui` | 0.28.1 | Debugging |

---

## 📊 Résumé

| Dossier | Fichiers | Description |
|---------|----------|-------------|
| `core/` | 2 | Caméra, game flow |
| `physics/` | 4 | Collisions, détection |
| `entities/` | 4 | Joueur, ennemis |
| `combat/` | 3 | Attaque, dégâts |
| `ui/` | 3 | HUD, menus |
| `world/` | 2 | Objets, inventaire |

---

*Structure réorganisée pour une meilleure maintenabilité du projet.*
