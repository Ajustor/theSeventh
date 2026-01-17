use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    config::KeyBindings,
    entities::player::Player,
    input::{get_left_stick_y, GamepadState},
    save::SaveMenuState,
};

#[derive(Clone, Default, Component, Reflect)]
pub struct SavePoint;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct SaveBundle {
    #[sprite_sheet(no_grid)]
    pub sprite_sheet: Sprite,
    pub save_point: SavePoint,
}

#[derive(Event)]
pub struct SaveGameEvent {
    pub position: Vec3,
}

pub fn handle_save_interaction(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    save_points: Query<&GlobalTransform, With<SavePoint>>,
    player: Query<&GlobalTransform, With<crate::entities::player::Player>>,
    current_state: Option<Res<State<SaveMenuState>>>,
    mut next_state: Option<ResMut<NextState<SaveMenuState>>>,
) {
    // Vérifier que les ressources d'état existent
    let Some(current_state) = current_state else {
        return;
    };
    let Some(ref mut next_state) = next_state else {
        return;
    };

    // Ne pas interagir si le menu est déjà ouvert
    if *current_state.get() == SaveMenuState::Open {
        return;
    }

    if keyboard_input.just_pressed(KeyCode::ArrowUp) || keyboard_input.just_pressed(KeyCode::KeyW) {
        let Ok(player_transform) = player.get_single() else {
            return;
        };

        let player_pos = player_transform.translation();

        for save_transform in save_points.iter() {
            let save_pos = save_transform.translation();
            let distance = player_pos.truncate().distance(save_pos.truncate());

            if distance < 32.0 {
                info!("Ouverture du menu de sauvegarde");
                next_state.set(SaveMenuState::Open);
                return;
            }
        }
    }
}
