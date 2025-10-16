//! Project file browser bridge systems.

use bevy::prelude::*;
use bevy_editor_assets::AssetBrowser;
use bevy_editor_frontend_api::project_browser::{
    ProjectBrowser as SharedProjectBrowser, ProjectBrowserPanelState,
};
use bevy_editor_project::CurrentProject;

pub use bevy_editor_frontend_api::project_browser::{FileEntry, FileType, ProjectBrowser};

/// System to refresh the project browser when needed.
pub fn refresh_project_browser_system(mut browser: ResMut<SharedProjectBrowser>) {
    if browser.needs_refresh {
        browser.refresh();
    }
}

/// Sync the project browser's root with the currently open project.
pub fn sync_project_browser_root(
    project: Option<Res<CurrentProject>>,
    mut browser: ResMut<SharedProjectBrowser>,
) {
    match project {
        Some(project) => {
            let project_root = project.metadata.root_path.clone();
            let should_update_root =
                browser.project_root.as_ref() != Some(&project_root) || project.is_changed();

            if should_update_root {
                browser.set_project_root(project_root);
            }
        }
        None => {
            if browser.project_root.is_some() {
                browser.project_root = None;
                browser.root_entries.clear();
                browser.selected_path = None;
                browser.needs_refresh = false;
            }
        }
    }
}

/// Keep the asset browser's root directory aligned with the active project.
pub fn sync_asset_browser_root(
    project: Option<Res<CurrentProject>>,
    mut asset_browser: ResMut<AssetBrowser>,
) {
    match project {
        Some(project) => {
            let assets_root = project.metadata.assets_path.clone();
            let needs_update = asset_browser
                .assets_directory
                .as_ref()
                .map(|existing| existing != &assets_root)
                .unwrap_or(true)
                || project.is_changed();

            if needs_update {
                asset_browser.set_assets_directory(assets_root);
                asset_browser.needs_rescan = true;
            }
        }
        None => {
            if asset_browser.assets_directory.is_some() {
                asset_browser.assets_directory = None;
                asset_browser.clear();
                asset_browser.needs_rescan = false;
            }
        }
    }
}

/// Helper to reset the panel dialog state when the outstanding project changes.
pub fn clear_panel_state_on_project_switch(
    project: Option<Res<CurrentProject>>,
    mut panel: ResMut<ProjectBrowserPanelState>,
) {
    if project.is_none() && panel.show_new_folder_dialog {
        panel.show_new_folder_dialog = false;
        panel.new_folder_name.clear();
        panel.new_folder_parent = None;
    }
}
