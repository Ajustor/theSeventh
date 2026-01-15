// This example shows off a more in-depth implementation of a game with `bevy_ecs_ldtk`.
// Please run with `--release`.

use bevy::{prelude::*, window::WindowResolution};
use bevy_ecs_ldtk::prelude::*;

use bevy_rapier2d::prelude::*;

pub const WINDOW_HEIGHT: usize = 720;
pub const WINDOW_WIDTH: usize = 1080;

// Module organization:
// - core: Core game systems (initialization, camera)
// - entities: Game entities (player, enemies, objects)
// - physics: Physics systems (collisions, ground detection, climbing)
// - systems: Game systems (inventory, etc.)

mod core;
mod entities;
mod physics;
mod systems;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "The Seventh".to_string(),
                        resolution: WindowResolution::new(
                            WINDOW_WIDTH as f32,
                            WINDOW_HEIGHT as f32,
                        ),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins((
            LdtkPlugin,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
        ))
        .insert_resource(LevelSelection::Uid(0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: true,
            },
            set_clear_color: SetClearColor::FromLevelBackground,
            ..Default::default()
        })
        .add_plugins(core::game_flow::GameFlowPlugin)
        .add_plugins(physics::walls::WallPlugin)
        .add_plugins(physics::ground_detection::GroundDetectionPlugin)
        .add_plugins(physics::climbing::ClimbingPlugin)
        .add_plugins(entities::player::PlayerPlugin)
        .add_plugins(entities::enemy::EnemyPlugin)
        .add_systems(Update, systems::inventory::dbg_print_inventory)
        .add_systems(Update, core::camera::camera_fit_inside_current_level)
        .add_plugins(entities::misc_objects::MiscObjectsPlugin)
        .run();
}
