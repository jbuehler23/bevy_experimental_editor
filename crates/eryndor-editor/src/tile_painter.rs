use bevy::prelude::*;
use eryndor_common::TileData;
use crate::{TilesetManager, LayerManager, EditorTool};

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
}

impl Default for TilePainter {
    fn default() -> Self {
        Self {
            mode: PaintMode::Single,
            flip_x: false,
            flip_y: false,
            drag_start: None,
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
) {
    // Only paint in Tile Brush mode
    if editor_state.current_tool != EditorTool::Platform {
        return;
    }

    // Don't paint if hovering over UI
    let ctx = contexts.ctx_mut();
    if ctx.is_pointer_over_area() {
        return;
    }

    // Get mouse world position
    let Some(mouse_world_pos) = get_mouse_world_position(&windows, &camera_q) else {
        return;
    };

    // Get selected tile
    let Some(selected_tile_id) = tileset_manager.get_selected_tile() else {
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

    // Handle painting
    if mouse_button.pressed(MouseButton::Left) {
        match tile_painter.mode {
            PaintMode::Single => {
                paint_single_tile(
                    tile_x,
                    tile_y,
                    selected_tile_id,
                    tile_painter.flip_x,
                    tile_painter.flip_y,
                    &mut layer_manager,
                );
            }
            PaintMode::Rectangle => {
                if mouse_button.just_pressed(MouseButton::Left) {
                    tile_painter.drag_start = Some((tile_x, tile_y));
                }
                // Rectangle is painted on release (see below)
            }
            PaintMode::Line => {
                if mouse_button.just_pressed(MouseButton::Left) {
                    tile_painter.drag_start = Some((tile_x, tile_y));
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
                    );
                }
            }
        }
    }

    // Handle rectangle/line completion
    if mouse_button.just_released(MouseButton::Left) {
        if let Some((start_x, start_y)) = tile_painter.drag_start {
            match tile_painter.mode {
                PaintMode::Rectangle => {
                    paint_rectangle(
                        start_x,
                        start_y,
                        tile_x,
                        tile_y,
                        selected_tile_id,
                        tile_painter.flip_x,
                        tile_painter.flip_y,
                        &mut layer_manager,
                    );
                }
                PaintMode::Line => {
                    paint_line(
                        start_x,
                        start_y,
                        tile_x,
                        tile_y,
                        selected_tile_id,
                        tile_painter.flip_x,
                        tile_painter.flip_y,
                        &mut layer_manager,
                    );
                }
                _ => {}
            }
            tile_painter.drag_start = None;
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
) {
    let tile = TileData {
        x,
        y,
        tile_id,
        flip_x,
        flip_y,
    };
    layer_manager.add_tile(tile);
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
) {
    let min_x = start_x.min(end_x);
    let max_x = start_x.max(end_x);
    let min_y = start_y.min(end_y);
    let max_y = start_y.max(end_y);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            paint_single_tile(x, y, tile_id, flip_x, flip_y, layer_manager);
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
        paint_single_tile(x as u32, y as u32, tile_id, flip_x, flip_y, layer_manager);

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

fn bucket_fill(
    start_x: u32,
    start_y: u32,
    tile_id: u32,
    flip_x: bool,
    flip_y: bool,
    layer_manager: &mut LayerManager,
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
        paint_single_tile(x, y, tile_id, flip_x, flip_y, layer_manager);

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
