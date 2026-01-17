use crate::{entities::player::Player, save::SaveMenuState, GameState};
use bevy::{prelude::*, render::camera};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct GameWorldMarker;

#[derive(Component)]
pub struct GameCameraMarker;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut rapier_config: Query<&mut RapierConfiguration>,
) {
    commands.spawn((Camera2d, GameCameraMarker));

    rapier_config.single_mut().gravity = Vec2::new(0.0, -2000.0);

    let ldtk_handle = asset_server.load("the_seventh_map.ldtk").into();
    commands.spawn((
        LdtkWorldBundle {
            ldtk_handle,
            visibility: Visibility::Hidden,
            ..Default::default()
        },
        GameWorldMarker,
    ));
}

pub fn update_level_selection(
    level_query: Query<(&LevelIid, &Transform), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
    mut level_selection: ResMut<LevelSelection>,
    ldtk_projects: Query<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    for (level_iid, level_transform) in &level_query {
        let ldtk_project = ldtk_project_assets
            .get(ldtk_projects.single())
            .expect("Project should be loaded if level has spawned");

        let level = ldtk_project
            .get_raw_level_by_iid(&level_iid.to_string())
            .expect("Spawned level should exist in LDtk project");

        let level_bounds = Rect {
            min: Vec2::new(level_transform.translation.x, level_transform.translation.y),
            max: Vec2::new(
                level_transform.translation.x + level.px_wid as f32,
                level_transform.translation.y + level.px_hei as f32,
            ),
        };

        for player_transform in &player_query {
            if player_transform.translation.x < level_bounds.max.x
                && player_transform.translation.x > level_bounds.min.x
                && player_transform.translation.y < level_bounds.max.y
                && player_transform.translation.y > level_bounds.min.y
                && !level_selection.is_match(&LevelIndices::default(), level)
            {
                *level_selection = LevelSelection::iid(level.iid.clone());
            }
        }
    }
}

pub fn show_world_when_loaded(
    player_query: Query<&Transform, With<Player>>,
    mut world_query: Query<&mut Visibility, With<GameWorldMarker>>,
) {
    // Si le joueur existe, afficher la carte
    if player_query.get_single().is_ok() {
        if let Ok(mut visibility) = world_query.get_single_mut() {
            if *visibility == Visibility::Hidden {
                *visibility = Visibility::Visible;
                info!("Carte affichée - chargement complet");
            }
        }
    }
}

pub fn cleanup_game_world(
    mut commands: Commands,
    world_query: Query<Entity, With<GameWorldMarker>>,
    camera_query: Query<Entity, With<GameCameraMarker>>,
) {
    // Despawn la carte et tout ce qui est lié au jeu
    for entity in world_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Despawn la caméra du jeu
    for entity in camera_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    info!("Carte et caméra déchargées");
}

pub struct GameFlowPlugin;

impl Plugin for GameFlowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_level_selection)
            .add_systems(OnEnter(GameState::InGame), setup)
            .add_systems(OnEnter(GameState::InGame), update_level_selection)
            .add_systems(OnExit(GameState::InGame), cleanup_game_world)
            .add_systems(
                Update,
                show_world_when_loaded
                    .run_if(in_state(GameState::InGame))
                    .run_if(in_state(SaveMenuState::Closed)),
            );
    }
}
