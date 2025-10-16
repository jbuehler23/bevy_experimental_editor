use bevy::prelude::*;
use bevy_editor_frontend_api::CliOutputPanelState;
use bevy_editor_project::BevyCLIRunner;
use bevy_egui::egui;

use crate::icons::{icon_label, Icons};

/// Check if CLI output panel should be visible
pub fn should_show_cli_output(cli_runner: &BevyCLIRunner) -> bool {
    cli_runner.is_running() || !cli_runner.output_lines.is_empty()
}

/// Render CLI output panel content (called from ui_system within a panel)
pub fn render_cli_output_content(
    ui: &mut egui::Ui,
    panel: &mut CliOutputPanelState,
    cli_runner: &mut BevyCLIRunner,
) {
    ui.horizontal(|ui| {
        ui.heading(icon_label(Icons::INFO, "CLI Output"));

        if let Some(command) = cli_runner.current_command() {
            ui.spinner();
            ui.label(
                egui::RichText::new(format!("Running {}", command.name()))
                    .italics()
                    .color(egui::Color32::from_rgb(180, 220, 255)),
            );
        } else if let Some(last_status) = cli_runner
            .output_lines
            .iter()
            .rev()
            .find(|line| line.text.starts_with("Process exited"))
        {
            let status_color = if last_status.is_error {
                egui::Color32::from_rgb(255, 120, 120)
            } else {
                egui::Color32::from_rgb(120, 200, 140)
            };
            ui.label(
                egui::RichText::new(&last_status.text)
                    .color(status_color)
                    .italics(),
            );
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui
                .button(icon_label(Icons::CLOSE, "Close"))
                .on_hover_text("Hide CLI output panel")
                .clicked()
            {
                panel.visible = false;
            }

            if ui
                .button(icon_label(Icons::REFRESH, "Clear"))
                .on_hover_text("Clear current output")
                .clicked()
            {
                cli_runner.clear_output();
            }

            let has_output = !cli_runner.output_lines.is_empty();
            if ui
                .add_enabled(
                    has_output,
                    egui::Button::new(icon_label(Icons::CLIPBOARD, "Copy")),
                )
                .on_hover_text("Copy output to the clipboard")
                .clicked()
            {
                let mut clipboard = String::new();
                for line in &cli_runner.output_lines {
                    clipboard.push_str(&line.text);
                    clipboard.push('\n');
                }
                ui.ctx().copy_text(clipboard);
            }

            ui.toggle_value(&mut panel.auto_scroll, "Auto-scroll");
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
                                .monospace(),
                        );

                        // Output text
                        ui.label(egui::RichText::new(&line.text).color(color).monospace());
                    });
                }
            }
        });

    // Show helpful message if no output
    if cli_runner.output_lines.is_empty() {
        ui.centered_and_justified(|ui| {
            ui.label(
                egui::RichText::new(
                    "No output yet. Click a CLI button in the toolbar to run a command.",
                )
                .color(egui::Color32::from_rgb(128, 128, 128)),
            );
        });
    }
}
