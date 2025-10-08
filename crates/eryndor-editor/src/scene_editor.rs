//! Scene editor module for managing game entities (separate from tilemap)
//! Uses Bevy's DynamicScene for serialization/deserialization

use bevy::prelude::*;
use bevy::scene::{DynamicScene, DynamicSceneBuilder, DynamicSceneRoot, SceneSpawner};

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

/// Save current EditorScene entities to .scn.ron file
pub fn save_editor_scene_to_file(
    world: &mut World,
    scene_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Get type registry for serialization
    let type_registry = world.resource::<AppTypeRegistry>().clone();

    // Collect all EditorSceneEntity entities
    let mut scene_entities = Vec::new();
    let mut query = world.query_filtered::<Entity, With<EditorSceneEntity>>();
    for entity in query.iter(world) {
        scene_entities.push(entity);
    }

    if scene_entities.is_empty() {
        warn!("No entities to save in scene");
        return Ok(());
    }

    // Remove VisibilityClass from all entities before serialization
    // (contains TypeId which can't be serialized)
    for &entity in &scene_entities {
        if let Ok(mut entity_mut) = world.get_entity_mut(entity) {
            entity_mut.remove::<bevy::render::view::visibility::VisibilityClass>();
        }
    }

    // Build DynamicScene from all scene entities
    let scene_builder = DynamicSceneBuilder::from_world(world)
        .extract_entities(scene_entities.into_iter());
    let dynamic_scene = scene_builder.build();

    // Serialize to RON
    let type_registry = type_registry.read();
    let ron_string = dynamic_scene.serialize(&type_registry)?;

    // Write to file
    std::fs::write(scene_path, ron_string)?;
    info!("Scene saved to: {}", scene_path);

    Ok(())
}

/// Load .scn.ron file into EditorScene using Bevy's asset system
pub fn load_editor_scene_from_file(
    commands: &mut Commands,
    asset_server: &AssetServer,
    scene_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Loading scene from: {}", scene_path);

    // Load scene using Bevy's asset system
    let scene_handle: Handle<DynamicScene> = asset_server.load(scene_path);

    // Spawn scene into world with marker
    commands.spawn((
        DynamicSceneRoot(scene_handle.clone()),
        EditorSceneEntity, // Mark the root so we can find it
    ));

    Ok(())
}

/// System to tag newly spawned scene entities with EditorSceneEntity marker
/// This runs after SceneSpawner has instantiated the scene
pub fn tag_spawned_scene_entities(
    mut commands: Commands,
    untagged_query: Query<Entity, (Without<EditorSceneEntity>, With<Transform>)>,
    scene_root_query: Query<&DynamicSceneRoot, With<EditorSceneEntity>>,
    mut editor_scene: ResMut<EditorScene>,
) {
    // If we have a scene root marker, tag all untagged Transform entities
    if scene_root_query.iter().count() > 0 {
        let mut tagged_count = 0;
        for entity in untagged_query.iter() {
            commands.entity(entity).insert(EditorSceneEntity);
            tagged_count += 1;

            // Set first tagged entity as root if not set
            if editor_scene.root_entity.is_none() {
                editor_scene.root_entity = Some(entity);
                info!("Set scene root to {:?}", entity);
            }
        }

        if tagged_count > 0 {
            info!("Tagged {} entities as EditorSceneEntity", tagged_count);
        }
    }
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

/// Event for editing entity name
#[derive(Event, Debug, Clone)]
pub struct NameEditEvent {
    pub entity: Entity,
    pub new_name: String,
}

/// Event for assigning texture to sprite
#[derive(Event, Debug, Clone)]
pub struct SpriteTextureEvent {
    pub entity: Entity,
    pub texture_handle: Handle<Image>,
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

/// System to handle name edit events
pub fn handle_name_edit_events(
    mut events: EventReader<NameEditEvent>,
    mut entity_query: Query<&mut Name, With<EditorSceneEntity>>,
    mut editor_scene: ResMut<EditorScene>,
) {
    for event in events.read() {
        if let Ok(mut name) = entity_query.get_mut(event.entity) {
            name.set(event.new_name.clone());
            editor_scene.mark_modified();
            info!("Renamed entity {:?} to '{}'", event.entity, event.new_name);
        }
    }
}

/// System to handle sprite texture assignment events
pub fn handle_sprite_texture_events(
    mut events: EventReader<SpriteTextureEvent>,
    mut sprite_query: Query<&mut Sprite, With<EditorSceneEntity>>,
    mut editor_scene: ResMut<EditorScene>,
) {
    for event in events.read() {
        // Update sprite's image handle and reset color to white for proper texture display
        if let Ok(mut sprite) = sprite_query.get_mut(event.entity) {
            sprite.image = event.texture_handle.clone();
            // Set color to white so the texture displays without tinting
            sprite.color = Color::WHITE;
            editor_scene.mark_modified();
            info!("Assigned texture to sprite entity {:?}", event.entity);
        } else {
            warn!("Attempted to assign texture to non-sprite entity {:?}", event.entity);
        }
    }
}

/// Plugin for scene editor functionality
pub struct SceneEditorPlugin;

impl Plugin for SceneEditorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EditorScene>()
            // Register marker component
            .register_type::<EditorSceneEntity>()
            // Register core Bevy components for scene serialization
            .register_type::<Name>()
            .register_type::<Transform>()
            .register_type::<GlobalTransform>()
            .register_type::<Visibility>()
            .register_type::<InheritedVisibility>()
            .register_type::<ViewVisibility>()
            // Register rendering components
            .register_type::<Sprite>()
            // Events
            .add_event::<TransformEditEvent>()
            .add_event::<NameEditEvent>()
            // Systems
            .add_systems(Startup, setup_editor_scene)
            .add_systems(Update, (
                handle_transform_edit_events,
                handle_name_edit_events,
                tag_spawned_scene_entities, // Tag entities after scene loads
            ));
    }
}
