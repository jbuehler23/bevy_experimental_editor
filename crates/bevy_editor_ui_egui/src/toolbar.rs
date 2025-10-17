use crate::icons::Icons;
use bevy::prelude::*;
use bevy_editor_foundation::{EditorState, EditorTool};
use bevy_editor_frontend_api::{EditorAction, ProjectCommand};
use bevy_editor_project::BevyCLIRunner;
use bevy_editor_tilemap::{PaintMode, TilePainter};
use bevy_egui::egui;

/// Render toolbar content (called from ui_system within a panel)
pub fn render_toolbar_content(
    ui: &mut egui::Ui,
    editor_state: &mut EditorState,
    tile_painter: &mut TilePainter,
    cli_runner: &BevyCLIRunner,
    editor_actions: &mut EventWriter<EditorAction>,
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
            editor_actions.write(EditorAction::SelectTool(EditorTool::Select));
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
            editor_actions.write(EditorAction::SelectTool(EditorTool::Platform));
        }

        if ui
            .selectable_label(
                editor_state.current_tool == EditorTool::Platform
                    && tile_painter.mode == PaintMode::Rectangle,
                format!("{} Rectangle (R)", Icons::BRUSH),
            )
            .on_hover_text("Draw filled rectangles\nShortcut: R")
            .clicked()
        {
            editor_state.current_tool = EditorTool::Platform;
            tile_painter.mode = PaintMode::Rectangle;
            editor_actions.write(EditorAction::SelectTool(EditorTool::Platform));
        }

        if ui
            .selectable_label(
                editor_state.current_tool == EditorTool::Platform
                    && tile_painter.mode == PaintMode::Line,
                format!("{} Line (L)", Icons::BRUSH),
            )
            .on_hover_text("Draw straight lines\nShortcut: L")
            .clicked()
        {
            editor_state.current_tool = EditorTool::Platform;
            tile_painter.mode = PaintMode::Line;
            editor_actions.write(EditorAction::SelectTool(EditorTool::Platform));
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
            editor_actions.write(EditorAction::SelectTool(EditorTool::Platform));
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
            editor_actions.write(EditorAction::SelectTool(EditorTool::Eyedropper));
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
            editor_actions.write(EditorAction::SelectTool(EditorTool::Erase));
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
            editor_actions.write(EditorAction::SetGridSnap {
                enabled: editor_state.grid_snap_enabled,
            });
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
                    editor_actions.write(EditorAction::CancelProjectCommand);
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
                    editor_actions.write(EditorAction::RunProjectCommand {
                        command: ProjectCommand::Run,
                    });
                }

                if ui
                    .button(format!("{} Play Scene", Icons::PLAY))
                    .on_hover_text("Run the currently active scene with game logic")
                    .clicked()
                {
                    editor_actions.write(EditorAction::RunProjectCommand {
                        command: ProjectCommand::RunScene,
                    });
                }

                if ui
                    .button(format!("{} Web", Icons::PLAY))
                    .on_hover_text("Run web build (bevy run web)")
                    .clicked()
                {
                    editor_actions.write(EditorAction::RunProjectCommand {
                        command: ProjectCommand::RunWeb,
                    });
                }

                if ui
                    .button(format!("{} Build", Icons::BUILD))
                    .on_hover_text("Build native (cargo build)")
                    .clicked()
                {
                    editor_actions.write(EditorAction::RunProjectCommand {
                        command: ProjectCommand::Build,
                    });
                }

                if ui
                    .button(format!("{} Lint", Icons::BUILD))
                    .on_hover_text("Run linter (bevy lint)")
                    .clicked()
                {
                    editor_actions.write(EditorAction::RunProjectCommand {
                        command: ProjectCommand::Lint,
                    });
                }
            });

            if !has_project {
                ui.label("No project loaded");
            }
        });
    });
}
