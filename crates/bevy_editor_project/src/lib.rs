//! # Bevy Editor Project
//!
//! Project and workspace management for Bevy-based editors.
//!
//! This crate provides:
//! - **Project Management**: Create, open, and configure Bevy game projects
//! - **Workspace Tracking**: Recent projects and session persistence
//! - **Project Templates**: Generate new projects from templates (Empty, Tilemap2D, etc.)
//! - **CLI Integration**: Bevy CLI runner for building and running projects
//!
//! ## Features
//!
//! - `workspace` (default): Workspace and recent project tracking
//! - `cli`: Bevy CLI integration for build/run commands
//! - `ui`: egui-powered UI for project selection and the project creation wizard
//!
//! ## Example
//!
//! ```rust,no_run
//! use bevy::prelude::*;
//! use bevy_editor_project::ProjectManagerPlugin;
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         .add_plugins(ProjectManagerPlugin)
//!         .run();
//! }
//! ```

pub mod project_generator;
pub mod project_manager;
pub mod scene_loader_template;

#[cfg(feature = "workspace")]
pub mod workspace;

#[cfg(feature = "cli")]
pub mod bevy_cli_runner;

#[cfg(feature = "ui")]
// UI components (note: these have egui dependencies and should eventually move to UI crate)
pub mod project_wizard;

#[cfg(feature = "ui")]
pub mod file_dialog_helper;

// Re-export commonly used types
pub use project_manager::{
    handle_project_selection, BuildProgress, CurrentProject, ProjectManagerSet, ProjectSelection,
    ProjectSelectionState,
};

pub use project_generator::{generate_project, get_package_name_from_cargo_toml, ProjectTemplate};
#[cfg(feature = "ui")]
pub use project_manager::project_selection_ui;

#[cfg(feature = "workspace")]
pub use workspace::{load_workspace_system, save_workspace_on_exit, EditorWorkspace};

#[cfg(feature = "cli")]
pub use bevy_cli_runner::{update_cli_runner, BevyCLIRunner, CLICommand, CLIOutput, CLIOutputLine};

#[cfg(feature = "ui")]
pub use project_wizard::{project_wizard_ui, ProjectWizard};

use bevy::prelude::*;

/// Main plugin for project management functionality
pub struct ProjectManagerPlugin;

impl Plugin for ProjectManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ProjectSelection>();

        #[cfg(feature = "ui")]
        {
            app.init_resource::<ProjectWizard>();
        }

        #[cfg(feature = "cli")]
        {
            app.init_resource::<BevyCLIRunner>();
        }

        #[cfg(feature = "workspace")]
        {
            app.add_systems(Startup, load_workspace_system);
        }

        app.configure_sets(Update, ProjectManagerSet);

        app.add_systems(Update, handle_project_selection.in_set(ProjectManagerSet));

        #[cfg(feature = "cli")]
        {
            app.add_systems(Update, update_cli_runner.in_set(ProjectManagerSet));
        }

        #[cfg(feature = "ui")]
        {
            app.add_systems(
                Update,
                (project_selection_ui, project_wizard_ui).in_set(ProjectManagerSet),
            );
        }
    }
}

/// Plugin without UI components (for headless/backend use)
pub struct ProjectCorePlugin;

impl Plugin for ProjectCorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ProjectSelection>();

        #[cfg(feature = "workspace")]
        {
            app.add_systems(Startup, load_workspace_system);
        }

        #[cfg(feature = "cli")]
        {
            app.init_resource::<BevyCLIRunner>();
        }

        app.configure_sets(Update, ProjectManagerSet);

        // Only core systems, no UI
        app.add_systems(Update, handle_project_selection.in_set(ProjectManagerSet));

        #[cfg(feature = "cli")]
        {
            app.add_systems(Update, update_cli_runner.in_set(ProjectManagerSet));
        }
    }
}
