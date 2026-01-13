use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::entities::enemy::Enemy;
use crate::entities::player::Player;
use crate::entities::stats::Stats;
use crate::GameState;

/// Composant pour gérer l'invincibilité temporaire après avoir pris des dégâts
#[derive(Component)]
pub struct Invincibility {
    pub timer: Timer,
    pub blink_timer: Timer,
    pub visible: bool,
}

impl Default for Invincibility {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.5, TimerMode::Once),
            blink_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            visible: true,
        }
    }
}

impl Invincibility {
    pub fn new(duration: f32) -> Self {
        Self {
            timer: Timer::from_seconds(duration, TimerMode::Once),
            blink_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            visible: true,
        }
    }
}

/// Composant pour gérer le recul (knockback)
#[derive(Component)]
pub struct Knockback {
    pub velocity: Vec2,
    pub timer: Timer,
}

impl Knockback {
    pub fn new(direction: Vec2, force: f32) -> Self {
        Self {
            velocity: direction.normalize_or_zero() * force,
            timer: Timer::from_seconds(0.2, TimerMode::Once),
        }
    }
}

/// Événement générique de dégâts pour toutes les entités
#[derive(Event)]
pub struct DamageEvent {
    pub target: Entity,
    pub source: Entity,
    pub amount: i32,
    pub knockback_direction: Option<Vec2>,
    pub knockback_force: Option<f32>,
}

impl DamageEvent {
    pub fn new(target: Entity, source: Entity, amount: i32) -> Self {
        Self {
            target,
            source,
            amount,
            knockback_direction: None,
            knockback_force: None,
        }
    }

    pub fn with_knockback(mut self, direction: Vec2, force: f32) -> Self {
        self.knockback_direction = Some(direction);
        self.knockback_force = Some(force);
        self
    }
}

/// Événement déclenché quand une entité meurt
#[derive(Event)]
pub struct DeathEvent {
    pub entity: Entity,
    pub is_player: bool,
}

/// Composant pour le flash visuel lors des dégâts
#[derive(Component)]
pub struct DamageFlash {
    pub timer: Timer,
}

impl Default for DamageFlash {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.1, TimerMode::Once),
        }
    }
}

/// Système qui détecte les collisions entre le joueur et les ennemis
fn handle_player_enemy_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    player_query: Query<Entity, (With<Player>, Without<Invincibility>)>,
    enemy_query: Query<(Entity, &Stats, &Transform), With<Enemy>>,
    player_transform_query: Query<&Transform, With<Player>>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _) = collision_event {
            // Trouver le joueur et l'ennemi dans la collision
            let (player_entity, enemy_data) = if let Ok(player_e) = player_query.get(*entity1) {
                if let Ok(enemy_data) = enemy_query.get(*entity2) {
                    (player_e, enemy_data)
                } else {
                    continue;
                }
            } else if let Ok(player_e) = player_query.get(*entity2) {
                if let Ok(enemy_data) = enemy_query.get(*entity1) {
                    (player_e, enemy_data)
                } else {
                    continue;
                }
            } else {
                continue;
            };

            let (enemy_entity, enemy_stats, enemy_transform) = enemy_data;

            // Calculer la direction du knockback
            let knockback_direction =
                if let Ok(player_transform) = player_transform_query.get(player_entity) {
                    (player_transform.translation - enemy_transform.translation)
                        .truncate()
                        .normalize_or_zero()
                } else {
                    Vec2::X
                };

            // Envoyer l'événement de dégâts avec knockback pour le joueur
            damage_events.send(
                DamageEvent::new(player_entity, enemy_entity, enemy_stats.damage)
                    .with_knockback(knockback_direction, 400.0),
            );
        }
    }
}

/// Système principal qui applique les dégâts à toutes les entités
fn apply_damage(
    mut commands: Commands,
    mut damage_events: EventReader<DamageEvent>,
    mut stats_query: Query<&mut Stats>,
    invincibility_query: Query<&Invincibility>,
    player_query: Query<Entity, With<Player>>,
    mut death_events: EventWriter<DeathEvent>,
) {
    info!("Traitement des événements de dégâts");
    for event in damage_events.read() {
        // Ignorer si l'entité est invincible
        if invincibility_query.get(event.target).is_ok() {
            continue;
        }

        // Appliquer les dégâts aux stats
        if let Ok(mut stats) = stats_query.get_mut(event.target) {
            stats.life -= event.amount;

            let is_player = player_query.get(event.target).is_ok();
            let entity_type = if is_player { "Joueur" } else { "Ennemi" };

            info!(
                "{} {:?} a pris {} dégâts - Vie: {}/{}",
                entity_type, event.target, event.amount, stats.life, stats.max_life
            );

            // Ajouter l'invincibilité temporaire
            let invincibility_duration = if is_player { 3.0 } else { 0.5 };
            commands
                .entity(event.target)
                .insert(Invincibility::new(invincibility_duration));

            // Ajouter le flash de dégâts
            commands.entity(event.target).insert(DamageFlash::default());

            // Ajouter le knockback si spécifié
            if let (Some(direction), Some(force)) =
                (event.knockback_direction, event.knockback_force)
            {
                commands
                    .entity(event.target)
                    .insert(Knockback::new(direction, force));
            }

            // Vérifier la mort
            if stats.life <= 0 {
                death_events.send(DeathEvent {
                    entity: event.target,
                    is_player,
                });
            }
        }
    }
}

/// Système qui gère la mort des entités
fn handle_deaths(
    mut commands: Commands,
    mut death_events: EventReader<DeathEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in death_events.read() {
        if event.is_player {
            info!("Le joueur est mort ! Game Over");
            next_state.set(GameState::GameOver);
        } else {
            info!("Ennemi {:?} est mort", event.entity);
            commands.entity(event.entity).despawn_recursive();
        }
    }
}

/// Système qui met à jour l'invincibilité
fn update_invincibility(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Invincibility, &mut Sprite)>,
) {
    for (entity, mut invincibility, mut sprite) in query.iter_mut() {
        invincibility.timer.tick(time.delta());

        if invincibility.timer.finished() {
            commands.entity(entity).remove::<Invincibility>();
            sprite.color = Color::WHITE;
        }
    }
}

/// Système qui fait clignoter les entités invincibles
fn update_invincibility_blink(
    time: Res<Time>,
    mut query: Query<(&mut Invincibility, &mut Sprite)>,
) {
    for (mut invincibility, mut sprite) in query.iter_mut() {
        invincibility.blink_timer.tick(time.delta());

        if invincibility.blink_timer.just_finished() {
            invincibility.visible = !invincibility.visible;
            sprite.color = if invincibility.visible {
                Color::WHITE
            } else {
                Color::srgba(1.0, 1.0, 1.0, 0.3)
            };
        }
    }
}

/// Système qui applique le knockback
fn update_knockback(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Knockback, &mut Velocity)>,
) {
    for (entity, mut knockback, mut velocity) in query.iter_mut() {
        knockback.timer.tick(time.delta());

        if knockback.timer.finished() {
            commands.entity(entity).remove::<Knockback>();
        } else {
            velocity.linvel = knockback.velocity;
        }
    }
}

/// Système qui gère le flash de dégâts
fn update_damage_flash(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DamageFlash, &mut Sprite)>,
) {
    for (entity, mut flash, mut sprite) in query.iter_mut() {
        // Appliquer la couleur rouge au début
        if flash.timer.elapsed_secs() == 0.0 {
            sprite.color = Color::srgb(1.0, 0.3, 0.3);
        }

        flash.timer.tick(time.delta());

        if flash.timer.finished() {
            sprite.color = Color::WHITE;
            commands.entity(entity).remove::<DamageFlash>();
        }
    }
}

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageEvent>()
            .add_event::<DeathEvent>()
            .add_systems(
                Update,
                (
                    handle_player_enemy_collisions,
                    apply_damage,
                    handle_deaths,
                    update_invincibility,
                    update_invincibility_blink,
                    update_knockback,
                    update_damage_flash,
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}
