use std::time::Duration;

use bevy::ecs::event::EventReader;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::enemy::Enemy;
use crate::entities::player::Player;
use crate::entities::stats::Stats;

#[derive(Component)]
struct FlashingTimer {
    timer: Timer,
}

fn handle_collisions(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut player_query: Query<(Entity, &mut Velocity, &mut Stats), (With<Player>, Without<Enemy>)>,
    enemy_query: Query<(Entity, &Stats), (With<Enemy>, Without<Player>)>,
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

                if let Some((entity, _, mut stats)) = player_entity {
                    if let Some((_, enemy_stats)) = enemy_entity {
                        // Logique pour infliger des dégâts au joueur
                        stats.life -= enemy_stats.damage;
                        commands.entity(entity).insert(FlashingTimer {
                            timer: Timer::new(Duration::from_millis(100), TimerMode::Once),
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

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_collisions, flashing));
    }
}
