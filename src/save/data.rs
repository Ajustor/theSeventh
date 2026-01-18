use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

pub const MAX_SAVE_SLOTS: usize = 3;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SaveData {
    pub player_position: [f32; 3],
    pub current_level: String,
    pub player_max_health: i32,
    pub inventory: Vec<String>,
    pub timestamp: u64,
    pub play_time_seconds: u64,
    /// Map de l'état des leviers: (level_id, lever_entity_id) -> is_active
    #[serde(default)]
    pub lever_states: std::collections::HashMap<String, bool>,
}

impl SaveData {
    pub fn new(position: Vec3, level: String, max_health: i32) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        Self {
            player_position: [position.x, position.y, position.z],
            current_level: level,
            player_max_health: max_health,
            inventory: Vec::new(),
            timestamp,
            play_time_seconds: 0,
            lever_states: std::collections::HashMap::new(),
        }
    }

    pub fn formatted_date(&self) -> String {
        use std::time::{Duration, UNIX_EPOCH};
        let datetime = UNIX_EPOCH + Duration::from_secs(self.timestamp);
        // Format simple sans dépendance externe
        format!("Sauvegarde - {}s depuis epoch", self.timestamp)
    }

    pub fn formatted_playtime(&self) -> String {
        let hours = self.play_time_seconds / 3600;
        let minutes = (self.play_time_seconds % 3600) / 60;
        format!("{:02}h {:02}m", hours, minutes)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SaveSlot {
    pub slot_index: usize,
    pub data: Option<SaveData>,
}

#[derive(Resource, Default, Serialize, Deserialize)]
pub struct SaveSlots {
    pub slots: [SaveSlot; MAX_SAVE_SLOTS],
    pub selected_slot: usize,
}

impl SaveSlots {
    pub fn new() -> Self {
        Self {
            slots: [
                SaveSlot {
                    slot_index: 0,
                    data: None,
                },
                SaveSlot {
                    slot_index: 1,
                    data: None,
                },
                SaveSlot {
                    slot_index: 2,
                    data: None,
                },
            ],
            selected_slot: 0,
        }
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SaveMenuState {
    #[default]
    Closed,
    Open,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameStartMenuState {
    #[default]
    Closed,
    Open,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SaveMenuMode {
    #[default]
    Save,
    Load,
}

#[derive(Resource, Default)]
pub struct CurrentSaveMenuMode(pub SaveMenuMode);

#[derive(Event)]
pub struct LoadGameEvent {
    pub slot_index: usize,
}

#[derive(Event)]
pub struct DeleteSaveEvent {
    pub slot_index: usize,
}
