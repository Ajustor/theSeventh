use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::engine::damage::{DamageEvent, Invincibility};
use crate::entities::player::{Player, Side};

/// Marqueur pour la hitbox d'attaque
#[derive(Component)]
pub struct AttackHitbox {
    pub damage: i32,
    pub owner: Entity,
}

/// État d'attaque du joueur
#[derive(Component, Default)]
pub struct AttackState {
    pub is_attacking: bool,
    pub attack_timer: Timer,
    pub cooldown_timer: Timer,
    pub can_attack: bool,
}

impl AttackState {
    pub fn new() -> Self {
        Self {
            is_attacking: false,
            attack_timer: Timer::from_seconds(0.2, TimerMode::Once), // Durée de l'attaque
            cooldown_timer: Timer::from_seconds(0.5, TimerMode::Once), // Cooldown entre attaques
            can_attack: true,
        }
    }
}

/// Événement déclenché quand une attaque touche une cible
#[derive(Event)]
pub struct AttackHitEvent {
    pub attacker: Entity,
    pub target: Entity,
    pub damage: i32,
}

pub fn setup_player_attack(
    mut commands: Commands,
    query: Query<Entity, (With<Player>, Without<AttackState>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert(AttackState::new());
    }
}

pub fn handle_attack_input(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(Entity, &Transform, &Side, &mut AttackState), With<Player>>,
    mut commands: Commands,
) {
    for (player_entity, transform, side, mut attack_state) in query.iter_mut() {
        if input.just_pressed(KeyCode::KeyK)
            && attack_state.can_attack
            && !attack_state.is_attacking
        {
            attack_state.is_attacking = true;
            attack_state.can_attack = false;
            attack_state.attack_timer.reset();
            attack_state.cooldown_timer.reset();

            // Calculer la position de la hitbox selon la direction
            let offset_x = match *side {
                Side::Right => 20.0,
                Side::Left => -20.0,
            };

            // Créer la hitbox d'attaque
            commands.spawn((
                AttackHitbox {
                    damage: 1,
                    owner: player_entity,
                },
                Transform::from_translation(Vec3::new(
                    transform.translation.x + offset_x,
                    transform.translation.y,
                    0.0,
                )),
                Collider::cuboid(15.0, 20.0),
                Sensor,
                ActiveEvents::COLLISION_EVENTS,
                GlobalTransform::default(),
            ));

            info!("Attaque lancée vers {:?}", side);
        }
    }
}

pub fn update_attack_state(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<&mut AttackState, With<Player>>,
    hitbox_query: Query<Entity, With<AttackHitbox>>,
) {
    for mut attack_state in query.iter_mut() {
        if attack_state.is_attacking {
            attack_state.attack_timer.tick(time.delta());

            if attack_state.attack_timer.finished() {
                attack_state.is_attacking = false;

                // Supprimer toutes les hitbox d'attaque
                for hitbox_entity in hitbox_query.iter() {
                    commands.entity(hitbox_entity).despawn();
                }
            }
        }

        if !attack_state.can_attack {
            attack_state.cooldown_timer.tick(time.delta());

            if attack_state.cooldown_timer.finished() {
                attack_state.can_attack = true;
            }
        }
    }
}

pub fn detect_attack_hits(
    mut collision_events: EventReader<CollisionEvent>,
    hitbox_query: Query<(&AttackHitbox, &Transform)>,
    target_query: Query<&Transform, Without<Invincibility>>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _) = event {
            // Vérifier si l'une des entités est une hitbox
            let (hitbox_entity, other_entity) = if hitbox_query.get(*entity1).is_ok() {
                (*entity1, *entity2)
            } else if hitbox_query.get(*entity2).is_ok() {
                (*entity2, *entity1)
            } else {
                continue;
            };

            if let Ok((hitbox, hitbox_transform)) = hitbox_query.get(hitbox_entity) {
                // Ne pas se toucher soi-même
                if other_entity == hitbox.owner {
                    continue;
                }

                // Vérifier que la cible existe et n'est pas invincible
                if let Ok(target_transform) = target_query.get(other_entity) {
                    // Calculer la direction du knockback
                    let knockback_direction = (target_transform.translation
                        - hitbox_transform.translation)
                        .truncate()
                        .normalize_or_zero();

                    damage_events.send(
                        DamageEvent::new(other_entity, hitbox.owner, hitbox.damage)
                            .with_knockback(knockback_direction, 300.0),
                    );
                }
            }
        }
    }
}

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AttackHitEvent>()
            .add_event::<DamageEvent>()
            .add_systems(
                Update,
                (
                    setup_player_attack,
                    handle_attack_input,
                    update_attack_state,
                    detect_attack_hits,
                ),
            );
    }
}
