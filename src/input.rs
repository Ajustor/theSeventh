use bevy::prelude::*;

/// Resource that tracks whether a gamepad is currently connected
#[derive(Resource, Default)]
pub struct GamepadState {
    /// The first active gamepad (if any)
    pub active_gamepad: Option<Entity>,
}

/// Plugin for gamepad detection and input handling
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GamepadState>()
            .add_systems(Update, gamepad_connection_system);
    }
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
