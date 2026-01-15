use bevy::prelude::*;
use bevy::input::gamepad::{GamepadConnection, GamepadConnectionEvent};

/// Module centralisé pour gérer les entrées clavier et manette
/// Provides a unified interface for keyboard and gamepad input

/// Resource pour stocker l'entité de la manette active
#[derive(Resource)]
pub struct ActiveGamepad(pub Option<Entity>);

impl Default for ActiveGamepad {
    fn default() -> Self {
        Self(None)
    }
}

/// Actions de mouvement du joueur
#[derive(Default)]
pub struct MovementInput {
    pub horizontal: f32, // -1.0 (gauche) à 1.0 (droite)
    pub vertical: f32,   // -1.0 (bas) à 1.0 (haut)
    pub jump: bool,
}

/// Actions de combat
#[derive(Default)]
pub struct CombatInput {
    pub attack: bool,
    pub interact: bool,
}

/// Actions de menu
#[derive(Default)]
pub struct MenuInput {
    pub up: bool,
    pub down: bool,
    pub confirm: bool,
    pub cancel: bool,
}

/// Système pour détecter et enregistrer les manettes connectées
pub fn gamepad_connections(
    mut active_gamepad: ResMut<ActiveGamepad>,
    mut gamepad_evr: EventReader<GamepadConnectionEvent>,
) {
    for ev in gamepad_evr.read() {
        let gamepad_entity = ev.gamepad;
        match &ev.connection {
            GamepadConnection::Connected { name, .. } => {
                info!(
                    "Manette connectée : {}",
                    name
                );
                // Si aucune manette n'est active, utiliser celle-ci
                if active_gamepad.0.is_none() {
                    active_gamepad.0 = Some(gamepad_entity);
                    info!("Manette définie comme active");
                }
            }
            GamepadConnection::Disconnected => {
                info!("Manette déconnectée");
                // Si c'était la manette active, la retirer
                if let Some(active) = active_gamepad.0 {
                    if active == gamepad_entity {
                        active_gamepad.0 = None;
                        info!("Manette active retirée");
                    }
                }
            }
        }
    }
}

/// Récupère les entrées de mouvement (clavier + manette)
pub fn get_movement_input(
    keyboard: &Res<ButtonInput<KeyCode>>,
    active_gamepad: &Res<ActiveGamepad>,
    gamepads: &Query<&Gamepad>,
) -> MovementInput {
    let mut input = MovementInput::default();

    // Entrées clavier
    let kb_right = if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        1.0
    } else {
        0.0
    };
    let kb_left = if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        1.0
    } else {
        0.0
    };
    let kb_up = if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        1.0
    } else {
        0.0
    };
    let kb_down = if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        1.0
    } else {
        0.0
    };

    input.horizontal = kb_right - kb_left;
    input.vertical = kb_up - kb_down;
    input.jump = keyboard.just_pressed(KeyCode::Space);

    // Entrées manette (prioritaires si présentes)
    if let Some(gamepad_entity) = active_gamepad.0 {
        if let Ok(gamepad) = gamepads.get(gamepad_entity) {
            // Stick analogique gauche pour le mouvement
            let left_stick_x = gamepad.get(GamepadAxis::LeftStickX).unwrap_or(0.0);
            let left_stick_y = gamepad.get(GamepadAxis::LeftStickY).unwrap_or(0.0);

            // Dead zone pour éviter le drift
            const DEADZONE: f32 = 0.15;
            if left_stick_x.abs() > DEADZONE {
                input.horizontal = left_stick_x;
            }
            if left_stick_y.abs() > DEADZONE {
                input.vertical = left_stick_y;
            }

            // D-pad comme alternative
            if gamepad.pressed(GamepadButton::DPadRight) {
                input.horizontal = 1.0;
            }
            if gamepad.pressed(GamepadButton::DPadLeft) {
                input.horizontal = -1.0;
            }
            if gamepad.pressed(GamepadButton::DPadUp) {
                input.vertical = 1.0;
            }
            if gamepad.pressed(GamepadButton::DPadDown) {
                input.vertical = -1.0;
            }

            // Bouton de saut (A/Cross sur la plupart des manettes, ou South button)
            if gamepad.just_pressed(GamepadButton::South) {
                input.jump = true;
            }
        }
    }

    input
}

/// Récupère les entrées de combat (clavier + manette)
pub fn get_combat_input(
    keyboard: &Res<ButtonInput<KeyCode>>,
    active_gamepad: &Res<ActiveGamepad>,
    gamepads: &Query<&Gamepad>,
) -> CombatInput {
    let mut input = CombatInput::default();

    // Entrées clavier
    input.attack = keyboard.just_pressed(KeyCode::KeyK);
    input.interact = keyboard.just_pressed(KeyCode::KeyO);

    // Entrées manette (prioritaires si présentes)
    if let Some(gamepad_entity) = active_gamepad.0 {
        if let Ok(gamepad) = gamepads.get(gamepad_entity) {
            // Bouton X/Square pour attaquer
            if gamepad.just_pressed(GamepadButton::West) {
                input.attack = true;
            }
            // Bouton B/Circle pour interagir
            if gamepad.just_pressed(GamepadButton::East) {
                input.interact = true;
            }
        }
    }

    input
}

/// Récupère les entrées de menu (clavier + manette)
pub fn get_menu_input(
    keyboard: &Res<ButtonInput<KeyCode>>,
    active_gamepad: &Res<ActiveGamepad>,
    gamepads: &Query<&Gamepad>,
) -> MenuInput {
    let mut input = MenuInput::default();

    // Entrées clavier
    input.up = keyboard.just_pressed(KeyCode::ArrowUp) || keyboard.just_pressed(KeyCode::KeyW);
    input.down = keyboard.just_pressed(KeyCode::ArrowDown) || keyboard.just_pressed(KeyCode::KeyS);
    input.confirm = keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space);
    input.cancel = keyboard.just_pressed(KeyCode::Escape);

    // Entrées manette (prioritaires si présentes)
    if let Some(gamepad_entity) = active_gamepad.0 {
        if let Ok(gamepad) = gamepads.get(gamepad_entity) {
            if gamepad.just_pressed(GamepadButton::DPadUp) {
                input.up = true;
            }
            if gamepad.just_pressed(GamepadButton::DPadDown) {
                input.down = true;
            }
            
            // Bouton A/Cross pour confirmer
            if gamepad.just_pressed(GamepadButton::South) {
                input.confirm = true;
            }
            // Bouton B/Circle pour annuler (même bouton que l'interaction en jeu)
            // Note: Cette assignation double est intentionnelle pour la cohérence des contrôles
            if gamepad.just_pressed(GamepadButton::East) {
                input.cancel = true;
            }
        }
    }

    input
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActiveGamepad>()
            .add_systems(Update, gamepad_connections);
    }
}
