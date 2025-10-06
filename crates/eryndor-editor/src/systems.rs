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
    mut open_scenes: ResMut<crate::scene_tabs::OpenScenes>,  // Changed from CurrentLevel
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

    // Add to active scene's level data
    if let Some(scene) = open_scenes.active_scene_mut() {
        let entity_id = scene.level_data.entities.len();
        scene.level_data.add_entity(spawn_config.clone());
        scene.is_modified = true;

        // Spawn visual representation
        let entity = spawn_entity_visual(&mut commands, &spawn_config, entity_id, &mut meshes, &mut materials);
        entity_map.entities.push(entity);

        info!("Placed entity at {:?}", final_pos);
    }
}

/// Handle creating/editing platforms
pub fn handle_platform_editing(
    mut commands: Commands,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    editor_state: Res<EditorState>,
    mut open_scenes: ResMut<crate::scene_tabs::OpenScenes>,  // Changed from CurrentLevel
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

    // Add to active scene's level data
    if let Some(scene) = open_scenes.active_scene_mut() {
        let platform_id = scene.level_data.platforms.len();
        scene.level_data.add_platform(platform.clone());
        scene.is_modified = true;

        // Spawn visual representation
        let entity = spawn_platform_visual(&mut commands, &platform, platform_id, &mut meshes, &mut materials);
        entity_map.platforms.push(entity);

        info!("Placed platform at {:?}", final_pos);
    }
}

/// Handle saving and loading levels
pub fn handle_save_load(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut open_scenes: ResMut<crate::scene_tabs::OpenScenes>,  // Changed from CurrentLevel
    editor_state: Res<EditorState>,
    mut tileset_manager: ResMut<crate::tileset_manager::TilesetManager>,
    map_dimensions: Res<crate::map_canvas::MapDimensions>,
    tilemap_query: Query<&bevy_ecs_tilemap::prelude::TileStorage, With<crate::map_canvas::MapCanvas>>,
    tile_query: Query<(&bevy_ecs_tilemap::prelude::TileTextureIndex, &bevy_ecs_tilemap::prelude::TileVisible, &bevy_ecs_tilemap::prelude::TilePos)>,
    mut pending_restore: ResMut<PendingTilemapRestore>,
    mut commands: Commands,
    existing_canvas: Query<Entity, With<crate::map_canvas::MapCanvas>>,
    mut current_project: Option<ResMut<crate::project_manager::CurrentProject>>,
) {
    // Ctrl+S to save
    if keyboard.pressed(KeyCode::ControlLeft) && keyboard.just_pressed(KeyCode::KeyS) {
        if let Some(scene) = open_scenes.active_scene_mut() {
            if let Some(path) = scene.file_path.clone() {
                save_level_with_tilemap(
                    scene,
                    &path,
                    &editor_state,
                    &tileset_manager,
                    &map_dimensions,
                    &tilemap_query,
                    &tile_query,
                );

                // Update last opened scene in project config
                if let Some(ref mut project) = current_project {
                    update_last_opened_scene(project, &path);
                }
            } else {
                // No path set, show Save As dialog
                save_as_dialog_with_tilemap(
                    scene,
                    &editor_state,
                    &tileset_manager,
                    &map_dimensions,
                    &tilemap_query,
                    &tile_query,
                );
            }
        }
    }

    // Ctrl+Shift+S for Save As
    if keyboard.pressed(KeyCode::ControlLeft)
        && keyboard.pressed(KeyCode::ShiftLeft)
        && keyboard.just_pressed(KeyCode::KeyS) {
        if let Some(scene) = open_scenes.active_scene_mut() {
            save_as_dialog_with_tilemap(
                scene,
                &editor_state,
                &tileset_manager,
                &map_dimensions,
                &tilemap_query,
                &tile_query,
            );
        }
    }

    // Ctrl+O to open
    if keyboard.pressed(KeyCode::ControlLeft) && keyboard.just_pressed(KeyCode::KeyO) {
        open_dialog(&mut open_scenes, &mut pending_restore, &mut tileset_manager, &mut commands, &existing_canvas);
    }
}

fn save_level_with_tilemap(
    scene: &mut crate::scene_tabs::OpenScene,  // Changed from CurrentLevel
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

    // Save tilesets - deduplicate by texture_path and convert to relative paths
    let mut seen_paths = HashSet::new();
    for (id, tileset_info) in tileset_manager.tilesets.iter() {
        if seen_paths.insert(tileset_info.data.texture_path.clone()) {
            // Convert absolute path to relative path for portability
            let relative_path = convert_to_relative_asset_path(&tileset_info.data.texture_path);

            tilemap_data.tilesets.push(LevelTilesetData {
                id: *id,
                identifier: tileset_info.data.identifier.clone(),
                texture_path: relative_path,
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
    scene.level_data.tilemap = Some(tilemap_data);

    // Create BevyScene and save to .bscene file
    let bevy_scene = eryndor_common::BevyScene::new(scene.level_data.clone());
    if let Err(e) = bevy_scene.save_to_file(path) {
        error!("Failed to save scene: {}", e);
    } else {
        scene.file_path = Some(path.to_string());
        scene.is_modified = false;

        // Update scene name from filename
        if let Some(filename) = std::path::Path::new(path)
            .file_stem()
            .and_then(|s| s.to_str()) {
            scene.name = filename.to_string();
        }

        info!("Scene saved to {}", path);
    }
}

fn save_as_dialog_with_tilemap(
    scene: &mut crate::scene_tabs::OpenScene,  // Changed from CurrentLevel
    editor_state: &EditorState,
    tileset_manager: &crate::tileset_manager::TilesetManager,
    map_dimensions: &crate::map_canvas::MapDimensions,
    tilemap_query: &Query<&bevy_ecs_tilemap::prelude::TileStorage, With<crate::map_canvas::MapCanvas>>,
    tile_query: &Query<(&bevy_ecs_tilemap::prelude::TileTextureIndex, &bevy_ecs_tilemap::prelude::TileVisible, &bevy_ecs_tilemap::prelude::TilePos)>,
) {
    if let Some(path) = rfd::FileDialog::new()
        .add_filter("Bevy Scene", &["bscene"])
        .set_directory("./assets/levels")
        .set_file_name("level.bscene")
        .save_file()
    {
        let path_str = path.to_string_lossy().to_string();
        save_level_with_tilemap(
            scene,
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
    open_scenes: &mut ResMut<crate::scene_tabs::OpenScenes>,  // Changed from CurrentLevel
    mut pending_restore: &mut ResMut<PendingTilemapRestore>,
    mut tileset_manager: &mut ResMut<crate::tileset_manager::TilesetManager>,
    mut commands: &mut Commands,
    existing_canvas: &Query<Entity, With<crate::map_canvas::MapCanvas>>,
) {
    // Check if active scene has unsaved changes
    if let Some(scene) = open_scenes.active_scene() {
        if scene.is_modified {
            warn!("Active scene has unsaved changes - save first!");
            // TODO: Show unsaved changes dialog
            return;
        }
    }

    if let Some(path) = rfd::FileDialog::new()
        .add_filter("Bevy Scene", &["bscene"])
        .set_directory("./assets/levels")
        .pick_file()
    {
        let path_str = path.to_string_lossy().to_string();
        // Load .bscene file and extract level data
        match eryndor_common::BevyScene::load_from_file(&path_str) {
            Ok(bevy_scene) => {
                // Clear existing tilemap
                for entity in existing_canvas.iter() {
                    commands.entity(entity).despawn();
                }

                // Clear tilesets
                tileset_manager.clear();

                // Create new scene and add to tabs
                let scene_name = path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("Untitled")
                    .to_string();

                let new_scene = crate::scene_tabs::OpenScene {
                    name: scene_name,
                    file_path: Some(path_str.clone()),
                    level_data: bevy_scene.data,
                    is_modified: false,
                };

                open_scenes.add_scene(new_scene);

                // Flag that we need to restore tilemap after canvas is created
                pending_restore.should_restore = true;

                info!("Scene loaded from {}", path_str);
                // TODO: Reload all visual entities
            }
            Err(e) => {
                error!("Failed to load scene: {}", e);
            }
        }
    }
}

/// System to restore tilemap data when a level is loaded
pub fn restore_tilemap_from_level(
    mut pending_restore: ResMut<PendingTilemapRestore>,
    open_scenes: Res<crate::scene_tabs::OpenScenes>,  // Changed from CurrentLevel
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

    // Get active scene's tilemap data
    let tilemap_data = open_scenes.active_scene()
        .and_then(|scene| scene.level_data.tilemap.as_ref());

    if let Some(tilemap_data) = tilemap_data {
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

/// Convert absolute asset path to relative path from assets folder
fn convert_to_relative_asset_path(absolute_path: &str) -> String {
    // Find "assets" in the path and return everything after it
    if let Some(idx) = absolute_path.find("assets") {
        // Skip "assets/" or "assets\" to get the relative path
        let relative = &absolute_path[idx + 7..]; // "assets/" is 7 chars
        return relative.replace('\\', "/"); // Normalize to forward slashes
    }

    // Fallback: return the original path if "assets" not found
    absolute_path.to_string()
}

/// Update the last opened scene in the project config
fn update_last_opened_scene(project: &mut crate::project_manager::CurrentProject, scene_path: &str) {
    use std::path::Path;

    // Convert absolute scene path to relative path (relative to assets/world/)
    let path = Path::new(scene_path);

    // Try to extract filename from path
    if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
        // Update last opened scene
        let _ = project.update_config(|config| {
            config.last_opened_scene = Some(filename.to_string());
            info!("Updated last opened scene to: {}", filename);
        });
    }
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

/// System to sync tilemap when switching scenes
/// This captures the current tilemap state, saves it to the old scene,
/// and loads the new scene's tilemap data
pub fn sync_tilemap_on_scene_switch(
    mut open_scenes: ResMut<crate::scene_tabs::OpenScenes>,
    mut previous_scene: Local<Option<usize>>,
    tilemap_query: Query<&bevy_ecs_tilemap::prelude::TileStorage, With<crate::map_canvas::MapCanvas>>,
    mut tile_query: Query<(&mut bevy_ecs_tilemap::prelude::TileTextureIndex, &mut bevy_ecs_tilemap::prelude::TileVisible)>,
    editor_state: Res<EditorState>,
    map_dimensions: Res<crate::map_canvas::MapDimensions>,
    tileset_manager: Res<crate::tileset_manager::TilesetManager>,
) {
    let current_index = open_scenes.active_index;

    // Check if scene actually changed
    if *previous_scene == Some(current_index) {
        return;
    }

    // PHASE 1: Save current tilemap to previous scene (if any)
    if let Some(prev_idx) = *previous_scene {
        if let Some(prev_scene) = open_scenes.scenes.get_mut(prev_idx) {
            if let Ok(tile_storage) = tilemap_query.get_single() {
                // Capture current tilemap state
                let tilemap_data = capture_tilemap_state(
                    tile_storage,
                    &tile_query,
                    &editor_state,
                    &map_dimensions,
                    &tileset_manager,
                );
                prev_scene.level_data.tilemap = Some(tilemap_data);
                info!("Saved tilemap state for scene '{}'", prev_scene.name);
            }
        }
    }

    // PHASE 2: Clear tilemap (set all tiles invisible AND reset texture)
    if let Ok(tile_storage) = tilemap_query.get_single() {
        for y in 0..map_dimensions.height {
            for x in 0..map_dimensions.width {
                let tile_pos = bevy_ecs_tilemap::prelude::TilePos { x, y };
                if let Some(tile_entity) = tile_storage.get(&tile_pos) {
                    if let Ok((mut tex, mut visible)) = tile_query.get_mut(tile_entity) {
                        tex.0 = 0;  // Reset texture index to 0
                        visible.0 = false;  // Make invisible
                    }
                }
            }
        }
    }

    // PHASE 3: Load active scene's tilemap
    if let Some(active_scene) = open_scenes.active_scene() {
        if let Some(tilemap_data) = &active_scene.level_data.tilemap {
            if let Ok(tile_storage) = tilemap_query.get_single() {
                // Restore tiles from data
                if !tilemap_data.layers.is_empty() {
                    let layer = &tilemap_data.layers[0];
                    info!("Loading {} tiles for scene '{}'", layer.tiles.len(), active_scene.name);

                    for tile_instance in &layer.tiles {
                        let tile_pos = bevy_ecs_tilemap::prelude::TilePos {
                            x: tile_instance.x,
                            y: tile_instance.y,
                        };
                        if let Some(tile_entity) = tile_storage.get(&tile_pos) {
                            if let Ok((mut tex, mut vis)) = tile_query.get_mut(tile_entity) {
                                tex.0 = tile_instance.tile_id;
                                vis.0 = true;
                            }
                        }
                    }
                }
            }
        } else {
            info!("Switched to scene '{}' (empty tilemap)", active_scene.name);
        }
    }

    // Update previous scene tracker
    *previous_scene = Some(current_index);
}

/// Helper function to capture current tilemap state into LevelTilemapData
fn capture_tilemap_state(
    tile_storage: &bevy_ecs_tilemap::prelude::TileStorage,
    tile_query: &Query<(&mut bevy_ecs_tilemap::prelude::TileTextureIndex, &mut bevy_ecs_tilemap::prelude::TileVisible)>,
    editor_state: &EditorState,
    map_dimensions: &crate::map_canvas::MapDimensions,
    tileset_manager: &crate::tileset_manager::TilesetManager,
) -> eryndor_common::LevelTilemapData {
    use eryndor_common::{LevelTilemapData, LevelLayerData, LevelTileInstance, LevelTilesetData};
    use std::collections::HashSet;

    let mut tiles = Vec::new();
    for y in 0..map_dimensions.height {
        for x in 0..map_dimensions.width {
            let tile_pos = bevy_ecs_tilemap::prelude::TilePos { x, y };
            if let Some(tile_entity) = tile_storage.get(&tile_pos) {
                if let Ok((tex, vis)) = tile_query.get(tile_entity) {
                    if vis.0 {  // Only save visible tiles
                        tiles.push(LevelTileInstance {
                            x,
                            y,
                            tile_id: tex.0,
                        });
                    }
                }
            }
        }
    }

    // Build tileset data
    let mut tilesets = Vec::new();
    let mut seen_paths = HashSet::new();
    for (id, tileset_info) in tileset_manager.tilesets.iter() {
        if seen_paths.insert(tileset_info.data.texture_path.clone()) {
            let relative_path = convert_to_relative_asset_path(&tileset_info.data.texture_path);
            tilesets.push(LevelTilesetData {
                id: *id,
                identifier: tileset_info.data.identifier.clone(),
                texture_path: relative_path,
                tile_width: tileset_info.data.tile_width,
                tile_height: tileset_info.data.tile_height,
            });
        }
    }

    LevelTilemapData {
        grid_size: editor_state.grid_size,
        map_width: map_dimensions.width,
        map_height: map_dimensions.height,
        tilesets,
        selected_tileset_id: tileset_manager.selected_tileset_id,
        layers: vec![LevelLayerData {
            id: 0,
            name: "Layer 0".to_string(),
            visible: true,
            tiles,
        }],
    }
}
