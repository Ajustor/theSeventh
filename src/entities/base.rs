use bevy::ecs::component::Component;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Component)]
pub struct Stats {
    pub life: u8,
    pub max_life: u8,
}

impl Default for Stats {
    fn default() -> Self {
        Stats {
            life: 10,
            max_life: 10,
        }
    }
}
