//! Gizmo manipulation system for editors
//!
//! Provides common gizmo modes (Move, Rotate, Scale) and state management.
//! The actual gizmo drawing and interaction logic can be customized per editor.

use bevy::prelude::*;

/// Gizmo manipulation mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Default)]
pub enum GizmoMode {
    /// Move/translate gizmo (Q key by default)
    #[default]
    Move,
    /// Rotate gizmo (W key by default)
    Rotate,
    /// Scale gizmo (E key by default)
    Scale,
}

impl GizmoMode {
    pub fn display_name(&self) -> &str {
        match self {
            GizmoMode::Move => "Move",
            GizmoMode::Rotate => "Rotate",
            GizmoMode::Scale => "Scale",
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            GizmoMode::Move => "↔",   // Arrows
            GizmoMode::Rotate => "↻", // Rotation arrow
            GizmoMode::Scale => "⤢",  // Diagonal arrows
        }
    }
}

/// Resource to track current gizmo mode
#[derive(Resource, Default, Reflect)]
#[reflect(Resource, Default)]
pub struct GizmoState {
    pub mode: GizmoMode,
}

/// System to handle gizmo mode switching with keyboard shortcuts
/// Q - Move mode, W - Rotate mode, E - Scale mode (Unity/Godot/Blender style)
///
/// Note: You may want to add conditions to prevent this from triggering
/// when typing in text fields or when certain tools are active.
pub fn handle_gizmo_mode_shortcuts(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut gizmo_state: ResMut<GizmoState>,
) {
    // Q - Move mode
    if keyboard.just_pressed(KeyCode::KeyQ) {
        gizmo_state.mode = GizmoMode::Move;
        info!("Switched to Move gizmo mode");
    }
    // W - Rotate mode
    else if keyboard.just_pressed(KeyCode::KeyW) {
        gizmo_state.mode = GizmoMode::Rotate;
        info!("Switched to Rotate gizmo mode");
    }
    // E - Scale mode
    else if keyboard.just_pressed(KeyCode::KeyE) {
        gizmo_state.mode = GizmoMode::Scale;
        info!("Switched to Scale gizmo mode");
    }
}

/// Plugin to add gizmo system to your editor
pub struct GizmoPlugin;

impl Plugin for GizmoPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GizmoState>()
            .register_type::<GizmoMode>()
            .register_type::<GizmoState>();
    }
}
