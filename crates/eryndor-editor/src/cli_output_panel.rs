use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::bevy_cli_runner::BeevyCLIRunner;

/// Resource to track CLI output panel visibility
#[derive(Resource)]
pub struct CLIOutputPanel {
    pub visible: bool,
    pub auto_scroll: bool,
}

impl Default for CLIOutputPanel {
    fn default() -> Self {
        Self {
            visible: false,
            auto_scroll: true,
        }
    }
}

/// System to render CLI output panel
pub fn cli_output_panel_ui(
    mut contexts: EguiContexts,
    mut panel: ResMut<CLIOutputPanel>,
    mut cli_runner: ResMut<BeevyCLIRunner>,
) {
    let ctx = contexts.ctx_mut();

    // Show panel when there's output or a process is running
    if cli_runner.is_running() || !cli_runner.output_lines.is_empty() {
        panel.visible = true;
    }

    if !panel.visible {
        return;
    }

    // Bottom panel for CLI output
    egui::TopBottomPanel::bottom("cli_output")
        .default_height(200.0)
        .resizable(true)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("CLI Output");

                if cli_runner.is_running() {
                    ui.spinner();
                    if let Some(cmd) = cli_runner.current_command() {
                        ui.label(format!("Running: {}", cmd.name()));
                    }
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("✖").on_hover_text("Close panel").clicked() {
                        panel.visible = false;
                    }

                    if ui.button("🗑").on_hover_text("Clear output").clicked() {
                        cli_runner.clear_output();
                    }

                    ui.checkbox(&mut panel.auto_scroll, "Auto-scroll");
                });
            });

            ui.separator();

            // Scrollable output area
            let text_style = egui::TextStyle::Monospace;
            let row_height = ui.text_style_height(&text_style);
            let total_rows = cli_runner.output_lines.len();

            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .stick_to_bottom(panel.auto_scroll)
                .show_rows(ui, row_height, total_rows, |ui, row_range| {
                    for row in row_range {
                        if let Some(line) = cli_runner.output_lines.get(row) {
                            let color = if line.is_error {
                                egui::Color32::from_rgb(255, 100, 100) // Red for errors
                            } else {
                                egui::Color32::from_rgb(200, 200, 200) // Light gray for normal output
                            };

                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing.x = 4.0;

                                // Timestamp (optional)
                                ui.label(
                                    egui::RichText::new(format!("[{:.1}s]", line.timestamp))
                                        .color(egui::Color32::from_rgb(128, 128, 128))
                                        .monospace()
                                );

                                // Output text
                                ui.label(
                                    egui::RichText::new(&line.text)
                                        .color(color)
                                        .monospace()
                                );
                            });
                        }
                    }
                });

            // Show helpful message if no output
            if cli_runner.output_lines.is_empty() {
                ui.centered_and_justified(|ui| {
                    ui.label(
                        egui::RichText::new("No output yet. Click a CLI button in the toolbar to run a command.")
                            .color(egui::Color32::from_rgb(128, 128, 128))
                    );
                });
            }
        });
}
