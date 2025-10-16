use bevy::prelude::*;

/// UI-agnostic state for the asset browser panel.
#[derive(Resource, Debug)]
pub struct AssetBrowserPanelState {
    pub visible: bool,
    pub thumbnail_size: f32,
    pub columns: usize,
}

impl Default for AssetBrowserPanelState {
    fn default() -> Self {
        Self {
            visible: true,
            thumbnail_size: 64.0,
            columns: 4,
        }
    }
}

impl AssetBrowserPanelState {
    pub fn new() -> Self {
        Self::default()
    }
}
