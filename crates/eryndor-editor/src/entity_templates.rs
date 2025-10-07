//! Entity templates for creating pre-configured entities
//! Each template defines a set of components that make up a specific entity type

use bevy::prelude::*;
use bevy::sprite::Anchor;
use crate::scene_editor::EditorSceneEntity;

/// Entity template types available in the editor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityTemplate {
    /// Empty entity with just Transform and Visibility
    Empty,
    /// 2D Sprite entity
    Sprite,
    /// 2D Camera entity
    Camera2D,
    /// UI Node (container/layout element)
    UINode,
    /// UI Button
    Button,
    /// UI Text
    Text,
}

impl EntityTemplate {
    /// Get the display name for this template
    pub fn display_name(&self) -> &'static str {
        match self {
            EntityTemplate::Empty => "Empty Entity",
            EntityTemplate::Sprite => "🎨 Sprite",
            EntityTemplate::Camera2D => "📷 Camera 2D",
            EntityTemplate::UINode => "🔲 UI Node",
            EntityTemplate::Button => "🔘 Button",
            EntityTemplate::Text => "📝 Text",
        }
    }

    /// Get the default entity name for this template
    pub fn default_name(&self) -> &'static str {
        match self {
            EntityTemplate::Empty => "New Entity",
            EntityTemplate::Sprite => "Sprite",
            EntityTemplate::Camera2D => "Camera",
            EntityTemplate::UINode => "UI Node",
            EntityTemplate::Button => "Button",
            EntityTemplate::Text => "Text",
        }
    }
}

/// Spawn an entity from a template with the specified parent
pub fn spawn_from_template(
    commands: &mut Commands,
    template: EntityTemplate,
    parent: Option<Entity>,
) -> Entity {
    match template {
        EntityTemplate::Empty => spawn_empty(commands, parent),
        EntityTemplate::Sprite => spawn_sprite(commands, parent),
        EntityTemplate::Camera2D => spawn_camera_2d(commands, parent),
        EntityTemplate::UINode => spawn_ui_node(commands, parent),
        EntityTemplate::Button => spawn_button(commands, parent),
        EntityTemplate::Text => spawn_text(commands, parent),
    }
}

/// Spawn an empty entity (Transform + Visibility only)
fn spawn_empty(commands: &mut Commands, parent: Option<Entity>) -> Entity {
    let mut entity_commands = commands.spawn((
        Name::new(EntityTemplate::Empty.default_name()),
        Transform::default(),
        Visibility::default(),
        EditorSceneEntity,
    ));

    if let Some(parent_entity) = parent {
        entity_commands.insert(ChildOf(parent_entity));
    }

    entity_commands.id()
}

/// Spawn a 2D sprite entity
fn spawn_sprite(commands: &mut Commands, parent: Option<Entity>) -> Entity {
    let mut entity_commands = commands.spawn((
        Name::new(EntityTemplate::Sprite.default_name()),
        Transform::default(),
        Visibility::default(),
        Sprite {
            color: Color::srgba(0.7, 0.7, 0.7, 0.8),
            custom_size: Some(Vec2::new(64.0, 64.0)),
            anchor: Anchor::Center,
            ..default()
        },
        EditorSceneEntity,
    ));

    if let Some(parent_entity) = parent {
        entity_commands.insert(ChildOf(parent_entity));
    }

    entity_commands.id()
}

/// Spawn a 2D camera entity
fn spawn_camera_2d(commands: &mut Commands, parent: Option<Entity>) -> Entity {
    let mut entity_commands = commands.spawn((
        Name::new(EntityTemplate::Camera2D.default_name()),
        Transform::default(),
        Visibility::default(),
        Camera2d,
        EditorSceneEntity,
    ));

    if let Some(parent_entity) = parent {
        entity_commands.insert(ChildOf(parent_entity));
    }

    entity_commands.id()
}

/// Spawn a UI node entity
fn spawn_ui_node(commands: &mut Commands, parent: Option<Entity>) -> Entity {
    let mut entity_commands = commands.spawn((
        Name::new(EntityTemplate::UINode.default_name()),
        Node {
            width: Val::Px(200.0),
            height: Val::Px(100.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.15, 0.15, 0.15, 0.8)),
        EditorSceneEntity,
    ));

    if let Some(parent_entity) = parent {
        entity_commands.insert(ChildOf(parent_entity));
    }

    entity_commands.id()
}

/// Spawn a UI button entity
fn spawn_button(commands: &mut Commands, parent: Option<Entity>) -> Entity {
    let mut entity_commands = commands.spawn((
        Name::new(EntityTemplate::Button.default_name()),
        Button,
        Node {
            width: Val::Px(150.0),
            height: Val::Px(50.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
        EditorSceneEntity,
    ));

    if let Some(parent_entity) = parent {
        entity_commands.insert(ChildOf(parent_entity));
    }

    let button_id = entity_commands.id();

    // Add text child to button
    commands.spawn((
        Text::new("Button"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
        EditorSceneEntity,
    )).insert(ChildOf(button_id));

    button_id
}

/// Spawn a UI text entity
fn spawn_text(commands: &mut Commands, parent: Option<Entity>) -> Entity {
    let mut entity_commands = commands.spawn((
        Name::new(EntityTemplate::Text.default_name()),
        Text::new("Text"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 1.0, 1.0)),
        EditorSceneEntity,
    ));

    if let Some(parent_entity) = parent {
        entity_commands.insert(ChildOf(parent_entity));
    }

    entity_commands.id()
}
