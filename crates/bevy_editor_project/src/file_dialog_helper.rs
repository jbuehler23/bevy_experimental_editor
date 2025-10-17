//! File dialog helpers with fallback for systems where native dialogs don't work (e.g., WSL)

use bevy_egui::egui;
use std::path::PathBuf;

/// State for a file dialog with manual fallback
#[derive(Default)]
pub struct FileDialogState {
    /// Manual path entry (used when dialog fails or on systems without dialog support)
    pub manual_path: String,
    /// Whether to show the manual entry UI
    pub show_manual_entry: bool,
    /// Last error message
    pub error_message: Option<String>,
}

impl FileDialogState {
    pub fn new() -> Self {
        Self {
            manual_path: String::new(),
            show_manual_entry: false,
            error_message: None,
        }
    }

    /// Try to open a folder picker dialog, with fallback to manual entry
    pub fn try_pick_folder(&mut self, title: &str) -> Option<PathBuf> {
        match rfd::FileDialog::new()
            .set_title(title)
            .pick_folder()
        {
            Some(folder) => {
                self.error_message = None;
                Some(folder)
            }
            None => {
                // Dialog was cancelled or failed
                // On WSL/systems without proper dialog support, show manual entry
                self.show_manual_entry = true;
                self.error_message = Some(
                    "File dialog not available. Please enter path manually.".to_string()
                );
                None
            }
        }
    }

    /// Render manual path entry UI
    pub fn render_manual_entry_ui(&mut self, ui: &mut egui::Ui, label: &str) -> Option<PathBuf> {
        let mut result = None;

        ui.vertical(|ui| {
            if let Some(ref error) = self.error_message {
                ui.colored_label(egui::Color32::YELLOW, format!("⚠ {}", error));
            }

            ui.horizontal(|ui| {
                ui.label(label);
                let response = ui.text_edit_singleline(&mut self.manual_path);

                if ui.button("📂 Select").clicked() {
                    result = self.try_pick_folder("Select Folder");
                }

                // Auto-focus the text input when shown
                if self.show_manual_entry && self.manual_path.is_empty() {
                    response.request_focus();
                }
            });

            if !self.manual_path.is_empty() {
                let path = PathBuf::from(&self.manual_path);
                if path.exists() {
                    if path.is_dir() {
                        ui.colored_label(egui::Color32::GREEN, "✓ Valid directory");
                        if ui.button("Use This Path").clicked() {
                            result = Some(path.clone());
                        }
                    } else {
                        ui.colored_label(egui::Color32::RED, "✗ Path exists but is not a directory");
                    }
                } else {
                    ui.colored_label(egui::Color32::GRAY, "Path doesn't exist yet (will be created)");
                    if ui.button("Create & Use").clicked() {
                        result = Some(path.clone());
                    }
                }
            }

            // Helpful hints for WSL users
            if cfg!(target_os = "linux") {
                ui.collapsing("💡 Tip: File Dialog Issues?", |ui| {
                    ui.label("If the file dialog doesn't work:");
                    ui.label("• You may be running in WSL without X11/display support");
                    ui.label("• Enter the full path manually (e.g., /home/user/projects)");
                    ui.label("• Or install xdg-desktop-portal: sudo apt install xdg-desktop-portal");
                });
            }
        });

        result
    }
}

/// Convenience function to show a simple folder picker with manual fallback
pub fn pick_folder_with_fallback(ui: &mut egui::Ui, state: &mut FileDialogState, title: &str, label: &str) -> Option<PathBuf> {
    if state.show_manual_entry {
        state.render_manual_entry_ui(ui, label)
    } else {
        if ui.button("Browse...").clicked() {
            return state.try_pick_folder(title);
        }
        None
    }
}
