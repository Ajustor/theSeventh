use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Clone, Component, Debug, Eq, Default, PartialEq)]
pub struct Stats {
    pub life: i32,
    pub max_life: i32,
}

impl From<&EntityInstance> for Stats {
    fn from(entity_instance: &EntityInstance) -> Self {
        Stats {
            life: *entity_instance
                .get_int_field("life")
                .expect("life stat must be present"),
            max_life: *entity_instance
                .get_int_field("max_life")
                .expect("life stat must be present"),
        }
    }
}
