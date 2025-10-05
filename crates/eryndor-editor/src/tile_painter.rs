use bevy::prelude::*;
use eryndor_common::TileData;
use crate::{TilesetManager, LayerManager, EditorTool};
use crate::map_canvas::PaintTileEvent;

/// Tile painting mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaintMode {
    Single,      // Paint single tiles
    Rectangle,   // Paint rectangular area
    BucketFill,  // Fill connected tiles
    Line,        // Draw line of tiles
}

/// Tile painter state
#[derive(Resource)]
pub struct TilePainter {
    pub mode: PaintMode,
    pub flip_x: bool,
    pub flip_y: bool,
    /// Start position for rectangle/line tools
    pub drag_start: Option<(u32, u32)>,
    /// Current cursor position (for preview)
    pub current_pos: Option<(u32, u32)>,
}

impl Default for TilePainter {
    fn default() -> Self {
        Self {
            mode: PaintMode::Single,
            flip_x: false,
            flip_y: false,
            drag_start: None,
            current_pos: None,
        }
    }
}

/// System to handle tile painting
pub fn handle_tile_painting(
    mouse_button: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    tileset_manager: Res<TilesetManager>,
    mut layer_manager: ResMut<LayerManager>,
    mut tile_painter: ResMut<TilePainter>,
    editor_state: Res<crate::EditorState>,
    mut contexts: bevy_egui::EguiContexts,
    mut paint_events: EventWriter<PaintTileEvent>,
) {
    // Only paint in Tile Brush mode
    if editor_state.current_tool != EditorTool::Platform {
        tile_painter.current_pos = None; // Clear preview when not in Platform tool
        if mouse_button.just_pressed(MouseButton::Left) {
            info!("Click ignored: current_tool = {:?}, need Platform tool", editor_state.current_tool);
        }
        return;
    }

    // Don't paint if hovering over UI
    let ctx = contexts.ctx_mut();
    if ctx.is_pointer_over_area() {
        tile_painter.current_pos = None; // Clear preview when over UI
        if mouse_button.just_pressed(MouseButton::Left) {
            info!("Click ignored: pointer over UI");
        }
        return;
    }

    // Get mouse world position
    let Some(mouse_world_pos) = get_mouse_world_position(&windows, &camera_q) else {
        tile_painter.current_pos = None; // Clear preview when no valid mouse position
        if mouse_button.just_pressed(MouseButton::Left) {
            info!("Click ignored: no valid mouse world position");
        }
        return;
    };

    // Get selected tile
    let Some(selected_tile_id) = tileset_manager.get_selected_tile() else {
        tile_painter.current_pos = None; // Clear preview when no tile selected
        if mouse_button.just_pressed(MouseButton::Left) {
            info!("Click ignored: no tile selected");
        }
        return;
    };

    // Get active layer
    let Some(active_layer) = layer_manager.get_active_layer() else {
        tile_painter.current_pos = None; // Clear preview when no active layer
        if mouse_button.just_pressed(MouseButton::Left) {
            warn!("Cannot paint: No layer exists! Create a layer first using the Layers panel.");
        }
        return;
    };

    let grid_size = active_layer.metadata.grid_size as f32;

    // Convert world position to tile grid coordinates
    let tile_x = (mouse_world_pos.x / grid_size).floor() as i32;
    let tile_y = (mouse_world_pos.y / grid_size).floor() as i32;

    // Ensure coordinates are within layer bounds
    if tile_x < 0 || tile_y < 0
        || tile_x >= active_layer.metadata.width as i32
        || tile_y >= active_layer.metadata.height as i32 {
        tile_painter.current_pos = None; // Clear preview when outside bounds
        if mouse_button.just_pressed(MouseButton::Left) {
            info!("Click ignored: outside bounds ({}, {}) - layer size {}x{}",
                tile_x, tile_y, active_layer.metadata.width, active_layer.metadata.height);
        }
        return;
    }

    let tile_x = tile_x as u32;
    let tile_y = tile_y as u32;

    // Update current position for preview
    tile_painter.current_pos = Some((tile_x, tile_y));

    // Log when we successfully reach painting logic
    if mouse_button.just_pressed(MouseButton::Left) {
        info!("PAINT ATTEMPT: pos ({}, {}), tool {:?}, mode {:?}, tile {}",
            tile_x, tile_y, editor_state.current_tool, tile_painter.mode, selected_tile_id);
    }

    // Handle painting
    if mouse_button.pressed(MouseButton::Left) {
        match tile_painter.mode {
            PaintMode::Single => {
                // Check if we're in stamp mode (multiple tiles selected)
                if tileset_manager.selected_tiles.len() > 1 {
                    // Paint stamp pattern
                    paint_stamp(
                        tile_x,
                        tile_y,
                        &tileset_manager,
                        tile_painter.flip_x,
                        tile_painter.flip_y,
                        &mut layer_manager,
                        &mut paint_events,
                    );
                } else {
                    // Paint single tile
                    paint_single_tile(
                        tile_x,
                        tile_y,
                        selected_tile_id,
                        tile_painter.flip_x,
                        tile_painter.flip_y,
                        &mut layer_manager,
                        &mut paint_events,
                    );
                }
            }
            PaintMode::Rectangle => {
                if mouse_button.just_pressed(MouseButton::Left) {
                    tile_painter.drag_start = Some((tile_x, tile_y));
                    info!("Rectangle tool: drag_start set to ({}, {})", tile_x, tile_y);
                }
                // Rectangle is painted on release (see below)
            }
            PaintMode::Line => {
                if mouse_button.just_pressed(MouseButton::Left) {
                    tile_painter.drag_start = Some((tile_x, tile_y));
                    info!("Line tool: drag_start set to ({}, {})", tile_x, tile_y);
                }
                // Line is painted on release (see below)
            }
            PaintMode::BucketFill => {
                if mouse_button.just_pressed(MouseButton::Left) {
                    bucket_fill(
                        tile_x,
                        tile_y,
                        selected_tile_id,
                        tile_painter.flip_x,
                        tile_painter.flip_y,
                        &mut layer_manager,
                        &mut paint_events,
                    );
                }
            }
        }
    }

    // Handle rectangle/line completion
    if mouse_button.just_released(MouseButton::Left) {
        if let Some((start_x, start_y)) = tile_painter.drag_start {
            info!("Mouse released with drag_start at ({}, {}), current pos ({}, {}), mode: {:?}",
                start_x, start_y, tile_x, tile_y, tile_painter.mode);

            match tile_painter.mode {
                PaintMode::Rectangle => {
                    info!("Painting rectangle from ({}, {}) to ({}, {})", start_x, start_y, tile_x, tile_y);
                    paint_rectangle(
                        start_x,
                        start_y,
                        tile_x,
                        tile_y,
                        selected_tile_id,
                        tile_painter.flip_x,
                        tile_painter.flip_y,
                        &mut layer_manager,
                        &mut paint_events,
                    );
                }
                PaintMode::Line => {
                    info!("Painting line from ({}, {}) to ({}, {})", start_x, start_y, tile_x, tile_y);
                    paint_line(
                        start_x,
                        start_y,
                        tile_x,
                        tile_y,
                        selected_tile_id,
                        tile_painter.flip_x,
                        tile_painter.flip_y,
                        &mut layer_manager,
                        &mut paint_events,
                    );
                }
                _ => {}
            }
            tile_painter.drag_start = None;
        } else {
            info!("Mouse released but drag_start is None, mode: {:?}", tile_painter.mode);
        }
    }

    // Handle erasing with right mouse button
    if mouse_button.pressed(MouseButton::Right) {
        layer_manager.remove_tile(tile_x, tile_y);
    }

    // Toggle flip with keyboard
    if keyboard.just_pressed(KeyCode::KeyX) {
        tile_painter.flip_x = !tile_painter.flip_x;
    }
    if keyboard.just_pressed(KeyCode::KeyY) {
        tile_painter.flip_y = !tile_painter.flip_y;
    }
}

fn paint_single_tile(
    x: u32,
    y: u32,
    tile_id: u32,
    flip_x: bool,
    flip_y: bool,
    layer_manager: &mut LayerManager,
    paint_events: &mut EventWriter<PaintTileEvent>,
) {
    let tile = TileData {
        x,
        y,
        tile_id,
        flip_x,
        flip_y,
    };
    layer_manager.add_tile(tile);

    // Send paint event to update visual tilemap
    paint_events.write(PaintTileEvent {
        layer_id: 0,
        x,
        y,
        tile_id,
    });
}

fn paint_rectangle(
    start_x: u32,
    start_y: u32,
    end_x: u32,
    end_y: u32,
    tile_id: u32,
    flip_x: bool,
    flip_y: bool,
    layer_manager: &mut LayerManager,
    paint_events: &mut EventWriter<PaintTileEvent>,
) {
    let min_x = start_x.min(end_x);
    let max_x = start_x.max(end_x);
    let min_y = start_y.min(end_y);
    let max_y = start_y.max(end_y);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            paint_single_tile(x, y, tile_id, flip_x, flip_y, layer_manager, paint_events);
        }
    }
}

fn paint_line(
    start_x: u32,
    start_y: u32,
    end_x: u32,
    end_y: u32,
    tile_id: u32,
    flip_x: bool,
    flip_y: bool,
    layer_manager: &mut LayerManager,
    paint_events: &mut EventWriter<PaintTileEvent>,
) {
    // Bresenham's line algorithm
    let dx = (end_x as i32 - start_x as i32).abs();
    let dy = (end_y as i32 - start_y as i32).abs();
    let sx = if start_x < end_x { 1 } else { -1 };
    let sy = if start_y < end_y { 1 } else { -1 };
    let mut err = dx - dy;

    let mut x = start_x as i32;
    let mut y = start_y as i32;

    loop {
        paint_single_tile(x as u32, y as u32, tile_id, flip_x, flip_y, layer_manager, paint_events);

        if x == end_x as i32 && y == end_y as i32 {
            break;
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
}

/// Paint a stamp pattern (multi-tile brush)
fn paint_stamp(
    x: u32,
    y: u32,
    tileset_manager: &TilesetManager,
    flip_x: bool,
    flip_y: bool,
    layer_manager: &mut LayerManager,
    paint_events: &mut EventWriter<PaintTileEvent>,
) {
    // Get stamp dimensions
    let Some((stamp_width, stamp_height)) = tileset_manager.get_selection_dimensions() else {
        return;
    };

    // Get the selection start position to determine pattern layout
    let Some((start_col, start_row)) = tileset_manager.selection_start else {
        return;
    };

    // Paint each tile in the stamp pattern
    for (index, &tile_id) in tileset_manager.selected_tiles.iter().enumerate() {
        // Calculate position within stamp pattern (row-major order)
        let offset_x = (index as u32) % stamp_width;
        let offset_y = (index as u32) / stamp_width;

        // Calculate world position
        // Y offset is inverted: first row in selection should be painted at bottom of stamp
        let world_x = x + offset_x;

        // Calculate inverted Y position safely
        // We want: first row (offset_y=0) at y+(height-1), last row at y+0
        let y_invert = stamp_height.saturating_sub(1).saturating_sub(offset_y);
        let world_y = y + y_invert;

        // Bounds check
        if let Some(layer) = layer_manager.get_active_layer() {
            if world_x >= layer.metadata.width || world_y >= layer.metadata.height {
                continue;
            }
        }

        paint_single_tile(world_x, world_y, tile_id, flip_x, flip_y, layer_manager, paint_events);
    }
}

fn bucket_fill(
    start_x: u32,
    start_y: u32,
    tile_id: u32,
    flip_x: bool,
    flip_y: bool,
    layer_manager: &mut LayerManager,
    paint_events: &mut EventWriter<PaintTileEvent>,
) {
    // Get the current tile at start position
    let target_tile = layer_manager.get_tile_at(start_x, start_y).copied();
    let target_tile_id = target_tile.map(|t| t.tile_id);

    // Don't fill if already the same tile
    if target_tile_id == Some(tile_id) {
        return;
    }

    // Get layer bounds
    let Some(layer) = layer_manager.get_active_layer() else {
        return;
    };
    let width = layer.metadata.width;
    let height = layer.metadata.height;

    // Flood fill using a stack
    let mut stack = vec![(start_x, start_y)];
    let mut visited = std::collections::HashSet::new();

    while let Some((x, y)) = stack.pop() {
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));

        // Check if this tile matches the target
        let current_tile = layer_manager.get_tile_at(x, y).copied();
        let current_tile_id = current_tile.map(|t| t.tile_id);

        if current_tile_id != target_tile_id {
            continue;
        }

        // Paint this tile
        paint_single_tile(x, y, tile_id, flip_x, flip_y, layer_manager, paint_events);

        // Add neighbors to stack
        if x > 0 {
            stack.push((x - 1, y));
        }
        if x < width - 1 {
            stack.push((x + 1, y));
        }
        if y > 0 {
            stack.push((x, y - 1));
        }
        if y < height - 1 {
            stack.push((x, y + 1));
        }
    }
}

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

/// System to handle eyedropper tool - picks tiles from canvas
pub fn handle_eyedropper(
    mouse_button: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut tileset_manager: ResMut<TilesetManager>,
    layer_manager: Res<LayerManager>,
    mut editor_state: ResMut<crate::EditorState>,
    mut tile_painter: ResMut<TilePainter>,
    mut contexts: bevy_egui::EguiContexts,
) {
    // Only handle eyedropper in eyedropper mode OR when Alt is held
    let is_alt_held = keyboard.pressed(KeyCode::AltLeft) || keyboard.pressed(KeyCode::AltRight);
    let is_eyedropper_active = editor_state.current_tool == crate::EditorTool::Eyedropper || is_alt_held;

    if !is_eyedropper_active {
        return;
    }

    // Don't pick if hovering over UI
    let ctx = contexts.ctx_mut();
    if ctx.is_pointer_over_area() {
        return;
    }

    // Get mouse world position
    let Some(mouse_world_pos) = get_mouse_world_position(&windows, &camera_q) else {
        return;
    };

    // Get active layer
    let Some(active_layer) = layer_manager.get_active_layer() else {
        return;
    };

    let grid_size = active_layer.metadata.grid_size as f32;

    // Convert world position to tile grid coordinates
    let tile_x = (mouse_world_pos.x / grid_size).floor() as i32;
    let tile_y = (mouse_world_pos.y / grid_size).floor() as i32;

    // Ensure coordinates are within layer bounds
    if tile_x < 0 || tile_y < 0
        || tile_x >= active_layer.metadata.width as i32
        || tile_y >= active_layer.metadata.height as i32 {
        return;
    }

    let tile_x = tile_x as u32;
    let tile_y = tile_y as u32;

    // Pick tile on click
    if mouse_button.just_pressed(MouseButton::Left) {
        if let Some(tile_data) = layer_manager.get_tile_at(tile_x, tile_y) {
            // Set the picked tile as the selected tile
            tileset_manager.selected_tile_id = Some(tile_data.tile_id);
            info!("Eyedropper picked tile ID: {}", tile_data.tile_id);

            // If Alt was held (temporary mode), switch back to brush tool
            if is_alt_held && editor_state.current_tool != crate::EditorTool::Eyedropper {
                // Keep current tool but apply the picked tile
                info!("Temporary eyedropper - picked tile, staying in current tool");
            } else {
                // Switch to brush mode after picking
                editor_state.current_tool = crate::EditorTool::Platform;
                tile_painter.mode = PaintMode::Single;
                info!("Switched to Brush tool with picked tile");
            }
        } else {
            info!("No tile at position ({}, {})", tile_x, tile_y);
        }
    }
}
