use bevy::prelude::*;
use bevy_editor_project::{ProjectCorePlugin, ProjectSelection};

#[test]
fn project_core_plugin_initializes_without_ui() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(ProjectCorePlugin);

    // Run startup systems once to ensure resources register.
    app.update();

    assert!(
        app.world().contains_resource::<ProjectSelection>(),
        "ProjectSelection resource should be available after ProjectCorePlugin runs"
    );
}
