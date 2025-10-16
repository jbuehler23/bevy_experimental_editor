//! # Bevy Editor Foundation
//!
//! Shared data types, traits, and lightweight plugins used across the modular
//! editor workspace. This crate intentionally keeps dependencies minimal so
//! downstream projects can reuse the core definitions without pulling in UI or
//! backend-specific logic.
//!
//! ## Modules
//!
//! - [`state`]: Core editor resources such as [`EditorState`] and
//!   [`EditorTool`].
//! - [`palette`]: Generic selection palettes that editor UIs can reuse.
//! - [`selection`]: Shared entity selection resource.

pub mod palette;
pub mod selection;
pub mod state;

pub use palette::EditorPalette;
pub use selection::Selection;
pub use state::{EditorState, EditorStatePlugin, EditorTool};
