//! Frontend contracts shared by modular Bevy editor user interfaces.
//!
//! The types in this crate encode UI-agnostic view models, events, and helper
//! traits that backend crates can populate while individual frontends decide
//! how to render them.

pub mod asset_browser;
pub mod frontend;
pub mod inspector;
pub mod panels;
pub mod project_browser;
pub mod scene_tree;

pub use asset_browser::AssetBrowserPanelState;
pub use frontend::{
    EditorAction, EditorEvent, EditorFrontend, EditorPanel, FrontendCapabilities, FrontendKind,
    ProjectCommand,
};
pub use inspector::{EntityComponentData, InspectorPanelState};
pub use panels::{CliOutputPanelState, SceneTreePanelState};
pub use project_browser::ProjectBrowserPanelState;
pub use scene_tree::{SceneEntityTemplate, SceneTreeCommand, SceneTreeNode};
