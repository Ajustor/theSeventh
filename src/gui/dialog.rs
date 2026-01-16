//! Module for displaying dialog windows at the top and bottom of the screen.
//!
//! This module provides tools to show dialog windows that can be used for
//! conversations, notifications, or any text-based communication with the player.

use bevy::prelude::*;

use crate::GameState;

/// Event to show a dialog at the top of the screen
#[derive(Event)]
pub struct ShowTopDialog {
    /// The text to display in the dialog
    pub text: String,
    /// Optional: speaker name to display
    pub speaker: Option<String>,
}

/// Event to hide the top dialog
#[derive(Event)]
pub struct HideTopDialog;

/// Event to show a dialog at the bottom of the screen
#[derive(Event)]
pub struct ShowBottomDialog {
    /// The text to display in the dialog
    pub text: String,
    /// Optional: speaker name to display
    pub speaker: Option<String>,
}

/// Event to hide the bottom dialog
#[derive(Event)]
pub struct HideBottomDialog;

/// Marker component for the top dialog container
#[derive(Component)]
pub struct TopDialogContainer;

/// Marker component for the bottom dialog container
#[derive(Component)]
pub struct BottomDialogContainer;

/// Marker component for dialog text
#[derive(Component)]
pub struct DialogText;

/// Marker component for speaker name text
#[derive(Component)]
pub struct SpeakerText;

/// Resource to track dialog UI entities
#[derive(Resource)]
struct DialogUI {
    top_container: Option<Entity>,
    bottom_container: Option<Entity>,
}

impl Default for DialogUI {
    fn default() -> Self {
        Self {
            top_container: None,
            bottom_container: None,
        }
    }
}

/// Plugin for managing dialog windows
pub struct DialogPlugin;

impl Plugin for DialogPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DialogUI>()
            .add_event::<ShowTopDialog>()
            .add_event::<HideTopDialog>()
            .add_event::<ShowBottomDialog>()
            .add_event::<HideBottomDialog>()
            .add_systems(
                Update,
                (
                    handle_show_top_dialog,
                    handle_hide_top_dialog,
                    handle_show_bottom_dialog,
                    handle_hide_bottom_dialog,
                )
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(OnExit(GameState::InGame), cleanup_dialogs);
    }
}

/// Creates a dialog box node with common styling
fn create_dialog_box(commands: &mut Commands, at_top: bool, text: &str, speaker: Option<&str>) -> Entity {
    let container = commands
        .spawn((
            Node {
                width: Val::Percent(80.0),
                min_height: Val::Px(80.0),
                position_type: PositionType::Absolute,
                left: Val::Percent(10.0),
                padding: UiRect::all(Val::Px(15.0)),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 0.9)),
            BorderRadius::all(Val::Px(10.0)),
        ))
        .id();

    // Position at top or bottom
    let position_component = if at_top {
        Node {
            width: Val::Percent(80.0),
            min_height: Val::Px(80.0),
            position_type: PositionType::Absolute,
            left: Val::Percent(10.0),
            top: Val::Px(20.0),
            padding: UiRect::all(Val::Px(15.0)),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::Center,
            ..default()
        }
    } else {
        Node {
            width: Val::Percent(80.0),
            min_height: Val::Px(80.0),
            position_type: PositionType::Absolute,
            left: Val::Percent(10.0),
            bottom: Val::Px(20.0),
            padding: UiRect::all(Val::Px(15.0)),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::Center,
            ..default()
        }
    };

    commands.entity(container).insert(position_component);

    // Add marker component
    if at_top {
        commands.entity(container).insert(TopDialogContainer);
    } else {
        commands.entity(container).insert(BottomDialogContainer);
    }

    commands.entity(container).with_children(|parent| {
        // Speaker name if provided
        if let Some(speaker_name) = speaker {
            parent.spawn((
                Text::new(speaker_name),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.7, 0.3)),
                Node {
                    margin: UiRect::bottom(Val::Px(8.0)),
                    ..default()
                },
                SpeakerText,
            ));
        }

        // Dialog text
        parent.spawn((
            Text::new(text),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            DialogText,
        ));
    });

    container
}

fn handle_show_top_dialog(
    mut commands: Commands,
    mut dialog_ui: ResMut<DialogUI>,
    mut events: EventReader<ShowTopDialog>,
) {
    for event in events.read() {
        // Remove existing top dialog if any
        if let Some(entity) = dialog_ui.top_container {
            commands.entity(entity).despawn_recursive();
        }

        // Create new dialog
        let container = create_dialog_box(
            &mut commands,
            true,
            &event.text,
            event.speaker.as_deref(),
        );
        dialog_ui.top_container = Some(container);
    }
}

fn handle_hide_top_dialog(
    mut commands: Commands,
    mut dialog_ui: ResMut<DialogUI>,
    mut events: EventReader<HideTopDialog>,
) {
    for _ in events.read() {
        if let Some(entity) = dialog_ui.top_container.take() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn handle_show_bottom_dialog(
    mut commands: Commands,
    mut dialog_ui: ResMut<DialogUI>,
    mut events: EventReader<ShowBottomDialog>,
) {
    for event in events.read() {
        // Remove existing bottom dialog if any
        if let Some(entity) = dialog_ui.bottom_container {
            commands.entity(entity).despawn_recursive();
        }

        // Create new dialog
        let container = create_dialog_box(
            &mut commands,
            false,
            &event.text,
            event.speaker.as_deref(),
        );
        dialog_ui.bottom_container = Some(container);
    }
}

fn handle_hide_bottom_dialog(
    mut commands: Commands,
    mut dialog_ui: ResMut<DialogUI>,
    mut events: EventReader<HideBottomDialog>,
) {
    for _ in events.read() {
        if let Some(entity) = dialog_ui.bottom_container.take() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn cleanup_dialogs(mut commands: Commands, mut dialog_ui: ResMut<DialogUI>) {
    if let Some(entity) = dialog_ui.top_container.take() {
        commands.entity(entity).despawn_recursive();
    }
    if let Some(entity) = dialog_ui.bottom_container.take() {
        commands.entity(entity).despawn_recursive();
    }
}
