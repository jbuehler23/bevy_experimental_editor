use crate::bevy_cli_runner::{BevyCLIRunner, CLICommand};
use crate::icons::Icons;
use crate::{
    tile_painter::{PaintMode, TilePainter},
    EditorState, EditorTool,
};
use bevy::prelude::*;
use bevy_egui::egui;

/// Render toolbar content (called from ui_system within a panel)
pub fn render_toolbar_content(
    ui: &mut egui::Ui,
    editor_state: &mut EditorState,
    tile_painter: &mut TilePainter,
    cli_runner: &mut BevyCLIRunner,
) {
    ui.horizontal(|ui| {
        ui.heading("Tools:");
        ui.separator();

        // Main editor tools
        if ui
            .selectable_label(
                editor_state.current_tool == EditorTool::Select,
                format!("{} Select (V)", Icons::ARROW_UP),
            )
            .on_hover_text("Select and move entities\nShortcut: V")
            .clicked()
        {
            editor_state.current_tool = EditorTool::Select;
        }

        if ui
            .selectable_label(
                editor_state.current_tool == EditorTool::Platform
                    && tile_painter.mode == PaintMode::Single,
                format!("{} Brush (B)", Icons::BRUSH),
            )
            .on_hover_text("Paint single tiles\nShortcut: B")
            .clicked()
        {
            editor_state.current_tool = EditorTool::Platform;
            tile_painter.mode = PaintMode::Single;
        }

        if ui
            .selectable_label(
                editor_state.current_tool == EditorTool::Platform
                    && tile_painter.mode == PaintMode::Rectangle,
                "▭ Rectangle (R)",
            )
            .on_hover_text("Draw filled rectangle\nShortcut: R")
            .clicked()
        {
            editor_state.current_tool = EditorTool::Platform;
            tile_painter.mode = PaintMode::Rectangle;
        }

        if ui
            .selectable_label(
                editor_state.current_tool == EditorTool::Platform
                    && tile_painter.mode == PaintMode::Line,
                "─ Line (L)",
            )
            .on_hover_text("Draw straight line\nShortcut: L")
            .clicked()
        {
            editor_state.current_tool = EditorTool::Platform;
            tile_painter.mode = PaintMode::Line;
        }

        if ui
            .selectable_label(
                editor_state.current_tool == EditorTool::Platform
                    && tile_painter.mode == PaintMode::BucketFill,
                format!("{} Fill (F)", Icons::BUCKET),
            )
            .on_hover_text("Bucket fill connected tiles\nShortcut: F")
            .clicked()
        {
            editor_state.current_tool = EditorTool::Platform;
            tile_painter.mode = PaintMode::BucketFill;
        }

        if ui
            .selectable_label(
                editor_state.current_tool == EditorTool::Eyedropper,
                format!("{} Eyedropper (I)", Icons::EYEDROPPER),
            )
            .on_hover_text("Pick tile from canvas\nShortcut: I or hold Alt")
            .clicked()
        {
            editor_state.current_tool = EditorTool::Eyedropper;
        }

        if ui
            .selectable_label(
                editor_state.current_tool == EditorTool::Erase,
                format!("{} Erase (E)", Icons::ERASER),
            )
            .on_hover_text("Erase tiles\nShortcut: E")
            .clicked()
        {
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
        if ui
            .checkbox(&mut editor_state.grid_snap_enabled, "Grid (G)")
            .changed()
        {
            info!(
                "Grid: {}",
                if editor_state.grid_snap_enabled {
                    "ON"
                } else {
                    "OFF"
                }
            );
        }

        // Bevy CLI buttons (right side)
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let has_project = cli_runner.project_path.is_some();
            let is_running = cli_runner.is_running();

            // Stop button (only shown when something is running)
            if is_running {
                if ui
                    .button(format!("{} Stop", Icons::STOP))
                    .on_hover_text("Stop running process")
                    .clicked()
                {
                    cli_runner.stop_current_process();
                }

                // Show what's running
                if let Some(cmd) = cli_runner.current_command() {
                    ui.label(format!("Running: {}", cmd.name()));
                }
            }

            ui.separator();

            // CLI command buttons
            ui.add_enabled_ui(has_project && !is_running, |ui| {
                if ui
                    .button(format!("{} Run", Icons::PLAY))
                    .on_hover_text("Run game (bevy run)")
                    .clicked()
                {
                    if let Err(e) = cli_runner.run_command(CLICommand::Run) {
                        error!("Failed to run game: {}", e);
                    }
                }

                if ui
                    .button(format!("{} Web", "🌐"))
                    .on_hover_text("Run web build (bevy run web)")
                    .clicked()
                {
                    if let Err(e) = cli_runner.run_command(CLICommand::RunWeb) {
                        error!("Failed to run web: {}", e);
                    }
                }

                if ui
                    .button(format!("{} Build", Icons::BUILD))
                    .on_hover_text("Build native (cargo build) - speeds up subsequent runs")
                    .clicked()
                {
                    if let Err(e) = cli_runner.run_command(CLICommand::Build) {
                        error!("Failed to build: {}", e);
                    }
                }

                if ui
                    .button(format!("{} Lint", "🔍"))
                    .on_hover_text("Run linter (bevy lint)")
                    .clicked()
                {
                    if let Err(e) = cli_runner.run_command(CLICommand::Lint) {
                        error!("Failed to lint: {}", e);
                    }
                }
            });

            if !has_project {
                ui.label("No project loaded");
            }
        });
    });
}
