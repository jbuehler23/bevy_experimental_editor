use spacetimedb::ReducerContext;
use crate::tables::*;

#[spacetimedb::reducer]
pub fn upload_world_data(ctx: &ReducerContext, world_json: String) -> Result<(), String> {
    log::info!("Uploading world data...");

    // Parse the world JSON
    let world_export: WorldExport = serde_json::from_str(&world_json)
        .map_err(|e| format!("Invalid JSON: {}", e))?;

    // Store the raw export in world_metadata for re-importing
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    ctx.db.world_metadata().insert(WorldMetadata {
        id: 0,
        version: world_export.version.clone(),
        last_updated: timestamp,
        editor_data: world_json,
    });

    // Clear existing tilemap data (but keep entity tables for backward compatibility)
    // Note: In production, you might want to be more selective about what to clear
    log::info!("Clearing existing tilemap data...");
    for layer in ctx.db.tilemap_layer().iter() {
        ctx.db.tilemap_layer().delete(layer);
    }
    for tile in ctx.db.tilemap_tile().iter() {
        ctx.db.tilemap_tile().delete(tile);
    }
    for cell in ctx.db.intgrid_cell().iter() {
        ctx.db.intgrid_cell().delete(cell);
    }

    // Insert tilesets
    log::info!("Inserting {} tilesets...", world_export.tilesets.len());
    for tileset_data in world_export.tilesets {
        ctx.db.tileset().insert(Tileset {
            id: tileset_data.id,
            identifier: tileset_data.identifier,
            texture_path: tileset_data.texture_path,
            tile_width: tileset_data.tile_width,
            tile_height: tileset_data.tile_height,
            columns: tileset_data.columns,
            rows: tileset_data.rows,
            spacing: tileset_data.spacing,
            padding: tileset_data.padding,
        });
    }

    // Insert layers and their tiles
    log::info!("Inserting {} layers...", world_export.layers.len());
    for layer_data in world_export.layers {
        // Insert layer metadata
        ctx.db.tilemap_layer().insert(TilemapLayer {
            id: layer_data.id,
            level_id: layer_data.level_id,
            identifier: layer_data.identifier,
            layer_type: layer_data.layer_type,
            tileset_id: layer_data.tileset_id,
            grid_size: layer_data.grid_size,
            width: layer_data.width,
            height: layer_data.height,
            z_index: layer_data.z_index,
            opacity: layer_data.opacity,
            parallax_x: layer_data.parallax_x,
            parallax_y: layer_data.parallax_y,
        });

        // Insert tiles for this layer
        for tile_data in layer_data.tiles {
            ctx.db.tilemap_tile().insert(TilemapTile {
                id: 0,  // auto-increment
                layer_id: layer_data.id,
                x: tile_data.x,
                y: tile_data.y,
                tile_id: tile_data.tile_id,
                flip_x: tile_data.flip_x,
                flip_y: tile_data.flip_y,
            });
        }

        // Insert intgrid cells for this layer
        for cell_data in layer_data.intgrid_cells {
            ctx.db.intgrid_cell().insert(IntGridCell {
                id: 0,  // auto-increment
                layer_id: layer_data.id,
                x: cell_data.x,
                y: cell_data.y,
                value: cell_data.value,
            });
        }
    }

    // Insert entity definitions
    log::info!("Inserting {} entity definitions...", world_export.entity_definitions.len());
    for entity_def in world_export.entity_definitions {
        ctx.db.entity_definition().insert(EntityDefinition {
            id: entity_def.id,
            identifier: entity_def.identifier,
            width: entity_def.width,
            height: entity_def.height,
            color: entity_def.color,
            field_definitions: entity_def.field_definitions,
        });
    }

    // Insert entity instances
    log::info!("Inserting {} entity instances...", world_export.entity_instances.len());
    for entity_inst in world_export.entity_instances {
        ctx.db.entity_instance().insert(EntityInstance {
            id: 0,  // auto-increment
            level_id: entity_inst.level_id,
            entity_def_id: entity_inst.entity_def_id,
            x: entity_inst.x,
            y: entity_inst.y,
            field_values: entity_inst.field_values,
        });
    }

    // Insert enum definitions
    log::info!("Inserting {} enum definitions...", world_export.enum_definitions.len());
    for enum_def in world_export.enum_definitions {
        ctx.db.enum_definition().insert(EnumDefinition {
            id: enum_def.id,
            identifier: enum_def.identifier,
            values: enum_def.values,
        });
    }

    log::info!("World data uploaded successfully!");
    Ok(())
}

// World export format - matches the JSON structure exported from the editor
#[derive(serde::Deserialize)]
struct WorldExport {
    version: String,
    tilesets: Vec<TilesetData>,
    layers: Vec<LayerData>,
    entity_definitions: Vec<EntityDefinitionData>,
    entity_instances: Vec<EntityInstanceData>,
    enum_definitions: Vec<EnumDefinitionData>,
}

#[derive(serde::Deserialize)]
struct TilesetData {
    id: u32,
    identifier: String,
    texture_path: String,
    tile_width: u32,
    tile_height: u32,
    columns: u32,
    rows: u32,
    spacing: u32,
    padding: u32,
}

#[derive(serde::Deserialize)]
struct LayerData {
    id: u32,
    level_id: u32,
    identifier: String,
    layer_type: String,
    tileset_id: Option<u32>,
    grid_size: u32,
    width: u32,
    height: u32,
    z_index: i32,
    opacity: f32,
    parallax_x: f32,
    parallax_y: f32,
    tiles: Vec<TileData>,
    intgrid_cells: Vec<IntGridCellData>,
}

#[derive(serde::Deserialize)]
struct TileData {
    x: u32,
    y: u32,
    tile_id: u32,
    flip_x: bool,
    flip_y: bool,
}

#[derive(serde::Deserialize)]
struct IntGridCellData {
    x: u32,
    y: u32,
    value: u32,
}

#[derive(serde::Deserialize)]
struct EntityDefinitionData {
    id: u32,
    identifier: String,
    width: u32,
    height: u32,
    color: String,
    field_definitions: String,
}

#[derive(serde::Deserialize)]
struct EntityInstanceData {
    level_id: u32,
    entity_def_id: u32,
    x: f32,
    y: f32,
    field_values: String,
}

#[derive(serde::Deserialize)]
struct EnumDefinitionData {
    id: u32,
    identifier: String,
    values: String,
}
