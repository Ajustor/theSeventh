use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Clone, Component, Debug, Eq, Default, PartialEq)]
pub struct Stats {
    pub life: i32,
    pub max_life: i32,
    pub damage: i32,
}

impl From<&EntityInstance> for Stats {
    fn from(entity_instance: &EntityInstance) -> Self {
        let mut damage: i32 = 1;
        let mut max_life: i32 = 10;
        let mut life: i32 = 10;

        if let Ok(damage_stat) = entity_instance.get_int_field("damage") {
            damage = *damage_stat;
        }
        if let Ok(life_stat) = entity_instance.get_int_field("life") {
            life = *life_stat;
        }
        if let Ok(max_life_stat) = entity_instance.get_int_field("max_life") {
            max_life = *max_life_stat;
        }

        Stats {
            life,
            max_life,
            damage,
        }
    }
}
