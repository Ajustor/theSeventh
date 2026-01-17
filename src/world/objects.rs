use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    physics::colliders::ColliderBundle,
    world::save_point::{handle_save_interaction, SaveBundle, SaveGameEvent, SavePoint},
};

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct ChestBundle {
    #[sprite_sheet]
    pub sprite_sheet: Sprite,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
}

#[derive(Clone, Default, Component, Reflect)]
pub struct Locked(pub bool);

impl From<&EntityInstance> for Locked {
    fn from(entity_instance: &EntityInstance) -> Self {
        Locked(*entity_instance.get_bool_field("locked").unwrap_or(&false))
    }
}

#[derive(Clone, Default, Component, Reflect)]
pub struct LockedBy(pub Option<String>);

impl From<&EntityInstance> for LockedBy {
    fn from(entity_instance: &EntityInstance) -> Self {
        LockedBy(entity_instance.get_string_field("locked_by").ok().cloned())
    }
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct DoorBundle {
    #[sprite_sheet]
    pub sprite_sheet: Sprite,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    #[from_entity_instance]
    pub locked: Locked,
    #[from_entity_instance]
    pub locked_by: LockedBy,
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PumpkinsBundle {
    #[sprite_sheet(no_grid)]
    pub sprite_sheet: Sprite,
}

#[derive(Clone, Default, Component, Reflect)]
pub struct IsActive(pub bool);

impl From<&EntityInstance> for IsActive {
    fn from(entity_instance: &EntityInstance) -> Self {
        IsActive(
            *entity_instance
                .get_bool_field("is_active")
                .unwrap_or(&false),
        )
    }
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct LeverBundle {
    #[sprite_sheet(no_grid)]
    pub sprite_sheet: Sprite,
    #[from_entity_instance]
    pub is_active: IsActive,
}

pub struct MiscObjectsPlugin;

impl Plugin for MiscObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SaveGameEvent>()
            .register_type::<Locked>()
            .register_type::<LockedBy>()
            .register_type::<SavePoint>()
            .register_type::<IsActive>()
            .register_ldtk_entity::<ChestBundle>("Chest")
            .register_ldtk_entity::<DoorBundle>("Door")
            .register_ldtk_entity::<PumpkinsBundle>("Pumpkins")
            .register_ldtk_entity::<LeverBundle>("Lever")
            .register_ldtk_entity::<SaveBundle>("Save")
            .add_systems(Update, handle_save_interaction);
    }
}
