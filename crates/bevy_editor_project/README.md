# bevy_editor_project

Project and workspace management for the modular Bevy editor. This crate
collects everything required to create, open, and track Bevy game projects from
editor shells or headless tooling.

## Features

| Feature      | Description                                                     |
|--------------|-----------------------------------------------------------------|
| `workspace`  | Loads and persists editor workspace metadata (recent projects). |
| `cli`        | Provides the threaded Bevy CLI runner and output buffering.     |
| `ui`         | Enables egui-powered project selection UI and wizard flows.     |

The default feature set enables `workspace` and `ui`. Turn features off/on to
build bespoke editor shells.

## Plugins

- `ProjectManagerPlugin`: full project stack (workspace loader, CLI runner, UI
  state). Use this in UI-driven editor apps.
- `ProjectCorePlugin`: headless subset (no egui UI). Useful for tests, CLI
  utilities, or alternate front-ends.

Both plugins register the shared `ProjectManagerSet` system set so applications
can order their own systems relative to the project lifecycle.

## Quick Start

```rust
use bevy::prelude::*;
use bevy_editor_project::ProjectManagerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ProjectManagerPlugin)
        .run();
}
```

### Headless / Testing

```rust
use bevy::prelude::*;
use bevy_editor_project::{ProjectCorePlugin, ProjectSelection};

fn main() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(ProjectCorePlugin);

    // Drive startup systems once (loads workspace if enabled).
    app.update();

    assert!(app.world().contains_resource::<ProjectSelection>());
}
```

## Integration Notes

- When pairing with the legacy `ProjectBrowser` panel from `bevy_editor`, call
  the helper in `bevy_editor::project_browser::sync_project_browser_root` to
  keep the browser aligned with the active project.
- The CLI runner spawns native processes; ensure your editor exposes controls to
  stop long-running jobs and display buffered output (see the egui toolbar for
  reference).

## Roadmap

- Split UI concerns into the future `bevy_editor_ui_egui` crate (Phase 5).
- Provide richer template facilities once asset/project crates are extracted.
