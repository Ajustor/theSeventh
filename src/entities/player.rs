use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::dynamics::Velocity;

use crate::gui::player_interface::PlayerInterfacePlugin;
use crate::GameState;
use crate::{climbing::Climber, inventory::Inventory};
use crate::{colliders::ColliderBundle, ground_detection::GroundDetection};

use super::player_animation::PlayerAnimationPlugin;
use super::stats::Stats;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, Component)]
pub enum Side {
    #[default]
    Right,
    Left,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Player;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    pub player: Player,
    #[worldly]
    pub worldly: Worldly,
    pub climber: Climber,
    pub ground_detection: GroundDetection,
    pub side: Side,
    #[from_entity_instance]
    pub stats: Stats,

    // Build Items Component manually by using `impl From<&EntityInstance>`
    #[from_entity_instance]
    items: Inventory,

    // The whole EntityInstance can be stored directly as an EntityInstance component
    #[from_entity_instance]
    entity_instance: EntityInstance,
}

/// Composant pour gérer l'invincibilité temporaire après avoir pris des dégâts
#[derive(Component)]
pub struct Invincibility {
    pub timer: Timer,
    pub blink_timer: Timer,
    pub visible: bool,
}

impl Default for Invincibility {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(3.0, TimerMode::Once),
            blink_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            visible: true,
        }
    }
}

/// Événement déclenché quand le joueur prend des dégâts
#[derive(Event)]
pub struct PlayerDamagedEvent {
    pub player_entity: Entity,
    pub damage: i32,
    pub knockback_direction: Vec2,
}

/// Composant pour gérer le recul (knockback)
#[derive(Component)]
pub struct Knockback {
    pub velocity: Vec2,
    pub timer: Timer,
}

impl Knockback {
    pub fn new(direction: Vec2, force: f32) -> Self {
        Self {
            velocity: direction.normalize_or_zero() * force,
            timer: Timer::from_seconds(0.2, TimerMode::Once),
        }
    }
}

pub fn player_movement(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Climber, &GroundDetection, &mut Side), With<Player>>,
) {
    for (mut velocity, mut climber, ground_detection, mut side) in &mut query {
        let right = if input.pressed(KeyCode::KeyD) { 1. } else { 0. };
        let left = if input.pressed(KeyCode::KeyA) { 1. } else { 0. };

        velocity.linvel.x = (right - left) * 200.;

        if right > 0. || left > 0. {
            if right > 0. {
                *side = Side::Right;
            } else {
                *side = Side::Left;
            }
        }

        if climber.intersecting_climbables.is_empty() {
            climber.climbing = false;
        } else if input.just_pressed(KeyCode::KeyW) || input.just_pressed(KeyCode::KeyS) {
            climber.climbing = true;
        }

        if climber.climbing {
            let up = if input.pressed(KeyCode::KeyW) { 1. } else { 0. };
            let down = if input.pressed(KeyCode::KeyS) { 1. } else { 0. };

            velocity.linvel.y = (up - down) * 200.;
        }

        if input.just_pressed(KeyCode::Space) && (ground_detection.on_ground || climber.climbing) {
            velocity.linvel.y = 500.;
            climber.climbing = false;
        }
    }
}

pub fn player_actions(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Climber, &GroundDetection), With<Player>>,
) {
    for (climber, ground_detection) in &mut query {
        if climber.climbing {
            return;
        }

        if input.just_pressed(KeyCode::KeyO) && ground_detection.on_ground {
            dbg!("Open element");
        }
        // L'attaque est maintenant gérée par le CombatPlugin
    }
}

pub fn check_player_death(
    query: Query<&Stats, With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for stats in query.iter() {
        if stats.life <= 0 {
            info!("Le joueur est mort ! Game Over");
            next_state.set(GameState::GameOver);
        }
    }
}

pub fn player_death_system(
    mut commands: Commands,
    player_query: Query<(Entity, &Stats), With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (entity, stats) in player_query.iter() {
        if stats.life <= 0 {
            commands.entity(entity).despawn();
            next_state.set(GameState::GameOver);
            info!("Player died!");
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (player_movement, player_actions, player_death_system),
        )
        .register_ldtk_entity::<PlayerBundle>("Player")
        .add_plugins(PlayerInterfacePlugin)
        .add_plugins(PlayerAnimationPlugin)
        .add_event::<PlayerDamagedEvent>()
        .add_systems(
            Update,
            (
                handle_player_damaged,
                update_invincibility,
                update_knockback,
                update_invincibility_blink,
            )
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            Update,
            check_player_death.run_if(in_state(GameState::InGame)),
        );
    }
}

/// Système qui gère les dégâts reçus par le joueur
pub fn handle_player_damaged(
    mut commands: Commands,
    mut damage_events: EventReader<PlayerDamagedEvent>,
    mut player_query: Query<(Entity, &mut Stats, Option<&Invincibility>), With<Player>>,
) {
    for event in damage_events.read() {
        if let Ok((entity, mut stats, invincibility)) = player_query.get_mut(event.player_entity) {
            // Ignorer les dégâts si le joueur est invincible
            if invincibility.is_some() {
                continue;
            }

            // Appliquer les dégâts
            stats.life -= event.damage;
            info!(
                "Joueur touché ! Vie restante: {}/{}",
                stats.life, stats.max_life
            );

            // Ajouter l'invincibilité
            commands.entity(entity).insert(Invincibility::default());

            // Ajouter le knockback (recul)
            let knockback_force = 300.0;
            commands
                .entity(entity)
                .insert(Knockback::new(event.knockback_direction, knockback_force));
        }
    }
}

/// Système qui met à jour le timer d'invincibilité
pub fn update_invincibility(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Invincibility, &mut Sprite), With<Player>>,
) {
    for (entity, mut invincibility, mut sprite) in query.iter_mut() {
        invincibility.timer.tick(time.delta());

        if invincibility.timer.finished() {
            // Retirer l'invincibilité et s'assurer que le sprite est visible
            commands.entity(entity).remove::<Invincibility>();
            sprite.color = Color::WHITE;
            info!("Fin de l'invincibilité");
        }
    }
}

/// Système qui fait clignoter le joueur pendant l'invincibilité
pub fn update_invincibility_blink(
    time: Res<Time>,
    mut query: Query<(&mut Invincibility, &mut Sprite), With<Player>>,
) {
    for (mut invincibility, mut sprite) in query.iter_mut() {
        invincibility.blink_timer.tick(time.delta());

        if invincibility.blink_timer.just_finished() {
            invincibility.visible = !invincibility.visible;

            if invincibility.visible {
                sprite.color = Color::WHITE;
            } else {
                sprite.color = Color::srgba(1.0, 1.0, 1.0, 0.3);
            }
        }
    }
}

/// Système qui applique le knockback (recul)
pub fn update_knockback(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Knockback, &mut Velocity), With<Player>>,
) {
    for (entity, mut knockback, mut velocity) in query.iter_mut() {
        knockback.timer.tick(time.delta());

        if knockback.timer.finished() {
            commands.entity(entity).remove::<Knockback>();
        } else {
            // Appliquer la vélocité de recul
            velocity.linvel = knockback.velocity;
        }
    }
}
