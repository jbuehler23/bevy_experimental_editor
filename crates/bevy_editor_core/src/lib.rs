//! # Bevy Editor Core
//!
//! Core editor primitives for building Bevy editors.
//!
//! This crate provides reusable building blocks for any Bevy editor:
//! - **Camera**: Editor camera with pan and zoom controls
//! - **Selection**: Entity selection system with multi-select support
//! - **Gizmos**: Transform gizmo modes (Move, Rotate, Scale)
//! - **History**: Undo/redo system with command pattern
//! - **Shortcuts**: Keyboard shortcut management
//!
//! ## Example
//!
//! ```rust,no_run
//! use bevy::prelude::*;
//! use bevy_editor_core::*;
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         .add_plugins((
//!             EditorCameraPlugin,
//!             SelectionPlugin,
//!             GizmoPlugin,
//!         ))
//!         .run();
//! }
//! ```

pub mod camera;
pub mod gizmos;
pub mod selection;
pub mod editor_history;
pub mod shortcuts;

// Re-export commonly used types
pub use camera::{EditorCamera, camera_pan_system, camera_zoom_system};
pub use selection::{Selectable, Selection, SelectionEvent, SelectionPlugin, handle_2d_selection_system};
pub use gizmos::{GizmoMode, GizmoState, GizmoPlugin, handle_gizmo_mode_shortcuts};
pub use editor_history::{EditorCommand, EditorHistory};
pub use shortcuts::{KeyboardShortcut, ShortcutRegistry};

/// Convenience plugin that adds all editor core systems
pub struct EditorCorePlugin;

impl bevy::app::Plugin for EditorCorePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins((
            SelectionPlugin,
            GizmoPlugin,
        ));
    }
}

/// Plugin for editor camera controls
pub struct EditorCameraPlugin;

impl bevy::app::Plugin for EditorCameraPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(bevy::app::Update, (camera::camera_pan_system, camera::camera_zoom_system));
    }
}
