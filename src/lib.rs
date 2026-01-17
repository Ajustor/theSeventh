// This is the library entry point for Android builds with cargo-apk.
// Android uses this instead of main() from main.rs.

// Re-export all modules from main.rs
pub mod combat;
pub mod config;
pub mod core;
pub mod engine;
pub mod entities;
pub mod gui;
pub mod input;
pub mod menu;
pub mod physics;
pub mod world;

use crate::combat::CombatPlugin;
use bevy::{prelude::*, window::WindowResolution};
use bevy_ecs_ldtk::prelude::*;
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
use bevy_rapier2d::prelude::*;
use engine::damage::DamagePlugin;

pub const WINDOW_HEIGHT: usize = 720;
pub const WINDOW_WIDTH: usize = 1080;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    InGame,
    _Inventory,
    GameOver,
}

// Shared App setup function used by both desktop and Android builds
pub fn setup_app() -> App {
    let mut app = App::new();

    app.add_plugins(EmbeddedAssetPlugin {
        mode: PluginMode::ReplaceDefault,
    })
    .add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "The Seventh".to_string(),
                    resolution: WindowResolution::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32),
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
    .add_plugins(engine::damage::DamagePlugin)
    .add_plugins(CombatPlugin)
    .add_plugins(core::game_over::GameOverPlugin)
    .add_plugins(gui::dialog::DialogPlugin)
    .add_plugins(gui::tooltip::TooltipPlugin);

    app
}

// This is the Android entry point that cargo-apk will use
#[bevy_main]
fn main() {
    setup_app().run();
}
