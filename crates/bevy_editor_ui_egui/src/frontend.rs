use bevy::prelude::*;
use bevy_editor_frontend_api::{EditorFrontend, FrontendCapabilities, FrontendKind};

use crate::EditorUiEguiPlugin;

/// Frontend descriptor for the egui-based editor UI.
#[derive(Clone, Copy, Default)]
pub struct EguiFrontend;

impl EditorFrontend for EguiFrontend {
    fn id(&self) -> &'static str {
        "egui"
    }

    fn kind(&self) -> FrontendKind {
        FrontendKind::Gui
    }

    fn capabilities(&self) -> FrontendCapabilities {
        FrontendCapabilities {
            requires_cli: true,
            supports_multiple_viewports: false,
        }
    }

    fn install(&self, app: &mut App) {
        app.add_plugins(EditorUiEguiPlugin);
    }
}
