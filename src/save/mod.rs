mod data;
mod game_start_menu;
mod persistence;
mod ui;

pub use data::*;
pub use game_start_menu::*;
pub use persistence::*;
pub use ui::*;

use crate::GameState;
use bevy::prelude::*;

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SaveMenuState>()
            .init_state::<GameStartMenuState>()
            .init_resource::<SaveSlots>()
            .init_resource::<PendingLoadEvent>()
            .add_event::<LoadGameEvent>()
            .add_event::<DeleteSaveEvent>()
            .add_event::<StartNewGameEvent>()
            .add_systems(Startup, load_save_slots)
            .add_systems(
                Update,
                (
                    handle_save_game,
                    handle_load_game,
                    process_pending_load,
                    handle_delete_save,
                    handle_start_new_game,
                ),
            )
            // Menu de sauvegarde
            .add_systems(OnEnter(SaveMenuState::Open), spawn_save_menu)
            .add_systems(OnExit(SaveMenuState::Open), despawn_save_menu)
            .add_systems(
                Update,
                (save_menu_interaction, save_menu_keyboard_navigation)
                    .run_if(in_state(SaveMenuState::Open)),
            )
            // Menu de démarrage de partie
            .add_systems(OnEnter(GameStartMenuState::Open), spawn_game_start_menu)
            .add_systems(OnExit(GameStartMenuState::Open), despawn_game_start_menu)
            .add_systems(
                Update,
                (
                    game_start_menu_interaction,
                    game_start_menu_keyboard,
                    toggle_game_start_menu_visibility,
                )
                    .run_if(in_state(GameStartMenuState::Open)),
            );
    }
}

fn handle_start_new_game(
    mut events: EventReader<StartNewGameEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for _ in events.read() {
        next_state.set(GameState::InGame);
    }
}
