pub mod attack;
pub mod damage;
pub mod health;

use attack::AttackPlugin;
use bevy::prelude::*;
use damage::DamagePlugin;
use health::HealthPlugin;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AttackPlugin, HealthPlugin, DamagePlugin));
    }
}
