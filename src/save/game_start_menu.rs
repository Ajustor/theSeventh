use bevy::prelude::*;

use crate::GameState;

use super::{GameStartMenuState, LoadGameEvent, SaveMenuState, SaveSlots};

#[derive(Component)]
pub struct GameStartMenuRoot;

#[derive(Component)]
pub struct NewGameButton;

#[derive(Component)]
pub struct LoadGameButton;

#[derive(Component)]
pub struct BackToMainMenuButton;

#[derive(Event)]
pub struct StartNewGameEvent;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
const DISABLED_BUTTON: Color = Color::srgb(0.08, 0.08, 0.08);

pub fn spawn_game_start_menu(mut commands: Commands, save_slots: Res<SaveSlots>) {
    let has_saves = save_slots.slots.iter().any(|s| s.data.is_some());

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
            GameStartMenuRoot,
        ))
        .with_children(|parent| {
            // Titre
            parent.spawn((
                Text::new("Choisir une option"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(50.0)),
                    ..default()
                },
            ));

            // Container des boutons
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(20.0),
                    ..default()
                })
                .with_children(|buttons| {
                    // Bouton Nouvelle Partie
                    buttons
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(300.0),
                                height: Val::Px(60.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(NORMAL_BUTTON),
                            NewGameButton,
                        ))
                        .with_children(|button| {
                            button.spawn((
                                Text::new("Nouvelle Partie"),
                                TextFont {
                                    font_size: 24.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });

                    // Bouton Charger Partie
                    let load_color = if has_saves {
                        NORMAL_BUTTON
                    } else {
                        DISABLED_BUTTON
                    };
                    let text_color = if has_saves {
                        Color::WHITE
                    } else {
                        Color::srgb(0.4, 0.4, 0.4)
                    };

                    buttons
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(300.0),
                                height: Val::Px(60.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            BackgroundColor(load_color),
                            LoadGameButton,
                        ))
                        .with_children(|button| {
                            button.spawn((
                                Text::new("Charger Partie"),
                                TextFont {
                                    font_size: 24.0,
                                    ..default()
                                },
                                TextColor(text_color),
                            ));

                            if !has_saves {
                                button.spawn((
                                    Text::new("(Aucune sauvegarde)"),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.4, 0.4, 0.4)),
                                ));
                            } else {
                                let save_count =
                                    save_slots.slots.iter().filter(|s| s.data.is_some()).count();
                                button.spawn((
                                    Text::new(format!("({} sauvegarde(s))", save_count)),
                                    TextFont {
                                        font_size: 14.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.6, 0.6, 0.6)),
                                ));
                            }
                        });

                    // Bouton Retour
                    buttons
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(300.0),
                                height: Val::Px(60.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect::top(Val::Px(30.0)),
                                ..default()
                            },
                            BackgroundColor(NORMAL_BUTTON),
                            BackToMainMenuButton,
                        ))
                        .with_children(|button| {
                            button.spawn((
                                Text::new("Retour"),
                                TextFont {
                                    font_size: 24.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });
                });
        });
}

pub fn despawn_game_start_menu(
    mut commands: Commands,
    menu_query: Query<Entity, With<GameStartMenuRoot>>,
) {
    for entity in menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn game_start_menu_interaction(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&NewGameButton>,
            Option<&LoadGameButton>,
            Option<&BackToMainMenuButton>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    save_slots: Res<SaveSlots>,
    mut next_game_start_state: ResMut<NextState<GameStartMenuState>>,
    mut next_save_menu_state: ResMut<NextState<SaveMenuState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    game_start_state: Res<State<GameStartMenuState>>,
    mut new_game_event: EventWriter<StartNewGameEvent>,
) {
    let has_saves = save_slots.slots.iter().any(|s| s.data.is_some());

    for (interaction, mut color, new_game, load_game, back) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if new_game.is_some() {
                    info!("Démarrage d'une nouvelle partie");
                    new_game_event.send(StartNewGameEvent);
                    next_game_start_state.set(GameStartMenuState::Closed);
                    *color = PRESSED_BUTTON.into();
                }

                if load_game.is_some() && has_saves {
                    info!("Ouverture du menu de chargement");
                    next_game_start_state.set(GameStartMenuState::Closed);
                    next_save_menu_state.set(SaveMenuState::Open);
                    // Transitionner vers InGame pour que les entités soient créées
                    next_game_state.set(GameState::InGame);
                    *color = PRESSED_BUTTON.into();
                }

                if back.is_some() {
                    match game_start_state.get() {
                        GameStartMenuState::Open => {
                            next_game_state.set(GameState::Menu);
                        }
                        _ => {}
                    };
                    next_game_start_state.set(GameStartMenuState::Closed);
                    next_save_menu_state.set(SaveMenuState::Closed);
                }
            }
            Interaction::Hovered => {
                if load_game.is_some() && !has_saves {
                    *color = DISABLED_BUTTON.into();
                } else {
                    *color = HOVERED_BUTTON.into();
                }
            }
            Interaction::None => {
                if load_game.is_some() && !has_saves {
                    *color = DISABLED_BUTTON.into();
                } else {
                    *color = NORMAL_BUTTON.into();
                }
            }
        }
    }
}

pub fn game_start_menu_keyboard(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameStartMenuState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameStartMenuState::Closed);
    }
}

pub fn toggle_game_start_menu_visibility(
    mut menu_query: Query<&mut Visibility, With<GameStartMenuRoot>>,
    save_menu_state: Res<State<SaveMenuState>>,
) {
    if let Ok(mut visibility) = menu_query.get_single_mut() {
        // Masquer le menu de démarrage quand le menu de sauvegarde est ouvert
        *visibility = if *save_menu_state.get() == SaveMenuState::Open {
            Visibility::Hidden
        } else {
            Visibility::Visible
        };
    }
}
