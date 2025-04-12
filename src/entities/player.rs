use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::dynamics::Velocity;

use crate::gui::player_interface::PlayerInterfacePlugin;
use crate::{climbing::Climber, inventory::Inventory};
use crate::{colliders::ColliderBundle, ground_detection::GroundDetection};

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
    #[sprite("player.png")]
    pub sprite: Sprite,
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
    mut query: Query<(&Climber, &GroundDetection, &Side), With<Player>>,
) {
    for (climber, ground_detection, side) in &mut query {
        if climber.climbing {
            return;
        }

        if input.just_pressed(KeyCode::KeyO) && ground_detection.on_ground {
            dbg!("Open element");
        }

        if input.just_pressed(KeyCode::KeyK) {
            dbg!(format!("Attack on {:?}", side));
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (player_movement, player_actions))
            .register_ldtk_entity::<PlayerBundle>("Player");
    }
}
