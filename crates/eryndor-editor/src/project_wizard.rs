use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::project_manager::{ProjectSelection, ProjectSelectionState};
use crate::project_generator::{ProjectTemplate, generate_project};
use std::path::PathBuf;

/// Resource to track the project wizard state
#[derive(Resource, Default)]
pub struct ProjectWizard {
    pub project_name: String,
    pub project_path: Option<PathBuf>,
    pub selected_template: ProjectTemplate,
    pub show_wizard: bool,
}

impl Default for ProjectTemplate {
    fn default() -> Self {
        ProjectTemplate::Tilemap2D
    }
}

/// UI for the project creation wizard
pub fn project_wizard_ui(
    mut contexts: EguiContexts,
    mut wizard: ResMut<ProjectWizard>,
    mut selection: ResMut<ProjectSelection>,
) {
    if !wizard.show_wizard {
        return;
    }

    egui::Window::new("Create New Project")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(contexts.ctx_mut(), |ui| {
            ui.heading("New Bevy Project");
            ui.add_space(10.0);

            // Project Name
            ui.label("Project Name:");
            ui.text_edit_singleline(&mut wizard.project_name);
            ui.add_space(10.0);

            // Project Location
            ui.label("Project Location:");
            ui.horizontal(|ui| {
                let location_text = wizard.project_path.as_ref()
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|| "Not selected".to_string());

                ui.label(location_text);

                if ui.button("Browse...").clicked() {
                    // Open file dialog to select directory
                    if let Some(folder) = rfd::FileDialog::new()
                        .set_title("Select Project Location")
                        .pick_folder()
                    {
                        wizard.project_path = Some(folder);
                    }
                }
            });
            ui.add_space(10.0);

            // Template Selection
            ui.label("Project Template:");
            ui.radio_value(&mut wizard.selected_template, ProjectTemplate::Empty, ProjectTemplate::Empty.name());
            ui.label(format!("  └─ {}", ProjectTemplate::Empty.description()));
            ui.add_space(5.0);

            ui.radio_value(&mut wizard.selected_template, ProjectTemplate::Tilemap2D, ProjectTemplate::Tilemap2D.name());
            ui.label(format!("  └─ {}", ProjectTemplate::Tilemap2D.description()));
            ui.add_space(15.0);

            // Action Buttons
            ui.horizontal(|ui| {
                let can_create = !wizard.project_name.is_empty() && wizard.project_path.is_some();

                if ui.add_enabled(can_create, egui::Button::new("Create Project")).clicked() {
                    // Create the project
                    let project_path = wizard.project_path.clone().unwrap().join(&wizard.project_name);

                    match generate_project(
                        &project_path,
                        &wizard.project_name,
                        wizard.selected_template.clone(),
                    ) {
                        Ok(_) => {
                            info!("Project created successfully at: {:?}", project_path);

                            // Trigger project opening
                            selection.state = ProjectSelectionState::Opening {
                                path: project_path.to_string_lossy().to_string(),
                            };

                            // Reset wizard
                            wizard.show_wizard = false;
                            wizard.project_name.clear();
                            wizard.project_path = None;
                        }
                        Err(e) => {
                            error!("Failed to create project: {}", e);
                            selection.state = ProjectSelectionState::Error(format!("Failed to create project: {}", e));
                            wizard.show_wizard = false;
                        }
                    }
                }

                if ui.button("Cancel").clicked() {
                    wizard.show_wizard = false;
                    wizard.project_name.clear();
                    wizard.project_path = None;
                }
            });
        });
}