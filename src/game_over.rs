use bevy::prelude::*;

use crate::GameState;

/// Marqueur pour les éléments UI de l'écran Game Over
#[derive(Component)]
pub struct GameOverUI;

/// Marqueur pour le bouton Restart
#[derive(Component)]
pub struct RestartButton;

/// Marqueur pour le bouton Menu
#[derive(Component)]
pub struct MenuButton;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), setup_game_over_screen)
            .add_systems(OnExit(GameState::GameOver), cleanup_game_over_screen)
            .add_systems(
                Update,
                handle_game_over_buttons.run_if(in_state(GameState::GameOver)),
            );
    }
}

fn setup_game_over_screen(mut commands: Commands) {
    // Container principal
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.85)),
            GameOverUI,
        ))
        .with_children(|parent| {
            // Titre "GAME OVER"
            parent.spawn((
                Text::new("GAME OVER"),
                TextFont {
                    font_size: 80.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.1, 0.1)),
                Node {
                    margin: UiRect::bottom(Val::Px(50.0)),
                    ..default()
                },
            ));

            // Sous-titre
            parent.spawn((
                Text::new("Vous êtes mort..."),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                Node {
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },
            ));

            // Bouton Recommencer
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(250.0),
                        height: Val::Px(60.0),
                        margin: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.3, 0.5, 0.3)),
                    RestartButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Recommencer"),
                        TextFont {
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            // Bouton Menu Principal
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(250.0),
                        height: Val::Px(60.0),
                        margin: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.5, 0.3, 0.3)),
                    MenuButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Menu Principal"),
                        TextFont {
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });

    info!("Écran Game Over affiché");
}

fn cleanup_game_over_screen(mut commands: Commands, query: Query<Entity, With<GameOverUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn handle_game_over_buttons(
    mut next_state: ResMut<NextState<GameState>>,
    restart_query: Query<&Interaction, (Changed<Interaction>, With<RestartButton>)>,
    menu_query: Query<&Interaction, (Changed<Interaction>, With<MenuButton>)>,
    mut button_colors: Query<(&Interaction, &mut BackgroundColor), With<Button>>,
) {
    // Effet de survol sur les boutons
    for (interaction, mut color) in button_colors.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.5, 0.5, 0.5));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.3, 0.3, 0.3));
            }
            _ => {}
        }
    }

    // Gestion du bouton Recommencer
    for interaction in restart_query.iter() {
        if *interaction == Interaction::Pressed {
            info!("Recommencer le jeu");
            next_state.set(GameState::InGame);
        }
    }

    // Gestion du bouton Menu
    for interaction in menu_query.iter() {
        if *interaction == Interaction::Pressed {
            info!("Retour au menu principal");
            next_state.set(GameState::Menu);
        }
    }
}
