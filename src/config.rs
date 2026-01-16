use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Serializable key binding storage (uses strings)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBindingsConfig {
    pub move_left: String,
    pub move_right: String,
    pub move_up: String,
    pub move_down: String,
    pub jump: String,
    pub attack: String,
    pub interact: String,
}

impl Default for KeyBindingsConfig {
    fn default() -> Self {
        Self {
            move_left: "KeyA".to_string(),
            move_right: "KeyD".to_string(),
            move_up: "KeyW".to_string(),
            move_down: "KeyS".to_string(),
            jump: "Space".to_string(),
            attack: "KeyK".to_string(),
            interact: "KeyO".to_string(),
        }
    }
}

/// Runtime key bindings with actual KeyCode values
#[derive(Debug, Clone, Resource)]
pub struct KeyBindings {
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub move_up: KeyCode,
    pub move_down: KeyCode,
    pub jump: KeyCode,
    pub attack: KeyCode,
    pub interact: KeyCode,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            move_left: KeyCode::KeyA,
            move_right: KeyCode::KeyD,
            move_up: KeyCode::KeyW,
            move_down: KeyCode::KeyS,
            jump: KeyCode::Space,
            attack: KeyCode::KeyK,
            interact: KeyCode::KeyO,
        }
    }
}

impl From<&KeyBindingsConfig> for KeyBindings {
    fn from(config: &KeyBindingsConfig) -> Self {
        Self {
            move_left: string_to_keycode(&config.move_left),
            move_right: string_to_keycode(&config.move_right),
            move_up: string_to_keycode(&config.move_up),
            move_down: string_to_keycode(&config.move_down),
            jump: string_to_keycode(&config.jump),
            attack: string_to_keycode(&config.attack),
            interact: string_to_keycode(&config.interact),
        }
    }
}

impl From<&KeyBindings> for KeyBindingsConfig {
    fn from(bindings: &KeyBindings) -> Self {
        Self {
            move_left: keycode_to_config_string(bindings.move_left),
            move_right: keycode_to_config_string(bindings.move_right),
            move_up: keycode_to_config_string(bindings.move_up),
            move_down: keycode_to_config_string(bindings.move_down),
            jump: keycode_to_config_string(bindings.jump),
            attack: keycode_to_config_string(bindings.attack),
            interact: keycode_to_config_string(bindings.interact),
        }
    }
}

/// Convert a string to KeyCode
pub fn string_to_keycode(s: &str) -> KeyCode {
    match s {
        "KeyA" => KeyCode::KeyA,
        "KeyB" => KeyCode::KeyB,
        "KeyC" => KeyCode::KeyC,
        "KeyD" => KeyCode::KeyD,
        "KeyE" => KeyCode::KeyE,
        "KeyF" => KeyCode::KeyF,
        "KeyG" => KeyCode::KeyG,
        "KeyH" => KeyCode::KeyH,
        "KeyI" => KeyCode::KeyI,
        "KeyJ" => KeyCode::KeyJ,
        "KeyK" => KeyCode::KeyK,
        "KeyL" => KeyCode::KeyL,
        "KeyM" => KeyCode::KeyM,
        "KeyN" => KeyCode::KeyN,
        "KeyO" => KeyCode::KeyO,
        "KeyP" => KeyCode::KeyP,
        "KeyQ" => KeyCode::KeyQ,
        "KeyR" => KeyCode::KeyR,
        "KeyS" => KeyCode::KeyS,
        "KeyT" => KeyCode::KeyT,
        "KeyU" => KeyCode::KeyU,
        "KeyV" => KeyCode::KeyV,
        "KeyW" => KeyCode::KeyW,
        "KeyX" => KeyCode::KeyX,
        "KeyY" => KeyCode::KeyY,
        "KeyZ" => KeyCode::KeyZ,
        "Space" => KeyCode::Space,
        "Enter" => KeyCode::Enter,
        "Escape" => KeyCode::Escape,
        "ArrowUp" => KeyCode::ArrowUp,
        "ArrowDown" => KeyCode::ArrowDown,
        "ArrowLeft" => KeyCode::ArrowLeft,
        "ArrowRight" => KeyCode::ArrowRight,
        "ShiftLeft" => KeyCode::ShiftLeft,
        "ShiftRight" => KeyCode::ShiftRight,
        "ControlLeft" => KeyCode::ControlLeft,
        "ControlRight" => KeyCode::ControlRight,
        "AltLeft" => KeyCode::AltLeft,
        "AltRight" => KeyCode::AltRight,
        "Tab" => KeyCode::Tab,
        "Digit1" => KeyCode::Digit1,
        "Digit2" => KeyCode::Digit2,
        "Digit3" => KeyCode::Digit3,
        "Digit4" => KeyCode::Digit4,
        "Digit5" => KeyCode::Digit5,
        "Digit6" => KeyCode::Digit6,
        "Digit7" => KeyCode::Digit7,
        "Digit8" => KeyCode::Digit8,
        "Digit9" => KeyCode::Digit9,
        "Digit0" => KeyCode::Digit0,
        _ => KeyCode::KeyA, // Default fallback
    }
}

/// Convert KeyCode to config string
pub fn keycode_to_config_string(key: KeyCode) -> String {
    match key {
        KeyCode::KeyA => "KeyA".to_string(),
        KeyCode::KeyB => "KeyB".to_string(),
        KeyCode::KeyC => "KeyC".to_string(),
        KeyCode::KeyD => "KeyD".to_string(),
        KeyCode::KeyE => "KeyE".to_string(),
        KeyCode::KeyF => "KeyF".to_string(),
        KeyCode::KeyG => "KeyG".to_string(),
        KeyCode::KeyH => "KeyH".to_string(),
        KeyCode::KeyI => "KeyI".to_string(),
        KeyCode::KeyJ => "KeyJ".to_string(),
        KeyCode::KeyK => "KeyK".to_string(),
        KeyCode::KeyL => "KeyL".to_string(),
        KeyCode::KeyM => "KeyM".to_string(),
        KeyCode::KeyN => "KeyN".to_string(),
        KeyCode::KeyO => "KeyO".to_string(),
        KeyCode::KeyP => "KeyP".to_string(),
        KeyCode::KeyQ => "KeyQ".to_string(),
        KeyCode::KeyR => "KeyR".to_string(),
        KeyCode::KeyS => "KeyS".to_string(),
        KeyCode::KeyT => "KeyT".to_string(),
        KeyCode::KeyU => "KeyU".to_string(),
        KeyCode::KeyV => "KeyV".to_string(),
        KeyCode::KeyW => "KeyW".to_string(),
        KeyCode::KeyX => "KeyX".to_string(),
        KeyCode::KeyY => "KeyY".to_string(),
        KeyCode::KeyZ => "KeyZ".to_string(),
        KeyCode::Space => "Space".to_string(),
        KeyCode::Enter => "Enter".to_string(),
        KeyCode::Escape => "Escape".to_string(),
        KeyCode::ArrowUp => "ArrowUp".to_string(),
        KeyCode::ArrowDown => "ArrowDown".to_string(),
        KeyCode::ArrowLeft => "ArrowLeft".to_string(),
        KeyCode::ArrowRight => "ArrowRight".to_string(),
        KeyCode::ShiftLeft => "ShiftLeft".to_string(),
        KeyCode::ShiftRight => "ShiftRight".to_string(),
        KeyCode::ControlLeft => "ControlLeft".to_string(),
        KeyCode::ControlRight => "ControlRight".to_string(),
        KeyCode::AltLeft => "AltLeft".to_string(),
        KeyCode::AltRight => "AltRight".to_string(),
        KeyCode::Tab => "Tab".to_string(),
        KeyCode::Digit1 => "Digit1".to_string(),
        KeyCode::Digit2 => "Digit2".to_string(),
        KeyCode::Digit3 => "Digit3".to_string(),
        KeyCode::Digit4 => "Digit4".to_string(),
        KeyCode::Digit5 => "Digit5".to_string(),
        KeyCode::Digit6 => "Digit6".to_string(),
        KeyCode::Digit7 => "Digit7".to_string(),
        KeyCode::Digit8 => "Digit8".to_string(),
        KeyCode::Digit9 => "Digit9".to_string(),
        KeyCode::Digit0 => "Digit0".to_string(),
        _ => format!("{:?}", key),
    }
}

/// Audio configuration
#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
pub struct AudioConfig {
    /// Master volume (0.0 to 1.0)
    pub master_volume: f32,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            master_volume: 0.35, // 35% by default
        }
    }
}

/// Serializable game configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfigData {
    pub key_bindings: KeyBindingsConfig,
    pub audio: AudioConfig,
}

impl Default for GameConfigData {
    fn default() -> Self {
        Self {
            key_bindings: KeyBindingsConfig::default(),
            audio: AudioConfig::default(),
        }
    }
}

/// Runtime game configuration resource
#[derive(Debug, Clone, Resource)]
pub struct GameConfig {
    pub key_bindings: KeyBindings,
    pub audio: AudioConfig,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            key_bindings: KeyBindings::default(),
            audio: AudioConfig::default(),
        }
    }
}

impl From<GameConfigData> for GameConfig {
    fn from(data: GameConfigData) -> Self {
        Self {
            key_bindings: KeyBindings::from(&data.key_bindings),
            audio: data.audio,
        }
    }
}

impl GameConfig {
    /// Get the configuration file path
    fn config_path() -> PathBuf {
        PathBuf::from("config.json")
    }

    /// Load configuration from file, or return default if file doesn't exist
    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            match fs::read_to_string(&path) {
                Ok(content) => match serde_json::from_str::<GameConfigData>(&content) {
                    Ok(data) => {
                        info!("Configuration loaded from {:?}", path);
                        return GameConfig::from(data);
                    }
                    Err(e) => {
                        warn!("Failed to parse config file: {}. Using defaults.", e);
                    }
                },
                Err(e) => {
                    warn!("Failed to read config file: {}. Using defaults.", e);
                }
            }
        } else {
            info!("No config file found. Using defaults.");
        }
        Self::default()
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<(), String> {
        let path = Self::config_path();
        let data = GameConfigData {
            key_bindings: KeyBindingsConfig::from(&self.key_bindings),
            audio: self.audio.clone(),
        };
        match serde_json::to_string_pretty(&data) {
            Ok(content) => match fs::write(&path, content) {
                Ok(_) => {
                    info!("Configuration saved to {:?}", path);
                    Ok(())
                }
                Err(e) => Err(format!("Failed to write config file: {}", e)),
            },
            Err(e) => Err(format!("Failed to serialize config: {}", e)),
        }
    }
}

/// Plugin for configuration management
pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        let config = GameConfig::load();
        let volume = config.audio.master_volume;

        // Insert resources
        app.insert_resource(config.key_bindings.clone())
            .insert_resource(config.audio.clone())
            .insert_resource(config)
            .insert_resource(GlobalVolume::new(volume));
    }
}
