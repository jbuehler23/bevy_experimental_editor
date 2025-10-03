use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::{EditorState, EditorTool, tile_painter::{TilePainter, PaintMode}};

/// Toolbar UI system - displays tool selection and paint modes
pub fn toolbar_ui(
    mut contexts: EguiContexts,
    mut editor_state: ResMut<EditorState>,
    mut tile_painter: ResMut<TilePainter>,
) {
    let ctx = contexts.ctx_mut();

    // Top toolbar panel below menu bar
    egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.heading("Tools:");
            ui.separator();

            // Main editor tools
            if ui.selectable_label(editor_state.current_tool == EditorTool::Select, "🖱 Select (V)")
                .on_hover_text("Select and move entities\nShortcut: V")
                .clicked() {
                editor_state.current_tool = EditorTool::Select;
            }

            if ui.selectable_label(
                editor_state.current_tool == EditorTool::Platform && tile_painter.mode == PaintMode::Single,
                "🖌 Brush (B)"
            ).on_hover_text("Paint single tiles\nShortcut: B").clicked() {
                editor_state.current_tool = EditorTool::Platform;
                tile_painter.mode = PaintMode::Single;
            }

            if ui.selectable_label(
                editor_state.current_tool == EditorTool::Platform && tile_painter.mode == PaintMode::Rectangle,
                "▭ Rectangle (R)"
            ).on_hover_text("Draw filled rectangle\nShortcut: R").clicked() {
                editor_state.current_tool = EditorTool::Platform;
                tile_painter.mode = PaintMode::Rectangle;
            }

            if ui.selectable_label(
                editor_state.current_tool == EditorTool::Platform && tile_painter.mode == PaintMode::Line,
                "─ Line (L)"
            ).on_hover_text("Draw straight line\nShortcut: L").clicked() {
                editor_state.current_tool = EditorTool::Platform;
                tile_painter.mode = PaintMode::Line;
            }

            if ui.selectable_label(
                editor_state.current_tool == EditorTool::Platform && tile_painter.mode == PaintMode::BucketFill,
                "🪣 Fill (F)"
            ).on_hover_text("Bucket fill connected tiles\nShortcut: F").clicked() {
                editor_state.current_tool = EditorTool::Platform;
                tile_painter.mode = PaintMode::BucketFill;
            }

            if ui.selectable_label(editor_state.current_tool == EditorTool::Eyedropper, "💧 Eyedropper (I)")
                .on_hover_text("Pick tile from canvas\nShortcut: I or hold Alt")
                .clicked() {
                editor_state.current_tool = EditorTool::Eyedropper;
            }

            if ui.selectable_label(editor_state.current_tool == EditorTool::Erase, "✖ Erase (E)")
                .on_hover_text("Erase tiles\nShortcut: E")
                .clicked() {
                editor_state.current_tool = EditorTool::Erase;
            }

            ui.separator();

            // Show current mode info
            let mode_text = match editor_state.current_tool {
                EditorTool::Platform => match tile_painter.mode {
                    PaintMode::Single => "Brush",
                    PaintMode::Rectangle => "Rectangle",
                    PaintMode::Line => "Line",
                    PaintMode::BucketFill => "Fill",
                },
                EditorTool::Select => "Select",
                EditorTool::Eyedropper => "Eyedropper",
                EditorTool::Erase => "Erase",
                EditorTool::EntityPlace => "Entity",
            };
            ui.label(format!("Active: {}", mode_text));

            // Grid toggle
            ui.separator();
            if ui.checkbox(&mut editor_state.grid_snap_enabled, "Grid (G)").changed() {
                info!("Grid: {}", if editor_state.grid_snap_enabled { "ON" } else { "OFF" });
            }
        });
    });
}
