use bevy::prelude::*;

use super::{
    DeleteSaveEvent, GameStartMenuState, LoadGameEvent, SaveMenuState, SaveSlots, MAX_SAVE_SLOTS,
};
use crate::{world::save_point::SaveGameEvent, GameState};

#[derive(Component)]
pub struct SaveMenuRoot;

#[derive(Component)]
pub struct SaveSlotButton {
    pub slot_index: usize,
}

#[derive(Component)]
pub struct DeleteSlotButton {
    pub slot_index: usize,
}

#[derive(Component)]
pub struct CloseMenuButton;

#[derive(Component)]
pub struct SelectedSlotMarker;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
const SELECTED_BUTTON: Color = Color::srgb(0.35, 0.35, 0.55);
const EMPTY_SLOT: Color = Color::srgb(0.1, 0.1, 0.1);
const DELETE_BUTTON: Color = Color::srgb(0.5, 0.1, 0.1);
const DELETE_HOVERED: Color = Color::srgb(0.7, 0.2, 0.2);

pub fn spawn_save_menu(mut commands: Commands, save_slots: Res<SaveSlots>) {
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
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            SaveMenuRoot,
        ))
        .with_children(|parent| {
            // Titre
            parent.spawn((
                Text::new("Sauvegardes"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(30.0)),
                    ..default()
                },
            ));

            // Container des slots
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(15.0),
                    ..default()
                })
                .with_children(|slots_container| {
                    for i in 0..MAX_SAVE_SLOTS {
                        let slot = &save_slots.slots[i];
                        let is_selected = save_slots.selected_slot == i;

                        slots_container
                            .spawn(Node {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(10.0),
                                ..default()
                            })
                            .with_children(|row| {
                                // Bouton du slot
                                let button_color = if is_selected {
                                    SELECTED_BUTTON
                                } else if slot.data.is_some() {
                                    NORMAL_BUTTON
                                } else {
                                    EMPTY_SLOT
                                };

                                row.spawn((
                                    Button,
                                    Node {
                                        width: Val::Px(400.0),
                                        height: Val::Px(80.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        flex_direction: FlexDirection::Column,
                                        padding: UiRect::all(Val::Px(10.0)),
                                        ..default()
                                    },
                                    BackgroundColor(button_color),
                                    SaveSlotButton { slot_index: i },
                                ))
                                .with_children(|button| {
                                    if let Some(ref data) = slot.data {
                                        button.spawn((
                                            Text::new(format!("Emplacement {}", i + 1)),
                                            TextFont {
                                                font_size: 24.0,
                                                ..default()
                                            },
                                            TextColor(Color::WHITE),
                                        ));
                                        button.spawn((
                                            Text::new(format!(
                                                "Niveau: {} | Temps: {}",
                                                data.current_level,
                                                data.formatted_playtime()
                                            )),
                                            TextFont {
                                                font_size: 16.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(0.7, 0.7, 0.7)),
                                        ));
                                    } else {
                                        button.spawn((
                                            Text::new(format!("Emplacement {} - Vide", i + 1)),
                                            TextFont {
                                                font_size: 24.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(0.5, 0.5, 0.5)),
                                        ));
                                    }
                                });

                                // Bouton supprimer (seulement si le slot n'est pas vide)
                                if slot.data.is_some() {
                                    row.spawn((
                                        Button,
                                        Node {
                                            width: Val::Px(80.0),
                                            height: Val::Px(80.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        BackgroundColor(DELETE_BUTTON),
                                        DeleteSlotButton { slot_index: i },
                                    ))
                                    .with_children(|button| {
                                        button.spawn((
                                            Text::new("X"),
                                            TextFont {
                                                font_size: 32.0,
                                                ..default()
                                            },
                                            TextColor(Color::WHITE),
                                        ));
                                    });
                                }
                            });
                    }
                });

            // Bouton fermer
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(30.0)),
                        ..default()
                    },
                    BackgroundColor(NORMAL_BUTTON),
                    CloseMenuButton,
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("Retour"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

pub fn despawn_save_menu(mut commands: Commands, menu_query: Query<Entity, With<SaveMenuRoot>>) {
    for entity in menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn save_menu_interaction(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&SaveSlotButton>,
            Option<&DeleteSlotButton>,
            Option<&CloseMenuButton>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut save_slots: ResMut<SaveSlots>,
    mut save_event: EventWriter<SaveGameEvent>,
    mut load_event: EventWriter<LoadGameEvent>,
    mut delete_event: EventWriter<DeleteSaveEvent>,
    mut next_save_state: ResMut<NextState<SaveMenuState>>,
    mut next_start_state: ResMut<NextState<GameStartMenuState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, slot_button, delete_button, close_button) in &mut interaction_query
    {
        match *interaction {
            Interaction::Pressed => {
                if let Some(slot) = slot_button {
                    save_slots.selected_slot = slot.slot_index;

                    if save_slots.slots[slot.slot_index].data.is_some() {
                        // Charger la sauvegarde existante
                        info!("LoadGameEvent envoyé pour slot {}", slot.slot_index);
                        load_event.send(LoadGameEvent {
                            slot_index: slot.slot_index,
                        });
                    } else {
                        // Sauvegarder dans un slot vide
                        info!("SaveGameEvent envoyé");
                        save_event.send(SaveGameEvent {
                            position: Vec3::ZERO, // La position sera récupérée dans le handler
                        });
                    }
                    next_save_state.set(SaveMenuState::Closed);
                    *color = PRESSED_BUTTON.into();
                }

                if let Some(delete) = delete_button {
                    delete_event.send(DeleteSaveEvent {
                        slot_index: delete.slot_index,
                    });
                    // Refresh menu
                    next_save_state.set(SaveMenuState::Closed);
                }

                if close_button.is_some() {
                    // Retour au menu de démarrage
                    next_save_state.set(SaveMenuState::Closed);
                    next_start_state.set(GameStartMenuState::Open);
                    next_game_state.set(GameState::Menu);
                }
            }
            Interaction::Hovered => {
                if delete_button.is_some() {
                    *color = DELETE_HOVERED.into();
                } else {
                    *color = HOVERED_BUTTON.into();
                }
            }
            Interaction::None => {
                if let Some(slot) = slot_button {
                    if save_slots.selected_slot == slot.slot_index {
                        *color = SELECTED_BUTTON.into();
                    } else if save_slots.slots[slot.slot_index].data.is_some() {
                        *color = NORMAL_BUTTON.into();
                    } else {
                        *color = EMPTY_SLOT.into();
                    }
                } else if delete_button.is_some() {
                    *color = DELETE_BUTTON.into();
                } else {
                    *color = NORMAL_BUTTON.into();
                }
            }
        }
    }
}

pub fn save_menu_keyboard_navigation(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut save_slots: ResMut<SaveSlots>,
    mut next_save_state: ResMut<NextState<SaveMenuState>>,
    mut next_start_state: ResMut<NextState<GameStartMenuState>>,
    mut save_event: EventWriter<SaveGameEvent>,
    mut load_event: EventWriter<LoadGameEvent>,
    mut delete_event: EventWriter<DeleteSaveEvent>,
) {
    // Fermer le menu et revenir au menu de démarrage
    if keyboard.just_pressed(KeyCode::Escape) {
        next_save_state.set(SaveMenuState::Closed);
        next_start_state.set(GameStartMenuState::Open);
        return;
    }

    // Navigation
    if keyboard.just_pressed(KeyCode::ArrowUp) || keyboard.just_pressed(KeyCode::KeyW) {
        if save_slots.selected_slot > 0 {
            save_slots.selected_slot -= 1;
        }
    }

    if keyboard.just_pressed(KeyCode::ArrowDown) || keyboard.just_pressed(KeyCode::KeyS) {
        if save_slots.selected_slot < MAX_SAVE_SLOTS - 1 {
            save_slots.selected_slot += 1;
        }
    }

    // Sauvegarder/Charger
    if keyboard.just_pressed(KeyCode::Enter) {
        let slot_index = save_slots.selected_slot;

        if save_slots.slots[slot_index].data.is_some() {
            load_event.send(LoadGameEvent { slot_index });
        } else {
            save_event.send(SaveGameEvent {
                position: Vec3::ZERO,
            });
        }
        next_save_state.set(SaveMenuState::Closed);
    }

    // Supprimer
    if keyboard.just_pressed(KeyCode::Delete) || keyboard.just_pressed(KeyCode::Backspace) {
        let slot_index = save_slots.selected_slot;
        if save_slots.slots[slot_index].data.is_some() {
            delete_event.send(DeleteSaveEvent { slot_index });
        }
    }
}
