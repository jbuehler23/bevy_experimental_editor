use bevy::prelude::*;
use eryndor_common::BevyScene;
use crate::{CurrentLevel, project_manager::CurrentProject};

/// Event to trigger scene loading
#[derive(Event)]
pub struct LoadSceneEvent {
    pub scene_path: String,
}

/// Resource to track if we should auto-load a scene
#[derive(Resource, Default)]
pub struct SceneAutoLoader {
    pub should_auto_load: bool,
    pub has_loaded: bool,
}

/// System to auto-load the last opened scene when a project is opened
pub fn auto_load_scene_system(
    mut commands: Commands,
    project: Option<Res<CurrentProject>>,
    mut current_level: ResMut<CurrentLevel>,
    mut auto_loader: ResMut<SceneAutoLoader>,
) {
    // Only run once when a project is first loaded
    if auto_loader.has_loaded {
        return;
    }

    let Some(project) = project else {
        return;
    };

    // Check if there's a last opened scene or default scene
    let scene_to_load = project.metadata.config.last_opened_scene.as_ref()
        .or(project.metadata.config.default_scene.as_ref());

    if let Some(scene_name) = scene_to_load {
        let scene_path = project.metadata.levels_path.join(scene_name);

        if scene_path.exists() {
            match BevyScene::load_from_file(&scene_path) {
                Ok(scene) => {
                    info!("Auto-loaded scene: {}", scene_name);
                    current_level.level_data = scene.data;
                    current_level.file_path = Some(scene_path.to_string_lossy().to_string());
                    current_level.is_modified = false;
                    auto_loader.has_loaded = true;
                }
                Err(e) => {
                    error!("Failed to auto-load scene {}: {}", scene_name, e);
                }
            }
        } else {
            warn!("Scene file not found: {:?}", scene_path);
        }
    } else {
        info!("No scene to auto-load");
        auto_loader.has_loaded = true;
    }
}

/// System to reset auto-loader when project changes
pub fn reset_auto_loader_on_project_change(
    mut auto_loader: ResMut<SceneAutoLoader>,
    project: Option<Res<CurrentProject>>,
) {
    // Detect project change by checking if Changed fires
    if project.is_some() {
        auto_loader.has_loaded = false;
    }
}
