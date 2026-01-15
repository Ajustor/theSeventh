# 📁 Structure du Projet - The Seventh

## Vue d'ensemble

Ce document propose une organisation des fichiers du projet "The Seventh", un jeu de plateforme 2D développé avec le moteur **Bevy** en Rust.

---

## 📂 Structure Actuelle

```
theSeventh/
├── .github/                    # Configuration GitHub (CI/CD, workflows)
├── assets/                     # Ressources graphiques et niveaux
│   ├── atlas/                  # Spritesheets et textures
│   ├── player.png
│   └── Typical_2D_platformer_example.ldtk
├── src/                        # Code source du jeu
│   ├── combat/                 # Système de combat
│   ├── engine/                 # Moteur de jeu (systèmes génériques)
│   ├── entities/               # Entités du jeu (joueur, ennemis)
│   ├── gui/                    # Interface utilisateur en jeu
│   ├── menu/                   # Menu principal
│   └── [fichiers racine]       # Modules principaux
├── Cargo.toml                  # Configuration du projet Rust
└── Cargo.lock                  # Verrouillage des dépendances
```

---

## 🎯 Classification par Utilité

### 1. ⚙️ **Configuration & Build**
| Fichier | Description | Utilité |
|---------|-------------|---------|
| `Cargo.toml` | Dépendances et configuration du projet | Configuration du build Rust |
| `Cargo.lock` | Versions exactes des dépendances | Reproductibilité du build |
| `.gitignore` | Fichiers ignorés par Git | Gestion du versionnement |
| `.github/` | Workflows CI/CD | Automatisation |

---

### 2. 🎮 **Point d'Entrée & État du Jeu**
| Fichier | Description | Utilité |
|---------|-------------|---------|
| `src/main.rs` | Point d'entrée principal | Initialise l'application Bevy, charge tous les plugins |
| `src/game_flow.rs` | Gestion des transitions de niveaux | Setup de niveau, détection de changement de niveau |

---

### 3. 👤 **Système du Joueur (Player)**
| Fichier | Description | Utilité |
|---------|-------------|---------|
| `src/entities/player.rs` | Logique du joueur | Mouvement, actions, knockback, invincibilité |
| `src/entities/player_animation.rs` | Animations du joueur | États d'animation (idle, run, jump, attack...) |
| `src/entities/stats.rs` | Statistiques des entités | Vie, dégâts - partagé entre joueur et ennemis |

---

### 4. 👾 **Système des Ennemis**
| Fichier | Description | Utilité |
|---------|-------------|---------|
| `src/entities/enemy.rs` | Logique des ennemis | Patrouille, comportement IA basique |
| `src/entities/mod.rs` | Module entities | Export des sous-modules |

---

### 5. ⚔️ **Système de Combat**
| Fichier | Description | Utilité |
|---------|-------------|---------|
| `src/combat/attack.rs` | Système d'attaque | Hitbox, animation de slash, détection des coups |
| `src/combat/health.rs` | Système de santé | Points de vie, dégâts, mort |
| `src/combat/mod.rs` | Module combat | Export des sous-modules |

---

### 6. 💥 **Moteur de Dégâts (Engine)**
| Fichier | Description | Utilité |
|---------|-------------|---------|
| `src/engine/damage.rs` | Système de dégâts générique | Collisions joueur-ennemi, knockback, invincibilité, flash visuel |
| `src/engine/mod.rs` | Module engine | Export des sous-modules |

---

### 7. 🎯 **Physique & Collisions**
| Fichier | Description | Utilité |
|---------|-------------|---------|
| `src/colliders.rs` | Bundles de collisions | Colliders pour Player, Mob, Chest, Door |
| `src/ground_detection.rs` | Détection du sol | Capteur de sol pour le saut |
| `src/walls.rs` | Génération des murs | Optimisation des collisions des murs (fusion des tiles) |
| `src/climbing.rs` | Système d'escalade | Détection et gestion des échelles |

---

### 8. 🖥️ **Interface Utilisateur (UI)**
| Fichier | Description | Utilité |
|---------|-------------|---------|
| `src/gui/player_interface.rs` | HUD du joueur | Barre de vie, affichage des stats |
| `src/gui/mod.rs` | Module GUI | Export des sous-modules |
| `src/menu/mod.rs` | Menu principal | Écran titre, boutons Jouer/Quitter, navigation clavier |
| `src/game_over.rs` | Écran Game Over | Affichage mort, boutons Recommencer/Menu |

---

### 9. 📷 **Caméra & Rendu**
| Fichier | Description | Utilité |
|---------|-------------|---------|
| `src/camera.rs` | Système de caméra | Suit le joueur, s'adapte aux limites du niveau |

---

### 10. 🎒 **Systèmes Divers**
| Fichier | Description | Utilité |
|---------|-------------|---------|
| `src/inventory.rs` | Inventaire | Gestion des objets du joueur (structure de base) |
| `src/misc_objects.rs` | Objets divers | Coffres, portes, citrouilles |

---

### 11. 🎨 **Assets (Ressources)**
| Fichier | Description | Utilité |
|---------|-------------|---------|
| `assets/Typical_2D_platformer_example.ldtk` | Fichier de niveau LDtk | Design des niveaux |
| `assets/player.png` | Sprite du joueur | Graphisme principal |
| `assets/atlas/*.png` | Spritesheets | Tuiles, icônes, animations |

---

## 🔄 Proposition de Réorganisation

Voici une structure améliorée pour une meilleure lisibilité et maintenabilité :

```
theSeventh/
├── assets/
│   ├── levels/                 # 📍 Niveaux (fichiers .ldtk)
│   │   └── Typical_2D_platformer_example.ldtk
│   ├── sprites/                # 📍 Sprites individuels
│   │   └── player.png
│   └── atlas/                  # Spritesheets (déjà bien organisé)
│
├── src/
│   ├── core/                   # 📍 Systèmes fondamentaux
│   │   ├── mod.rs
│   │   ├── game_state.rs       # États du jeu (Menu, InGame, GameOver)
│   │   ├── camera.rs           # Système de caméra
│   │   └── game_flow.rs        # Transitions de niveaux
│   │
│   ├── physics/                # 📍 Physique et collisions
│   │   ├── mod.rs
│   │   ├── colliders.rs        # Bundles de collisions
│   │   ├── ground_detection.rs # Détection du sol
│   │   ├── walls.rs            # Génération des murs
│   │   └── climbing.rs         # Système d'escalade
│   │
│   ├── entities/               # Entités (déjà bien organisé)
│   │   ├── mod.rs
│   │   ├── player/             # 📍 Sous-dossier joueur
│   │   │   ├── mod.rs
│   │   │   ├── movement.rs     # Mouvement du joueur
│   │   │   ├── animation.rs    # Animations
│   │   │   └── effects.rs      # Invincibilité, knockback
│   │   ├── enemy.rs
│   │   └── stats.rs
│   │
│   ├── combat/                 # Combat (déjà bien organisé)
│   │   ├── mod.rs
│   │   ├── attack.rs
│   │   ├── health.rs
│   │   └── damage.rs           # 📍 Déplacer depuis engine/
│   │
│   ├── ui/                     # 📍 Renommer gui/ -> ui/
│   │   ├── mod.rs
│   │   ├── hud.rs              # 📍 Renommer player_interface.rs
│   │   ├── menu.rs             # 📍 Déplacer depuis menu/mod.rs
│   │   └── game_over.rs        # 📍 Déplacer depuis racine
│   │
│   ├── world/                  # 📍 Objets du monde
│   │   ├── mod.rs
│   │   ├── objects.rs          # Coffres, portes, etc.
│   │   └── inventory.rs
│   │
│   └── main.rs                 # Point d'entrée
│
├── Cargo.toml
├── Cargo.lock
├── README.md                   # 📍 À créer
└── STRUCTURE.md                # Ce fichier
```

---

## 📊 Résumé des Catégories

| Catégorie | Fichiers | % du Code |
|-----------|----------|-----------|
| **Physique & Collisions** | 4 fichiers | ~20% |
| **Entités (Player/Enemy)** | 5 fichiers | ~25% |
| **Combat & Dégâts** | 4 fichiers | ~20% |
| **Interface Utilisateur** | 4 fichiers | ~15% |
| **Core (Caméra, Flow)** | 3 fichiers | ~10% |
| **Monde & Objets** | 2 fichiers | ~10% |

---

## 🔧 Dépendances Principales

| Dépendance | Utilité |
|------------|---------|
| `bevy` | Moteur de jeu ECS |
| `bevy_ecs_ldtk` | Import de niveaux LDtk |
| `bevy_rapier2d` | Physique 2D |
| `bevy-inspector-egui` | Debugging (inspecteur) |

---

## 💡 Recommandations

1. **Modularité** : La structure actuelle est déjà bien modulaire avec les dossiers `combat/`, `entities/`, `engine/`, `gui/`, `menu/`.

2. **À améliorer** :
   - Fusionner `engine/damage.rs` avec `combat/` (même domaine)
   - Créer un dossier `physics/` pour regrouper `colliders.rs`, `ground_detection.rs`, `walls.rs`, `climbing.rs`
   - Renommer `gui/` en `ui/` (convention plus standard)
   - Regrouper les fichiers UI (`menu/`, `game_over.rs`, `gui/`) dans un seul dossier

3. **Assets** : Créer des sous-dossiers `levels/` et `sprites/` pour une meilleure organisation.

4. **Documentation** : Ajouter un `README.md` avec les instructions de build et les contrôles du jeu.

---

*Document généré pour faciliter la navigation et la maintenance du projet.*
