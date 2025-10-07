//! Scene editor module for managing game entities (separate from tilemap)
//! Uses Bevy's DynamicScene for serialization/deserialization

use bevy::prelude::*;
use bevy::scene::{DynamicScene, DynamicSceneBuilder};

/// Marker component for entities that are part of the edited scene
/// (not editor UI elements)
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct EditorSceneEntity;

/// Resource managing the currently edited scene
#[derive(Resource)]
pub struct EditorScene {
    /// Root entity that all scene entities are parented to
    pub root_entity: Option<Entity>,
    /// Currently selected entity in the scene tree
    pub selected_entity: Option<Entity>,
    /// Whether the scene has unsaved changes
    pub is_modified: bool,
}

impl Default for EditorScene {
    fn default() -> Self {
        Self {
            root_entity: None,
            selected_entity: None,
            is_modified: false,
        }
    }
}

impl EditorScene {
    /// Create a new empty scene with a root entity
    pub fn new(commands: &mut Commands) -> Self {
        let root_entity = commands
            .spawn((
                Name::new("Scene Root"),
                Transform::default(),
                Visibility::default(),
                EditorSceneEntity,
            ))
            .id();

        Self {
            root_entity: Some(root_entity),
            selected_entity: None,
            is_modified: false,
        }
    }

    /// Select an entity in the scene tree
    pub fn select_entity(&mut self, entity: Entity) {
        self.selected_entity = Some(entity);
    }

    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.selected_entity = None;
    }

    /// Check if an entity is selected
    pub fn is_selected(&self, entity: Entity) -> bool {
        self.selected_entity == Some(entity)
    }

    /// Mark scene as modified
    pub fn mark_modified(&mut self) {
        self.is_modified = true;
    }

    /// Mark scene as saved
    pub fn mark_saved(&mut self) {
        self.is_modified = false;
    }
}

/// Export the current scene to DynamicScene (for saving)
pub fn export_scene_to_dynamic(
    world: &World,
    editor_scene: &EditorScene,
) -> Option<DynamicScene> {
    let root_entity = editor_scene.root_entity?;

    // Create a scene builder
    let builder = DynamicSceneBuilder::from_world(world);

    // Add the root entity and all its descendants
    let builder = builder.extract_entity(root_entity);

    // Build and return the scene
    Some(builder.build())
}

/// Serialize scene to RON string
pub fn serialize_scene(
    scene: &DynamicScene,
    type_registry: &AppTypeRegistry,
) -> Result<String, Box<dyn std::error::Error>> {
    let type_registry = type_registry.read();
    let serialized = scene.serialize(&type_registry)?;
    Ok(serialized)
}

/// Deserialize scene from RON string (placeholder - needs proper implementation)
pub fn deserialize_scene(
    _ron_string: &str,
    _type_registry: &AppTypeRegistry,
) -> Result<DynamicScene, Box<dyn std::error::Error>> {
    // TODO: Implement proper scene deserialization using Bevy's scene loader
    // For now, return an empty scene
    Ok(DynamicScene::default())
}

/// Load a scene from a .scn.ron file
pub fn load_scene_from_file(
    path: &str,
    type_registry: &AppTypeRegistry,
) -> Result<DynamicScene, Box<dyn std::error::Error>> {
    let ron_string = std::fs::read_to_string(path)?;
    deserialize_scene(&ron_string, type_registry)
}

/// Save a scene to a .scn.ron file
pub fn save_scene_to_file(
    path: &str,
    scene: &DynamicScene,
    type_registry: &AppTypeRegistry,
) -> Result<(), Box<dyn std::error::Error>> {
    let serialized = serialize_scene(scene, type_registry)?;
    std::fs::write(path, serialized)?;
    Ok(())
}

/// Spawn a loaded scene into the world
pub fn spawn_scene(
    commands: &mut Commands,
    scene: &DynamicScene,
    editor_scene: &mut EditorScene,
) {
    // Spawn the scene
    let root = commands.spawn_empty().id();

    // TODO: Actually spawn the scene entities using scene.write_to_world()
    // This requires more complex integration with Bevy's scene system

    editor_scene.root_entity = Some(root);
    editor_scene.selected_entity = None;
    editor_scene.is_modified = false;
}

/// System to initialize editor scene on startup
pub fn setup_editor_scene(mut commands: Commands) {
    let editor_scene = EditorScene::new(&mut commands);
    commands.insert_resource(editor_scene);
}

/// Event for editing entity transforms
#[derive(Event, Debug, Clone)]
pub enum TransformEditEvent {
    /// Set position (replaces current position)
    SetPosition { entity: Entity, position: Vec2 },
    /// Translate by delta (adds to current position)
    Translate { entity: Entity, delta: Vec2 },
    /// Set rotation (replaces current rotation)
    SetRotation { entity: Entity, rotation: f32 },
    /// Set scale (replaces current scale)
    SetScale { entity: Entity, scale: Vec2 },
}

/// System to handle transform edit events
pub fn handle_transform_edit_events(
    mut events: EventReader<TransformEditEvent>,
    mut entity_query: Query<&mut Transform, With<EditorSceneEntity>>,
    mut editor_scene: ResMut<EditorScene>,
) {
    for event in events.read() {
        match event {
            TransformEditEvent::SetPosition { entity, position } => {
                if let Ok(mut transform) = entity_query.get_mut(*entity) {
                    transform.translation.x = position.x;
                    transform.translation.y = position.y;
                    editor_scene.mark_modified();
                    info!("Set entity {:?} position to {:?}", entity, position);
                }
            }
            TransformEditEvent::Translate { entity, delta } => {
                if let Ok(mut transform) = entity_query.get_mut(*entity) {
                    transform.translation.x += delta.x;
                    transform.translation.y += delta.y;
                    editor_scene.mark_modified();
                    info!("Translated entity {:?} by {:?}", entity, delta);
                }
            }
            TransformEditEvent::SetRotation { entity, rotation } => {
                if let Ok(mut transform) = entity_query.get_mut(*entity) {
                    transform.rotation = Quat::from_rotation_z(*rotation);
                    editor_scene.mark_modified();
                    info!("Set entity {:?} rotation to {}", entity, rotation);
                }
            }
            TransformEditEvent::SetScale { entity, scale } => {
                if let Ok(mut transform) = entity_query.get_mut(*entity) {
                    transform.scale.x = scale.x;
                    transform.scale.y = scale.y;
                    editor_scene.mark_modified();
                    info!("Set entity {:?} scale to {:?}", entity, scale);
                }
            }
        }
    }
}

/// Plugin for scene editor functionality
pub struct SceneEditorPlugin;

impl Plugin for SceneEditorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EditorScene>()
            .register_type::<EditorSceneEntity>()
            .add_event::<TransformEditEvent>()
            .add_systems(Startup, setup_editor_scene)
            .add_systems(Update, handle_transform_edit_events);
    }
}
