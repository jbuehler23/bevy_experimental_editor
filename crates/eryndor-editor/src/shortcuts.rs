use bevy::prelude::*;
use crate::{EditorState, EditorTool, tile_painter::{TilePainter, PaintMode}};

/// System to handle global keyboard shortcuts
/// Runs before UI systems to capture shortcuts first
pub fn handle_global_shortcuts(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut editor_state: ResMut<EditorState>,
    mut tile_painter: ResMut<TilePainter>,
    mut contexts: bevy_egui::EguiContexts,
) {
    // Don't process shortcuts if typing in a text field
    let ctx = contexts.ctx_mut();
    if ctx.wants_keyboard_input() {
        return;
    }

    // Tool selection shortcuts (no modifiers)
    if !keyboard.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight, KeyCode::ShiftLeft, KeyCode::ShiftRight, KeyCode::AltLeft, KeyCode::AltRight]) {
        if keyboard.just_pressed(KeyCode::KeyB) {
            editor_state.current_tool = EditorTool::Platform;
            tile_painter.mode = PaintMode::Single;
            info!("Switched to Brush tool");
        } else if keyboard.just_pressed(KeyCode::KeyR) {
            editor_state.current_tool = EditorTool::Platform;
            tile_painter.mode = PaintMode::Rectangle;
            info!("Switched to Rectangle tool");
        } else if keyboard.just_pressed(KeyCode::KeyF) {
            editor_state.current_tool = EditorTool::Platform;
            tile_painter.mode = PaintMode::BucketFill;
            info!("Switched to Bucket Fill tool");
        } else if keyboard.just_pressed(KeyCode::KeyL) {
            editor_state.current_tool = EditorTool::Platform;
            tile_painter.mode = PaintMode::Line;
            info!("Switched to Line tool");
        } else if keyboard.just_pressed(KeyCode::KeyE) {
            editor_state.current_tool = EditorTool::Erase;
            info!("Switched to Erase tool");
        } else if keyboard.just_pressed(KeyCode::KeyI) {
            editor_state.current_tool = EditorTool::Eyedropper;
            info!("Switched to Eyedropper tool");
        } else if keyboard.just_pressed(KeyCode::KeyV) {
            editor_state.current_tool = EditorTool::Select;
            info!("Switched to Select tool");
        } else if keyboard.just_pressed(KeyCode::KeyG) {
            editor_state.grid_snap_enabled = !editor_state.grid_snap_enabled;
            info!("Grid: {}", if editor_state.grid_snap_enabled { "ON" } else { "OFF" });
        }
    }

    // Alt key for temporary eyedropper (handled in tile painting system)
    // Space key for pan camera (handled in camera system)
}

/// Check if Alt key is held for temporary eyedropper mode
pub fn is_eyedropper_modifier_held(keyboard: &ButtonInput<KeyCode>) -> bool {
    keyboard.pressed(KeyCode::AltLeft) || keyboard.pressed(KeyCode::AltRight)
}
