use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::*, utils::ldtk_pixel_coords_to_translation_pivoted};
use bevy_rapier2d::dynamics::Velocity;
use bevy_rapier2d::prelude::*;

use crate::combat::health::Health;
use crate::{colliders::ColliderBundle, entities::stats::Stats};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Enemy;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct MobBundle {
    #[sprite_sheet]
    pub sprite_sheet: Sprite,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    pub enemy: Enemy,
    #[ldtk_entity]
    pub patrol: Patrol,

    #[from_entity_instance]
    pub stats: Stats,
}

#[derive(Clone, PartialEq, Debug, Default, Component)]
pub struct Patrol {
    pub points: Vec<Vec2>,
    pub index: usize,
    pub forward: bool,
}

impl LdtkEntity for Patrol {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        layer_instance: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        _: &AssetServer,
        _: &mut Assets<TextureAtlasLayout>,
    ) -> Patrol {
        let mut points = Vec::new();
        points.push(ldtk_pixel_coords_to_translation_pivoted(
            entity_instance.px,
            layer_instance.c_hei * layer_instance.grid_size,
            IVec2::new(entity_instance.width, entity_instance.height),
            entity_instance.pivot,
        ));

        let ldtk_patrol_points = entity_instance
            .iter_points_field("patrol")
            .expect("patrol field should be correclty typed");

        for ldtk_point in ldtk_patrol_points {
            // The +1 is necessary here due to the pivot of the entities in the sample
            // file.
            // The patrols set up in the file look flat and grounded,
            // but technically they're not if you consider the pivot,
            // which is at the bottom-center for the skulls.
            let pixel_coords = (ldtk_point.as_vec2() + Vec2::new(0.5, 1.))
                * Vec2::splat(layer_instance.grid_size as f32);

            points.push(ldtk_pixel_coords_to_translation_pivoted(
                pixel_coords.as_ivec2(),
                layer_instance.c_hei * layer_instance.grid_size,
                IVec2::new(entity_instance.width, entity_instance.height),
                entity_instance.pivot,
            ));
        }

        Patrol {
            points,
            index: 1,
            forward: true,
        }
    }
}

pub fn patrol(mut query: Query<(&mut Transform, &mut Velocity, &mut Patrol)>) {
    for (mut transform, mut velocity, mut patrol) in &mut query {
        if patrol.points.len() <= 1 {
            continue;
        }

        let mut new_velocity =
            (patrol.points[patrol.index] - transform.translation.truncate()).normalize() * 75.;

        if new_velocity.dot(velocity.linvel) < 0. {
            if patrol.index == 0 {
                patrol.forward = true;
            } else if patrol.index == patrol.points.len() - 1 {
                patrol.forward = false;
            }

            transform.translation.x = patrol.points[patrol.index].x;
            transform.translation.y = patrol.points[patrol.index].y;

            if patrol.forward {
                patrol.index += 1;
            } else {
                patrol.index -= 1;
            }

            new_velocity =
                (patrol.points[patrol.index] - transform.translation.truncate()).normalize() * 75.;
        }

        velocity.linvel = new_velocity;
    }
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub health: Health,
    pub sprite: Sprite,
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for EnemyBundle {
    fn default() -> Self {
        Self {
            enemy: Enemy,
            health: Health::new(100),
            sprite: Sprite {
                color: Color::srgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            collider: Collider::cuboid(16.0, 16.0),
            rigid_body: RigidBody::Dynamic,
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
        }
    }
}

pub fn spawn_test_enemy(mut commands: Commands) {
    commands.spawn(EnemyBundle {
        transform: Transform::from_translation(Vec3::new(200.0, 100.0, 0.0)),
        ..default()
    });
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, patrol)
            .register_ldtk_entity::<MobBundle>("Mob");
    }
}
