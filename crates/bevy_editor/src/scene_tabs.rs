use bevy::prelude::*;
use bevy_egui::egui;
use crate::formats::{LevelData, BevyScene};
use std::path::PathBuf;
use crate::icons::Icons;

/// Represents a single open scene/level
#[derive(Clone)]
pub struct OpenScene {
    pub name: String,
    pub file_path: Option<String>,
    pub level_data: LevelData,
    pub is_modified: bool,
}

impl OpenScene {
    pub fn new(name: String, level_data: LevelData) -> Self {
        Self {
            name,
            file_path: None,
            level_data,
            is_modified: false,
        }
    }

    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let scene = BevyScene::load_from_file(path)?;
        let name = PathBuf::from(path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Untitled")
            .to_string();

        Ok(Self {
            name,
            file_path: Some(path.to_string()),
            level_data: scene.data,
            is_modified: false,
        })
    }
}

/// Resource managing multiple open scenes with tabs
#[derive(Resource)]
pub struct OpenScenes {
    pub scenes: Vec<OpenScene>,
    pub active_index: usize,
}

impl Default for OpenScenes {
    fn default() -> Self {
        // Start with one untitled scene
        Self {
            scenes: vec![OpenScene::new(
                "Untitled".to_string(),
                LevelData::new("Untitled Level".to_string(), 2000.0, 1000.0),
            )],
            active_index: 0,
        }
    }
}

impl OpenScenes {
    /// Get the currently active scene
    pub fn active_scene(&self) -> Option<&OpenScene> {
        self.scenes.get(self.active_index)
    }

    /// Get the currently active scene mutably
    pub fn active_scene_mut(&mut self) -> Option<&mut OpenScene> {
        self.scenes.get_mut(self.active_index)
    }

    /// Add a new scene and make it active
    pub fn add_scene(&mut self, scene: OpenScene) {
        self.scenes.push(scene);
        self.active_index = self.scenes.len() - 1;
    }

    /// Close a scene by index
    pub fn close_scene(&mut self, index: usize) {
        if self.scenes.len() <= 1 {
            // Don't close the last scene, just reset it
            if let Some(scene) = self.scenes.get_mut(0) {
                *scene = OpenScene::new(
                    "Untitled".to_string(),
                    LevelData::new("Untitled Level".to_string(), 2000.0, 1000.0),
                );
            }
            self.active_index = 0;
            return;
        }

        self.scenes.remove(index);

        // Adjust active index
        if self.active_index >= self.scenes.len() {
            self.active_index = self.scenes.len() - 1;
        } else if self.active_index > index {
            self.active_index -= 1;
        }
    }

    /// Switch to a specific scene
    pub fn set_active(&mut self, index: usize) {
        if index < self.scenes.len() {
            self.active_index = index;
        }
    }

    /// Check if any scenes are modified
    pub fn has_unsaved_changes(&self) -> bool {
        self.scenes.iter().any(|s| s.is_modified)
    }
}

/// Event triggered when scene tab changes
#[derive(Event)]
pub struct SceneTabChanged {
    pub new_index: usize,
}

/// Render scene tabs content (called from ui_system within a panel)
pub fn render_scene_tabs_content(
    ui: &mut egui::Ui,
    open_scenes: &mut OpenScenes,
    tab_changed_events: &mut EventWriter<SceneTabChanged>,
) {
    ui.horizontal(|ui| {
                let mut scene_to_close: Option<usize> = None;
                let mut new_active_index: Option<usize> = None;

                for (idx, scene) in open_scenes.scenes.iter().enumerate() {
                    let is_active = idx == open_scenes.active_index;

                    // Tab button styling
                    let button_text = if scene.is_modified {
                        format!("● {}", scene.name)
                    } else {
                        scene.name.clone()
                    };

                    let response = ui.selectable_label(is_active, button_text);

                    if response.clicked() {
                        new_active_index = Some(idx);
                    }

                    // Close button (X) for each tab
                    if ui.small_button(Icons::CLOSE)
                        .on_hover_text("Close scene")
                        .clicked()
                    {
                        scene_to_close = Some(idx);
                    }

                    ui.separator();
                }

                // New tab button
                if ui.button(Icons::NEW).on_hover_text("New scene").clicked() {
                    let new_scene = OpenScene::new(
                        format!("Untitled {}", open_scenes.scenes.len() + 1),
                        LevelData::new("New Level".to_string(), 2000.0, 1000.0),
                    );
                    open_scenes.add_scene(new_scene);
                    tab_changed_events.write(SceneTabChanged { new_index: open_scenes.active_index });
                }

                // Apply changes after iteration
                if let Some(index) = new_active_index {
                    let old_index = open_scenes.active_index;
                    open_scenes.set_active(index);
                    if old_index != index {
                        tab_changed_events.write(SceneTabChanged { new_index: index });
                    }
                }
                if let Some(index) = scene_to_close {
                    if open_scenes.scenes[index].is_modified {
                        // TODO: Show "Save changes?" dialog
                        info!("Scene has unsaved changes, close anyway? (dialog not yet implemented)");
                    }
                    open_scenes.close_scene(index);
                    tab_changed_events.write(SceneTabChanged { new_index: open_scenes.active_index });
                }
            });
}

/// Marker component to track which scene root is currently loading
#[derive(Component)]
pub struct LoadingSceneRoot;

/// System to sync EditorScene with OpenScenes when tabs change
pub fn sync_editor_scene_on_tab_change(
    mut tab_events: EventReader<SceneTabChanged>,
    mut commands: Commands,
    mut editor_scene: ResMut<crate::scene_editor::EditorScene>,
    scene_entities: Query<Entity, With<crate::scene_editor::EditorSceneEntity>>,
    mut name_buffer: ResMut<crate::panel_manager::NameEditBuffer>,
    open_scenes: Res<OpenScenes>,
    asset_server: Res<AssetServer>,
) {
    for event in tab_events.read() {
        info!("Scene tab changed to index {}, clearing editor scene entities", event.new_index);

        // Despawn all existing scene entities
        for entity in scene_entities.iter() {
            commands.entity(entity).despawn();
        }

        // Reset editor scene state
        editor_scene.selected_entity = None;
        editor_scene.is_modified = false;

        // Clear name edit buffer
        name_buffer.buffer.clear();

        // Get the new scene and load it if it has a file path
        if let Some(scene) = open_scenes.scenes.get(event.new_index) {
            if let Some(file_path) = &scene.file_path {
                // Load scene from file
                info!("Loading scene from file: {}", file_path);
                let scene_handle = asset_server.load::<DynamicScene>(file_path.clone());
                let root = commands.spawn((
                    DynamicSceneRoot(scene_handle),
                    crate::scene_editor::EditorSceneEntity,
                    LoadingSceneRoot,
                )).id();
                editor_scene.root_entity = Some(root);
                info!("Spawned scene root for loading: {:?}", root);
            } else {
                // New unsaved scene - create empty root
                let root = commands
                    .spawn((
                        Name::new("Scene Root"),
                        Transform::default(),
                        Visibility::default(),
                        crate::scene_editor::EditorSceneEntity,
                    ))
                    .id();

                editor_scene.root_entity = Some(root);
                info!("Created new empty scene root: {:?}", root);
            }
        }
    }
}

/// System to mark loaded scene entities with EditorSceneEntity component
/// This runs after DynamicSceneRoot spawns entities from the loaded scene
pub fn mark_loaded_scene_entities(
    mut commands: Commands,
    loading_roots: Query<Entity, With<LoadingSceneRoot>>,
    scene_instance_query: Query<&bevy::scene::SceneInstance>,
    scenes: Res<bevy::scene::SceneSpawner>,
    unmarked_entities: Query<(Entity, Option<&Name>, Option<&Children>), (Without<crate::scene_editor::EditorSceneEntity>, Without<Window>)>,
    mut editor_scene: ResMut<crate::scene_editor::EditorScene>,
) {
    // Check if any loading roots have spawned their scene instances
    for root_entity in loading_roots.iter() {
        // Check if this entity has a SceneInstance component that has finished loading
        if let Ok(scene_instance) = scene_instance_query.get(root_entity) {
            // Check if the scene instance is ready
            if scenes.instance_is_ready(**scene_instance) {
                info!("Scene instance ready for entity {:?}, marking spawned entities", root_entity);

                // Find the actual scene root entity (the one with "Scene Root" name and children)
                let mut actual_root = None;
                let mut spawned_entities = Vec::new();

                for spawned_entity in scenes.iter_instance_entities(**scene_instance) {
                    if let Ok((entity, name, children)) = unmarked_entities.get(spawned_entity) {
                        spawned_entities.push(entity);

                        // Look for entity named "Scene Root" with children
                        if let Some(entity_name) = name {
                            if entity_name.as_str() == "Scene Root" && children.is_some_and(|c| !c.is_empty()) {
                                actual_root = Some(entity);
                                info!("Found actual scene root: {:?}", entity);
                            }
                        }
                    }
                }

                // Mark all spawned entities
                for entity in &spawned_entities {
                    commands.entity(*entity).insert(crate::scene_editor::EditorSceneEntity);
                }
                info!("Marked {} loaded entities as EditorSceneEntity", spawned_entities.len());

                // Update EditorScene.root_entity to point to the actual scene root
                if let Some(new_root) = actual_root {
                    info!("Updating EditorScene.root_entity from {:?} to {:?}", editor_scene.root_entity, new_root);
                    editor_scene.root_entity = Some(new_root);

                    // Remove EditorSceneEntity from loading container so it doesn't show in scene tree
                    // Don't despawn it as it's still needed for scene management
                    commands.entity(root_entity)
                        .remove::<crate::scene_editor::EditorSceneEntity>()
                        .remove::<LoadingSceneRoot>();
                } else {
                    warn!("Could not find actual scene root entity in loaded scene");
                    // Remove the LoadingSceneRoot marker anyway
                    commands.entity(root_entity).remove::<LoadingSceneRoot>();
                }
            }
        }
    }
}
