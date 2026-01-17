pub mod attack;

use attack::AttackPlugin;
use bevy::prelude::*;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AttackPlugin));
    }
}
