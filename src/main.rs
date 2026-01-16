// This example shows off a more in-depth implementation of a game with `bevy_ecs_ldtk`.
// Please run with `--release`.
#![allow(clippy::type_complexity)]
use bevy::{prelude::*, window::WindowResolution};
use bevy_ecs_ldtk::prelude::*;
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use engine::damage::DamagePlugin;

use crate::combat::CombatPlugin;

pub const WINDOW_HEIGHT: usize = 720;
pub const WINDOW_WIDTH: usize = 1080;

mod combat;
/// Configuration module (key bindings, volume, etc.)
mod config;
/// Core game systems (camera, game flow, inventory, game over)
mod core;
mod engine;
mod entities;
mod gui;
/// Input handling (keyboard and gamepad)
mod input;
mod menu;
/// Physics-related modules (colliders, climbing, ground detection, walls)
mod physics;
/// World objects (chests, doors, etc.)
mod world;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    InGame,
    _Inventory,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(EmbeddedAssetPlugin {
            mode: PluginMode::ReplaceDefault,
        })
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
        .add_plugins(config::ConfigPlugin)
        .add_plugins(input::InputPlugin)
        .init_state::<GameState>()
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
        .add_plugins(menu::MenuPlugin)
        .add_plugins(physics::walls::WallPlugin)
        .add_plugins(physics::ground_detection::GroundDetectionPlugin)
        .add_plugins(physics::climbing::ClimbingPlugin)
        .add_plugins(entities::player::PlayerPlugin)
        .add_plugins(entities::enemy::EnemyPlugin)
        // .add_systems(Update, core::inventory::dbg_print_inventory)
        .add_systems(Update, core::camera::camera_fit_inside_current_level)
        .add_plugins(world::objects::MiscObjectsPlugin)
        // .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(DamagePlugin)
        .add_plugins(CombatPlugin)
        .add_plugins(core::game_over::GameOverPlugin)
        .run();
}
