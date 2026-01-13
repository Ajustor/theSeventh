use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::engine::damage::DamageEvent;
use crate::entities::player::{Player, Side};

/// Marqueur pour la hitbox d'attaque
#[derive(Component)]
pub struct AttackHitbox {
    pub damage: i32,
    pub owner: Entity,
}

/// Composant pour animer le slash de l'attaque
#[derive(Component)]
pub struct SlashAnimation {
    pub start_angle: f32,
    pub end_angle: f32,
    pub duration: Timer,
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
            attack_timer: Timer::from_seconds(0.25, TimerMode::Once),
            cooldown_timer: Timer::from_seconds(0.4, TimerMode::Once),
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

            // Configuration selon le côté du joueur
            let (offset_x, start_angle, end_angle) = match *side {
                Side::Right => (
                    16.0,
                    std::f32::consts::FRAC_PI_4,
                    -std::f32::consts::FRAC_PI_2,
                ),
                Side::Left => (
                    -16.0,
                    -std::f32::consts::FRAC_PI_4,
                    std::f32::consts::FRAC_PI_2,
                ),
            };

            // Position de départ (légèrement au-dessus du joueur)
            let spawn_pos = Vec3::new(
                transform.translation.x + offset_x,
                transform.translation.y + 8.0,
                transform.translation.z + 10.0,
            );

            // Créer le slash visuel avec animation
            commands.spawn((
                AttackHitbox {
                    damage: 1,
                    owner: player_entity,
                },
                Sprite {
                    color: Color::srgba(0.9, 0.9, 1.0, 0.9),
                    custom_size: Some(Vec2::new(28.0, 6.0)),
                    ..default()
                },
                Transform::from_translation(spawn_pos)
                    .with_rotation(Quat::from_rotation_z(start_angle)),
                // Ajouter tous les composants nécessaires pour la détection de collision
                Collider::cuboid(14.0, 12.0),
                Sensor,
                ActiveEvents::COLLISION_EVENTS,
                ActiveCollisionTypes::all(),
                CollisionGroups::new(Group::GROUP_1, Group::ALL),
                SlashAnimation {
                    start_angle,
                    end_angle,
                    duration: Timer::from_seconds(0.15, TimerMode::Once),
                },
            ));

            info!("Attaque slash vers {:?}", side);
        }
    }
}

/// Système qui anime le slash (rotation de haut en bas)
pub fn animate_slash(
    time: Res<Time>,
    player_query: Query<(&Transform, &Side), With<Player>>,
    mut slash_query: Query<(&mut SlashAnimation, &mut Transform, &AttackHitbox), Without<Player>>,
) {
    for (mut animation, mut transform, hitbox) in slash_query.iter_mut() {
        animation.duration.tick(time.delta());

        // Interpoler l'angle de rotation
        let progress = animation.duration.fraction();
        // Utiliser une courbe ease-out pour un mouvement plus naturel
        let eased_progress = 1.0 - (1.0 - progress).powi(2);

        let current_angle =
            animation.start_angle + (animation.end_angle - animation.start_angle) * eased_progress;

        transform.rotation = Quat::from_rotation_z(current_angle);

        // Suivre la position du joueur
        if let Ok((player_transform, side)) = player_query.get(hitbox.owner) {
            let offset_x = match *side {
                Side::Right => 16.0,
                Side::Left => -16.0,
            };

            // Ajuster la position Y pendant l'animation (arc de cercle)
            let y_offset = 8.0 - (eased_progress * 16.0);

            transform.translation.x = player_transform.translation.x + offset_x;
            transform.translation.y = player_transform.translation.y + y_offset;
        }

        // Effet de fondu à la fin
        // Note: On pourrait aussi modifier l'alpha du sprite ici
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
    target_query: Query<&Transform>,
    mut damage_events: EventWriter<DamageEvent>,
    mut hit_events: EventWriter<AttackHitEvent>,
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

                // Vérifier que la cible existe
                if let Ok(target_transform) = target_query.get(other_entity) {
                    // Calculer la direction du knockback
                    let knockback_direction = (target_transform.translation
                        - hitbox_transform.translation)
                        .truncate()
                        .normalize_or_zero();

                    hit_events.send(AttackHitEvent {
                        attacker: hitbox.owner,
                        target: other_entity,
                        damage: hitbox.damage,
                    });

                    damage_events.send(
                        DamageEvent::new(other_entity, hitbox.owner, hitbox.damage)
                            .with_knockback(knockback_direction, 350.0),
                    );
                }
            }
        }
    }
}

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AttackHitEvent>().add_systems(
            Update,
            (
                setup_player_attack,
                handle_attack_input,
                animate_slash,
                update_attack_state,
                detect_attack_hits,
            ),
        );
    }
}
