use bevy::prelude::*;
use crate::formats::BevyScene;
use crate::project_manager::CurrentProject;

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
    commands: Commands,
    project: Option<Res<CurrentProject>>,
    mut open_scenes: ResMut<crate::scene_tabs::OpenScenes>,  // Changed from CurrentLevel
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
                Ok(bevy_scene) => {
                    info!("Auto-loaded scene: {}", scene_name);

                    let new_scene = crate::scene_tabs::OpenScene {
                        name: scene_name.clone(),
                        file_path: Some(scene_path.to_string_lossy().to_string()),
                        level_data: bevy_scene.data,
                        is_modified: false,
                    };

                    // Replace the default untitled scene with the loaded scene
                    if open_scenes.scenes.len() == 1 &&
                       open_scenes.scenes[0].name.starts_with("Untitled") &&
                       !open_scenes.scenes[0].is_modified {
                        open_scenes.scenes[0] = new_scene;
                    } else {
                        open_scenes.add_scene(new_scene);
                    }

                    auto_loader.has_loaded = true;
                }
                Err(e) => {
                    error!("Failed to auto-load scene {}: {}", scene_name, e);
                }
            }
        } else {
            debug!("Scene file not found: {:?}", scene_path);
            // Mark as loaded so we don't spam the console every frame
            auto_loader.has_loaded = true;
        }
    } else {
        debug!("No scene to auto-load");
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
