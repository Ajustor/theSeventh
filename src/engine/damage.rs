use std::time::Duration;

use bevy::ecs::event::EventReader;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::entities::enemy::Enemy;
use crate::entities::player::{Invincibility, Player, PlayerDamagedEvent};
use crate::entities::stats::Stats;

#[derive(Component)]
struct FlashingTimer {
    timer: Timer,
}

/// Événement déclenché quand un mob prend des dégâts
#[derive(Event)]
pub struct DamagedEvent {
    pub target: Entity,
    pub source: Entity,
    pub amount: i32,
}

fn handle_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    mut player_query: Query<(Entity, &mut Velocity), (With<Player>, Without<Enemy>)>,
    enemy_query: Query<(Entity, &Stats), (With<Enemy>, Without<Player>)>,
    mut player_damaged_events: EventWriter<PlayerDamagedEvent>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => {
                let player_entity = if player_query.get(*entity1).is_ok() {
                    player_query.get_mut(*entity1).ok()
                } else {
                    player_query.get_mut(*entity2).ok()
                };

                let enemy_entity = enemy_query
                    .get(*entity1)
                    .ok()
                    .or_else(|| enemy_query.get(*entity2).ok());

                if let Some((entity, velocity)) = player_entity {
                    if let Some((_, enemy_stats)) = enemy_entity {
                        player_damaged_events.send(PlayerDamagedEvent {
                            player_entity: entity,
                            damage: enemy_stats.damage,
                            knockback_direction: -velocity.linvel.normalize_or_zero(),
                        });
                    }
                }
            }
            CollisionEvent::Stopped(_, _, _) => {}
        }
    }
}

fn flashing(
    mut commands: Commands,
    mut flashing_query: Query<(&mut FlashingTimer, Entity, &mut Sprite)>,
    time: Res<Time>,
) {
    for (mut timer, timer_e, mut timer_sprite) in flashing_query.iter_mut() {
        timer_sprite.color = Color::srgba(255., 255., 255., 1.); // bright white color

        timer.timer.tick(time.delta());

        if timer.timer.finished() {
            timer_sprite.color = Color::srgba(1.0, 1.0, 1.0, 1.0); // resets the color back to normal
            commands.entity(timer_e).remove::<FlashingTimer>(); // removes the FlashingTimer component from the entity
        }
    }
}

/// Système qui applique les dégâts aux entités (sauf le joueur qui a son propre système)
pub fn apply_damage(
    mut commands: Commands,
    mut damage_events: EventReader<DamagedEvent>,
    mut stats_query: Query<&mut Stats, Without<Player>>,
    player_query: Query<Entity, (With<Player>, Without<Invincibility>)>,
    transform_query: Query<&Transform>,
    mut player_damaged_events: EventWriter<PlayerDamagedEvent>,
) {
    for event in damage_events.read() {
        // Si c'est le joueur, utiliser le système dédié
        if player_query.get(event.target).is_ok() {
            // Calculer la direction du knockback
            let knockback_direction = if let (Ok(target_transform), Ok(source_transform)) = (
                transform_query.get(event.target),
                transform_query.get(event.source),
            ) {
                (target_transform.translation - source_transform.translation)
                    .truncate()
                    .normalize_or_zero()
            } else {
                Vec2::X // Direction par défaut
            };

            player_damaged_events.send(PlayerDamagedEvent {
                player_entity: event.target,
                damage: event.amount,
                knockback_direction,
            });
            continue;
        }

        // Pour les autres entités, appliquer les dégâts directement
        if let Ok(mut stats) = stats_query.get_mut(event.target) {
            stats.life -= event.amount;
            info!(
                "Entité {:?} a pris {} dégâts {}/{}",
                event.target, event.amount, stats.life, stats.max_life
            );

            if stats.life <= 0 {
                commands.entity(event.target).despawn_recursive();
                info!("Entité {:?} est morte", event.target);
            }
        }
    }
}

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_collisions, flashing, apply_damage));
    }
}
