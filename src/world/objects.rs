use bevy::{prelude::*, state::commands};
use bevy_ecs_ldtk::prelude::*;

use crate::{
    physics::colliders::ColliderBundle,
    world::save_point::{handle_save_interaction, SaveBundle, SaveGameEvent, SavePoint},
};

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct ChestBundle {
    #[sprite_sheet]
    pub sprite_sheet: Sprite,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
}

#[derive(Clone, Default, Component, Reflect)]
pub struct Locked(pub bool);

impl From<&EntityInstance> for Locked {
    fn from(entity_instance: &EntityInstance) -> Self {
        Locked(*entity_instance.get_bool_field("locked").unwrap_or(&false))
    }
}

#[derive(Clone, Default, Component, Reflect)]
pub struct LockedBy {
    pub id: String,
}

impl From<&EntityInstance> for LockedBy {
    fn from(entity_instance: &EntityInstance) -> Self {
        // locked_by est un EntityRef, on récupère l'IID de l'entité référencée
        let entity_ref = entity_instance
            .get_entity_ref_field("locked_by")
            .ok()
            .unwrap();

        LockedBy {
            id: entity_ref.entity_iid.clone(),
        }
    }
}

/// Marqueur pour identifier les portes
#[derive(Clone, Copy, Default, Component)]
pub struct Door;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct DoorBundle {
    #[sprite_sheet]
    pub sprite_sheet: Sprite,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    #[from_entity_instance]
    pub locked: Locked,
    #[from_entity_instance]
    pub locked_by: LockedBy,
    pub door: Door,
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PumpkinsBundle {
    #[sprite_sheet(no_grid)]
    pub sprite_sheet: Sprite,
}

#[derive(Clone, Default, Component, Reflect)]
pub struct IsActive(pub bool);

impl From<&EntityInstance> for IsActive {
    fn from(entity_instance: &EntityInstance) -> Self {
        IsActive(
            *entity_instance
                .get_bool_field("is_active")
                .unwrap_or(&false),
        )
    }
}

/// Marqueur pour identifier les leviers
#[derive(Clone, Copy, Default, Component)]
pub struct Lever;

/// Identifiant du levier pour le lier à une porte (utilise l'IID de l'entité)
#[derive(Clone, Default, Component, Reflect)]
pub struct LeverId(pub String);

impl From<&EntityInstance> for LeverId {
    fn from(entity_instance: &EntityInstance) -> Self {
        // Utiliser l'IID de l'entité comme identifiant unique
        LeverId(entity_instance.iid.clone())
    }
}

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct LeverBundle {
    #[sprite_sheet(no_grid)]
    pub sprite_sheet: Sprite,
    #[from_entity_instance]
    pub is_active: IsActive,
    #[from_entity_instance]
    pub lever_id: LeverId,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    pub lever: Lever,
}

pub struct MiscObjectsPlugin;

/// Ressource pour tracker les mobs morts et leurs identifiants
#[derive(Resource, Default)]
pub struct DeadMobs {
    pub ids: Vec<String>,
}

impl Plugin for MiscObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SaveGameEvent>()
            .init_resource::<DeadMobs>()
            .register_type::<Locked>()
            .register_type::<LockedBy>()
            .register_type::<SavePoint>()
            .register_type::<IsActive>()
            .register_type::<LeverId>()
            .register_ldtk_entity::<ChestBundle>("Chest")
            .register_ldtk_entity::<DoorBundle>("Door")
            .register_ldtk_entity::<PumpkinsBundle>("Pumpkins")
            .register_ldtk_entity::<LeverBundle>("Lever")
            .register_ldtk_entity::<SaveBundle>("Save")
            .add_systems(
                Update,
                (
                    handle_save_interaction,
                    handle_lever_hit,
                    update_doors_from_levers,
                    track_enemy_deaths,
                    update_doors_from_dead_mobs,
                    despawn_open_doors,
                    update_active_levers,
                ),
            );
    }
}

/// Gère les leviers frappés par des attaques
pub fn handle_lever_hit(
    mut collision_events: EventReader<bevy_rapier2d::prelude::CollisionEvent>,
    attack_hitbox_query: Query<&crate::combat::attack::AttackHitbox>,
    mut lever_query: Query<(&mut IsActive, &mut Visibility), With<Lever>>,
) {
    for collision_event in collision_events.read() {
        if let bevy_rapier2d::prelude::CollisionEvent::Started(entity1, entity2, _) =
            collision_event
        {
            // Vérifier si l'une des entités est une hitbox d'attaque
            let is_hitbox_1 = attack_hitbox_query.get(*entity1).is_ok();
            let is_hitbox_2 = attack_hitbox_query.get(*entity2).is_ok();

            if !is_hitbox_1 && !is_hitbox_2 {
                continue;
            }

            let other_entity = if is_hitbox_1 { *entity2 } else { *entity1 };

            // Vérifier si l'autre entité est un levier
            if let Ok((mut is_active, mut visibility)) = lever_query.get_mut(other_entity) {
                if !is_active.0 {
                    // Activer le levier
                    is_active.0 = true;
                    info!("Levier cassé par attaque!");
                }
            }
        }
    }
}

/// Ouvre les portes liées à des leviers activés
pub fn update_doors_from_levers(
    lever_query: Query<(&IsActive, &LeverId), With<Lever>>,
    mut door_query: Query<(&mut Locked, &LockedBy), With<Door>>,
) {
    for (mut locked, locked_by) in door_query.iter_mut() {
        if !locked.0 {
            continue; // Porte déjà ouverte
        }

        // Chercher le levier correspondant par son ID
        for (is_active, lever_id) in lever_query.iter() {
            // Comparer l'ID du levier avec le locked_by de la porte
            if lever_id.0 == locked_by.id && is_active.0 {
                // Ce levier spécifique est activé, ouvrir la porte
                locked.0 = false;
                info!("Porte ouverte par levier '{}'!", locked_by.id);
                break;
            }
        }
    }
}

/// Track les morts d'ennemis
pub fn track_enemy_deaths(
    mut death_events: EventReader<crate::engine::damage::DeathEvent>,
    enemy_query: Query<&crate::entities::enemy::EnemyIid>,
    mut dead_mobs: ResMut<DeadMobs>,
) {
    for event in death_events.read() {
        if !event.is_player {
            // Récupérer l'IID du mob mort
            if let Ok(enemy_iid) = enemy_query.get(event.entity) {
                dead_mobs.ids.push(enemy_iid.0.clone());
                info!(
                    "Mob mort tracké avec IID: {}! Total: {}",
                    enemy_iid.0,
                    dead_mobs.ids.len()
                );
            } else {
                // Fallback si pas d'IID
                dead_mobs.ids.push("mob".to_string());
                info!("Mob mort tracké (sans IID)! Total: {}", dead_mobs.ids.len());
            }
        }
    }
}

/// Ouvre les portes liées à des mobs morts
pub fn update_doors_from_dead_mobs(
    dead_mobs: Res<DeadMobs>,
    mut door_query: Query<(&mut Locked, &mut Visibility, &LockedBy), With<Door>>,
) {
    if dead_mobs.ids.is_empty() {
        return;
    }

    for (mut locked, mut visibility, locked_by) in door_query.iter_mut() {
        if !locked.0 {
            continue; // Porte déjà ouverte
        }

        // Vérifier si l'IID du mob mort correspond à locked_by
        if dead_mobs.ids.contains(&locked_by.id) {
            // Le mob lié à cette porte est mort, ouvrir la porte
            locked.0 = false;
            *visibility = Visibility::Hidden;
            info!("Porte ouverte par mort du mob '{}'!", locked_by.id);
        }
    }
}

/// Despawn les portes qui sont ouvertes
pub fn despawn_open_doors(
    mut commands: Commands,
    door_query: Query<(Entity, &Locked), With<Door>>,
) {
    for (entity, locked) in door_query.iter() {
        if !locked.0 {
            // La porte est ouverte, la despawn
            commands.entity(entity).despawn_recursive();
            info!("Porte despawnée car ouverte");
        }
    }
}

pub fn update_active_levers(mut lever_query: Query<(&IsActive, &mut Visibility), With<Lever>>) {
    for (is_active, mut visibility) in lever_query.iter_mut() {
        // Mettre à jour l'état du levier en actif
        if is_active.0 {
            *visibility = Visibility::Hidden;
        } else {
            *visibility = Visibility::Visible;
        }
    }
}
