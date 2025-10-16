use bevy::prelude::*;
use bevy_editor_formats::{CollisionShape, Vector2};

/// Tools available for collision authoring.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollisionTool {
    Select,
    Rectangle,
    Ellipse,
    Polygon,
    Polyline,
    Point,
}

/// Backend collision authoring state shared with the UI layer.
#[derive(Resource)]
pub struct CollisionEditor {
    pub active: bool,
    pub current_tool: CollisionTool,
    pub current_tile_id: Option<u32>,
    pub shapes: Vec<CollisionShape>,
    pub selected_shape: Option<usize>,

    pub drawing: bool,
    pub drag_start: Option<Vec2>,
    pub polygon_points: Vec<Vector2>,
}

impl Default for CollisionEditor {
    fn default() -> Self {
        Self {
            active: false,
            current_tool: CollisionTool::Select,
            current_tile_id: None,
            shapes: Vec::new(),
            selected_shape: None,
            drawing: false,
            drag_start: None,
            polygon_points: Vec::new(),
        }
    }
}
