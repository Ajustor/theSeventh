use std::ptr::null;

use bevy::{
    color::palettes::css::{RED, WHITE},
    core_pipeline::bloom::{self, Bloom},
    prelude::*,
};

use crate::{
    entities::{player::Player, stats::Stats},
    GameState,
};

#[derive(Component)]
struct Bar;

#[derive(Resource)]
struct PlayerInterface {
    container: Entity,
}

fn setup(mut commands: Commands) {
    let container = commands
        .spawn(Node {
            // center button
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            top: Val::Px(0.),
            left: Val::Px(0.),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        border: UiRect::all(Val::Px(1.)),
                        width: Val::Px(200.),
                        height: Val::Px(30.),
                        left: Val::Px(2.),
                        top: Val::Px(5.),
                        // vertically center child text
                        align_items: AlignItems::Baseline,
                        ..default()
                    },
                    BorderColor(WHITE.into()),
                ))
                .with_child((
                    Node {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        ..default()
                    },
                    BackgroundColor(RED.into()),
                    Bar,
                    TextSpan::default(),
                    TextFont {
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        })
        .id();

    commands.insert_resource(PlayerInterface { container });
}

fn menu(
    player_stats: Query<&Stats, With<Player>>,
    mut bar: Single<&mut Node, With<Bar>>,
    mut text: Single<&mut TextSpan, With<Bar>>,
) {
    for stats in player_stats.iter() {
        bar.width = Val::Percent(((stats.life / stats.max_life) * 100) as f32);
        **text = format!("{}/{}", stats.life, stats.max_life).into();
    }
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
