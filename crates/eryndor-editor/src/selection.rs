use bevy::prelude::*;
use std::collections::HashSet;

use crate::EditorState;

/// Marker component for entities that can be selected in the editor
#[derive(Component, Debug, Clone)]
pub struct EditorEntity {
    pub entity_id: usize,          // Index into level data
    pub entity_type: EditorEntityType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorEntityType {
    Platform,
    NpcSpawn,
    ResourceNode,
    InteractiveObject,
    SpawnPoint,
}

/// Resource tracking currently selected entities
#[derive(Resource, Default)]
pub struct Selection {
    pub selected: HashSet<Entity>,
}

impl Selection {
    pub fn clear(&mut self) {
        self.selected.clear();
    }

    pub fn select(&mut self, entity: Entity) {
        self.selected.clear();
        self.selected.insert(entity);
    }

    pub fn toggle(&mut self, entity: Entity) {
        if self.selected.contains(&entity) {
            self.selected.remove(&entity);
        } else {
            self.selected.insert(entity);
        }
    }

    pub fn is_selected(&self, entity: Entity) -> bool {
        self.selected.contains(&entity)
    }
}

/// Maps level data indices to Bevy entities
#[derive(Resource, Default)]
pub struct EditorEntityMap {
    pub platforms: Vec<Entity>,
    pub entities: Vec<Entity>,
}

/// Handle entity selection with mouse clicks
pub fn handle_selection(
    mouse_button: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    editor_entities: Query<(Entity, &Transform, &EditorEntity)>,
    mut selection: ResMut<Selection>,
    editor_state: Res<EditorState>,
    mut contexts: bevy_egui::EguiContexts,
) {
    // Don't select if hovering over UI
    let ctx = contexts.ctx_mut();
    if ctx.is_pointer_over_area() {
        return;
    }

    // Only allow selection with Select tool
    if editor_state.current_tool != crate::EditorTool::Select {
        return;
    }

    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    let Some(cursor_world_pos) = get_cursor_world_pos(&windows, &camera_q) else {
        // Clicked on nothing, clear selection if not holding Ctrl
        if !keyboard.pressed(KeyCode::ControlLeft) {
            selection.clear();
        }
        return;
    };

    // Find entity under cursor
    let mut closest_entity = None;
    let mut closest_distance = f32::MAX;

    for (entity, transform, _editor_entity) in editor_entities.iter() {
        let distance = transform.translation.xy().distance(cursor_world_pos);

        // Use a reasonable selection radius
        if distance < 20.0 && distance < closest_distance {
            closest_distance = distance;
            closest_entity = Some(entity);
        }
    }

    if let Some(entity) = closest_entity {
        if keyboard.pressed(KeyCode::ControlLeft) {
            // Multi-select with Ctrl
            selection.toggle(entity);
        } else {
            // Single select
            selection.select(entity);
        }
    } else if !keyboard.pressed(KeyCode::ControlLeft) {
        // Clicked on empty space, clear selection
        selection.clear();
    }
}

/// Handle entity deletion with Delete/Backspace keys
pub fn handle_entity_deletion(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    selection: Res<Selection>,
    editor_entities: Query<&EditorEntity>,
    mut open_scenes: ResMut<crate::scene_tabs::OpenScenes>,  // Changed from CurrentLevel
    mut entity_map: ResMut<EditorEntityMap>,
) {
    if !keyboard.just_pressed(KeyCode::Delete) && !keyboard.just_pressed(KeyCode::Backspace) {
        return;
    }

    if selection.selected.is_empty() {
        return;
    }

    // Delete selected entities from active scene
    if let Some(scene) = open_scenes.active_scene_mut() {
        for entity in selection.selected.iter() {
            if let Ok(editor_entity) = editor_entities.get(*entity) {
                // Remove from level data
                match editor_entity.entity_type {
                    EditorEntityType::Platform => {
                        if editor_entity.entity_id < scene.level_data.platforms.len() {
                            scene.level_data.platforms.remove(editor_entity.entity_id);
                        }
                    }
                    _ => {
                        if editor_entity.entity_id < scene.level_data.entities.len() {
                            scene.level_data.entities.remove(editor_entity.entity_id);
                        }
                    }
                }

                // Despawn visual entity
                commands.entity(*entity).despawn();
                scene.is_modified = true;
            }
        }

        // Rebuild entity map after deletion
        // TODO: Make this more efficient
    }
}

// Helper functions

fn get_cursor_world_pos(
    windows: &Query<&Window>,
    camera_q: &Query<(&Camera, &GlobalTransform)>,
) -> Option<Vec2> {
    let window = windows.get_single().ok()?;
    let (camera, camera_transform) = camera_q.get_single().ok()?;

    window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor).ok())
}
