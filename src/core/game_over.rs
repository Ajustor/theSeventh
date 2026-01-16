use bevy::prelude::*;

use crate::input::{is_button_just_pressed, is_dpad_down_just_pressed, is_dpad_up_just_pressed, GamepadState};
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

/// Button index for gamepad navigation
#[derive(Component)]
pub struct GameOverButtonIndex(pub usize);

/// Resource to track selected button in game over screen
#[derive(Resource, Default)]
pub struct SelectedGameOverButton {
    pub index: usize,
}

const NORMAL_BUTTON_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);
const SELECTED_BUTTON_COLOR: Color = Color::srgb(0.3, 0.5, 0.8);

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedGameOverButton>()
            .add_systems(OnEnter(GameState::GameOver), setup_game_over_screen)
            .add_systems(OnExit(GameState::GameOver), cleanup_game_over_screen)
            .add_systems(
                Update,
                (
                    handle_game_over_navigation,
                    handle_game_over_buttons,
                    update_game_over_button_visuals,
                )
                    .run_if(in_state(GameState::GameOver)),
            );
    }
}

fn setup_game_over_screen(mut commands: Commands, mut selected: ResMut<SelectedGameOverButton>) {
    // Reset selection
    selected.index = 0;

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

            // Bouton Recommencer (index 0)
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
                    BackgroundColor(SELECTED_BUTTON_COLOR), // Selected by default
                    RestartButton,
                    GameOverButtonIndex(0),
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

            // Bouton Menu Principal (index 1)
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
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                    MenuButton,
                    GameOverButtonIndex(1),
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

/// Navigation with keyboard and gamepad in game over screen
fn handle_game_over_navigation(
    input: Res<ButtonInput<KeyCode>>,
    gamepad_state: Res<GamepadState>,
    gamepads: Query<&Gamepad>,
    mut selected: ResMut<SelectedGameOverButton>,
) {
    let mut direction: i32 = 0;

    if let Some(gamepad_entity) = gamepad_state.active_gamepad {
        // Gamepad navigation (has priority)
        if is_dpad_up_just_pressed(&gamepads, gamepad_entity) {
            direction = -1;
        }
        if is_dpad_down_just_pressed(&gamepads, gamepad_entity) {
            direction = 1;
        }
    }

    // Keyboard navigation (fallback if no gamepad direction)
    if direction == 0 {
        if input.just_pressed(KeyCode::ArrowUp) || input.just_pressed(KeyCode::KeyW) {
            direction = -1;
        }
        if input.just_pressed(KeyCode::ArrowDown) || input.just_pressed(KeyCode::KeyS) {
            direction = 1;
        }
    }

    if direction != 0 {
        let new_index = (selected.index as i32 + direction).rem_euclid(2);
        selected.index = new_index as usize;
    }
}

/// Update button visuals based on selection
fn update_game_over_button_visuals(
    selected: Res<SelectedGameOverButton>,
    mut button_query: Query<(&GameOverButtonIndex, &mut BackgroundColor, &Interaction), With<Button>>,
) {
    if !selected.is_changed() {
        return;
    }

    for (button_index, mut color, interaction) in button_query.iter_mut() {
        // Don't override if mouse is hovering
        if *interaction != Interaction::None {
            continue;
        }

        if button_index.0 == selected.index {
            *color = SELECTED_BUTTON_COLOR.into();
        } else {
            *color = NORMAL_BUTTON_COLOR.into();
        }
    }
}

fn handle_game_over_buttons(
    mut next_state: ResMut<NextState<GameState>>,
    input: Res<ButtonInput<KeyCode>>,
    gamepad_state: Res<GamepadState>,
    gamepads: Query<&Gamepad>,
    selected: Res<SelectedGameOverButton>,
    restart_query: Query<&Interaction, (Changed<Interaction>, With<RestartButton>)>,
    menu_query: Query<&Interaction, (Changed<Interaction>, With<MenuButton>)>,
    mut button_colors: Query<(&Interaction, &mut BackgroundColor, &GameOverButtonIndex), With<Button>>,
) {
    // Effet de survol sur les boutons
    for (interaction, mut color, button_index) in button_colors.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.5, 0.5, 0.5));
            }
            Interaction::None => {
                if button_index.0 == selected.index {
                    *color = BackgroundColor(SELECTED_BUTTON_COLOR);
                } else {
                    *color = BackgroundColor(NORMAL_BUTTON_COLOR);
                }
            }
            _ => {}
        }
    }

    // Check for selection input from gamepad (priority) or keyboard
    let select_pressed = if let Some(gamepad_entity) = gamepad_state.active_gamepad {
        // Gamepad: South button (A/Cross) to select
        is_button_just_pressed(&gamepads, gamepad_entity, GamepadButton::South)
    } else {
        false
    } || input.just_pressed(KeyCode::Enter)
        || input.just_pressed(KeyCode::Space);

    if select_pressed {
        match selected.index {
            0 => {
                info!("Recommencer le jeu");
                next_state.set(GameState::InGame);
            }
            1 => {
                info!("Retour au menu principal");
                next_state.set(GameState::Menu);
            }
            _ => {}
        }
        return;
    }

    // Gestion du bouton Recommencer (click)
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
