use eryndor_common::{CollisionShape, Vector2};
use crate::math::DbVector2;

/// AABB (Axis-Aligned Bounding Box) collision detection
pub fn check_aabb_rect_collision(
    aabb_pos: &DbVector2,
    aabb_size: &DbVector2,
    rect_pos: (f32, f32),
    rect_size: (f32, f32),
) -> bool {
    let aabb_min_x = aabb_pos.x - aabb_size.x / 2.0;
    let aabb_max_x = aabb_pos.x + aabb_size.x / 2.0;
    let aabb_min_y = aabb_pos.y - aabb_size.y / 2.0;
    let aabb_max_y = aabb_pos.y + aabb_size.y / 2.0;

    let rect_min_x = rect_pos.0;
    let rect_max_x = rect_pos.0 + rect_size.0;
    let rect_min_y = rect_pos.1;
    let rect_max_y = rect_pos.1 + rect_size.1;

    aabb_max_x > rect_min_x
        && aabb_min_x < rect_max_x
        && aabb_max_y > rect_min_y
        && aabb_min_y < rect_max_y
}

/// AABB vs Ellipse collision (simplified to circle)
pub fn check_aabb_ellipse_collision(
    aabb_pos: &DbVector2,
    aabb_size: &DbVector2,
    ellipse_center: (f32, f32),
    ellipse_radius: (f32, f32),
) -> bool {
    // Find the closest point on the AABB to the ellipse center
    let aabb_min_x = aabb_pos.x - aabb_size.x / 2.0;
    let aabb_max_x = aabb_pos.x + aabb_size.x / 2.0;
    let aabb_min_y = aabb_pos.y - aabb_size.y / 2.0;
    let aabb_max_y = aabb_pos.y + aabb_size.y / 2.0;

    let closest_x = ellipse_center.0.clamp(aabb_min_x, aabb_max_x);
    let closest_y = ellipse_center.1.clamp(aabb_min_y, aabb_max_y);

    // Calculate distance from closest point to ellipse center
    let dx = (closest_x - ellipse_center.0) / ellipse_radius.0;
    let dy = (closest_y - ellipse_center.1) / ellipse_radius.1;

    // Check if distance is less than radius (normalized)
    (dx * dx + dy * dy) < 1.0
}

/// AABB vs Point collision
pub fn check_aabb_point_collision(
    aabb_pos: &DbVector2,
    aabb_size: &DbVector2,
    point: (f32, f32),
) -> bool {
    let aabb_min_x = aabb_pos.x - aabb_size.x / 2.0;
    let aabb_max_x = aabb_pos.x + aabb_size.x / 2.0;
    let aabb_min_y = aabb_pos.y - aabb_size.y / 2.0;
    let aabb_max_y = aabb_pos.y + aabb_size.y / 2.0;

    point.0 >= aabb_min_x && point.0 <= aabb_max_x && point.1 >= aabb_min_y && point.1 <= aabb_max_y
}

/// AABB vs Polygon collision using Separating Axis Theorem (SAT)
pub fn check_aabb_polygon_collision(
    aabb_pos: &DbVector2,
    aabb_size: &DbVector2,
    polygon_points: &[Vector2],
) -> bool {
    if polygon_points.len() < 3 {
        return false;
    }

    // Convert AABB to polygon corners
    let half_width = aabb_size.x / 2.0;
    let half_height = aabb_size.y / 2.0;
    let aabb_corners = [
        Vector2::new(aabb_pos.x - half_width, aabb_pos.y - half_height),
        Vector2::new(aabb_pos.x + half_width, aabb_pos.y - half_height),
        Vector2::new(aabb_pos.x + half_width, aabb_pos.y + half_height),
        Vector2::new(aabb_pos.x - half_width, aabb_pos.y + half_height),
    ];

    // Test AABB axes
    if !sat_test_axis(Vector2::new(1.0, 0.0), &aabb_corners, polygon_points) {
        return false;
    }
    if !sat_test_axis(Vector2::new(0.0, 1.0), &aabb_corners, polygon_points) {
        return false;
    }

    // Test polygon edges as axes
    for i in 0..polygon_points.len() {
        let p1 = polygon_points[i];
        let p2 = polygon_points[(i + 1) % polygon_points.len()];
        let edge = Vector2::new(p2.x - p1.x, p2.y - p1.y);
        let axis = Vector2::new(-edge.y, edge.x).normalized();

        if !sat_test_axis(axis, &aabb_corners, polygon_points) {
            return false;
        }
    }

    true
}

/// SAT test for a single axis
fn sat_test_axis(axis: Vector2, poly1: &[Vector2], poly2: &[Vector2]) -> bool {
    let (mut min1, mut max1) = project_polygon(poly1, &axis);
    let (mut min2, mut max2) = project_polygon(poly2, &axis);

    // Add small epsilon for floating point comparison
    const EPSILON: f32 = 0.001;
    min1 -= EPSILON;
    max1 += EPSILON;
    min2 -= EPSILON;
    max2 += EPSILON;

    // Check for overlap
    max1 >= min2 && max2 >= min1
}

/// Project polygon onto axis
fn project_polygon(points: &[Vector2], axis: &Vector2) -> (f32, f32) {
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    for point in points {
        let projection = point.x * axis.x + point.y * axis.y;
        min = min.min(projection);
        max = max.max(projection);
    }

    (min, max)
}

/// Check collision against any shape with tile offset
pub fn check_shape_collision(
    aabb_pos: &DbVector2,
    aabb_size: &DbVector2,
    shape: &CollisionShape,
    tile_pos: &DbVector2,
) -> bool {
    match shape {
        CollisionShape::Rectangle { x, y, width, height } => {
            check_aabb_rect_collision(
                aabb_pos,
                aabb_size,
                (tile_pos.x + x, tile_pos.y + y),
                (*width, *height),
            )
        }
        CollisionShape::Ellipse { x, y, rx, ry } => {
            check_aabb_ellipse_collision(
                aabb_pos,
                aabb_size,
                (tile_pos.x + x, tile_pos.y + y),
                (*rx, *ry),
            )
        }
        CollisionShape::Point { x, y } => {
            check_aabb_point_collision(aabb_pos, aabb_size, (tile_pos.x + x, tile_pos.y + y))
        }
        CollisionShape::Polygon { points } => {
            let offset_points: Vec<Vector2> = points
                .iter()
                .map(|p| Vector2::new(p.x + tile_pos.x, p.y + tile_pos.y))
                .collect();
            check_aabb_polygon_collision(aabb_pos, aabb_size, &offset_points)
        }
        CollisionShape::Polyline { points } => {
            // Treat polyline as a series of line segments
            for i in 0..points.len().saturating_sub(1) {
                let p1 = Vector2::new(points[i].x + tile_pos.x, points[i].y + tile_pos.y);
                let p2 = Vector2::new(points[i + 1].x + tile_pos.x, points[i + 1].y + tile_pos.y);

                if check_aabb_line_collision(aabb_pos, aabb_size, p1, p2) {
                    return true;
                }
            }
            false
        }
    }
}

/// AABB vs Line segment collision
fn check_aabb_line_collision(
    aabb_pos: &DbVector2,
    aabb_size: &DbVector2,
    line_start: Vector2,
    line_end: Vector2,
) -> bool {
    // Simple implementation: check if either endpoint is in AABB or if line intersects AABB edges
    if check_aabb_point_collision(aabb_pos, aabb_size, (line_start.x, line_start.y))
        || check_aabb_point_collision(aabb_pos, aabb_size, (line_end.x, line_end.y))
    {
        return true;
    }

    // Check intersection with AABB edges (simplified)
    let half_width = aabb_size.x / 2.0;
    let half_height = aabb_size.y / 2.0;
    let aabb_min_x = aabb_pos.x - half_width;
    let aabb_max_x = aabb_pos.x + half_width;
    let aabb_min_y = aabb_pos.y - half_height;
    let aabb_max_y = aabb_pos.y + half_height;

    // Test against each edge of AABB
    line_intersects_segment(line_start, line_end, Vector2::new(aabb_min_x, aabb_min_y), Vector2::new(aabb_max_x, aabb_min_y))
        || line_intersects_segment(line_start, line_end, Vector2::new(aabb_max_x, aabb_min_y), Vector2::new(aabb_max_x, aabb_max_y))
        || line_intersects_segment(line_start, line_end, Vector2::new(aabb_max_x, aabb_max_y), Vector2::new(aabb_min_x, aabb_max_y))
        || line_intersects_segment(line_start, line_end, Vector2::new(aabb_min_x, aabb_max_y), Vector2::new(aabb_min_x, aabb_min_y))
}

/// Check if two line segments intersect
fn line_intersects_segment(p1: Vector2, p2: Vector2, p3: Vector2, p4: Vector2) -> bool {
    let d1 = direction(p3, p4, p1);
    let d2 = direction(p3, p4, p2);
    let d3 = direction(p1, p2, p3);
    let d4 = direction(p1, p2, p4);

    if ((d1 > 0.0 && d2 < 0.0) || (d1 < 0.0 && d2 > 0.0))
        && ((d3 > 0.0 && d4 < 0.0) || (d3 < 0.0 && d4 > 0.0))
    {
        return true;
    }

    false
}

/// Helper for line intersection
fn direction(p1: Vector2, p2: Vector2, p3: Vector2) -> f32 {
    (p3.x - p1.x) * (p2.y - p1.y) - (p2.x - p1.x) * (p3.y - p1.y)
}

/// Resolve collision by pushing AABB out of shape
pub fn resolve_collision(
    aabb_pos: &mut DbVector2,
    aabb_size: &DbVector2,
    shape: &CollisionShape,
    tile_pos: &DbVector2,
) {
    // Simple resolution: push away from collision center
    match shape {
        CollisionShape::Rectangle { x, y, width, height } => {
            let rect_center_x = tile_pos.x + x + width / 2.0;
            let rect_center_y = tile_pos.y + y + height / 2.0;

            let dx = aabb_pos.x - rect_center_x;
            let dy = aabb_pos.y - rect_center_y;

            // Calculate penetration depth
            let overlap_x = (aabb_size.x / 2.0 + width / 2.0) - dx.abs();
            let overlap_y = (aabb_size.y / 2.0 + height / 2.0) - dy.abs();

            // Resolve along axis of least penetration
            if overlap_x < overlap_y {
                aabb_pos.x += if dx > 0.0 { overlap_x } else { -overlap_x };
            } else {
                aabb_pos.y += if dy > 0.0 { overlap_y } else { -overlap_y };
            }
        }
        CollisionShape::Ellipse { x, y, .. } => {
            let ellipse_center_x = tile_pos.x + x;
            let ellipse_center_y = tile_pos.y + y;

            let dx = aabb_pos.x - ellipse_center_x;
            let dy = aabb_pos.y - ellipse_center_y;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist > 0.0 {
                let push_amount = 2.0; // Simple constant push
                aabb_pos.x += (dx / dist) * push_amount;
                aabb_pos.y += (dy / dist) * push_amount;
            }
        }
        _ => {
            // For other shapes, use simplified resolution
            // Push up (assume collision from above for platforms)
            aabb_pos.y += 2.0;
        }
    }
}
