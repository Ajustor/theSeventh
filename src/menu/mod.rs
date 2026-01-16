pub mod settings;

use bevy::prelude::*;

use crate::input::{
    get_left_stick_y, is_button_just_pressed, is_dpad_down_just_pressed,
    is_dpad_up_just_pressed, GamepadState, STICK_NAVIGATION_THRESHOLD,
};
use crate::GameState;
use settings::*;

/// Marqueur pour les entités du menu
#[derive(Component)]
pub struct MenuEntity;

/// Marqueur pour le bouton "Jouer"
#[derive(Component)]
pub struct PlayButton;

/// Marqueur pour le bouton "Options"
#[derive(Component)]
pub struct OptionsButton;

/// Marqueur pour le bouton "Quitter"
#[derive(Component)]
pub struct QuitButton;

/// Index du bouton dans le menu (pour la navigation)
#[derive(Component)]
pub struct MenuButtonIndex(pub usize);

/// Ressource pour suivre le bouton actuellement sélectionné
#[derive(Resource, Default)]
pub struct SelectedMenuButton {
    pub index: usize,
    pub total: usize,
}

/// Resource to track stick navigation state (for debounce)
#[derive(Resource, Default)]
pub struct StickNavigationState {
    /// Whether the stick was in a navigation position last frame
    pub was_navigating: bool,
}

/// State for the settings menu
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum SettingsMenuState {
    #[default]
    Closed,
    Open,
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const SELECTED_BUTTON: Color = Color::srgb(0.3, 0.5, 0.8);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedMenuButton>()
            .init_resource::<StickNavigationState>()
            .init_resource::<EditingKeyBinding>()
            .init_state::<SettingsMenuState>()
            .add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(OnExit(GameState::Menu), cleanup_menu)
            .add_systems(
                Update,
                (
                    keyboard_navigation,
                    keyboard_selection,
                    button_system,
                    update_button_visuals,
                    play_button_action,
                    options_button_action,
                    quit_button_action,
                )
                    .run_if(in_state(GameState::Menu))
                    .run_if(in_state(SettingsMenuState::Closed)),
            )
            // Settings menu systems
            .add_systems(OnEnter(SettingsMenuState::Open), setup_settings_menu)
            .add_systems(OnExit(SettingsMenuState::Open), cleanup_settings_menu)
            .add_systems(
                Update,
                (
                    handle_volume_buttons,
                    handle_volume_button_hover,
                    handle_key_binding_buttons,
                    update_key_binding_button_colors,
                    capture_key_input,
                    handle_back_button,
                    handle_back_button_hover,
                )
                    .run_if(in_state(GameState::Menu))
                    .run_if(in_state(SettingsMenuState::Open)),
            );
    }
}

fn setup_menu(mut commands: Commands, mut selected: ResMut<SelectedMenuButton>) {
    // Réinitialiser la sélection
    selected.index = 0;
    selected.total = 3; // Nombre de boutons (Jouer, Options, Quitter)

    // Caméra UI pour le menu
    commands.spawn((Camera2d, MenuEntity));

    // Container principal
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
            MenuEntity,
        ))
        .with_children(|parent| {
            // Titre du jeu
            parent.spawn((
                Text::new("The Seventh"),
                TextFont {
                    font_size: 80.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                Node {
                    margin: UiRect::bottom(Val::Px(50.0)),
                    ..default()
                },
            ));

            // Bouton Jouer (index 0)
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(65.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(SELECTED_BUTTON), // Sélectionné par défaut
                    BorderRadius::all(Val::Px(8.0)),
                    PlayButton,
                    MenuButtonIndex(0),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Jouer"),
                        TextFont {
                            font_size: 33.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));
                });

            // Bouton Options (index 1)
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(65.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(NORMAL_BUTTON),
                    BorderRadius::all(Val::Px(8.0)),
                    OptionsButton,
                    MenuButtonIndex(1),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Options"),
                        TextFont {
                            font_size: 33.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));
                });

            // Bouton Quitter (index 2)
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(65.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(NORMAL_BUTTON),
                    BorderRadius::all(Val::Px(8.0)),
                    QuitButton,
                    MenuButtonIndex(2),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Quitter"),
                        TextFont {
                            font_size: 33.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));
                });

            // Instructions de navigation
            parent.spawn((
                Text::new("↑↓ ou W/S : Naviguer  |  Entrée/Espace : Sélectionner"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.5, 0.5)),
                Node {
                    margin: UiRect::top(Val::Px(40.0)),
                    ..default()
                },
            ));
        });
}

fn cleanup_menu(mut commands: Commands, menu_query: Query<Entity, With<MenuEntity>>) {
    for entity in menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Navigation clavier et manette dans le menu
fn keyboard_navigation(
    input: Res<ButtonInput<KeyCode>>,
    gamepad_state: Res<GamepadState>,
    gamepads: Query<&Gamepad>,
    mut selected: ResMut<SelectedMenuButton>,
    mut stick_nav: ResMut<StickNavigationState>,
) {
    let mut direction: i32 = 0;
    let mut is_stick_navigating = false;

    if let Some(gamepad_entity) = gamepad_state.active_gamepad {
        // Gamepad navigation (has priority)
        if is_dpad_up_just_pressed(&gamepads, gamepad_entity) {
            direction = -1;
        }
        if is_dpad_down_just_pressed(&gamepads, gamepad_entity) {
            direction = 1;
        }
        // Also support left stick for navigation with debounce
        let stick_y = get_left_stick_y(&gamepads, gamepad_entity);
        if stick_y.abs() > STICK_NAVIGATION_THRESHOLD {
            is_stick_navigating = true;
            // Only trigger navigation when stick first enters the zone
            if !stick_nav.was_navigating && direction == 0 {
                if stick_y > STICK_NAVIGATION_THRESHOLD {
                    direction = -1;
                } else if stick_y < -STICK_NAVIGATION_THRESHOLD {
                    direction = 1;
                }
            }
        }
    }

    // Update stick navigation state for next frame
    stick_nav.was_navigating = is_stick_navigating;

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
        let new_index = (selected.index as i32 + direction).rem_euclid(selected.total as i32);
        selected.index = new_index as usize;
    }
}

/// Validation avec Entrée, Espace ou bouton A de la manette
fn keyboard_selection(
    input: Res<ButtonInput<KeyCode>>,
    gamepad_state: Res<GamepadState>,
    gamepads: Query<&Gamepad>,
    selected: Res<SelectedMenuButton>,
    mut next_state: ResMut<NextState<GameState>>,
    mut settings_state: ResMut<NextState<SettingsMenuState>>,
    mut exit: EventWriter<AppExit>,
) {
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
            0 => next_state.set(GameState::InGame), // Jouer
            1 => settings_state.set(SettingsMenuState::Open), // Options
            2 => {
                exit.send(AppExit::Success); // Quitter
            }
            _ => {}
        }
    }
}

/// Mise à jour visuelle des boutons selon la sélection clavier
fn update_button_visuals(
    selected: Res<SelectedMenuButton>,
    mut button_query: Query<(&MenuButtonIndex, &mut BackgroundColor, &Interaction), With<Button>>,
) {
    if !selected.is_changed() {
        return;
    }

    for (button_index, mut color, interaction) in button_query.iter_mut() {
        // Ne pas écraser si la souris est dessus
        if *interaction != Interaction::None {
            continue;
        }

        if button_index.0 == selected.index {
            *color = SELECTED_BUTTON.into();
        } else {
            *color = NORMAL_BUTTON.into();
        }
    }
}

/// Gestion des interactions souris (met aussi à jour la sélection)
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MenuButtonIndex),
        (Changed<Interaction>, With<Button>),
    >,
    mut selected: ResMut<SelectedMenuButton>,
) {
    for (interaction, mut color, button_index) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                // Synchroniser la sélection clavier avec le survol souris
                selected.index = button_index.0;
            }
            Interaction::None => {
                if button_index.0 == selected.index {
                    *color = SELECTED_BUTTON.into();
                } else {
                    *color = NORMAL_BUTTON.into();
                }
            }
        }
    }
}

fn play_button_action(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::InGame);
        }
    }
}

fn options_button_action(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<OptionsButton>)>,
    mut settings_state: ResMut<NextState<SettingsMenuState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            settings_state.set(SettingsMenuState::Open);
        }
    }
}

fn quit_button_action(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>,
    mut exit: EventWriter<AppExit>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            exit.send(AppExit::Success);
        }
    }
}
