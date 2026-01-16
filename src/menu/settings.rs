use bevy::prelude::*;

use crate::config::{AudioConfig, GameConfig, KeyBindings};
use crate::input::GamepadState;

/// Marker for settings menu entities
#[derive(Component)]
pub struct SettingsEntity;

/// Marker for the back button
#[derive(Component)]
pub struct BackButton;

/// Marker for volume value text
#[derive(Component)]
pub struct VolumeValueText;

/// Marker for decrease volume button
#[derive(Component)]
pub struct VolumeDecreaseButton;

/// Marker for increase volume button
#[derive(Component)]
pub struct VolumeIncreaseButton;

/// Marker for key binding buttons
#[derive(Component)]
pub struct KeyBindingButton {
    pub action: KeyAction,
}

/// Marker for the key bindings section container (to hide when gamepad is connected)
#[derive(Component)]
pub struct KeyBindingsSection;

/// Marker for the instructions text (to hide when gamepad is connected)
#[derive(Component)]
pub struct KeyBindingsInstructions;

/// Key actions that can be remapped
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyAction {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    Jump,
    Attack,
    Interact,
}

impl KeyAction {
    pub fn label(&self) -> &'static str {
        match self {
            KeyAction::MoveLeft => "Gauche",
            KeyAction::MoveRight => "Droite",
            KeyAction::MoveUp => "Haut",
            KeyAction::MoveDown => "Bas",
            KeyAction::Jump => "Sauter",
            KeyAction::Attack => "Attaquer",
            KeyAction::Interact => "Interagir",
        }
    }

    pub fn get_key(&self, bindings: &KeyBindings) -> KeyCode {
        match self {
            KeyAction::MoveLeft => bindings.move_left,
            KeyAction::MoveRight => bindings.move_right,
            KeyAction::MoveUp => bindings.move_up,
            KeyAction::MoveDown => bindings.move_down,
            KeyAction::Jump => bindings.jump,
            KeyAction::Attack => bindings.attack,
            KeyAction::Interact => bindings.interact,
        }
    }

    pub fn set_key(&self, bindings: &mut KeyBindings, key: KeyCode) {
        match self {
            KeyAction::MoveLeft => bindings.move_left = key,
            KeyAction::MoveRight => bindings.move_right = key,
            KeyAction::MoveUp => bindings.move_up = key,
            KeyAction::MoveDown => bindings.move_down = key,
            KeyAction::Jump => bindings.jump = key,
            KeyAction::Attack => bindings.attack = key,
            KeyAction::Interact => bindings.interact = key,
        }
    }
}

/// State for which key is being edited (if any)
#[derive(Resource, Default)]
pub struct EditingKeyBinding {
    pub action: Option<KeyAction>,
}

/// Marker for key value text components
#[derive(Component)]
pub struct KeyValueText {
    pub action: KeyAction,
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const SELECTED_BUTTON: Color = Color::srgb(0.3, 0.5, 0.8);
const EDITING_BUTTON: Color = Color::srgb(0.8, 0.5, 0.3);

/// Convert KeyCode to display string
pub fn keycode_to_string(key: KeyCode) -> String {
    match key {
        KeyCode::KeyA => "A".to_string(),
        KeyCode::KeyB => "B".to_string(),
        KeyCode::KeyC => "C".to_string(),
        KeyCode::KeyD => "D".to_string(),
        KeyCode::KeyE => "E".to_string(),
        KeyCode::KeyF => "F".to_string(),
        KeyCode::KeyG => "G".to_string(),
        KeyCode::KeyH => "H".to_string(),
        KeyCode::KeyI => "I".to_string(),
        KeyCode::KeyJ => "J".to_string(),
        KeyCode::KeyK => "K".to_string(),
        KeyCode::KeyL => "L".to_string(),
        KeyCode::KeyM => "M".to_string(),
        KeyCode::KeyN => "N".to_string(),
        KeyCode::KeyO => "O".to_string(),
        KeyCode::KeyP => "P".to_string(),
        KeyCode::KeyQ => "Q".to_string(),
        KeyCode::KeyR => "R".to_string(),
        KeyCode::KeyS => "S".to_string(),
        KeyCode::KeyT => "T".to_string(),
        KeyCode::KeyU => "U".to_string(),
        KeyCode::KeyV => "V".to_string(),
        KeyCode::KeyW => "W".to_string(),
        KeyCode::KeyX => "X".to_string(),
        KeyCode::KeyY => "Y".to_string(),
        KeyCode::KeyZ => "Z".to_string(),
        KeyCode::Space => "Espace".to_string(),
        KeyCode::Enter => "Entrée".to_string(),
        KeyCode::Escape => "Échap".to_string(),
        KeyCode::ArrowUp => "↑".to_string(),
        KeyCode::ArrowDown => "↓".to_string(),
        KeyCode::ArrowLeft => "←".to_string(),
        KeyCode::ArrowRight => "→".to_string(),
        KeyCode::ShiftLeft => "Shift G".to_string(),
        KeyCode::ShiftRight => "Shift D".to_string(),
        KeyCode::ControlLeft => "Ctrl G".to_string(),
        KeyCode::ControlRight => "Ctrl D".to_string(),
        KeyCode::AltLeft => "Alt G".to_string(),
        KeyCode::AltRight => "Alt D".to_string(),
        KeyCode::Tab => "Tab".to_string(),
        KeyCode::Digit1 => "1".to_string(),
        KeyCode::Digit2 => "2".to_string(),
        KeyCode::Digit3 => "3".to_string(),
        KeyCode::Digit4 => "4".to_string(),
        KeyCode::Digit5 => "5".to_string(),
        KeyCode::Digit6 => "6".to_string(),
        KeyCode::Digit7 => "7".to_string(),
        KeyCode::Digit8 => "8".to_string(),
        KeyCode::Digit9 => "9".to_string(),
        KeyCode::Digit0 => "0".to_string(),
        _ => format!("{:?}", key),
    }
}

pub fn setup_settings_menu(
    mut commands: Commands,
    key_bindings: Res<KeyBindings>,
    audio_config: Res<AudioConfig>,
    gamepad_state: Res<GamepadState>,
) {
    let gamepad_connected = gamepad_state.is_connected();
    // Camera for the settings UI
    commands.spawn((Camera2d, SettingsEntity));

    // Main container
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexStart,
                padding: UiRect::all(Val::Px(20.0)),
                row_gap: Val::Px(15.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
            SettingsEntity,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Options"),
                TextFont {
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                Node {
                    margin: UiRect::bottom(Val::Px(30.0)),
                    ..default()
                },
            ));

            // Volume section
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(10.0),
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                })
                .with_children(|parent| {
                    // Volume label
                    parent.spawn((
                        Text::new("Volume Principal"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    ));

                    // Volume controls row
                    parent
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            align_items: AlignItems::Center,
                            column_gap: Val::Px(15.0),
                            ..default()
                        })
                        .with_children(|parent| {
                            // Decrease button
                            parent
                                .spawn((
                                    Button,
                                    Node {
                                        width: Val::Px(40.0),
                                        height: Val::Px(40.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    BackgroundColor(NORMAL_BUTTON),
                                    BorderRadius::all(Val::Px(5.0)),
                                    VolumeDecreaseButton,
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Text::new("-"),
                                        TextFont {
                                            font_size: 28.0,
                                            ..default()
                                        },
                                        TextColor(Color::WHITE),
                                    ));
                                });

                            // Volume value display
                            parent.spawn((
                                Text::new(format!("{}%", (audio_config.master_volume * 100.0) as i32)),
                                TextFont {
                                    font_size: 24.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                                Node {
                                    width: Val::Px(60.0),
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                VolumeValueText,
                            ));

                            // Increase button
                            parent
                                .spawn((
                                    Button,
                                    Node {
                                        width: Val::Px(40.0),
                                        height: Val::Px(40.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    BackgroundColor(NORMAL_BUTTON),
                                    BorderRadius::all(Val::Px(5.0)),
                                    VolumeIncreaseButton,
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Text::new("+"),
                                        TextFont {
                                            font_size: 28.0,
                                            ..default()
                                        },
                                        TextColor(Color::WHITE),
                                    ));
                                });
                        });
                });

            // Key bindings section - only show when no gamepad is connected
            if !gamepad_connected {
                parent
                    .spawn((
                        Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            row_gap: Val::Px(8.0),
                            margin: UiRect::bottom(Val::Px(20.0)),
                            ..default()
                        },
                        KeyBindingsSection,
                    ))
                    .with_children(|parent| {
                        // Key bindings label
                        parent.spawn((
                            Text::new("Contrôles"),
                            TextFont {
                                font_size: 24.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.8, 0.8, 0.8)),
                            Node {
                                margin: UiRect::bottom(Val::Px(10.0)),
                                ..default()
                            },
                        ));

                        // Key binding rows
                        let actions = [
                            KeyAction::MoveLeft,
                            KeyAction::MoveRight,
                            KeyAction::MoveUp,
                            KeyAction::MoveDown,
                            KeyAction::Jump,
                            KeyAction::Attack,
                            KeyAction::Interact,
                        ];

                        for action in actions {
                            parent
                                .spawn(Node {
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::SpaceBetween,
                                    width: Val::Px(300.0),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    // Action label
                                    parent.spawn((
                                        Text::new(action.label()),
                                        TextFont {
                                            font_size: 20.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.7, 0.7, 0.7)),
                                    ));

                                    // Key button
                                    parent
                                        .spawn((
                                            Button,
                                            Node {
                                                width: Val::Px(100.0),
                                                height: Val::Px(35.0),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            BackgroundColor(NORMAL_BUTTON),
                                            BorderRadius::all(Val::Px(5.0)),
                                            KeyBindingButton { action },
                                        ))
                                        .with_children(|parent| {
                                            let key = action.get_key(&key_bindings);
                                            parent.spawn((
                                                Text::new(keycode_to_string(key)),
                                                TextFont {
                                                    font_size: 18.0,
                                                    ..default()
                                                },
                                                TextColor(Color::WHITE),
                                                KeyValueText { action },
                                            ));
                                        });
                                });
                        }
                    });

                // Instructions for keyboard controls
                parent.spawn((
                    Text::new("Cliquez sur une touche pour la modifier"),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.5, 0.5, 0.5)),
                    Node {
                        margin: UiRect::bottom(Val::Px(20.0)),
                        ..default()
                    },
                    KeyBindingsInstructions,
                ));
            } else {
                // Show message when gamepad is connected
                parent.spawn((
                    Text::new("Manette détectée - Contrôles clavier désactivés"),
                    TextFont {
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.6, 0.8, 0.6)),
                    Node {
                        margin: UiRect::bottom(Val::Px(20.0)),
                        ..default()
                    },
                ));
            }

            // Back button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(SELECTED_BUTTON),
                    BorderRadius::all(Val::Px(8.0)),
                    BackButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Retour"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

pub fn cleanup_settings_menu(
    mut commands: Commands,
    query: Query<Entity, With<SettingsEntity>>,
    mut editing: ResMut<EditingKeyBinding>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    editing.action = None;
}

pub fn handle_volume_buttons(
    decrease_query: Query<&Interaction, (Changed<Interaction>, With<VolumeDecreaseButton>)>,
    increase_query: Query<&Interaction, (Changed<Interaction>, With<VolumeIncreaseButton>)>,
    mut audio_config: ResMut<AudioConfig>,
    mut config: ResMut<GameConfig>,
    mut volume_text: Query<&mut Text, With<VolumeValueText>>,
    mut global_volume: ResMut<GlobalVolume>,
) {
    let mut changed = false;

    for interaction in decrease_query.iter() {
        if *interaction == Interaction::Pressed {
            audio_config.master_volume = (audio_config.master_volume - 0.05).max(0.0);
            changed = true;
        }
    }

    for interaction in increase_query.iter() {
        if *interaction == Interaction::Pressed {
            audio_config.master_volume = (audio_config.master_volume + 0.05).min(1.0);
            changed = true;
        }
    }

    if changed {
        // Update the display
        for mut text in volume_text.iter_mut() {
            **text = format!("{}%", (audio_config.master_volume * 100.0) as i32);
        }

        // Update config and save
        config.audio = audio_config.clone();
        if let Err(e) = config.save() {
            warn!("Failed to save config: {}", e);
        }

        // Update global volume
        *global_volume = GlobalVolume::new(audio_config.master_volume);
    }
}

pub fn handle_key_binding_buttons(
    interaction_query: Query<
        (&Interaction, &KeyBindingButton, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut editing: ResMut<EditingKeyBinding>,
) {
    for (interaction, button, _color) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            if editing.action == Some(button.action) {
                // Cancel editing
                editing.action = None;
            } else {
                // Start editing this key
                editing.action = Some(button.action);
            }
        }
    }
}

pub fn update_key_binding_button_colors(
    editing: Res<EditingKeyBinding>,
    mut button_query: Query<(&KeyBindingButton, &mut BackgroundColor, &Interaction)>,
) {
    for (button, mut color, interaction) in button_query.iter_mut() {
        if editing.action == Some(button.action) {
            *color = EDITING_BUTTON.into();
        } else if *interaction == Interaction::Hovered {
            *color = HOVERED_BUTTON.into();
        } else {
            *color = NORMAL_BUTTON.into();
        }
    }
}

pub fn capture_key_input(
    input: Res<ButtonInput<KeyCode>>,
    mut editing: ResMut<EditingKeyBinding>,
    mut key_bindings: ResMut<KeyBindings>,
    mut config: ResMut<GameConfig>,
    mut key_text_query: Query<(&KeyValueText, &mut Text)>,
) {
    if let Some(action) = editing.action {
        // Check for any key press
        for key in input.get_just_pressed() {
            // Skip escape (used to cancel)
            if *key == KeyCode::Escape {
                editing.action = None;
                return;
            }

            // Set the new key
            action.set_key(&mut key_bindings, *key);

            // Update the text display
            for (text_action, mut text) in key_text_query.iter_mut() {
                if text_action.action == action {
                    **text = keycode_to_string(*key);
                }
            }

            // Update config and save
            config.key_bindings = key_bindings.clone();
            if let Err(e) = config.save() {
                warn!("Failed to save config: {}", e);
            }

            // Stop editing
            editing.action = None;
            return;
        }
    }
}

pub fn handle_back_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<BackButton>)>,
    gamepad_state: Res<GamepadState>,
    gamepads: Query<&Gamepad>,
    mut next_state: ResMut<NextState<super::SettingsMenuState>>,
) {
    // Check for back action from gamepad (East button / B) or mouse click
    let back_pressed = if let Some(gamepad_entity) = gamepad_state.active_gamepad {
        if let Ok(gamepad) = gamepads.get(gamepad_entity) {
            gamepad.just_pressed(GamepadButton::East)
        } else {
            false
        }
    } else {
        false
    };

    if back_pressed {
        next_state.set(super::SettingsMenuState::Closed);
        return;
    }

    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(super::SettingsMenuState::Closed);
        }
    }
}

pub fn handle_volume_button_hover(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (
            Changed<Interaction>,
            Or<(With<VolumeDecreaseButton>, With<VolumeIncreaseButton>)>,
        ),
    >,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
            Interaction::Pressed => {
                *color = SELECTED_BUTTON.into();
            }
        }
    }
}

pub fn handle_back_button_hover(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<BackButton>)>,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = SELECTED_BUTTON.into();
            }
            Interaction::Pressed => {
                *color = Color::srgb(0.2, 0.4, 0.7).into();
            }
        }
    }
}
