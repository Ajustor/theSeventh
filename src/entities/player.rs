use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::dynamics::Velocity;

use crate::config::KeyBindings;
use crate::gui::player_interface::PlayerInterfacePlugin;
use crate::input::{
    get_left_stick_x, get_left_stick_y, is_button_just_pressed, GamepadState,
};
use crate::GameState;
use crate::physics::climbing::Climber;
use crate::core::inventory::Inventory;
use crate::physics::colliders::ColliderBundle;
use crate::physics::ground_detection::GroundDetection;

use super::player_animation::PlayerAnimationPlugin;
use super::stats::Stats;

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

pub fn player_movement(
    input: Res<ButtonInput<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    gamepad_state: Res<GamepadState>,
    gamepads: Query<&Gamepad>,
    mut query: Query<(&mut Velocity, &mut Climber, &GroundDetection, &mut Side), With<Player>>,
) {
    for (mut velocity, mut climber, ground_detection, mut side) in &mut query {
        let (right, left, up, down, jump) =
            if let Some(gamepad_entity) = gamepad_state.active_gamepad {
                // Gamepad input (has priority)
                let stick_x = get_left_stick_x(&gamepads, gamepad_entity);
                let stick_y = get_left_stick_y(&gamepads, gamepad_entity);

                let right = if stick_x > 0.0 { stick_x } else { 0.0 };
                let left = if stick_x < 0.0 { -stick_x } else { 0.0 };
                let up = if stick_y > 0.0 { stick_y } else { 0.0 };
                let down = if stick_y < 0.0 { -stick_y } else { 0.0 };
                let jump = is_button_just_pressed(
                    &gamepads,
                    gamepad_entity,
                    GamepadButton::South,
                );

                (right, left, up, down, jump)
            } else {
                // Keyboard input (fallback)
                let right = if input.pressed(key_bindings.move_right) {
                    1.0
                } else {
                    0.0
                };
                let left = if input.pressed(key_bindings.move_left) {
                    1.0
                } else {
                    0.0
                };
                let up = if input.pressed(key_bindings.move_up) {
                    1.0
                } else {
                    0.0
                };
                let down = if input.pressed(key_bindings.move_down) {
                    1.0
                } else {
                    0.0
                };
                let jump = input.just_pressed(key_bindings.jump);

                (right, left, up, down, jump)
            };

        velocity.linvel.x = (right - left) * 200.;

        if right > 0.0 || left > 0.0 {
            if right > left {
                *side = Side::Right;
            } else {
                *side = Side::Left;
            }
        }

        if climber.intersecting_climbables.is_empty() {
            climber.climbing = false;
        } else if up > 0.0 || down > 0.0 {
            // Start climbing when moving up or down on climbable surface
            if !climber.climbing && (up > 0.5 || down > 0.5) {
                climber.climbing = true;
            }
        }

        if climber.climbing {
            velocity.linvel.y = (up - down) * 200.;
        }

        if jump && (ground_detection.on_ground || climber.climbing) {
            velocity.linvel.y = 500.;
            climber.climbing = false;
        }
    }
}

pub fn player_actions(
    input: Res<ButtonInput<KeyCode>>,
    key_bindings: Res<KeyBindings>,
    gamepad_state: Res<GamepadState>,
    gamepads: Query<&Gamepad>,
    mut query: Query<(&Climber, &GroundDetection), With<Player>>,
) {
    for (climber, ground_detection) in &mut query {
        if climber.climbing {
            return;
        }

        let interact = if let Some(gamepad_entity) = gamepad_state.active_gamepad {
            // Gamepad input (has priority) - use East button (B/Circle) for interact
            is_button_just_pressed(&gamepads, gamepad_entity, GamepadButton::East)
        } else {
            // Keyboard input (fallback)
            input.just_pressed(key_bindings.interact)
        };

        if interact && ground_detection.on_ground {
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

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (player_movement, player_actions).run_if(in_state(GameState::InGame)),
        )
        .register_ldtk_entity::<PlayerBundle>("Player")
        .add_plugins(PlayerInterfacePlugin)
        .add_plugins(PlayerAnimationPlugin)
        .add_systems(
            Update,
            (
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
