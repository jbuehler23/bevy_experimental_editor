use bevy::prelude::*;

/// Shared panel configuration for the scene tree view.
#[derive(Resource, Debug)]
pub struct SceneTreePanelState {
    pub visible: bool,
    pub width: f32,
}

impl Default for SceneTreePanelState {
    fn default() -> Self {
        Self {
            visible: true,
            width: 250.0,
        }
    }
}

impl SceneTreePanelState {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Shared state for the CLI output panel.
#[derive(Resource, Debug)]
pub struct CliOutputPanelState {
    pub visible: bool,
    pub auto_scroll: bool,
}

impl Default for CliOutputPanelState {
    fn default() -> Self {
        Self {
            visible: false,
            auto_scroll: true,
        }
    }
}

impl CliOutputPanelState {
    pub fn new() -> Self {
        Self::default()
    }
}
