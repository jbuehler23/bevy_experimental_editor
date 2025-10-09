//! Concrete implementations of editor commands for undo/redo

use bevy_editor_core::EditorCommand;
use crate::entity_templates::EntityTemplate;
use crate::scene_editor::EditorScene;
use bevy::prelude::*;

/// Command to create a new entity
pub struct CreateEntityCommand {
    /// The entity that was/will be created
    entity: Option<Entity>,
    /// The template to use for creation
    template: EntityTemplate,
    /// The parent entity (if any)
    parent: Option<Entity>,
    /// Saved component data for redo
    saved_components: Option<SavedEntityData>,
}

#[derive(Clone)]
struct SavedEntityData {
    name: String,
    transform: Transform,
    // Add more components as needed
}

impl CreateEntityCommand {
    pub fn new(template: EntityTemplate, parent: Option<Entity>) -> Self {
        Self {
            entity: None,
            template,
            parent,
            saved_components: None,
        }
    }
}

impl EditorCommand for CreateEntityCommand {
    fn execute(&mut self, world: &mut World) {
        // Spawn the entity using the template
        let entity = crate::entity_templates::spawn_from_template(
            &mut world.commands(),
            self.template,
            self.parent,
        );

        self.entity = Some(entity);

        // Update editor scene selection
        if let Some(mut editor_scene) = world.get_resource_mut::<EditorScene>() {
            editor_scene.select_entity(entity);
            editor_scene.mark_modified();
        }

        info!("Created entity {:?} from template {:?}", entity, self.template);
    }

    fn undo(&mut self, world: &mut World) {
        if let Some(entity) = self.entity {
            // Save component data before deleting (for potential redo)
            if let Ok(entity_ref) = world.get_entity(entity) {
                if let Some(name) = entity_ref.get::<Name>() {
                    if let Some(transform) = entity_ref.get::<Transform>() {
                        self.saved_components = Some(SavedEntityData {
                            name: name.to_string(),
                            transform: *transform,
                        });
                    }
                }
            }

            // Despawn the entity
            if let Ok(entity_commands) = world.get_entity_mut(entity) {
                entity_commands.despawn();
                info!("Undid entity creation - despawned {:?}", entity);
            }

            // Clear selection if this was the selected entity
            if let Some(mut editor_scene) = world.get_resource_mut::<EditorScene>() {
                if editor_scene.selected_entity == Some(entity) {
                    editor_scene.select_entity(Entity::PLACEHOLDER);
                }
                editor_scene.mark_modified();
            }
        }
    }

    fn description(&self) -> String {
        format!("Create {}", self.template.display_name())
    }
}

/// Command to delete an entity
pub struct DeleteEntityCommand {
    /// The entity that was/will be deleted
    entity: Entity,
    /// Saved data to restore the entity on undo
    saved_data: Option<SavedDeletedEntity>,
}

#[derive(Clone)]
struct SavedDeletedEntity {
    name: String,
    transform: Transform,
    visibility: Visibility,
    template: EntityTemplate,
    parent: Option<Entity>,
    // TODO: Save all components via reflection/serialization
}

impl DeleteEntityCommand {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            saved_data: None,
        }
    }
}

impl EditorCommand for DeleteEntityCommand {
    fn execute(&mut self, world: &mut World) {
        // Save entity data before deleting
        if let Ok(entity_ref) = world.get_entity(self.entity) {
            let name = entity_ref.get::<Name>().map(|n| n.to_string()).unwrap_or_default();
            let transform = entity_ref.get::<Transform>().copied().unwrap_or_default();
            let visibility = entity_ref.get::<Visibility>().copied().unwrap_or_default();
            let parent = entity_ref.get::<ChildOf>().map(|p| p.0);

            // Determine template type from components
            let template = if entity_ref.contains::<Sprite>() {
                EntityTemplate::Sprite
            } else if entity_ref.contains::<Node>() {
                if entity_ref.contains::<Button>() {
                    EntityTemplate::Button
                } else {
                    EntityTemplate::UINode
                }
            } else if entity_ref.contains::<Text>() {
                EntityTemplate::Text
            } else {
                EntityTemplate::Empty
            };

            self.saved_data = Some(SavedDeletedEntity {
                name,
                transform,
                visibility,
                template,
                parent,
            });
        }

        // Despawn the entity
        if let Ok(entity_commands) = world.get_entity_mut(self.entity) {
            entity_commands.despawn();
            info!("Deleted entity {:?}", self.entity);
        }

        // Clear selection
        if let Some(mut editor_scene) = world.get_resource_mut::<EditorScene>() {
            if editor_scene.selected_entity == Some(self.entity) {
                editor_scene.select_entity(Entity::PLACEHOLDER);
            }
            editor_scene.mark_modified();
        }
    }

    fn undo(&mut self, world: &mut World) {
        if let Some(ref data) = self.saved_data {
            // Recreate the entity
            let entity = crate::entity_templates::spawn_from_template(
                &mut world.commands(),
                data.template,
                data.parent,
            );

            // Restore components
            if let Ok(mut entity_commands) = world.get_entity_mut(entity) {
                entity_commands.insert(Name::new(data.name.clone()));
                entity_commands.insert(data.transform);
                entity_commands.insert(data.visibility);
            }

            // Update stored entity ID
            self.entity = entity;

            // Update editor scene
            if let Some(mut editor_scene) = world.get_resource_mut::<EditorScene>() {
                editor_scene.select_entity(entity);
                editor_scene.mark_modified();
            }

            info!("Undid entity deletion - recreated {:?}", entity);
        }
    }

    fn description(&self) -> String {
        if let Some(ref data) = self.saved_data {
            format!("Delete '{}'", data.name)
        } else {
            format!("Delete entity {:?}", self.entity)
        }
    }
}

/// Command to modify an entity's transform
pub struct TransformCommand {
    entity: Entity,
    old_transform: Transform,
    new_transform: Transform,
    property_name: String,  // "Position", "Rotation", or "Scale"
}

impl TransformCommand {
    pub fn new_position(entity: Entity, old_pos: Vec2, new_pos: Vec2, current_transform: Transform) -> Self {
        let mut old_transform = current_transform;
        old_transform.translation.x = old_pos.x;
        old_transform.translation.y = old_pos.y;

        let mut new_transform = current_transform;
        new_transform.translation.x = new_pos.x;
        new_transform.translation.y = new_pos.y;

        Self {
            entity,
            old_transform,
            new_transform,
            property_name: "Position".to_string(),
        }
    }

    pub fn new_rotation(entity: Entity, old_rot: f32, new_rot: f32, current_transform: Transform) -> Self {
        let mut old_transform = current_transform;
        old_transform.rotation = Quat::from_rotation_z(old_rot);

        let mut new_transform = current_transform;
        new_transform.rotation = Quat::from_rotation_z(new_rot);

        Self {
            entity,
            old_transform,
            new_transform,
            property_name: "Rotation".to_string(),
        }
    }

    pub fn new_scale(entity: Entity, old_scale: Vec2, new_scale: Vec2, current_transform: Transform) -> Self {
        let mut old_transform = current_transform;
        old_transform.scale.x = old_scale.x;
        old_transform.scale.y = old_scale.y;

        let mut new_transform = current_transform;
        new_transform.scale.x = new_scale.x;
        new_transform.scale.y = new_scale.y;

        Self {
            entity,
            old_transform,
            new_transform,
            property_name: "Scale".to_string(),
        }
    }
}

impl EditorCommand for TransformCommand {
    fn execute(&mut self, world: &mut World) {
        if let Ok(mut entity_commands) = world.get_entity_mut(self.entity) {
            entity_commands.insert(self.new_transform);
        }

        if let Some(mut editor_scene) = world.get_resource_mut::<EditorScene>() {
            editor_scene.mark_modified();
        }
    }

    fn undo(&mut self, world: &mut World) {
        if let Ok(mut entity_commands) = world.get_entity_mut(self.entity) {
            entity_commands.insert(self.old_transform);
        }

        if let Some(mut editor_scene) = world.get_resource_mut::<EditorScene>() {
            editor_scene.mark_modified();
        }
    }

    fn description(&self) -> String {
        format!("Change {}", self.property_name)
    }

    fn can_merge_with(&self, _other: &dyn EditorCommand) -> bool {
        // Try to downcast to TransformCommand
        // This is a simplified version - in practice, you'd want better type checking
        false  // For now, don't merge transform commands
    }
}

/// Command to rename an entity
pub struct RenameEntityCommand {
    entity: Entity,
    old_name: String,
    new_name: String,
}

impl RenameEntityCommand {
    pub fn new(entity: Entity, old_name: String, new_name: String) -> Self {
        Self {
            entity,
            old_name,
            new_name,
        }
    }
}

impl EditorCommand for RenameEntityCommand {
    fn execute(&mut self, world: &mut World) {
        if let Ok(mut entity_commands) = world.get_entity_mut(self.entity) {
            entity_commands.insert(Name::new(self.new_name.clone()));
        }

        if let Some(mut editor_scene) = world.get_resource_mut::<EditorScene>() {
            editor_scene.mark_modified();
        }
    }

    fn undo(&mut self, world: &mut World) {
        if let Ok(mut entity_commands) = world.get_entity_mut(self.entity) {
            entity_commands.insert(Name::new(self.old_name.clone()));
        }

        if let Some(mut editor_scene) = world.get_resource_mut::<EditorScene>() {
            editor_scene.mark_modified();
        }
    }

    fn description(&self) -> String {
        format!("Rename to '{}'", self.new_name)
    }
}
