//! Runtime plugin for loading Bevy editor scenes in games.
//!
//! This crate provides an optional plugin that allows games to load `.scn.ron` scene files
//! created by the Bevy editor. It's designed to be lightweight and non-intrusive.
//!
//! # Usage
//!
//! Add the plugin to your Bevy app:
//!
//! ```no_run
//! use bevy::prelude::*;
//! use bevy_editor_runtime::EditorSceneLoaderPlugin;
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         .add_plugins(EditorSceneLoaderPlugin)
//!         .run();
//! }
//! ```
//!
//! # Scene Loading
//!
//! The plugin checks for the `BEVY_EDITOR_SCENE` environment variable at startup.
//! If set, it loads the corresponding scene from `assets/world/<scene_name>.scn.ron`.
//!
//! Example: `BEVY_EDITOR_SCENE=level1 cargo run`
//!
//! This will load `assets/world/level1.scn.ron` when the game starts.

use bevy::prelude::*;

/// Plugin that loads editor scenes based on environment variables.
///
/// This plugin is completely optional - games can work without it.
/// When enabled, it checks for the `BEVY_EDITOR_SCENE` environment variable
/// and loads the specified scene from the assets directory.
pub struct EditorSceneLoaderPlugin;

impl Plugin for EditorSceneLoaderPlugin {
    fn build(&self, app: &mut App) {
        // Check for scene to load at startup
        if let Ok(scene_name) = std::env::var("BEVY_EDITOR_SCENE") {
            info!("Editor scene loader: Loading scene '{}'", scene_name);
            app.insert_resource(SceneToLoad {
                name: scene_name.clone(),
                path: format!("world/{}.scn.ron", scene_name),
                loaded: false,
            });
        } else {
            info!("Editor scene loader: No scene specified (set BEVY_EDITOR_SCENE to load a scene)");
        }

        app.add_systems(Startup, load_editor_scene);
    }
}

/// Resource tracking which scene should be loaded.
#[derive(Resource)]
struct SceneToLoad {
    name: String,
    path: String,
    loaded: bool,
}

/// System that loads the editor scene if one is specified.
fn load_editor_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    scene_to_load: Option<ResMut<SceneToLoad>>,
) {
    if let Some(mut scene_to_load) = scene_to_load {
        if !scene_to_load.loaded {
            info!("Loading editor scene from: {}", scene_to_load.path);

            // Load the scene file
            let scene_handle: Handle<DynamicScene> = asset_server.load(&scene_to_load.path);

            // Spawn the scene
            commands.spawn((
                Name::new(format!("EditorScene[{}]", scene_to_load.name)),
                DynamicSceneRoot(scene_handle),
            ));

            scene_to_load.loaded = true;
            info!("Editor scene '{}' queued for loading", scene_to_load.name);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plugin_builds_without_env_var() {
        std::env::remove_var("BEVY_EDITOR_SCENE");
        let mut app = App::new();
        app.add_plugins(EditorSceneLoaderPlugin);
        // Should not panic
    }
}
