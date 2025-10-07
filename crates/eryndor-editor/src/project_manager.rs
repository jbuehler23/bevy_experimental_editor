use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use eryndor_common::{ProjectMetadata, ProjectConfig};
use std::path::PathBuf;

/// Resource holding the current project information
#[derive(Resource)]
pub struct CurrentProject {
    pub metadata: ProjectMetadata,
}

impl CurrentProject {
    /// Create a new project at the given path
    pub fn create_new<P: Into<PathBuf>>(path: P, name: String) -> Result<Self, Box<dyn std::error::Error>> {
        let path_buf = path.into();

        // Create the project directory if it doesn't exist
        std::fs::create_dir_all(&path_buf)?;

        // Create project config
        let mut config = ProjectConfig::new(name.clone());
        config.client_config.window_title = name.clone();
        config.default_scene = Some("main.bscene".to_string());

        // Save config as .bvy file
        let config_path = path_buf.join("project.bvy");
        config.save_to_file(&config_path)?;

        // Load metadata (this will create the directory structure)
        let metadata = ProjectMetadata::from_project_path(&path_buf)?;

        // Auto-create default scene (main.bscene)
        let default_scene_path = metadata.levels_path.join("main.bscene");
        if !default_scene_path.exists() {
            let default_level_data = eryndor_common::LevelData::new(
                format!("{} - Main Scene", name),
                2000.0, // default width
                1000.0, // default height
            );
            let scene = eryndor_common::BevyScene::new(default_level_data);
            scene.save_to_file(&default_scene_path)?;
            info!("Created default scene: {:?}", default_scene_path);
        }

        info!("Created new project at: {:?}", path_buf);

        Ok(Self { metadata })
    }

    /// Open an existing project
    pub fn open_existing<P: Into<PathBuf>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let path_buf = path.into();

        // Check if project.bvy exists
        let config_path = path_buf.join("project.bvy");
        if !config_path.exists() {
            return Err(format!("No project.bvy found at {:?}", path_buf).into());
        }

        // Load metadata
        let metadata = ProjectMetadata::from_project_path(&path_buf)?;

        info!("Opened project: {} at {:?}", metadata.config.name, path_buf);

        Ok(Self { metadata })
    }

    /// Get the assets directory path
    pub fn assets_path(&self) -> &PathBuf {
        &self.metadata.assets_path
    }

    /// Get the levels directory path
    pub fn levels_path(&self) -> &PathBuf {
        &self.metadata.levels_path
    }

    /// Get the project root path
    pub fn root_path(&self) -> &PathBuf {
        &self.metadata.root_path
    }

    /// Get the project name
    pub fn name(&self) -> &str {
        &self.metadata.config.name
    }

    /// Update and save the project configuration
    pub fn update_config<F>(&mut self, update_fn: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnOnce(&mut ProjectConfig),
    {
        update_fn(&mut self.metadata.config);
        self.metadata.save_config()
    }
}

/// Build progress information
#[derive(Clone)]
pub struct BuildProgress {
    pub start_time: std::time::Instant,
    pub output_lines: Vec<String>,
    pub current_stage: String,
}

impl BuildProgress {
    pub fn new() -> Self {
        Self {
            start_time: std::time::Instant::now(),
            output_lines: Vec::new(),
            current_stage: "Starting build...".to_string(),
        }
    }

    pub fn elapsed_secs(&self) -> f32 {
        self.start_time.elapsed().as_secs_f32()
    }
}

/// Project selection state for UI
#[derive(Default)]
pub enum ProjectSelectionState {
    #[default]
    NeedSelection,
    Creating {
        path: String,
        name: String,
    },
    Opening {
        path: String,
    },
    GeneratingTemplate,  // Running bevy new or custom template generation
    InitialBuild(BuildProgress),  // First build to warm cache
    Ready,
    Error(String),
}

/// Resource to track project selection UI state
#[derive(Resource, Default)]
pub struct ProjectSelection {
    pub state: ProjectSelectionState,
}

/// System to handle project selection before editor is fully initialized
pub fn handle_project_selection(
    mut commands: Commands,
    mut selection: ResMut<ProjectSelection>,
    mut workspace: Option<ResMut<crate::workspace::EditorWorkspace>>,
    mut cli_runner: Option<ResMut<crate::bevy_cli_runner::BevyCLIRunner>>,
    mut project_browser: Option<ResMut<crate::project_browser::ProjectBrowser>>,
) {
    match &selection.state {
        ProjectSelectionState::Creating { path, name } => {
            match CurrentProject::create_new(path.clone(), name.clone()) {
                Ok(project) => {
                    info!("Project created successfully");

                    // Add to workspace recent projects
                    if let Some(ref mut workspace) = workspace {
                        workspace.add_recent_project(path.clone());
                    }

                    // Update CLI runner with new project path
                    if let Some(ref mut cli_runner) = cli_runner {
                        cli_runner.set_project_path(std::path::PathBuf::from(path));
                    }

                    // Initialize project browser with project root
                    if let Some(ref mut browser) = project_browser {
                        browser.set_project_root(project.metadata.root_path.clone());
                        info!("Project browser initialized with root: {:?}", project.metadata.root_path);
                    }

                    commands.insert_resource(project);
                    selection.state = ProjectSelectionState::Ready;
                }
                Err(e) => {
                    error!("Failed to create project: {}", e);
                    selection.state = ProjectSelectionState::Error(format!("Failed to create project: {}", e));
                }
            }
        }
        ProjectSelectionState::Opening { path } => {
            match CurrentProject::open_existing(path.clone()) {
                Ok(project) => {
                    info!("Project opened successfully");

                    // Add to workspace recent projects
                    if let Some(ref mut workspace) = workspace {
                        workspace.add_recent_project(path.clone());
                    }

                    // Update CLI runner with new project path
                    if let Some(ref mut cli_runner) = cli_runner {
                        cli_runner.set_project_path(std::path::PathBuf::from(path));
                    }

                    // Initialize project browser with project root
                    if let Some(ref mut browser) = project_browser {
                        browser.set_project_root(project.metadata.root_path.clone());
                        info!("Project browser initialized with root: {:?}", project.metadata.root_path);
                    }

                    commands.insert_resource(project);
                    selection.state = ProjectSelectionState::Ready;
                }
                Err(e) => {
                    error!("Failed to open project: {}", e);
                    selection.state = ProjectSelectionState::Error(format!("Failed to open project: {}", e));
                }
            }
        }
        _ => {}
    }
}

/// UI for project selection dialog
pub fn project_selection_ui(
    mut contexts: EguiContexts,
    mut selection: ResMut<ProjectSelection>,
    mut wizard: ResMut<crate::project_wizard::ProjectWizard>,
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
