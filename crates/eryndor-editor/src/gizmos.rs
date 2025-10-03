use bevy::prelude::*;

use crate::{EditorState, Selection, EditorEntity, EditorEntityType};

/// Draw grid in the viewport
pub fn draw_grid(
    mut gizmos: Gizmos,
    editor_state: Res<EditorState>,
    camera_q: Query<&Transform, With<Camera2d>>,
) {
    if !editor_state.grid_snap_enabled {
        return;
    }

    let Ok(camera_transform) = camera_q.get_single() else {
        return;
    };

    let grid_size = editor_state.grid_size;
    let camera_pos = camera_transform.translation.xy();

    // Draw grid lines in view
    let grid_extent = 2000.0; // How far to draw grid

    // Vertical lines
    let start_x = ((camera_pos.x - grid_extent) / grid_size).floor() * grid_size;
    let end_x = camera_pos.x + grid_extent;
    let mut x = start_x;
    while x <= end_x {
        gizmos.line_2d(
            Vec2::new(x, camera_pos.y - grid_extent),
            Vec2::new(x, camera_pos.y + grid_extent),
            Color::srgba(1.0, 1.0, 1.0, 0.1),
        );
        x += grid_size;
    }

    // Horizontal lines
    let start_y = ((camera_pos.y - grid_extent) / grid_size).floor() * grid_size;
    let end_y = camera_pos.y + grid_extent;
    let mut y = start_y;
    while y <= end_y {
        gizmos.line_2d(
            Vec2::new(camera_pos.x - grid_extent, y),
            Vec2::new(camera_pos.x + grid_extent, y),
            Color::srgba(1.0, 1.0, 1.0, 0.1),
        );
        y += grid_size;
    }

    // Draw origin lines
    gizmos.line_2d(
        Vec2::new(0.0, camera_pos.y - grid_extent),
        Vec2::new(0.0, camera_pos.y + grid_extent),
        Color::srgba(0.0, 1.0, 0.0, 0.3),
    );
    gizmos.line_2d(
        Vec2::new(camera_pos.x - grid_extent, 0.0),
        Vec2::new(camera_pos.x + grid_extent, 0.0),
        Color::srgba(1.0, 0.0, 0.0, 0.3),
    );
}

/// Draw selection highlights and move handles
pub fn draw_selection_gizmos(
    mut gizmos: Gizmos,
    selection: Res<Selection>,
    editor_entities: Query<(&Transform, &EditorEntity)>,
) {
    for selected_entity in selection.selected.iter() {
        if let Ok((transform, editor_entity)) = editor_entities.get(*selected_entity) {
            let pos = transform.translation.xy();

            // Draw selection highlight
            match editor_entity.entity_type {
                EditorEntityType::Platform => {
                    // Draw platform outline (rectangle around platform)
                    let size = Vec2::new(100.0, 20.0); // TODO: Get actual size from component
                    let half_size = size / 2.0;
                    // Draw rectangle using lines
                    let corners = [
                        pos + Vec2::new(-half_size.x, -half_size.y),
                        pos + Vec2::new(half_size.x, -half_size.y),
                        pos + Vec2::new(half_size.x, half_size.y),
                        pos + Vec2::new(-half_size.x, half_size.y),
                    ];
                    for i in 0..4 {
                        gizmos.line_2d(corners[i], corners[(i + 1) % 4], Color::srgb(1.0, 1.0, 0.0));
                    }
                }
                _ => {
                    // Draw circle highlight for entities
                    gizmos.circle_2d(pos, 20.0, Color::srgb(1.0, 1.0, 0.0));
                }
            }

            // Draw move handles
            draw_move_handles(&mut gizmos, pos);
        }
    }
}

fn draw_move_handles(gizmos: &mut Gizmos, position: Vec2) {
    let handle_size = 8.0;

    // Center handle
    gizmos.circle_2d(position, handle_size, Color::srgb(0.0, 1.0, 0.0));

    // Axis handles
    // X-axis (red)
    gizmos.line_2d(
        position,
        position + Vec2::new(30.0, 0.0),
        Color::srgb(1.0, 0.0, 0.0),
    );
    gizmos.circle_2d(
        position + Vec2::new(30.0, 0.0),
        handle_size * 0.7,
        Color::srgb(1.0, 0.0, 0.0),
    );

    // Y-axis (green)
    gizmos.line_2d(
        position,
        position + Vec2::new(0.0, 30.0),
        Color::srgb(0.0, 1.0, 0.0),
    );
    gizmos.circle_2d(
        position + Vec2::new(0.0, 30.0),
        handle_size * 0.7,
        Color::srgb(0.0, 1.0, 0.0),
    );
}
