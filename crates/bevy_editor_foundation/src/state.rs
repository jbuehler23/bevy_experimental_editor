use bevy::prelude::*;

/// Primary editor configuration shared across plugins.
#[derive(Resource, Debug, Clone)]
pub struct EditorState {
    /// Currently active tool in the editor.
    pub current_tool: EditorTool,
    /// When enabled, placement snaps to a fixed-size grid.
    pub grid_snap_enabled: bool,
    /// Grid size used when snapping is active.
    pub grid_size: f32,
    /// Whether the editor is currently playing the game.
    pub is_playing: bool,
}

impl Default for EditorState {
    fn default() -> Self {
        Self {
            current_tool: EditorTool::default(),
            grid_snap_enabled: true,
            grid_size: 32.0,
            is_playing: false,
        }
    }
}

/// Set of tools exposed by the editor UI.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorTool {
    #[default]
    Select,
    Platform,
    EntityPlace,
    Erase,
    Eyedropper,
}

/// Plugin that registers [`EditorState`] as a resource.
pub struct EditorStatePlugin;

impl Plugin for EditorStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EditorState>();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_state_uses_select_tool() {
        let state = EditorState::default();
        assert_eq!(state.current_tool, EditorTool::Select);
        assert!(state.grid_snap_enabled);
        assert_eq!(state.grid_size, 32.0);
        assert!(!state.is_playing);
    }
}
