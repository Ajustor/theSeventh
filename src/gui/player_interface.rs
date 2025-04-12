use bevy::{
    color::palettes::css::{RED, WHITE},
    core_pipeline::bloom::{self, Bloom},
    prelude::*,
};

use crate::{
    entities::{base, player::Player},
    GameState,
};

#[derive(Component)]
struct Bar;

#[derive(Resource)]
struct PlayerInterface {
    container: Entity,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let container = commands
        .spawn(Node {
            // center button
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|parent| {
            let mut life_container = parent
                .spawn((
                    Node {
                        border: UiRect::all(Val::Px(5.)),
                        width: Val::Px(150.),
                        height: Val::Px(65.),
                        // vertically center child text
                        align_items: AlignItems::Baseline,
                        ..default()
                    },
                    BorderColor(WHITE.into()),
                ))
                .with_child((
                    Node {
                        width: Val::Percent(100.),
                        height: Val::Px(65.),
                        ..default()
                    },
                    BackgroundColor(RED.into()),
                    Bar,
                ));
        })
        .id();

    commands.insert_resource(PlayerInterface { container });
}

fn menu(
    player_stats: Query<&base::Stats, With<Player>>,
    mut bar_node: Single<&mut Node, With<Bar>>,
) {
    let stats = match player_stats.get_single() {
        Err(error) => panic!("{:?}", error),
        Ok(stats) => stats,
    };

    bar_node.width = Val::Percent(((stats.life / stats.max_life) * 100) as f32);
}

fn cleanup(mut commands: Commands, player_interface: Res<PlayerInterface>) {
    commands
        .entity(player_interface.container)
        .despawn_recursive();
}

pub struct PlayerInterfacePlugin;

impl Plugin for PlayerInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup)
            .add_systems(Update, menu.run_if(in_state(GameState::InGame)))
            .add_systems(OnExit(GameState::InGame), cleanup);
    }
}
