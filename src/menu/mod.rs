use bevy::prelude::*;

use crate::input::{get_menu_input, ActiveGamepad};
use crate::GameState;

/// Marqueur pour les entités du menu
#[derive(Component)]
pub struct MenuEntity;

/// Marqueur pour le bouton "Jouer"
#[derive(Component)]
pub struct PlayButton;

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

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const SELECTED_BUTTON: Color = Color::srgb(0.3, 0.5, 0.8);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedMenuButton>()
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
                    quit_button_action,
                )
                    .run_if(in_state(GameState::Menu)),
            );
    }
}

fn setup_menu(mut commands: Commands, mut selected: ResMut<SelectedMenuButton>) {
    // Réinitialiser la sélection
    selected.index = 0;
    selected.total = 2; // Nombre de boutons

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

            // Bouton Quitter (index 1)
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
                    MenuButtonIndex(1),
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
                Text::new("↑↓ WASD D-Pad : Naviguer  |  Entrée Espace A : Sélectionner"),
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

/// Navigation clavier dans le menu
fn keyboard_navigation(
    keyboard: Res<ButtonInput<KeyCode>>,
    active_gamepad: Res<ActiveGamepad>,
    gamepads: Query<&Gamepad>,
    mut selected: ResMut<SelectedMenuButton>,
) {
    let input = get_menu_input(&keyboard, &active_gamepad, &gamepads);
    
    let mut direction: i32 = 0;

    if input.up {
        direction = -1;
    }
    if input.down {
        direction = 1;
    }

    if direction != 0 {
        let new_index = (selected.index as i32 + direction).rem_euclid(selected.total as i32);
        selected.index = new_index as usize;
    }
}

/// Validation avec Entrée ou Espace
fn keyboard_selection(
    keyboard: Res<ButtonInput<KeyCode>>,
    active_gamepad: Res<ActiveGamepad>,
    gamepads: Query<&Gamepad>,
    selected: Res<SelectedMenuButton>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
) {
    let input = get_menu_input(&keyboard, &active_gamepad, &gamepads);
    
    if input.confirm {
        match selected.index {
            0 => next_state.set(GameState::InGame), // Jouer
            1 => {
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
