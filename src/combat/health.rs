use bevy::prelude::*;

use crate::{entities::player::Player, GameState};

use super::attack::AttackHitEvent;

/// Composant de santé pour toute entité pouvant subir des dégâts
#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
    pub invincibility_timer: Timer,
    pub is_invincible: bool,
}

impl Health {
    pub fn new(max: i32) -> Self {
        Self {
            current: max,
            max,
            invincibility_timer: Timer::from_seconds(0.5, TimerMode::Once),
            is_invincible: false,
        }
    }

    pub fn take_damage(&mut self, amount: i32) -> bool {
        if self.is_invincible {
            return false;
        }

        self.current = (self.current - amount).max(0);
        self.is_invincible = true;
        self.invincibility_timer.reset();

        true
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0
    }

    pub fn heal(&mut self, amount: i32) {
        self.current = (self.current + amount).min(self.max);
    }
}

impl Default for Health {
    fn default() -> Self {
        Self::new(100)
    }
}

/// Événement déclenché quand une entité meurt
#[derive(Event)]
pub struct DeathEvent {
    pub entity: Entity,
}

pub fn apply_damage_from_attacks(
    mut hit_events: EventReader<AttackHitEvent>,
    mut health_query: Query<&mut Health>,
) {
    for event in hit_events.read() {
        if let Ok(mut health) = health_query.get_mut(event.target) {
            if health.take_damage(event.damage) {
                info!(
                    "Entité {:?} a pris {} dégâts. Vie restante: {}/{}",
                    event.target, event.damage, health.current, health.max
                );
            }
        }
    }
}

pub fn update_invincibility(time: Res<Time>, mut query: Query<&mut Health>) {
    for mut health in query.iter_mut() {
        if health.is_invincible {
            health.invincibility_timer.tick(time.delta());

            if health.invincibility_timer.finished() {
                health.is_invincible = false;
            }
        }
    }
}

pub fn check_deaths(
    query: Query<(Entity, &Health), With<Player>>,
    mut death_events: EventWriter<DeathEvent>,
    mut commands: Commands,
    mut app_state: ResMut<NextState<GameState>>,
) {
    for (entity, health) in query.iter() {
        if health.is_dead() {
            death_events.send(DeathEvent { entity });
            commands.entity(entity).despawn_recursive();
            app_state.set(GameState::GameOver);
            info!("Player is dead! Game Over.");
        }
    }
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DeathEvent>().add_systems(
            Update,
            (
                apply_damage_from_attacks,
                update_invincibility,
                check_deaths,
            ),
        );
    }
}
