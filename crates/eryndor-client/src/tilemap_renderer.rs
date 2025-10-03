use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use crate::level_loader::LevelLoadedEvent;

/// Component to mark entities as part of the current level (for despawning on reload)
#[derive(Component)]
pub struct LevelEntity;

/// System to render tilemap when level is loaded
pub fn handle_level_loaded(
    mut events: EventReader<LevelLoadedEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // Query existing level entities to despawn them
    existing_entities: Query<Entity, With<LevelEntity>>,
) {
    for event in events.read() {
        // Despawn existing level entities
        for entity in &existing_entities {
            commands.entity(entity).despawn();
        }

        // Check if level has tilemap data
        let Some(tilemap_data) = &event.level_data.tilemap else {
            info!("Level has no tilemap data, skipping tilemap rendering");
            continue;
        };

        info!("Rendering tilemap: {} layers, {} tilesets",
            tilemap_data.layers.len(),
            tilemap_data.tilesets.len()
        );

        // Load all tilesets and create tilemaps
        for (layer_idx, layer) in tilemap_data.layers.iter().enumerate() {
            if layer.tiles.is_empty() {
                info!("Layer {} '{}' has no tiles, skipping", layer_idx, layer.name);
                continue;
            }

            // Find the first tileset (in a real implementation, you'd match by tileset ID)
            let Some(tileset) = tilemap_data.tilesets.first() else {
                warn!("No tilesets found for layer {}", layer_idx);
                continue;
            };

            // Load tileset texture
            let texture_handle: Handle<Image> = asset_server.load(&tileset.texture_path);

            // Create tilemap
            let map_size = TilemapSize {
                x: tilemap_data.map_width,
                y: tilemap_data.map_height,
            };

            let tile_size = TilemapTileSize {
                x: tileset.tile_width as f32,
                y: tileset.tile_height as f32,
            };

            let grid_size = TilemapGridSize {
                x: tileset.tile_width as f32,
                y: tileset.tile_height as f32,
            };

            let mut tile_storage = TileStorage::empty(map_size);

            let tilemap_entity = commands.spawn((
                LevelEntity,
                Name::new(format!("Tilemap Layer: {}", layer.name)),
            )).id();

            // Spawn tiles
            for tile_instance in &layer.tiles {
                let tile_pos = TilePos {
                    x: tile_instance.x,
                    y: tile_instance.y,
                };

                let tile_entity = commands.spawn((
                    TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(tile_instance.tile_id),
                        visible: TileVisible(true),
                        ..default()
                    },
                    LevelEntity,
                )).id();

                tile_storage.set(&tile_pos, tile_entity);
            }

            // Half-tile offset for proper grid alignment (same as editor)
            let half_tile_x = tileset.tile_width as f32 / 2.0;
            let half_tile_y = tileset.tile_height as f32 / 2.0;

            // Add tilemap bundle to the tilemap entity
            commands.entity(tilemap_entity).insert(TilemapBundle {
                grid_size,
                size: map_size,
                storage: tile_storage,
                texture: TilemapTexture::Single(texture_handle),
                tile_size,
                transform: Transform::from_xyz(half_tile_x, half_tile_y, layer_idx as f32),
                map_type: TilemapType::Square,
                ..default()
            });

            info!("Spawned {} tiles for layer '{}'", layer.tiles.len(), layer.name);
        }
    }
}
