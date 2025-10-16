# Bevy Experimental Editor

Modular scene, tilemap, and project tooling for [Bevy](https://bevyengine.org/) 0.16.0.  
The workspace packages the editor into opt-in crates so you can run the full egui experience, embed only the scene state systems, or assemble your own shell around the shared backends.

> **Status:** active development & refactor complete. API stability is not guaranteed.

---

## Quick Links

- **User Guide:** [`docs/USER_GUIDE.md`](docs/USER_GUIDE.md)
- **Architecture Overview:** [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md)
- **Refactor journal:** [`docs/REFACTOR_PROGRESS.md`](docs/REFACTOR_PROGRESS.md)
- **Roadmap:** [`docs/EDITOR_ROADMAP.md`](docs/EDITOR_ROADMAP.md)

---

## Feature Highlights

- **Scene authoring**
  - Multi-tab scene workspace with automatic runtime snapshots
  - Transform gizmos (Q/W/E), selection marquee, and undoable commands
  - Inspector for common components (Name, Transform, Sprite, UI nodes)

- **Project & asset management**
  - Workspace picker, Bevy CLI integration, and template generation
  - Asset browser with rescan support and sprite drop-to-assign
  - CLI output panel (build/run/lint) wired through the shared action bus

- **Tilemap tooling**
  - Brush, rectangle, stamp, line, and bucket tools backed by the tilemap crate
  - Tileset panel, layer management, collision editor

- **Frontend-backend separation**
  - `EditorFrontend` trait plus `EditorAction` / `EditorEvent` bus
  - `bevy_editor_frontend_api` view models reused across frontend crates
  - `bevy_editor_app` is just a shell: swap in your own frontend or omit egui entirely

- **Modular crates**
  - Each subsystem (`scene`, `project`, `assets`, `tilemap`, `foundation`, etc.) ships as its own Bevy plugin
  - Headless tests for backends, frontend-specific code isolated in UI crates

---

## Workspace Overview

| Crate | Purpose |
| ----- | ------- |
| `bevy_editor_foundation` | Shared editor state (`EditorState`, tool enums, action events) with zero heavy deps |
| `bevy_editor_core` | Camera, selection, gizmo systems, and shortcut helpers |
| `bevy_editor_commands` | Undo/redo history and command traits |
| `bevy_editor_scene` | Multi-scene state (`OpenScenes`), runtime snapshots, scene persistence |
| `bevy_editor_project` | Workspace metadata, Bevy CLI orchestration, template generation |
| `bevy_editor_assets` | Asset scanning, browser state, and texture helpers |
| `bevy_editor_tilemap` | Tile layers, painting tools, tileset management, collision editor |
| `bevy_editor_frontend_api` | View models, panel state, action/event definitions shared by frontends |
| `bevy_editor_ui_egui` | egui-based frontend implementing `EditorFrontend` |
| `bevy_editor_app` | Thin application shell wiring the plugins together (default binary) |

The root package (`bevy_editor`) simply depends on `bevy_editor_app` and the egui frontend to provide the out-of-the-box editor.

---

## Getting Started

### Prerequisites

- Rust **1.70+** (stable)
- Bevy-compatible graphics drivers (Vulkan/Metal/DirectX11+)
- Windows, Linux, or macOS

### Run the bundled editor

```bash
git clone https://github.com/<org>/bevy-experimental-editor.git
cd bevy-experimental-editor

# Run the egui shell
cargo run -p bevy_editor
```

The first launch initialises a workspace file at `~/.bevy_experimental_editor/workspace.json`.  
From there you can create a project, open an existing Bevy project, or explore the sample workspace.

### Build your own shell

Want to embed the scene/project backends inside your game tools?

```toml
[dependencies]
bevy = "0.16"
bevy_editor_scene = { path = "crates/bevy_editor_scene" }
bevy_editor_project = { path = "crates/bevy_editor_project" }
# Add whichever plugins you need…
```

```rust
use bevy::prelude::*;
use bevy_editor_project::ProjectManagerPlugin;
use bevy_editor_scene::SceneEditorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((ProjectManagerPlugin, SceneEditorPlugin))
        .run();
}
```

Frontends implement the `EditorFrontend` trait and are fed `EditorAction` / `EditorEvent` enums through the shared API. Refer to `bevy_editor_ui_egui` for a full implementation.

---

## Using the Editor

A condensed walkthrough is in [`docs/USER_GUIDE.md`](docs/USER_GUIDE.md). Core workflows:

1. **Open or create a project** – the CLI panel will seed a template if required.
2. **Scene tabs** – each tab represents an `OpenScene`. Unsaved changes are snapshot to memory as you switch tabs; hitting **Ctrl+S** writes to disk.
3. **Viewport** – select entities (click), change gizmo mode (Q/W/E), pan with middle mouse, zoom with the wheel.
4. **Inspector** – edit component fields, pick sprite textures (from project-relative paths), manage UI nodes and transforms.
5. **Asset & project browsers** – double-click a texture to assign it to the selected sprite, or navigate the project tree.
6. **Tilemap tools** – choose a layer, load tilesets, and paint with the brush/rectangle/line/fill tools.
7. **CLI panel** – run `cargo run`, `bevy run`, web builds, or `bevy lint` directly from the toolbar.

Keyboard shortcuts:

| Action | Shortcut |
| ------ | -------- |
| Move / Rotate / Scale gizmo | Q / W / E |
| Toggle grid snap | G |
| Switch tool to Select / Eyedropper / Erase | V / I / E |
| Save scene | Ctrl+S |
| Undo / Redo | Ctrl+Z / Ctrl+Shift+Z |

---

## Development & Testing

```bash
# Format, lint, and test everything
cargo fmt --all
cargo clippy --workspace -- -D warnings
cargo test --workspace
```

The CI matrix (see `.github/workflows`) exercises the workspace in headless mode and under different feature sets. Add new crates with minimal features so `cargo check --all-targets` stays fast.

When hacking on the modular crates:

- Each crate has a concise `README.md` describing its purpose and public API.
- `bevy_editor_scene` includes unit tests for `OpenScenes`, tab snapshots, and runtime restoration.
- `bevy_editor_app` contains integration tests that validate the action bridge without spinning up egui.

---

## Contributing

Pull requests are welcome! Please:

1. Read [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) for the layering guidelines.
2. Update `docs/REFACTOR_PROGRESS.md` if you complete a refactor milestone.
3. Include tests or a manual verification checklist for new behaviour.
4. Run formatting and clippy before pushing.

Open an issue if you plan larger changes so we can align on the direction (asset pipeline, alternative frontends, etc.).

---

## License

Dual-licensed under either:

- [MIT](LICENSE-MIT) or
- [Apache 2.0](LICENSE-APACHE)

at your option.

---

## Acknowledgements

- [Bevy](https://bevyengine.org/) – ECS, rendering, and CLI tooling
- [bevy_egui](https://github.com/vladbat00/bevy_egui) – egui integration
- [bevy-inspector-egui](https://github.com/jakobhellermann/bevy-inspector-egui) – inspector widgets
- [bevy_ecs_tilemap](https://github.com/StarArawn/bevy_ecs_tilemap) – tilemap rendering

Thanks to the Bevy community for the feedback that shaped this refactor!
