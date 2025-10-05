use bevy::prelude::*;
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
