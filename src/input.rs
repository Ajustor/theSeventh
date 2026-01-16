use bevy::prelude::*;

/// Resource that tracks whether a gamepad is currently connected
#[derive(Resource, Default)]
pub struct GamepadState {
    /// The first active gamepad (if any)
    pub active_gamepad: Option<Entity>,
}

/// Marker component for the mobile gamepad error UI
#[derive(Component)]
pub struct MobileGamepadErrorUI;

/// Plugin for gamepad detection and input handling
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GamepadState>()
            .add_systems(Update, gamepad_connection_system)
            .add_systems(Update, mobile_gamepad_error_system.run_if(is_mobile_platform));
    }
}

/// Check if running on a mobile platform (Android or iOS)
fn is_mobile_platform() -> bool {
    cfg!(target_os = "android") || cfg!(target_os = "ios")
}

/// System that detects gamepad connection/disconnection events
fn gamepad_connection_system(
    mut gamepad_state: ResMut<GamepadState>,
    gamepads: Query<(Entity, &Gamepad)>,
) {
    // Find the first connected gamepad
    if let Some((entity, _)) = gamepads.iter().next() {
        if gamepad_state.active_gamepad != Some(entity) {
            gamepad_state.active_gamepad = Some(entity);
            info!("Gamepad connected: {:?}", entity);
        }
    } else if gamepad_state.active_gamepad.is_some() {
        info!("Gamepad disconnected");
        gamepad_state.active_gamepad = None;
    }
}

/// System that displays an error message on mobile if no gamepad is connected
fn mobile_gamepad_error_system(
    mut commands: Commands,
    gamepad_state: Res<GamepadState>,
    error_ui_query: Query<Entity, With<MobileGamepadErrorUI>>,
) {
    let error_exists = !error_ui_query.is_empty();
    let gamepad_connected = gamepad_state.is_connected();

    // If no gamepad is connected and error UI doesn't exist, spawn it
    if !gamepad_connected && !error_exists {
        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.85)),
            GlobalZIndex(1000),
            MobileGamepadErrorUI,
        )).with_children(|parent| {
            // Error icon (gamepad symbol)
            parent.spawn((
                Text::new("🎮"),
                TextFont {
                    font_size: 80.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.3, 0.3)),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ));
            
            // Error title
            parent.spawn((
                Text::new("Aucune manette détectée"),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                Node {
                    margin: UiRect::bottom(Val::Px(15.0)),
                    ..default()
                },
            ));
            
            // Error description
            parent.spawn((
                Text::new("Veuillez connecter une manette pour jouer"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
            ));
            
            // Additional hint
            parent.spawn((
                Text::new("(Bluetooth ou USB)"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.5, 0.5)),
            ));
        });
        
        info!("Mobile: No gamepad detected, displaying error message");
    }
    
    // If gamepad is connected and error UI exists, remove it
    if gamepad_connected && error_exists {
        for entity in error_ui_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        info!("Mobile: Gamepad connected, hiding error message");
    }
}

/// Helper functions for reading gamepad input
impl GamepadState {
    /// Check if a gamepad is connected
    pub fn is_connected(&self) -> bool {
        self.active_gamepad.is_some()
    }
}

/// Deadzone for analog sticks
pub const STICK_DEADZONE: f32 = 0.2;

/// Threshold for stick navigation in menus (higher than deadzone to avoid accidental navigation)
pub const STICK_NAVIGATION_THRESHOLD: f32 = 0.5;

/// Get the left stick X axis value from a gamepad
pub fn get_left_stick_x(gamepads: &Query<&Gamepad>, gamepad_entity: Entity) -> f32 {
    if let Ok(gamepad) = gamepads.get(gamepad_entity) {
        let value = gamepad.get(GamepadAxis::LeftStickX).unwrap_or(0.0);
        if value.abs() > STICK_DEADZONE {
            value
        } else {
            0.0
        }
    } else {
        0.0
    }
}

/// Get the left stick Y axis value from a gamepad
pub fn get_left_stick_y(gamepads: &Query<&Gamepad>, gamepad_entity: Entity) -> f32 {
    if let Ok(gamepad) = gamepads.get(gamepad_entity) {
        let value = gamepad.get(GamepadAxis::LeftStickY).unwrap_or(0.0);
        if value.abs() > STICK_DEADZONE {
            value
        } else {
            0.0
        }
    } else {
        0.0
    }
}

/// Check if a gamepad button was just pressed
pub fn is_button_just_pressed(
    gamepads: &Query<&Gamepad>,
    gamepad_entity: Entity,
    button: GamepadButton,
) -> bool {
    if let Ok(gamepad) = gamepads.get(gamepad_entity) {
        gamepad.just_pressed(button)
    } else {
        false
    }
}

/// Check if a gamepad button is pressed
pub fn is_button_pressed(
    gamepads: &Query<&Gamepad>,
    gamepad_entity: Entity,
    button: GamepadButton,
) -> bool {
    if let Ok(gamepad) = gamepads.get(gamepad_entity) {
        gamepad.pressed(button)
    } else {
        false
    }
}

/// Check if D-pad up was just pressed
pub fn is_dpad_up_just_pressed(gamepads: &Query<&Gamepad>, gamepad_entity: Entity) -> bool {
    is_button_just_pressed(gamepads, gamepad_entity, GamepadButton::DPadUp)
}

/// Check if D-pad down was just pressed
pub fn is_dpad_down_just_pressed(gamepads: &Query<&Gamepad>, gamepad_entity: Entity) -> bool {
    is_button_just_pressed(gamepads, gamepad_entity, GamepadButton::DPadDown)
}

/// Check if D-pad left is pressed
pub fn is_dpad_left_pressed(gamepads: &Query<&Gamepad>, gamepad_entity: Entity) -> bool {
    is_button_pressed(gamepads, gamepad_entity, GamepadButton::DPadLeft)
}

/// Check if D-pad right is pressed
pub fn is_dpad_right_pressed(gamepads: &Query<&Gamepad>, gamepad_entity: Entity) -> bool {
    is_button_pressed(gamepads, gamepad_entity, GamepadButton::DPadRight)
}
