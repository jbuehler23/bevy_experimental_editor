use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::project_manager::{ProjectSelection, ProjectSelectionState, CurrentProject};
use crate::client_launcher::StandaloneClient;

/// UI for project selection dialog
pub fn project_selection_ui(
    mut contexts: EguiContexts,
    mut selection: ResMut<ProjectSelection>,
) {
    // Clone the error message if needed to avoid borrow issues
    let error_msg = if let ProjectSelectionState::Error(msg) = &selection.state {
        Some(msg.clone())
    } else {
        None
    };

    match selection.state {
        ProjectSelectionState::NeedSelection => {
            egui::Window::new("Welcome to Eryndor Editor")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(contexts.ctx_mut(), |ui| {
                    ui.heading("Select a Project");
                    ui.add_space(10.0);

                    ui.label("Create a new project or open an existing one");
                    ui.add_space(20.0);

                    if ui.button("Create New Project").clicked() {
                        // For now, create a default project in current directory
                        selection.state = ProjectSelectionState::Creating {
                            path: "test-project".to_string(),
                            name: "Test Project".to_string(),
                        };
                    }

                    ui.add_space(10.0);

                    if ui.button("Open Existing Project").clicked() {
                        // For now, try to open test-project
                        selection.state = ProjectSelectionState::Opening {
                            path: "test-project".to_string(),
                        };
                    }
                });
        }
        ProjectSelectionState::Error(_) => {
            if let Some(msg) = error_msg {
                egui::Window::new("Error")
                    .collapsible(false)
                    .resizable(false)
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                    .show(contexts.ctx_mut(), |ui| {
                        ui.colored_label(egui::Color32::RED, "Error:");
                        ui.label(&msg);
                        ui.add_space(10.0);

                        if ui.button("Try Again").clicked() {
                            selection.state = ProjectSelectionState::NeedSelection;
                        }
                    });
            }
        }
        _ => {}
    }
}

/// UI for play controls (Run/Stop client)
pub fn play_controls_ui(
    mut contexts: EguiContexts,
    project: Option<Res<CurrentProject>>,
    mut client: ResMut<StandaloneClient>,
) {
    let Some(project) = project else {
        return; // No project loaded yet
    };

    egui::Window::new("Play Controls")
        .default_pos([10.0, 100.0])
        .show(contexts.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                if client.is_running {
                    if ui.button("⏹ Stop").clicked() {
                        client.stop();
                    }
                    ui.label("Client is running");
                } else {
                    if ui.button("▶ Run Standalone").clicked() {
                        let project_path = project.root_path().clone();
                        match client.launch(project_path, None) {
                            Ok(_) => info!("Client launched successfully"),
                            Err(e) => error!("Failed to launch client: {}", e),
                        }
                    }
                }
            });

            ui.add_space(10.0);

            ui.label(format!("Project: {}", project.name()));
            ui.label(format!("Path: {}", project.root_path().display()));
        });
}
