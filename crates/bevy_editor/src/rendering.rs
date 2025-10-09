use bevy::prelude::*;
use bevy::sprite::{ColorMaterial, MeshMaterial2d};
use bevy::render::mesh::Mesh2d;
use crate::formats::*;

use crate::selection::{EditorEntity, EditorEntityType};

/// Component for storing platform size
#[derive(Component, Clone, Copy)]
pub struct PlatformSize(pub Vec2);

/// Get color for entity type
pub fn get_entity_color(entity_type: &EntityType) -> Color {
    match entity_type {
        EntityType::Player => Color::srgb(0.0, 1.0, 0.0),
        EntityType::Npc(NpcType::Friendly) => Color::srgb(0.0, 0.5, 1.0),
        EntityType::Npc(NpcType::Hostile) => Color::srgb(1.0, 0.0, 0.0),
        EntityType::Npc(NpcType::Neutral) => Color::srgb(0.7, 0.7, 0.0),
        EntityType::Npc(NpcType::Vendor) => Color::srgb(1.0, 0.7, 0.0),
        EntityType::Resource(_) => Color::srgb(0.5, 0.3, 0.1),
        EntityType::Interactive(_) => Color::srgb(0.8, 0.0, 0.8),
        EntityType::SpawnPoint(_) => Color::srgb(1.0, 1.0, 0.0),
    }
}

/// Get color for platform
pub fn get_platform_color() -> Color {
    Color::srgb(0.5, 0.5, 0.5)
}

/// Spawn visual representation of an entity
pub fn spawn_entity_visual(
    commands: &mut Commands,
    config: &EntitySpawnConfig,
    entity_id: usize,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> Entity {
    let position: Vec3 = config.position.into();
    let color = get_entity_color(&config.entity_type);

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(16.0))),
        MeshMaterial2d(materials.add(color)),
        Transform::from_translation(position),
        EditorEntity {
            entity_id,
            entity_type: match &config.entity_type {
                EntityType::Npc(_) => EditorEntityType::NpcSpawn,
                EntityType::Resource(_) => EditorEntityType::ResourceNode,
                EntityType::Interactive(_) => EditorEntityType::InteractiveObject,
                EntityType::SpawnPoint(_) => EditorEntityType::SpawnPoint,
                EntityType::Player => EditorEntityType::SpawnPoint,
            },
        },
    )).id()
}

/// Spawn visual representation of a platform
pub fn spawn_platform_visual(
    commands: &mut Commands,
    platform: &PlatformData,
    platform_id: usize,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> Entity {
    let position: Vec3 = platform.position.into();
    let size: Vec2 = platform.size.into();

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(size.x, size.y))),
        MeshMaterial2d(materials.add(get_platform_color())),
        Transform::from_translation(position),
        EditorEntity {
            entity_id: platform_id,
            entity_type: EditorEntityType::Platform,
        },
        PlatformSize(size),
    )).id()
}
