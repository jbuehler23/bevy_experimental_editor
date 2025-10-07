//! Viewport entity selection and gizmo interaction system
//! Handles clicking and dragging entities in the viewport

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::EguiContext;

use crate::scene_editor::{EditorScene, EditorSceneEntity, TransformEditEvent};

/// Resource to track gizmo drag state
#[derive(Resource, Default)]
pub struct GizmoDragState {
    pub is_dragging: bool,
    pub dragged_entity: Option<Entity>,
    pub drag_start_world: Vec2,
    pub entity_start_pos: Vec2,
}

/// System to handle clicking entities in the viewport
pub fn viewport_entity_selection_system(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    entity_query: Query<(Entity, &GlobalTransform, Option<&Sprite>), With<EditorSceneEntity>>,
    mut editor_scene: ResMut<EditorScene>,
    mut egui_contexts: Query<&mut EguiContext, With<PrimaryWindow>>,
) {
    // Only handle left click
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    // Don't select if mouse is over egui UI
    if let Ok(mut egui_context) = egui_contexts.get_single_mut() {
        if egui_context.get_mut().is_pointer_over_area() {
            return;
        }
    }

    let Ok(window) = windows.single() else {
        return;
    };

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let Ok((camera, camera_transform)) = camera_q.single() else {
        return;
    };

    // Convert cursor position to world position
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    let world_position = ray.origin.truncate();

    // Find the closest entity to the click position
    let mut closest_entity: Option<Entity> = None;
    let mut closest_distance = f32::MAX;

    for (entity, transform, sprite) in entity_query.iter() {
        let entity_pos = transform.translation().truncate();

        // Calculate bounds based on sprite size or default
        let size = if let Some(sprite_comp) = sprite {
            sprite_comp.custom_size.unwrap_or(Vec2::new(32.0, 32.0))
        } else {
            Vec2::new(32.0, 32.0) // Default size
        };

        let half_size = size / 2.0;

        // Check if click is within entity bounds
        let min = entity_pos - half_size;
        let max = entity_pos + half_size;

        if world_position.x >= min.x
            && world_position.x <= max.x
            && world_position.y >= min.y
            && world_position.y <= max.y
        {
            let distance = entity_pos.distance(world_position);
            if distance < closest_distance {
                closest_distance = distance;
                closest_entity = Some(entity);
            }
        }
    }

    // Update selection
    if let Some(entity) = closest_entity {
        editor_scene.select_entity(entity);
        info!("Selected entity in viewport: {:?}", entity);
    }
    // Note: Don't clear selection when clicking empty space
    // This allows clicking on panels without losing selection
}

/// System to handle dragging selected entities via gizmo
pub fn gizmo_drag_interaction_system(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut editor_scene: ResMut<EditorScene>,
    mut drag_state: ResMut<GizmoDragState>,
    mut entity_query: Query<(&mut Transform, &GlobalTransform, Option<&ChildOf>), With<EditorSceneEntity>>,
    parent_query: Query<&GlobalTransform>,
    mut egui_contexts: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut transform_events: EventWriter<TransformEditEvent>,
) {
    // Don't interact if mouse is over egui UI
    if let Ok(mut egui_context) = egui_contexts.get_single_mut() {
        if egui_context.get_mut().is_pointer_over_area() {
            return;
        }
    }

    let Ok(window) = windows.single() else {
        return;
    };

    let Ok((camera, camera_transform)) = camera_q.single() else {
        return;
    };

    // Helper function to get world position from cursor
    let get_world_pos = || -> Option<Vec2> {
        let cursor_position = window.cursor_position()?;
        let ray = camera.viewport_to_world(camera_transform, cursor_position).ok()?;
        Some(ray.origin.truncate())
    };

    // Start dragging
    if mouse_button.just_pressed(MouseButton::Left) && !drag_state.is_dragging {
        if let Some(selected_entity) = editor_scene.selected_entity {
            if let Ok((transform, global_transform, _)) = entity_query.get(selected_entity) {
                if let Some(world_pos) = get_world_pos() {
                    // Check if clicking on selected entity (within gizmo range)
                    // Use GlobalTransform for accurate world position
                    let entity_world_pos = global_transform.translation().truncate();
                    let distance = entity_world_pos.distance(world_pos);

                    // Allow dragging if clicking within gizmo range (50 pixels in world space)
                    if distance < 50.0 {
                        drag_state.is_dragging = true;
                        drag_state.dragged_entity = Some(selected_entity);
                        drag_state.drag_start_world = world_pos;
                        drag_state.entity_start_pos = entity_world_pos;
                        info!("Started dragging entity: {:?}", selected_entity);
                    }
                }
            }
        }
    }

    // During drag - update entity position
    if drag_state.is_dragging {
        if mouse_button.pressed(MouseButton::Left) {
            if let Some(dragged_entity) = drag_state.dragged_entity {
                if let Ok((mut transform, _, parent)) = entity_query.get_mut(dragged_entity) {
                    if let Some(current_world_pos) = get_world_pos() {
                        // Calculate delta from drag start in world space
                        let delta = current_world_pos - drag_state.drag_start_world;
                        let new_world_pos = drag_state.entity_start_pos + delta;

                        // Convert world position to local space if entity has a parent
                        let local_pos = if let Some(child_of) = parent {
                            // Get parent's global transform
                            if let Ok(parent_global_transform) = parent_query.get(child_of.0) {
                                // Convert world position to parent's local space
                                let parent_inverse = parent_global_transform.affine().inverse();
                                let local_pos_3d = parent_inverse.transform_point3(new_world_pos.extend(0.0));
                                local_pos_3d.truncate()
                            } else {
                                // Fallback: if parent not found, use world position
                                new_world_pos
                            }
                        } else {
                            // No parent - local position is same as world position
                            new_world_pos
                        };

                        // Update entity local transform
                        transform.translation.x = local_pos.x;
                        transform.translation.y = local_pos.y;
                    }
                }
            }
        } else {
            // Mouse released - end drag and send event
            if let Some(dragged_entity) = drag_state.dragged_entity {
                if let Ok((transform, _, _)) = entity_query.get(dragged_entity) {
                    let final_pos = transform.translation.truncate();

                    // Send SetPosition event for undo/redo support
                    transform_events.send(TransformEditEvent::SetPosition {
                        entity: dragged_entity,
                        position: final_pos,
                    });

                    // Mark scene as modified
                    editor_scene.mark_modified();

                    info!("Ended dragging entity {:?} at position {:?}", dragged_entity, final_pos);
                }
            }

            drag_state.is_dragging = false;
            drag_state.dragged_entity = None;
        }
    }
}
