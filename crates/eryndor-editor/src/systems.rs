use bevy::prelude::*;
use bevy::sprite::ColorMaterial;
use eryndor_common::*;

use crate::{CurrentLevel, EditorState, EditorTool, EntityPalette, EditorEntityMap};
use crate::rendering::{spawn_entity_visual, spawn_platform_visual};

/// Resource to track pending tilemap restoration
#[derive(Resource, Default)]
pub struct PendingTilemapRestore {
    pub should_restore: bool,
}

/// Handle placing entities in the level
pub fn handle_entity_placement(
    mut commands: Commands,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    editor_state: Res<EditorState>,
    mut current_level: ResMut<CurrentLevel>,
    entity_palette: Res<EntityPalette>,
    mut entity_map: ResMut<EditorEntityMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut contexts: bevy_egui::EguiContexts,
) {
    // Don't place if hovering over UI
    let ctx = contexts.ctx_mut();
    if ctx.is_pointer_over_area() {
        return;
    }

    if editor_state.current_tool != EditorTool::EntityPlace {
        return;
    }

    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    let Some(selected_entity) = &entity_palette.selected_entity else {
        return;
    };

    // Get mouse position in world space
    let Some(world_pos) = get_mouse_world_position(&windows, &camera_q) else {
        return;
    };

    // Snap to grid if enabled
    let final_pos = if editor_state.grid_snap_enabled {
        snap_to_grid(world_pos, editor_state.grid_size)
    } else {
        world_pos
    };

    // Create entity spawn config
    let spawn_config = create_spawn_config_for_type(selected_entity.clone(), final_pos.into());

    // Add to level data
    let entity_id = current_level.level_data.entities.len();
    current_level.level_data.add_entity(spawn_config.clone());
    current_level.is_modified = true;

    // Spawn visual representation
    let entity = spawn_entity_visual(&mut commands, &spawn_config, entity_id, &mut meshes, &mut materials);
    entity_map.entities.push(entity);

    info!("Placed entity at {:?}", final_pos);
}

/// Handle creating/editing platforms
pub fn handle_platform_editing(
    mut commands: Commands,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    editor_state: Res<EditorState>,
    mut current_level: ResMut<CurrentLevel>,
    mut entity_map: ResMut<EditorEntityMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut contexts: bevy_egui::EguiContexts,
) {
    // Don't place if hovering over UI
    let ctx = contexts.ctx_mut();
    if ctx.is_pointer_over_area() {
        return;
    }

    if editor_state.current_tool != EditorTool::Platform {
        return;
    }

    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    let Some(world_pos) = get_mouse_world_position(&windows, &camera_q) else {
        return;
    };

    let final_pos = if editor_state.grid_snap_enabled {
        snap_to_grid(world_pos, editor_state.grid_size)
    } else {
        world_pos
    };

    // Create a default platform
    let platform = PlatformData {
        position: final_pos.into(),
        size: Vector2::new(100.0, 20.0),
        is_one_way: false,
    };

    let platform_id = current_level.level_data.platforms.len();
    current_level.level_data.add_platform(platform.clone());
    current_level.is_modified = true;

    // Spawn visual representation
    let entity = spawn_platform_visual(&mut commands, &platform, platform_id, &mut meshes, &mut materials);
    entity_map.platforms.push(entity);

    info!("Placed platform at {:?}", final_pos);
}

/// Handle saving and loading levels
pub fn handle_save_load(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut current_level: ResMut<CurrentLevel>,
    editor_state: Res<EditorState>,
    mut tileset_manager: ResMut<crate::tileset_manager::TilesetManager>,
    map_dimensions: Res<crate::map_canvas::MapDimensions>,
    tilemap_query: Query<&bevy_ecs_tilemap::prelude::TileStorage, With<crate::map_canvas::MapCanvas>>,
    tile_query: Query<(&bevy_ecs_tilemap::prelude::TileTextureIndex, &bevy_ecs_tilemap::prelude::TileVisible, &bevy_ecs_tilemap::prelude::TilePos)>,
    mut pending_restore: ResMut<PendingTilemapRestore>,
    mut commands: Commands,
    existing_canvas: Query<Entity, With<crate::map_canvas::MapCanvas>>,
) {
    // Ctrl+S to save
    if keyboard.pressed(KeyCode::ControlLeft) && keyboard.just_pressed(KeyCode::KeyS) {
        if let Some(path) = current_level.file_path.clone() {
            save_level_with_tilemap(
                &mut current_level,
                &path,
                &editor_state,
                &tileset_manager,
                &map_dimensions,
                &tilemap_query,
                &tile_query,
            );
        } else {
            // No path set, show Save As dialog
            save_as_dialog_with_tilemap(
                &mut current_level,
                &editor_state,
                &tileset_manager,
                &map_dimensions,
                &tilemap_query,
                &tile_query,
            );
        }
    }

    // Ctrl+Shift+S for Save As
    if keyboard.pressed(KeyCode::ControlLeft)
        && keyboard.pressed(KeyCode::ShiftLeft)
        && keyboard.just_pressed(KeyCode::KeyS) {
        save_as_dialog_with_tilemap(
            &mut current_level,
            &editor_state,
            &tileset_manager,
            &map_dimensions,
            &tilemap_query,
            &tile_query,
        );
    }

    // Ctrl+O to open
    if keyboard.pressed(KeyCode::ControlLeft) && keyboard.just_pressed(KeyCode::KeyO) {
        open_dialog(&mut current_level, &mut pending_restore, &mut tileset_manager, &mut commands, &existing_canvas);
    }
}

fn save_level_with_tilemap(
    current_level: &mut ResMut<CurrentLevel>,
    path: &str,
    editor_state: &EditorState,
    tileset_manager: &crate::tileset_manager::TilesetManager,
    map_dimensions: &crate::map_canvas::MapDimensions,
    tilemap_query: &Query<&bevy_ecs_tilemap::prelude::TileStorage, With<crate::map_canvas::MapCanvas>>,
    tile_query: &Query<(&bevy_ecs_tilemap::prelude::TileTextureIndex, &bevy_ecs_tilemap::prelude::TileVisible, &bevy_ecs_tilemap::prelude::TilePos)>,
) {
    use eryndor_common::{LevelTilemapData, LevelTilesetData, LevelLayerData, LevelTileInstance};
    use std::collections::HashSet;

    // Build tilemap data
    let mut tilemap_data = LevelTilemapData {
        grid_size: editor_state.grid_size,
        map_width: map_dimensions.width,
        map_height: map_dimensions.height,
        tilesets: Vec::new(),
        selected_tileset_id: tileset_manager.selected_tileset_id,
        layers: Vec::new(),
    };

    // Save tilesets - deduplicate by texture_path
    let mut seen_paths = HashSet::new();
    for (id, tileset_info) in tileset_manager.tilesets.iter() {
        if seen_paths.insert(tileset_info.data.texture_path.clone()) {
            tilemap_data.tilesets.push(LevelTilesetData {
                id: *id,
                identifier: tileset_info.data.identifier.clone(),
                texture_path: tileset_info.data.texture_path.clone(),
                tile_width: tileset_info.data.tile_width,
                tile_height: tileset_info.data.tile_height,
            });
        }
    }

    // Save tile data
    let mut layer_tiles = Vec::new();
    if let Ok(tile_storage) = tilemap_query.get_single() {
        for tile_entity in tile_storage.iter().flatten() {
            if let Ok((texture_index, visible, tile_pos)) = tile_query.get(*tile_entity) {
                if visible.0 {
                    layer_tiles.push(LevelTileInstance {
                        x: tile_pos.x,
                        y: tile_pos.y,
                        tile_id: texture_index.0,
                    });
                }
            }
        }
        info!("Saving {} tiles to level", layer_tiles.len());
    } else {
        warn!("No tilemap found when trying to save!");
    }

    tilemap_data.layers.push(LevelLayerData {
        id: 0,
        name: "Layer 0".to_string(),
        visible: true,
        tiles: layer_tiles,
    });

    // Update level data with tilemap
    current_level.level_data.tilemap = Some(tilemap_data);

    // Save to file
    if let Err(e) = current_level.level_data.save_to_json(path) {
        error!("Failed to save level: {}", e);
    } else {
        current_level.file_path = Some(path.to_string());
        current_level.is_modified = false;
        info!("Level saved to {}", path);
    }
}

fn save_as_dialog_with_tilemap(
    current_level: &mut ResMut<CurrentLevel>,
    editor_state: &EditorState,
    tileset_manager: &crate::tileset_manager::TilesetManager,
    map_dimensions: &crate::map_canvas::MapDimensions,
    tilemap_query: &Query<&bevy_ecs_tilemap::prelude::TileStorage, With<crate::map_canvas::MapCanvas>>,
    tile_query: &Query<(&bevy_ecs_tilemap::prelude::TileTextureIndex, &bevy_ecs_tilemap::prelude::TileVisible, &bevy_ecs_tilemap::prelude::TilePos)>,
) {
    if let Some(path) = rfd::FileDialog::new()
        .add_filter("JSON", &["json"])
        .set_directory("./assets/levels")
        .set_file_name("level.json")
        .save_file()
    {
        let path_str = path.to_string_lossy().to_string();
        save_level_with_tilemap(
            current_level,
            &path_str,
            editor_state,
            tileset_manager,
            map_dimensions,
            tilemap_query,
            tile_query,
        );
    }
}

fn open_dialog(
    current_level: &mut ResMut<CurrentLevel>,
    mut pending_restore: &mut ResMut<PendingTilemapRestore>,
    mut tileset_manager: &mut ResMut<crate::tileset_manager::TilesetManager>,
    mut commands: &mut Commands,
    existing_canvas: &Query<Entity, With<crate::map_canvas::MapCanvas>>,
) {
    if current_level.is_modified {
        warn!("Level has unsaved changes - save first!");
        // TODO: Show unsaved changes dialog
        return;
    }

    if let Some(path) = rfd::FileDialog::new()
        .add_filter("JSON", &["json"])
        .set_directory("./assets/levels")
        .pick_file()
    {
        let path_str = path.to_string_lossy().to_string();
        match eryndor_common::LevelData::load_from_json(&path_str) {
            Ok(level_data) => {
                // Clear existing tilemap
                for entity in existing_canvas.iter() {
                    commands.entity(entity).despawn();
                }

                // Clear tilesets
                tileset_manager.clear();

                // Load new level data
                current_level.level_data = level_data;
                current_level.file_path = Some(path_str.clone());
                current_level.is_modified = false;

                // Flag that we need to restore tilemap after canvas is created
                pending_restore.should_restore = true;

                info!("Level loaded from {}", path_str);
                // TODO: Reload all visual entities
            }
            Err(e) => {
                error!("Failed to load level: {}", e);
            }
        }
    }
}

/// System to restore tilemap data when a level is loaded
pub fn restore_tilemap_from_level(
    mut pending_restore: ResMut<PendingTilemapRestore>,
    current_level: Res<CurrentLevel>,
    mut editor_state: ResMut<EditorState>,
    mut map_dimensions: ResMut<crate::map_canvas::MapDimensions>,
    mut load_tileset_events: EventWriter<crate::tileset_manager::LoadTilesetEvent>,
    tilemap_query: Query<&bevy_ecs_tilemap::prelude::TileStorage, (With<crate::map_canvas::MapCanvas>, Added<crate::map_canvas::MapCanvas>)>,
    mut tile_query: Query<(&mut bevy_ecs_tilemap::prelude::TileTextureIndex, &mut bevy_ecs_tilemap::prelude::TileVisible)>,
) {
    // Only run if we have a pending restore
    if !pending_restore.should_restore {
        return;
    }

    if let Some(tilemap_data) = &current_level.level_data.tilemap {
        // Check if the tilemap canvas has been created
        if let Ok(tile_storage) = tilemap_query.get_single() {
            info!("Restoring tilemap from level data...");

            // Update editor state
            editor_state.grid_size = tilemap_data.grid_size;
            map_dimensions.width = tilemap_data.map_width;
            map_dimensions.height = tilemap_data.map_height;

            // Load tilesets
            for tileset in &tilemap_data.tilesets {
                load_tileset_events.write(crate::tileset_manager::LoadTilesetEvent {
                    path: tileset.texture_path.clone(),
                    identifier: tileset.identifier.clone(),
                    tile_width: tileset.tile_width,
                    tile_height: tileset.tile_height,
                });
            }

            // Restore tiles
            if !tilemap_data.layers.is_empty() {
                let layer = &tilemap_data.layers[0];
                info!("Restoring {} tiles", layer.tiles.len());

                for tile_instance in &layer.tiles {
                    let tile_pos = bevy_ecs_tilemap::prelude::TilePos {
                        x: tile_instance.x,
                        y: tile_instance.y,
                    };

                    if let Some(tile_entity) = tile_storage.get(&tile_pos) {
                        if let Ok((mut texture_index, mut visible)) = tile_query.get_mut(tile_entity) {
                            texture_index.0 = tile_instance.tile_id;
                            visible.0 = true;
                        }
                    }
                }
            }

            // Clear the pending restore flag
            pending_restore.should_restore = false;
        } else {
            // Tilemap canvas not yet created, we need to initialize it
            // Update settings so the canvas will be created with correct dimensions
            editor_state.grid_size = tilemap_data.grid_size;
            map_dimensions.width = tilemap_data.map_width;
            map_dimensions.height = tilemap_data.map_height;

            // Load tilesets - this will trigger tilemap canvas creation
            for tileset in &tilemap_data.tilesets {
                load_tileset_events.write(crate::tileset_manager::LoadTilesetEvent {
                    path: tileset.texture_path.clone(),
                    identifier: tileset.identifier.clone(),
                    tile_width: tileset.tile_width,
                    tile_height: tileset.tile_height,
                });
            }
            // Don't clear pending_restore - we'll restore tiles next frame when canvas exists
        }
    }
}

// Helper functions

fn get_mouse_world_position(
    windows: &Query<&Window>,
    camera_q: &Query<(&Camera, &GlobalTransform)>,
) -> Option<Vec2> {
    let window = windows.get_single().ok()?;
    let (camera, camera_transform) = camera_q.get_single().ok()?;

    window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
}

fn snap_to_grid(pos: Vec2, grid_size: f32) -> Vec2 {
    Vec2::new(
        (pos.x / grid_size).round() * grid_size,
        (pos.y / grid_size).round() * grid_size,
    )
}

fn create_spawn_config_for_type(entity_type: EntityType, position: Vector2) -> EntitySpawnConfig {
    match entity_type {
        EntityType::Npc(npc_type) => EntitySpawnConfig::npc(
            position,
            "NPC".to_string(),
            npc_type,
            100,
            vec![],
        ),
        EntityType::Resource(resource_type) => {
            EntitySpawnConfig::resource(position, resource_type, 100, 30.0)
        }
        EntityType::Interactive(object_type) => {
            EntitySpawnConfig::interactive(position, object_type)
        }
        EntityType::SpawnPoint(spawn_type) => {
            EntitySpawnConfig::spawn_point(position, spawn_type, "default".to_string())
        }
        EntityType::Player => EntitySpawnConfig::player(position, "Player".to_string(), 100),
    }
}
