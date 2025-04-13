use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::entities::player::Player;

#[derive(Clone, Component, Debug, Eq, Default, PartialEq)]
pub struct Inventory(Vec<String>);

impl From<&EntityInstance> for Inventory {
    fn from(entity_instance: &EntityInstance) -> Self {
        Inventory(
            entity_instance
                .iter_enums_field("items")
                .expect("items field should be correctly typed")
                .cloned()
                .collect(),
        )
    }
}
