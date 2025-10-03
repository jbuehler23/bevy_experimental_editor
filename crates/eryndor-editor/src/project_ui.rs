use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::project_manager::{ProjectSelection, ProjectSelectionState, CurrentProject};
use crate::client_launcher::StandaloneClient;
use crate::project_wizard::ProjectWizard;
use crate::build_manager::{BuildManager, BuildStatus};
use crate::project_generator::get_package_name_from_cargo_toml;
use crate::CurrentLevel;

/// UI for project selection dialog
pub fn project_selection_ui(
    mut contexts: EguiContexts,
    mut selection: ResMut<ProjectSelection>,
    mut wizard: ResMut<ProjectWizard>,
) {
    // Clone the error message if needed to avoid borrow issues
    let error_msg = if let ProjectSelectionState::Error(msg) = &selection.state {
        Some(msg.clone())
    } else {
        None
    };

    match selection.state {
        ProjectSelectionState::NeedSelection => {
            egui::Window::new("Welcome to Bevy Editor")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(contexts.ctx_mut(), |ui| {
                    ui.heading("Select a Project");
                    ui.add_space(10.0);

                    ui.label("Create a new Bevy project or open an existing one");
                    ui.add_space(20.0);

                    if ui.button("📁 Create New Project").clicked() {
                        // Show project wizard
                        wizard.show_wizard = true;
                    }

                    ui.add_space(10.0);

                    if ui.button("📂 Open Existing Project").clicked() {
                        // Open file dialog to select project directory
                        if let Some(folder) = rfd::FileDialog::new()
                            .set_title("Open Project")
                            .pick_folder()
                        {
                            selection.state = ProjectSelectionState::Opening {
                                path: folder.to_string_lossy().to_string(),
                            };
                        }
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
    current_level: Option<Res<CurrentLevel>>,
    mut client: ResMut<StandaloneClient>,
    mut build_manager: ResMut<BuildManager>,
) {
    let Some(project) = project else {
        return; // No project loaded yet
    };

    egui::Window::new("Play Controls")
        .default_pos([10.0, 100.0])
        .show(contexts.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                // Check if build is running
                if build_manager.is_building() {
                    if ui.button("🛑 Cancel Build").clicked() {
                        build_manager.cancel_build();
                    }
                    ui.spinner();
                    ui.label(build_manager.current_stage());
                } else if client.is_running {
                    if ui.button("⏹ Stop").clicked() {
                        client.stop();
                    }
                    ui.label("Game is running");
                } else {
                    if ui.button("▶ Run").clicked() {
                        let project_path = project.root_path().clone();

                        // Get package name
                        match get_package_name_from_cargo_toml(&project_path) {
                            Ok(package_name) => {
                                // Check if exe exists
                                let exe_name = if cfg!(windows) {
                                    format!("{}.exe", package_name)
                                } else {
                                    package_name.clone()
                                };
                                let exe_path = project_path.join("target").join("debug").join(&exe_name);

                                if !exe_path.exists() {
                                    // Start async build
                                    build_manager.start_build(project_path, package_name);
                                } else {
                                    // Launch directly with current scene path if available
                                    let level_path = current_level.as_ref()
                                        .and_then(|level| level.file_path.clone());
                                    match client.launch(project_path, level_path) {
                                        Ok(_) => info!("Game launched successfully"),
                                        Err(e) => error!("Failed to launch game: {}", e),
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Failed to get package name: {}", e);
                            }
                        }
                    }
                }
            });

            // Show build status
            match &build_manager.current_status {
                BuildStatus::Building { stage, elapsed_secs } => {
                    ui.add_space(5.0);
                    ui.label(format!("⏱ {:.1}s elapsed", elapsed_secs));
                    ui.label(format!("Status: {}", stage));
                }
                BuildStatus::Finished { duration_secs } => {
                    ui.add_space(5.0);
                    ui.label(format!("✅ Build completed in {:.1}s", duration_secs));

                    // Auto-launch after successful build
                    if !client.is_running {
                        let project_path = project.root_path().clone();
                        let level_path = current_level.as_ref()
                            .and_then(|level| level.file_path.clone());
                        match client.launch(project_path, level_path) {
                            Ok(_) => info!("Game launched after build"),
                            Err(e) => error!("Failed to launch game: {}", e),
                        }
                        // Reset to idle
                        build_manager.current_status = BuildStatus::Idle;
                    }
                }
                BuildStatus::Failed { error } => {
                    ui.add_space(5.0);
                    ui.colored_label(egui::Color32::RED, format!("❌ Build failed: {}", error));
                    if ui.button("Retry").clicked() {
                        let project_path = project.root_path().clone();
                        if let Ok(package_name) = get_package_name_from_cargo_toml(&project_path) {
                            build_manager.start_build(project_path, package_name);
                        }
                    }
                }
                BuildStatus::Idle => {}
            }

            ui.add_space(10.0);

            ui.label(format!("Project: {}", project.name()));
            ui.label(format!("Path: {}", project.root_path().display()));
        });
}
