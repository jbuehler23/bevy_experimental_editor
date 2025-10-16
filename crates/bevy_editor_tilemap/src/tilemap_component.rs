//! Tilemap as a scene component
//! This allows tilemaps to be entities in the scene editor

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

/// Component that marks an entity as a tilemap in the scene
/// This allows tilemaps to be part of the scene graph
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct TilemapComponent {
    /// Reference to the actual tilemap entity created by bevy_ecs_tilemap
    pub tilemap_entity: Option<Entity>,
    /// Width in tiles
    pub width: u32,
    /// Height in tiles
    pub height: u32,
    /// Tileset ID being used
    pub tileset_id: Option<u32>,
}

impl Default for TilemapComponent {
    fn default() -> Self {
        Self {
            tilemap_entity: None,
            width: 64,
            height: 64,
            tileset_id: None,
        }
    }
}

/// Component to store layer information for a tilemap
#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
#[derive(Default)]
pub struct TilemapLayers {
    /// Layer IDs that belong to this tilemap
    pub layer_ids: Vec<u32>,
    /// Currently active layer for editing
    pub active_layer: Option<u32>,
}

/// System to create/update tilemap entities when TilemapComponent is added
pub fn sync_tilemap_entities(
    mut commands: Commands,
    mut tilemap_query: Query<
        (Entity, &mut TilemapComponent, &Transform),
        Changed<TilemapComponent>,
    >,
    tilemap_storage_query: Query<Entity, With<TileStorage>>,
    tileset_manager: Res<crate::TilesetManager>,
) {
    for (entity, mut tilemap_comp, transform) in tilemap_query.iter_mut() {
        // If tilemap entity already exists, skip
        if let Some(tilemap_entity) = tilemap_comp.tilemap_entity {
            if tilemap_storage_query.get(tilemap_entity).is_ok() {
                continue;
            }
        }

        // Get tileset info if available
        if let Some(tileset_id) = tilemap_comp.tileset_id {
            if let Some(tileset_info) = tileset_manager.tilesets.get(&tileset_id) {
                info!(
                    "Creating tilemap entity for TilemapComponent on entity {:?}",
                    entity
                );

                let map_size = TilemapSize {
                    x: tilemap_comp.width,
                    y: tilemap_comp.height,
                };

                let tile_size = TilemapTileSize {
                    x: tileset_info.data.tile_width as f32,
                    y: tileset_info.data.tile_height as f32,
                };

                let grid_size = TilemapGridSize {
                    x: tileset_info.data.tile_width as f32,
                    y: tileset_info.data.tile_height as f32,
                };

                // Create the tilemap entity
                let tilemap_entity = commands.spawn_empty().id();
                let mut tile_storage = TileStorage::empty(map_size);

                // Spawn tiles (initially hidden)
                for x in 0..map_size.x {
                    for y in 0..map_size.y {
                        let tile_pos = TilePos { x, y };
                        let tile_entity = commands
                            .spawn(TileBundle {
                                position: tile_pos,
                                tilemap_id: TilemapId(tilemap_entity),
                                texture_index: TileTextureIndex(0),
                                visible: TileVisible(false),
                                ..Default::default()
                            })
                            .id();
                        tile_storage.set(&tile_pos, tile_entity);
                    }
                }

                // Offset by half a tile so tiles are centered on grid intersections
                let half_tile_x = tileset_info.data.tile_width as f32 / 2.0;
                let half_tile_y = tileset_info.data.tile_height as f32 / 2.0;

                // Use the parent entity's transform
                let tilemap_transform = Transform::from_xyz(
                    transform.translation.x + half_tile_x,
                    transform.translation.y + half_tile_y,
                    transform.translation.z,
                );

                commands.entity(tilemap_entity).insert(TilemapBundle {
                    grid_size,
                    size: map_size,
                    storage: tile_storage,
                    texture: TilemapTexture::Single(tileset_info.texture_handle.clone()),
                    tile_size,
                    transform: tilemap_transform,
                    map_type: TilemapType::Square,
                    ..Default::default()
                });

                // Update the component to reference the tilemap entity
                tilemap_comp.tilemap_entity = Some(tilemap_entity);

                info!(
                    "Created tilemap entity: {:?} for TilemapComponent",
                    tilemap_entity
                );
            }
        }
    }
}

/// System to clean up tilemap entities when TilemapComponent is removed
pub fn cleanup_tilemap_entities(
    mut commands: Commands,
    mut removed: RemovedComponents<TilemapComponent>,
    tilemap_query: Query<&TilemapComponent>,
) {
    for entity in removed.read() {
        if let Ok(tilemap_comp) = tilemap_query.get(entity) {
            if let Some(tilemap_entity) = tilemap_comp.tilemap_entity {
                commands.entity(tilemap_entity).despawn();
                info!("Cleaned up tilemap entity: {:?}", tilemap_entity);
            }
        }
    }
}
