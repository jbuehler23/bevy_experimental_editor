use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Editor workspace state - persisted between sessions
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct EditorWorkspace {
    /// List of recently opened projects (most recent first)
    pub recent_projects: Vec<String>,
    /// Path to the last opened project
    pub last_project: Option<String>,
    /// Maximum number of recent projects to track
    #[serde(skip)]
    pub max_recent: usize,
}

impl Default for EditorWorkspace {
    fn default() -> Self {
        Self {
            recent_projects: Vec::new(),
            last_project: None,
            max_recent: 10,
        }
    }
}

impl EditorWorkspace {
    /// Get the workspace file path (~/.bevy_experimental_editor/workspace.json)
    pub fn workspace_path() -> PathBuf {
        let home = dirs::home_dir().expect("Could not find home directory");
        home.join(".bevy_experimental_editor").join("workspace.json")
    }

    /// Load workspace from disk
    pub fn load() -> Self {
        let path = Self::workspace_path();

        if !path.exists() {
            info!("No workspace file found, creating new workspace");
            return Self::default();
        }

        match std::fs::read_to_string(&path) {
            Ok(contents) => match serde_json::from_str::<Self>(&contents) {
                Ok(mut workspace) => {
                    workspace.max_recent = 10; // Ensure max_recent is set
                    info!("Loaded workspace with {} recent projects", workspace.recent_projects.len());
                    workspace
                }
                Err(e) => {
                    error!("Failed to parse workspace file: {}", e);
                    Self::default()
                }
            },
            Err(e) => {
                error!("Failed to read workspace file: {}", e);
                Self::default()
            }
        }
    }

    /// Save workspace to disk
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::workspace_path();

        // Ensure directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(&path, json)?;

        info!("Saved workspace to {:?}", path);
        Ok(())
    }

    /// Add a project to recent projects (most recent first)
    pub fn add_recent_project(&mut self, project_path: String) {
        // Remove if already in list
        self.recent_projects.retain(|p| p != &project_path);

        // Add to front
        self.recent_projects.insert(0, project_path.clone());

        // Trim to max
        if self.recent_projects.len() > self.max_recent {
            self.recent_projects.truncate(self.max_recent);
        }

        // Update last project
        self.last_project = Some(project_path);

        // Auto-save
        if let Err(e) = self.save() {
            error!("Failed to save workspace: {}", e);
        }
    }

    /// Remove a project from recent projects
    pub fn remove_recent_project(&mut self, project_path: &str) {
        self.recent_projects.retain(|p| p != project_path);

        if self.last_project.as_deref() == Some(project_path) {
            self.last_project = self.recent_projects.first().cloned();
        }

        // Auto-save
        if let Err(e) = self.save() {
            error!("Failed to save workspace: {}", e);
        }
    }

    /// Clear all recent projects
    pub fn clear_recent(&mut self) {
        self.recent_projects.clear();
        self.last_project = None;

        // Auto-save
        if let Err(e) = self.save() {
            error!("Failed to save workspace: {}", e);
        }
    }
}

/// System to load workspace on startup
pub fn load_workspace_system(mut commands: Commands) {
    let workspace = EditorWorkspace::load();
    commands.insert_resource(workspace);
}

/// System to save workspace on exit
pub fn save_workspace_on_exit(workspace: Res<EditorWorkspace>) {
    if let Err(e) = workspace.save() {
        error!("Failed to save workspace on exit: {}", e);
    }
}
