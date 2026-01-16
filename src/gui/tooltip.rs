//! Module for displaying tooltips above entities in the game world.
//!
//! This module provides a `Tooltip` component that can be attached to any entity
//! to display a text label above it in world space.

use bevy::prelude::*;

use crate::GameState;

/// Component to attach a tooltip to an entity.
/// The tooltip will be displayed above the entity in world space.
#[derive(Component)]
pub struct Tooltip {
    /// The text to display in the tooltip
    pub text: String,
    /// Vertical offset from the entity (in pixels)
    pub offset_y: f32,
    /// Whether the tooltip is currently visible
    pub visible: bool,
}

impl Tooltip {
    /// Create a new tooltip with the given text
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            offset_y: 30.0,
            visible: true,
        }
    }

    /// Create a new tooltip with custom offset
    pub fn with_offset(text: impl Into<String>, offset_y: f32) -> Self {
        Self {
            text: text.into(),
            offset_y,
            visible: true,
        }
    }

    /// Set the visibility of the tooltip
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

impl Default for Tooltip {
    fn default() -> Self {
        Self {
            text: String::new(),
            offset_y: 30.0,
            visible: true,
        }
    }
}

/// Marker component for tooltip display entities
#[derive(Component)]
pub struct TooltipDisplay {
    /// The entity this tooltip is attached to
    pub target_entity: Entity,
}

/// Plugin for managing entity tooltips
pub struct TooltipPlugin;

impl Plugin for TooltipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_tooltip_displays, update_tooltip_positions, update_tooltip_visibility)
                .chain()
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(OnExit(GameState::InGame), cleanup_tooltips);
    }
}

/// System to spawn tooltip display entities for entities with Tooltip component
fn spawn_tooltip_displays(
    mut commands: Commands,
    query: Query<(Entity, &Tooltip), Added<Tooltip>>,
) {
    for (entity, tooltip) in query.iter() {
        // Spawn a text entity as a child for the tooltip display
        commands.spawn((
            Text2d::new(&tooltip.text),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 1.0, 1.0)),
            Transform::from_xyz(0.0, tooltip.offset_y, 100.0),
            TooltipDisplay {
                target_entity: entity,
            },
        ));
    }
}

/// System to update tooltip positions to follow their target entities
fn update_tooltip_positions(
    entity_query: Query<(&Transform, &Tooltip), Without<TooltipDisplay>>,
    mut tooltip_query: Query<(&mut Transform, &TooltipDisplay), Without<Tooltip>>,
) {
    for (mut tooltip_transform, tooltip_display) in tooltip_query.iter_mut() {
        if let Ok((entity_transform, tooltip)) = entity_query.get(tooltip_display.target_entity) {
            tooltip_transform.translation.x = entity_transform.translation.x;
            tooltip_transform.translation.y = entity_transform.translation.y + tooltip.offset_y;
        }
    }
}

/// System to update tooltip visibility and text based on the Tooltip component
fn update_tooltip_visibility(
    entity_query: Query<&Tooltip, Changed<Tooltip>>,
    mut tooltip_query: Query<(&mut Visibility, &mut Text2d, &TooltipDisplay)>,
) {
    for (mut visibility, mut text, tooltip_display) in tooltip_query.iter_mut() {
        if let Ok(tooltip) = entity_query.get(tooltip_display.target_entity) {
            *visibility = if tooltip.visible {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
            // Only update text if it has actually changed
            if text.0 != tooltip.text {
                text.0.clone_from(&tooltip.text);
            }
        }
    }
}

/// System to clean up tooltips when target entities are despawned
fn cleanup_tooltips(
    mut commands: Commands,
    tooltip_query: Query<Entity, With<TooltipDisplay>>,
) {
    for entity in tooltip_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// System to handle despawned target entities (can be added for runtime cleanup)
pub fn cleanup_orphaned_tooltips(
    mut commands: Commands,
    entity_query: Query<Entity, With<Tooltip>>,
    tooltip_query: Query<(Entity, &TooltipDisplay)>,
) {
    for (tooltip_entity, tooltip_display) in tooltip_query.iter() {
        if entity_query.get(tooltip_display.target_entity).is_err() {
            commands.entity(tooltip_entity).despawn_recursive();
        }
    }
}
