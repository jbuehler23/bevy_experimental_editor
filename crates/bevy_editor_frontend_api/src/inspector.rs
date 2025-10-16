use bevy::prelude::*;

/// Snapshot of components required to render the inspector for a single entity.
#[derive(Clone, Debug)]
pub struct EntityComponentData {
    pub entity: Entity,
    pub name: Option<String>,
    pub transform: Option<Transform>,
    pub visibility: Option<Visibility>,
    pub sprite: Option<Sprite>,
    pub has_camera2d: bool,
    pub node: Option<Node>,
    pub has_button: bool,
    pub text: Option<Text>,
}

impl EntityComponentData {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            name: None,
            transform: None,
            visibility: None,
            sprite: None,
            has_camera2d: false,
            node: None,
            has_button: false,
            text: None,
        }
    }
}

/// Persistent inspector panel configuration shared across frontends.
#[derive(Resource, Debug)]
pub struct InspectorPanelState {
    pub visible: bool,
    pub width: f32,
}

impl Default for InspectorPanelState {
    fn default() -> Self {
        Self {
            visible: true,
            width: 300.0,
        }
    }
}

impl InspectorPanelState {
    pub fn new() -> Self {
        Self::default()
    }
}
