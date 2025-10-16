use bevy::app::App;
use bevy::MinimalPlugins;
use bevy_editor_app::EditorAppPlugin;
use bevy_editor_frontend_api::{EditorFrontend, FrontendCapabilities, FrontendKind};

#[derive(Clone, Default)]
struct HeadlessFrontend;

impl EditorFrontend for HeadlessFrontend {
    fn id(&self) -> &'static str {
        "headless"
    }

    fn kind(&self) -> FrontendKind {
        FrontendKind::Headless
    }

    fn capabilities(&self) -> FrontendCapabilities {
        FrontendCapabilities::default()
    }

    fn install(&self, _app: &mut App) {
        // No UI systems; headless tests can wire their own listeners here.
    }
}

fn main() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(EditorAppPlugin::new(HeadlessFrontend::default()));
}
