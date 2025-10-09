//! Viewport entity selection and gizmo interaction system
//! Handles clicking and dragging entities in the viewport

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::EguiContext;

use crate::scene_editor::{EditorScene, EditorSceneEntity, TransformEditEvent};
use crate::gizmos::{GizmoMode, GizmoState};
use bevy_editor_core::EditorHistory;
use crate::editor_commands::TransformCommand;

/// Which specific gizmo handle is being dragged
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GizmoHandle {
    Center,        // Center handle (move in both axes)
    XAxis,         // X-axis handle (move/scale X)
    YAxis,         // Y-axis handle (move/scale Y)
    RotateHandle,  // Any rotation handle
    ScaleCorner,   // Corner scale handle
    ScaleEdge(Vec2), // Edge scale handle with direction
}

/// Resource to track gizmo drag state
#[derive(Resource, Default)]
pub struct GizmoDragState {
    pub is_dragging: bool,
    pub dragged_entity: Option<Entity>,
    pub drag_start_world: Vec2,
    pub entity_start_pos: Vec2,
    pub entity_start_rotation: f32,
    pub entity_start_scale: Vec3,
    pub active_handle: Option<GizmoHandle>,
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
    mut entity_query: Query<(&mut Transform, &GlobalTransform, Option<&Sprite>, Option<&ChildOf>), With<EditorSceneEntity>>,
    parent_query: Query<&GlobalTransform>,
    images: Res<Assets<Image>>,
    mut egui_contexts: Query<&mut EguiContext, With<PrimaryWindow>>,
    mut transform_events: EventWriter<TransformEditEvent>,
    gizmo_state: Res<GizmoState>,
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
            if let Ok((transform, global_transform, sprite, _)) = entity_query.get(selected_entity) {
                if let Some(world_pos) = get_world_pos() {
                    // Use GlobalTransform for accurate world position
                    let entity_world_pos = global_transform.translation().truncate();

                    // Get entity bounds for handle positioning
                    let bounds = if let Some(sprite_comp) = sprite {
                        let base_size = if let Some(custom_size) = sprite_comp.custom_size {
                            custom_size
                        } else if let Some(image) = images.get(&sprite_comp.image) {
                            image.size().as_vec2()
                        } else {
                            Vec2::new(64.0, 64.0)
                        };
                        base_size * transform.scale.xy()
                    } else {
                        Vec2::new(32.0, 32.0) * transform.scale.xy()
                    };

                    // Detect which handle is being clicked based on gizmo mode
                    let handle = detect_gizmo_handle(
                        world_pos,
                        entity_world_pos,
                        bounds,
                        gizmo_state.mode,
                    );

                    if let Some(handle) = handle {
                        drag_state.is_dragging = true;
                        drag_state.dragged_entity = Some(selected_entity);
                        drag_state.drag_start_world = world_pos;
                        drag_state.entity_start_pos = entity_world_pos;
                        drag_state.entity_start_rotation = transform.rotation.to_euler(EulerRot::XYZ).2;
                        drag_state.entity_start_scale = transform.scale;
                        drag_state.active_handle = Some(handle);
                        info!("Started dragging {:?} handle on entity {:?}", handle, selected_entity);
                    }
                }
            }
        }
    }

    // During drag - update entity transform based on active handle
    if drag_state.is_dragging {
        if mouse_button.pressed(MouseButton::Left) {
            if let Some(dragged_entity) = drag_state.dragged_entity {
                if let Ok((mut transform, global_transform, _, parent)) = entity_query.get_mut(dragged_entity) {
                    if let Some(current_world_pos) = get_world_pos() {
                        let delta = current_world_pos - drag_state.drag_start_world;

                        match drag_state.active_handle {
                            Some(GizmoHandle::Center) | Some(GizmoHandle::XAxis) | Some(GizmoHandle::YAxis) => {
                                // Move mode - translate entity
                                let constrained_delta = match drag_state.active_handle {
                                    Some(GizmoHandle::XAxis) => Vec2::new(delta.x, 0.0),
                                    Some(GizmoHandle::YAxis) => Vec2::new(0.0, delta.y),
                                    _ => delta,
                                };

                                let new_world_pos = drag_state.entity_start_pos + constrained_delta;

                                // Convert to local space if needed
                                let local_pos = if let Some(child_of) = parent {
                                    if let Ok(parent_global_transform) = parent_query.get(child_of.0) {
                                        let parent_inverse = parent_global_transform.affine().inverse();
                                        let local_pos_3d = parent_inverse.transform_point3(new_world_pos.extend(0.0));
                                        local_pos_3d.truncate()
                                    } else {
                                        new_world_pos
                                    }
                                } else {
                                    new_world_pos
                                };

                                transform.translation.x = local_pos.x;
                                transform.translation.y = local_pos.y;
                            }

                            Some(GizmoHandle::RotateHandle) => {
                                // Rotate mode - rotate around center
                                let entity_world_pos = global_transform.translation().truncate();
                                let start_vec = (drag_state.drag_start_world - entity_world_pos).normalize();
                                let current_vec = (current_world_pos - entity_world_pos).normalize();

                                let angle = start_vec.x * current_vec.y - start_vec.y * current_vec.x;
                                let angle = angle.atan2(start_vec.dot(current_vec));

                                let new_rotation = drag_state.entity_start_rotation + angle;
                                transform.rotation = Quat::from_rotation_z(new_rotation);
                            }

                            Some(GizmoHandle::ScaleCorner) => {
                                // Uniform scale from corner
                                let entity_world_pos = global_transform.translation().truncate();
                                let start_dist = drag_state.drag_start_world.distance(entity_world_pos);
                                let current_dist = current_world_pos.distance(entity_world_pos);

                                if start_dist > 0.1 {
                                    let scale_factor = current_dist / start_dist;
                                    transform.scale = drag_state.entity_start_scale * scale_factor;
                                }
                            }

                            Some(GizmoHandle::ScaleEdge(axis)) => {
                                // Axis-aligned scale
                                let projected_delta = delta.dot(axis);
                                let scale_factor = 1.0 + (projected_delta / 100.0);

                                if axis.x.abs() > 0.5 {
                                    // X-axis scale
                                    transform.scale.x = (drag_state.entity_start_scale.x * scale_factor).max(0.1);
                                } else {
                                    // Y-axis scale
                                    transform.scale.y = (drag_state.entity_start_scale.y * scale_factor).max(0.1);
                                }
                            }

                            None => {}
                        }
                    }
                }
            }
        } else {
            // Mouse released - end drag and send appropriate event with undo history
            if let Some(dragged_entity) = drag_state.dragged_entity {
                if let Ok((transform, _, _, _)) = entity_query.get(dragged_entity) {
                    // Send appropriate transform event based on what was modified
                    // Events will be intercepted by transform_with_undo system
                    match drag_state.active_handle {
                        Some(GizmoHandle::Center) | Some(GizmoHandle::XAxis) | Some(GizmoHandle::YAxis) => {
                            transform_events.send(TransformEditEvent::SetPosition {
                                entity: dragged_entity,
                                position: transform.translation.truncate(),
                            });
                        }
                        Some(GizmoHandle::RotateHandle) => {
                            transform_events.send(TransformEditEvent::SetRotation {
                                entity: dragged_entity,
                                rotation: transform.rotation.to_euler(EulerRot::XYZ).2,
                            });
                        }
                        Some(GizmoHandle::ScaleCorner) | Some(GizmoHandle::ScaleEdge(_)) => {
                            transform_events.send(TransformEditEvent::SetScale {
                                entity: dragged_entity,
                                scale: transform.scale.truncate(),
                            });
                        }
                        None => {}
                    }

                    editor_scene.mark_modified();
                    info!("Ended dragging entity {:?}", dragged_entity);
                }
            }

            drag_state.is_dragging = false;
            drag_state.dragged_entity = None;
            drag_state.active_handle = None;
        }
    }
}

/// System to convert transform edits into undo/redo commands
/// Runs AFTER handle_transform_edit_events (transform already applied)
pub fn transform_with_undo_system(
    mut events: EventReader<TransformEditEvent>,
    entity_query: Query<&Transform, With<EditorSceneEntity>>,
    drag_state: Res<GizmoDragState>,
    mut history: ResMut<EditorHistory>,
) {
    for event in events.read() {
        match event {
            TransformEditEvent::SetPosition { entity, position } => {
                // Get old position from drag_state (if available)
                if let Ok(transform) = entity_query.get(*entity) {
                    let old_pos = if drag_state.dragged_entity == Some(*entity) {
                        drag_state.entity_start_pos
                    } else {
                        // Fallback: use current position (no undo possible)
                        *position
                    };

                    // Only create command if we have different old/new values
                    if old_pos != *position {
                        let command = TransformCommand::new_position(
                            *entity,
                            old_pos,
                            *position,
                            *transform,
                        );

                        // Add to history without re-executing (transform already applied)
                        history.add_executed(Box::new(command));
                    }
                }
            }
            TransformEditEvent::SetRotation { entity, rotation } => {
                if let Ok(transform) = entity_query.get(*entity) {
                    let old_rot = if drag_state.dragged_entity == Some(*entity) {
                        drag_state.entity_start_rotation
                    } else {
                        *rotation
                    };

                    if (old_rot - *rotation).abs() > 0.001 {
                        let command = TransformCommand::new_rotation(
                            *entity,
                            old_rot,
                            *rotation,
                            *transform,
                        );

                        history.add_executed(Box::new(command));
                    }
                }
            }
            TransformEditEvent::SetScale { entity, scale } => {
                if let Ok(transform) = entity_query.get(*entity) {
                    let old_scale = if drag_state.dragged_entity == Some(*entity) {
                        drag_state.entity_start_scale.truncate()
                    } else {
                        *scale
                    };

                    if old_scale != *scale {
                        let command = TransformCommand::new_scale(
                            *entity,
                            old_scale,
                            *scale,
                            *transform,
                        );

                        history.add_executed(Box::new(command));
                    }
                }
            }
            TransformEditEvent::Translate { .. } => {
                // Translate events don't support undo yet - they modify incrementally
                // TODO: Implement if needed
            }
        }
    }
}

/// Detect which gizmo handle (if any) is being clicked
fn detect_gizmo_handle(
    click_pos: Vec2,
    entity_pos: Vec2,
    bounds: Vec2,
    mode: GizmoMode,
) -> Option<GizmoHandle> {
    let handle_size = 8.0;
    let axis_length = 30.0;

    match mode {
        GizmoMode::Move => {
            // Check X-axis handle (higher priority)
            let x_handle_pos = entity_pos + Vec2::new(axis_length, 0.0);
            if click_pos.distance(x_handle_pos) < handle_size {
                return Some(GizmoHandle::XAxis);
            }

            // Check Y-axis handle (higher priority)
            let y_handle_pos = entity_pos + Vec2::new(0.0, axis_length);
            if click_pos.distance(y_handle_pos) < handle_size {
                return Some(GizmoHandle::YAxis);
            }

            // Check center handle
            if click_pos.distance(entity_pos) < handle_size * 1.5 {
                return Some(GizmoHandle::Center);
            }

            // Allow clicking anywhere within entity bounds for free movement
            let half_size = bounds / 2.0;
            let min = entity_pos - half_size;
            let max = entity_pos + half_size;

            if click_pos.x >= min.x && click_pos.x <= max.x
                && click_pos.y >= min.y && click_pos.y <= max.y
            {
                return Some(GizmoHandle::Center);
            }

            None
        }

        GizmoMode::Rotate => {
            let radius = (bounds.length() / 2.0) + 20.0;

            // Check rotation handles at cardinal directions
            let handle_positions = [
                entity_pos + Vec2::new(radius, 0.0),
                entity_pos + Vec2::new(0.0, radius),
                entity_pos + Vec2::new(-radius, 0.0),
                entity_pos + Vec2::new(0.0, -radius),
            ];

            for handle_pos in handle_positions {
                if click_pos.distance(handle_pos) < handle_size {
                    return Some(GizmoHandle::RotateHandle);
                }
            }

            // Check if clicking on rotation circle (within threshold)
            let dist_from_center = click_pos.distance(entity_pos);
            if (dist_from_center - radius).abs() < 10.0 {
                return Some(GizmoHandle::RotateHandle);
            }

            None
        }

        GizmoMode::Scale => {
            let half_size = bounds / 2.0;

            // Check corner handles (uniform scale)
            let corners = [
                entity_pos + Vec2::new(-half_size.x, -half_size.y),
                entity_pos + Vec2::new(half_size.x, -half_size.y),
                entity_pos + Vec2::new(half_size.x, half_size.y),
                entity_pos + Vec2::new(-half_size.x, half_size.y),
            ];

            for corner in corners {
                if click_pos.distance(corner) < handle_size {
                    return Some(GizmoHandle::ScaleCorner);
                }
            }

            // Check edge handles (axis-aligned scale)
            let edges = [
                (entity_pos + Vec2::new(0.0, -half_size.y), Vec2::Y),    // Bottom
                (entity_pos + Vec2::new(half_size.x, 0.0), Vec2::X),     // Right
                (entity_pos + Vec2::new(0.0, half_size.y), Vec2::Y),     // Top
                (entity_pos + Vec2::new(-half_size.x, 0.0), Vec2::X),    // Left
            ];

            for (edge_pos, axis) in edges {
                if click_pos.distance(edge_pos) < handle_size {
                    return Some(GizmoHandle::ScaleEdge(axis));
                }
            }

            None
        }
    }
}
