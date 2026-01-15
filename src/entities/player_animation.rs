use bevy::prelude::*;

use super::player::{Player, Side};
use crate::physics::climbing::Climber;
use crate::physics::ground_detection::GroundDetection;

/// États d'animation du joueur
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Component)]
pub enum PlayerAnimationState {
    #[default]
    Idle,
    Run,
    Jump,
    Fall,
    Climb,
    Attack,
    Hurt,
}

/// Configuration des animations (à adapter selon ton atlas)
#[derive(Resource)]
pub struct PlayerAnimationConfig {
    pub idle: AnimationIndices,
    pub run: AnimationIndices,
    pub jump: AnimationIndices,
    pub fall: AnimationIndices,
    pub climb: AnimationIndices,
    pub attack: AnimationIndices,
    pub hurt: AnimationIndices,
}

#[derive(Clone)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

impl Default for PlayerAnimationConfig {
    fn default() -> Self {
        // TODO: Adapter ces indices selon ton spritesheet
        Self {
            idle: AnimationIndices { first: 0, last: 3 },
            run: AnimationIndices { first: 6, last: 11 },
            jump: AnimationIndices {
                first: 30,
                last: 30,
            },
            fall: AnimationIndices {
                first: 31,
                last: 31,
            },
            climb: AnimationIndices {
                first: 12,
                last: 15,
            },
            attack: AnimationIndices {
                first: 18,
                last: 20,
            },
            hurt: AnimationIndices {
                first: 24,
                last: 25,
            },
        }
    }
}

pub fn setup_player_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    query: Query<Entity, (With<Player>, Without<Sprite>)>,
) {
    for entity in query.iter() {
        // TODO: Adapter le chemin et les dimensions selon ton atlas
        let texture: Handle<Image> = asset_server.load("atlas/SunnyLand-player.png");

        // Créer le layout de l'atlas (adapter les dimensions)
        let layout = TextureAtlasLayout::from_grid(
            UVec2::new(32, 32), // Taille d'une frame
            6,                  // Colonnes
            6,                  // Lignes
            None,               // Padding
            None,               // Offset
        );
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        commands.entity(entity).insert((
            Sprite::from_atlas_image(
                texture,
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: 0,
                },
            ),
            PlayerAnimationState::default(),
            AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
        ));
    }
}

pub fn update_player_animation_state(
    mut query: Query<
        (
            &mut PlayerAnimationState,
            &bevy_rapier2d::dynamics::Velocity,
            &GroundDetection,
            &Climber,
        ),
        With<Player>,
    >,
) {
    for (mut anim_state, velocity, ground_detection, climber) in query.iter_mut() {
        let new_state = if climber.climbing {
            PlayerAnimationState::Climb
        } else if !ground_detection.on_ground {
            if velocity.linvel.y > 0.0 {
                PlayerAnimationState::Jump
            } else {
                PlayerAnimationState::Fall
            }
        } else if velocity.linvel.x.abs() > 10.0 {
            PlayerAnimationState::Run
        } else {
            PlayerAnimationState::Idle
        };

        if *anim_state != new_state {
            *anim_state = new_state;
        }
    }
}

pub fn animate_player(
    time: Res<Time>,
    config: Res<PlayerAnimationConfig>,
    mut query: Query<
        (
            &PlayerAnimationState,
            &mut AnimationTimer,
            &mut Sprite,
            &Side,
        ),
        With<Player>,
    >,
) {
    for (anim_state, mut timer, mut sprite, side) in query.iter_mut() {
        timer.tick(time.delta());

        if timer.just_finished() {
            let indices = match anim_state {
                PlayerAnimationState::Idle => &config.idle,
                PlayerAnimationState::Run => &config.run,
                PlayerAnimationState::Jump => &config.jump,
                PlayerAnimationState::Fall => &config.fall,
                PlayerAnimationState::Climb => &config.climb,
                PlayerAnimationState::Attack => &config.attack,
                PlayerAnimationState::Hurt => &config.hurt,
            };

            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index >= indices.last || atlas.index < indices.first {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }

        // Flip le sprite selon la direction
        sprite.flip_x = *side == Side::Left;
    }
}

pub struct PlayerAnimationPlugin;

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerAnimationConfig>().add_systems(
            Update,
            (
                setup_player_sprite,
                update_player_animation_state,
                animate_player,
            )
                .chain(),
        );
    }
}
