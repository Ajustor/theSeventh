use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::physics::colliders::ColliderBundle;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct ChestBundle {
    #[sprite_sheet]
    pub sprite_sheet: Sprite,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct DoorBundle {
    #[sprite_sheet]
    pub sprite_sheet: Sprite,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    // #[from_entity_instance]
    // pub locked_by: EntityRef,
    #[from_entity_instance]
    pub locked: bool,
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PumpkinsBundle {
    #[sprite_sheet(no_grid)]
    pub sprite_sheet: Sprite,
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct LeverBundle {
    #[sprite_sheet(no_grid)]
    pub sprite_sheet: Sprite,

    pub state: bool,
}

pub struct MiscObjectsPlugin;

impl Plugin for MiscObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<ChestBundle>("Chest")
            .register_ldtk_entity::<DoorBundle>("Door")
            .register_ldtk_entity::<PumpkinsBundle>("Pumpkins")
            .register_ldtk_entity::<LeverBundle>("Lever");
    }
}
