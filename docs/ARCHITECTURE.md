# Architecture

How the editor is organized and why.

---

## Design Goals

The editor is built from independent crates so you can:
- Use just the scene backend in your game tools
- Swap out the egui UI for native Bevy UI (feathers!!!!) or a web frontend
- Run headless tests without pulling in UI dependencies
- Enable/disable features like tilemap support via cargo features

Each crate is a Bevy plugin. Wire up the ones you need, skip the rest.

---

## Crate Map

```
foundation → core → backends → frontend API → UI → app shell
```

### Foundation (`bevy_editor_foundation`)
Core types and enums used everywhere. No heavy dependencies.

- `EditorState` - Active tool, selection, grid snap
- `EditorTool` - Enum for Select, Move, Paint, etc.
- Action/event types for the frontend bridge

### Core (`bevy_editor_core`)
Reusable editor systems that don't depend on a specific UI.

- Camera controls (pan, zoom)
- Selection and gizmos (move/rotate/scale)
- Keyboard shortcut handling

### Backends

| Crate | What It Does |
| ----- | ------------ |
| `bevy_editor_scene` | Multi-tab scene state, runtime snapshots, save/load |
| `bevy_editor_assets` | Asset scanning, texture helpers |
| `bevy_editor_project` | Workspace metadata, Bevy CLI integration |
| `bevy_editor_tilemap` | Tile layers, painting tools, collision editor |
| `bevy_editor_commands` | Undo/redo history |

Each backend is headless - no UI code. They expose Bevy resources and events.

### Frontend API (`bevy_editor_frontend_api`)
Shared contracts for building UIs.

- View models (scene tree data, inspector fields, panel state)
- `EditorFrontend` trait
- `EditorAction` / `EditorEvent` enums

The egui frontend implements this trait, but you could write your own (native Bevy UI, web, etc.).

### UI (`bevy_editor_ui_egui`)
egui-based panels and widgets. Reads view models from the frontend API, emits actions/events.

- Scene tree panel
- Inspector panel
- Asset/project browsers
- Tilemap panels
- CLI output

### App Shell (`bevy_editor_app`)
Thin glue layer that wires up all the plugins. The root `bevy_editor` binary just runs this with the egui frontend enabled.

---

## How Scene Tabs Work

Switching tabs:
1. UI emits `SceneTabChanged` event
2. Backend snapshots the current scene into memory
3. Backend loads the target tab (from disk or cached snapshot)
4. Entities get tagged with `EditorSceneEntity`
5. UI refreshes to show the new hierarchy

Unsaved changes stay in memory until you hit Ctrl+S. This lets you switch tabs without losing work.

---

## Using Just the Backends

Example: embed scene management in your game without the editor UI.

```toml
[dependencies]
bevy = "0.16"
bevy_editor_scene = { path = "path/to/crates/bevy_editor_scene" }
```

```rust
use bevy::prelude::*;
use bevy_editor_scene::SceneEditorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SceneEditorPlugin)
        .run();
}
```

The backend tracks `OpenScenes` and handles save/load. You can query it from your own systems or build a custom UI on top.

---

## Extending the Editor

**Add a new backend:** Create a crate with a plugin and resource. Expose events for the frontend to consume.

**Add a new frontend:** Implement the `EditorFrontend` trait. Subscribe to `EditorEvent`, emit `EditorAction`.

**Add a new panel:** Add it to `bevy_editor_ui_egui` (or your custom UI crate). Read from the frontend API's view models.

---

## File Layout

Each crate has:
- `README.md` - Purpose and quick start
- `Cargo.toml` - Features and dependencies
- `src/lib.rs` - Public API and plugin definition

The root workspace ties everything together. CI runs `cargo check --workspace` to ensure all crates build independently.

---

## Next Steps

- [`USER_GUIDE.md`](USER_GUIDE.md) - How to use the editor
- [`EDITOR_ROADMAP.md`](EDITOR_ROADMAP.md) - Planned features
- Individual crate READMEs for API details
