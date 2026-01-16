# The Seventh

Un jeu de plateforme 2D développé en Rust avec le moteur de jeu [Bevy](https://bevyengine.org/).

## 🎮 Description

The Seventh est un jeu de plateforme 2D avec des éléments de combat. Le joueur explore des niveaux créés avec [LDtk](https://ldtk.io/), combat des ennemis et interagit avec divers objets du monde.

## 🛠️ Technologies utilisées

- **[Rust](https://www.rust-lang.org/)** - Langage de programmation
- **[Bevy](https://bevyengine.org/) 0.15** - Moteur de jeu ECS
- **[bevy_ecs_ldtk](https://github.com/Trouv/bevy_ecs_ldtk)** - Intégration des cartes LDtk
- **[bevy_rapier2d](https://github.com/dimforge/bevy_rapier)** - Physique 2D
- **[bevy-inspector-egui](https://github.com/jakobhellermann/bevy-inspector-egui)** - Outil de débogage

## 📁 Architecture du projet

```
theSeventh/
├── assets/                     # Ressources du jeu
│   ├── atlas/                  # Spritesheets et textures
│   ├── player.png              # Sprite du joueur
│   └── the_seventh_map.ldtk    # Carte du jeu (éditeur LDtk)
├── src/
│   ├── main.rs                 # Point d'entrée, configuration de l'application
│   ├── core/                   # Systèmes principaux du jeu
│   │   ├── camera.rs           # Gestion de la caméra (suivi du niveau)
│   │   ├── game_flow.rs        # Flux du jeu (initialisation, changement de niveau)
│   │   ├── game_over.rs        # Écran de game over
│   │   └── inventory.rs        # Système d'inventaire
│   ├── engine/                 # Moteur de jeu
│   │   └── damage.rs           # Système de dégâts génériques
│   ├── entities/               # Entités du jeu
│   │   ├── player.rs           # Logique du joueur (mouvement, actions)
│   │   ├── player_animation.rs # Animations du joueur
│   │   ├── enemy.rs            # Logique des ennemis (patrouille)
│   │   └── stats.rs            # Statistiques (vie, dégâts)
│   ├── combat/                 # Système de combat
│   │   ├── attack.rs           # Système d'attaque (slash animé)
│   │   └── health.rs           # Gestion de la santé
│   ├── physics/                # Physique du jeu
│   │   ├── colliders.rs        # Bundles de colliders
│   │   ├── climbing.rs         # Système d'escalade
│   │   ├── ground_detection.rs # Détection du sol
│   │   └── walls.rs            # Gestion des murs
│   ├── gui/                    # Interface utilisateur
│   │   └── player_interface.rs # Barre de vie
│   ├── menu/                   # Menu principal
│   │   └── mod.rs              # Navigation clavier/souris
│   └── world/                  # Objets du monde
│       └── objects.rs          # Coffres, portes, citrouilles
└── Cargo.toml                  # Dépendances du projet
```

## 🎯 États du jeu (GameState)

Le jeu utilise une machine à états pour gérer les différentes phases :

- **Menu** - Menu principal avec options "Jouer" et "Quitter"
- **InGame** - Jeu en cours
- **GameOver** - Écran de fin de partie

## 🔧 Plugins Bevy

Le jeu est organisé en plugins modulaires :

| Plugin | Description |
|--------|-------------|
| `GameFlowPlugin` | Gère l'initialisation et les transitions de niveau |
| `MenuPlugin` | Menu principal avec navigation clavier/souris |
| `WallPlugin` | Gestion des collisions avec les murs |
| `GroundDetectionPlugin` | Détection du sol pour le saut |
| `ClimbingPlugin` | Système d'escalade |
| `PlayerPlugin` | Mouvement, actions et animations du joueur |
| `EnemyPlugin` | IA des ennemis (patrouille) |
| `CombatPlugin` | Système de combat (attaque + santé) |
| `DamagePlugin` | Gestion des dégâts, knockback et invincibilité |
| `GameOverPlugin` | Écran de game over |
| `MiscObjectsPlugin` | Objets interactifs du monde |
| `PlayerInterfacePlugin` | Interface utilisateur (barre de vie) |

## 🎮 Contrôles

### Menu
| Touche | Action |
|--------|--------|
| ↑/↓ ou W/S | Naviguer dans le menu |
| Entrée/Espace | Sélectionner |

### En jeu
| Touche | Action |
|--------|--------|
| A/D | Se déplacer gauche/droite |
| W/S | Grimper (sur les surfaces grimpables) |
| Espace | Sauter |
| K | Attaquer (slash) |
| O | Interagir (ouvrir) |

## 🚀 Installation et exécution

### Prérequis

- [Rust](https://www.rust-lang.org/tools/install) (édition 2021)
- Dépendances système pour Bevy ([voir la documentation](https://bevyengine.org/learn/book/getting-started/setup/))

### Compilation et exécution

```bash
# Cloner le repository
git clone https://github.com/Ajustor/theSeventh.git
cd theSeventh

# Compiler et exécuter en mode release (recommandé pour les performances)
cargo run --release
```

> ⚠️ **Note** : Le jeu est optimisé pour être exécuté en mode `--release` pour de meilleures performances.

## 📐 Configuration de la fenêtre

- **Résolution** : 1080 x 720 pixels
- **Redimensionnable** : Non

## 🎨 Assets

Les assets graphiques proviennent de différentes sources :
- SunnyLand par Ansimuz
- NuclearBlaze par deepnight
- MV Icons Complete Sheet Free

## 📜 Licence

[À définir]
