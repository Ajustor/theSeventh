use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use std::fs;
use std::path::PathBuf;

use crate::entities::player::Player;
use crate::entities::stats::Stats;
use crate::world::save_point::SaveGameEvent;

pub use super::{DeleteSaveEvent, LoadGameEvent, SaveData, SaveSlots};

struct Level {
    pub name: &'static str,
    pub id: &'static str,
}

const LEVELS_WITH_SAVE: [Level; 1] = [Level {
    name: "Le depart",
    id: "475fdc50-d380-11f0-8dda-6f08c954d8b4",
}];

#[derive(Resource, Default)]
pub struct PendingLoadEvent(pub Option<usize>);

const SAVE_DIRECTORY: &str = "saves";
const SAVE_FILE_NAME: &str = "savegame.json";

fn get_save_path() -> PathBuf {
    let mut path =
        if let Some(proj_dirs) = directories::ProjectDirs::from("com", "Ajustor", "TheSeventh") {
            proj_dirs.data_dir().to_path_buf()
        } else {
            PathBuf::from(".")
        };
    path.push(SAVE_DIRECTORY);
    path
}

fn get_save_file_path() -> PathBuf {
    let mut path = get_save_path();
    path.push(SAVE_FILE_NAME);
    path
}

pub fn load_save_slots(mut save_slots: ResMut<SaveSlots>) {
    let path = get_save_file_path();

    if path.exists() {
        match fs::read_to_string(&path) {
            Ok(contents) => match serde_json::from_str::<SaveSlots>(&contents) {
                Ok(loaded) => {
                    *save_slots = loaded;
                    info!("Sauvegardes chargées depuis {:?}", path);
                }
                Err(e) => {
                    warn!("Erreur de parsing des sauvegardes: {}", e);
                    *save_slots = SaveSlots::new();
                }
            },
            Err(e) => {
                warn!("Erreur de lecture des sauvegardes: {}", e);
                *save_slots = SaveSlots::new();
            }
        }
    } else {
        info!("Pas de fichier de sauvegarde trouvé, création d'un nouveau");
        *save_slots = SaveSlots::new();
    }
}

fn save_slots_to_file(save_slots: &SaveSlots) {
    let path = get_save_path();

    if let Err(e) = fs::create_dir_all(&path) {
        error!("Impossible de créer le répertoire de sauvegarde: {}", e);
        return;
    }

    let file_path = get_save_file_path();

    match serde_json::to_string_pretty(save_slots) {
        Ok(json) => {
            if let Err(e) = fs::write(&file_path, json) {
                error!("Erreur d'écriture de la sauvegarde: {}", e);
            } else {
                info!("Sauvegarde écrite dans {:?}", file_path);
            }
        }
        Err(e) => {
            error!("Erreur de sérialisation: {}", e);
        }
    }
}

pub fn handle_save_game(
    mut save_events: EventReader<SaveGameEvent>,
    mut save_slots: ResMut<SaveSlots>,
    player_query: Query<(&Transform, &Stats), With<Player>>,
    level_selection: Res<LevelSelection>,
) {
    for _ in save_events.read() {
        let Ok((player_transform, player_stats)) = player_query.get_single() else {
            warn!("Impossible de trouver le joueur pour la sauvegarde");
            continue;
        };

        let slot_index = save_slots.selected_slot;

        // Récupérer l'identifiant du niveau actuel
        let level_id = match level_selection.as_ref() {
            LevelSelection::Identifier(id) => id.clone(),
            LevelSelection::Uid(uid) => format!("level_{}", uid),
            LevelSelection::Iid(iid) => iid.to_string(),
            LevelSelection::Indices(indices) => format!("level_{:?}", indices),
        };

        let save_data = SaveData::new(
            player_transform.translation,
            LEVELS_WITH_SAVE
                .iter()
                .find(|lvl| lvl.id == level_id.as_str())
                .map(|lvl| lvl.name)
                .unwrap_or("Unknown Level")
                .to_string(),
            player_stats.max_life,
        );

        save_slots.slots[slot_index].data = Some(save_data);
        save_slots_to_file(&save_slots);

        info!("Jeu sauvegardé dans l'emplacement {}", slot_index + 1);
    }
}

pub fn handle_load_game(
    mut load_events: EventReader<LoadGameEvent>,
    save_slots: Res<SaveSlots>,
    mut player_query: Query<(&mut Transform, &mut Stats), With<Player>>,
    mut pending_load: ResMut<PendingLoadEvent>,
) {
    for event in load_events.read() {
        info!("handle_load_game appelé pour slot {}", event.slot_index);
        let slot = &save_slots.slots[event.slot_index];

        if let Some(ref data) = slot.data {
            if let Ok((mut player_transform, mut player_stats)) = player_query.get_single_mut() {
                player_transform.translation = Vec3::new(
                    data.player_position[0],
                    data.player_position[1],
                    data.player_position[2],
                );
                player_stats.max_life = data.player_max_health;
                player_stats.life = data.player_max_health;
                info!("Jeu chargé depuis l'emplacement {}", event.slot_index + 1);
            } else {
                // Joueur pas encore spawné, on stocke l'événement en attente
                info!(
                    "Joueur pas trouvé, stockage en attente pour slot {}",
                    event.slot_index
                );
                pending_load.0 = Some(event.slot_index);
            }
        } else {
            warn!("Emplacement {} vide", event.slot_index + 1);
        }
    }
}

pub fn process_pending_load(
    mut pending_load: ResMut<PendingLoadEvent>,
    save_slots: Res<SaveSlots>,
    mut player_query: Query<(&mut Transform, &mut Stats), With<Player>>,
) {
    if let Some(slot_index) = pending_load.0.take() {
        info!("process_pending_load traite slot {}", slot_index);
        let slot = &save_slots.slots[slot_index];

        if let Some(ref data) = slot.data {
            if let Ok((mut player_transform, mut player_stats)) = player_query.get_single_mut() {
                info!("Joueur trouvé, chargement en cours...");
                player_transform.translation = Vec3::new(
                    data.player_position[0],
                    data.player_position[1],
                    data.player_position[2],
                );
                player_stats.max_life = data.player_max_health;
                player_stats.life = data.player_max_health;
                info!("Jeu chargé depuis l'emplacement {}", slot_index + 1);
            } else {
                info!("Joueur toujours pas trouvé dans process_pending_load");
            }
        }
    }
}

pub fn handle_delete_save(
    mut delete_events: EventReader<DeleteSaveEvent>,
    mut save_slots: ResMut<SaveSlots>,
) {
    for event in delete_events.read() {
        save_slots.slots[event.slot_index].data = None;
        save_slots_to_file(&save_slots);
        info!("Sauvegarde {} supprimée", event.slot_index + 1);
    }
}
