//! Command infrastructure for modular Bevy editors.
//!
//! This crate provides the undo/redo history resource and the [`EditorCommand`]
//! trait used across the workspace. Backends and applications can register
//! their own commands while relying on this shared implementation.

mod history;

pub use history::{handle_undo_redo_shortcuts, EditorCommand, EditorHistory, HistoryStats};
