# Bevy Experimental Editor

A VERY experimental scene editor for [Bevy](https://bevyengine.org/) games.

![Create Project](docs/screenshots/create_project.png)

---

## What's Here (so far!)

### Scene Editing
Click and drag to place entities, use gizmos to move/rotate/scale. The inspector lets you tweak transforms, sprites, and UI components. Undo/redo works, and you can have multiple scenes open in tabs.

### Asset Browser
![Asset Browser](docs/screenshots/asset_browser.png)

Drag textures onto sprites. The browser scans your project's `assets/` folder and lets you preview images before assigning them.

### Tilemap Tools - needs work
Load a tileset PNG, pick a layer, and paint. Includes brush, rectangle, line, and fill tools. You can also edit per-tile collision shapes.

### CLI Integration
![Terminal](docs/screenshots/cli.png)

Run `cargo build` or `bevy run` from the toolbar. Output shows up in a terminal-style panel with color-coded compilation progress.

---

## Quick Start

```bash
git clone https://github.com/jbuehler23/bevy_experimental_editor.git
cd bevy_experimental_editor
cargo run -p bevy_editor
```

First launch prompts you to create or open a project. The editor creates a workspace file at `~/.bevy_experimental_editor/workspace.json` to remember your recent projects.

### Running Your Game
![Run Game](docs/screenshots/run_game.png)

Click the **Run** button in the toolbar. The editor builds your project and launches it in a new window. You can stop it anytime from the CLI panel.

---

## What's Working (kinda)

- **Scene editing**: Multi-tab workspace, transform gizmos, entity hierarchy
- **Inspector**: Edit common components (Transform, Sprite, UI nodes)
- **Asset browser**: Texture preview and drag-to-assign
- **Tilemap painting**: Brush/fill/line/rectangle tools with layers
- **CLI panel**: Run builds, see output, stop processes
- **Project templates**: Generate starter projects with the wizard
- **Keyboard shortcuts**: Ctrl+S to save, Q/W/E for gizmos, Ctrl+Z for undo

## What's Planned

- **Prefab system**: Save entity groups as reusable templates
- **Animation editor**: Keyframe animations directly in the editor
- **Physics gizmos**: Visual collision shape editing
- **Play mode**: Test your game without leaving the editor
- **Script hooks**: Attach custom behaviors to entities
- **UI builder**: Visual layout tools for Bevy UI

Full roadmap: [`docs/EDITOR_ROADMAP.md`](docs/EDITOR_ROADMAP.md)

---

## Building from Source

**Requirements:**
- Rust 1.70 or newer
- GPU drivers supporting Vulkan, Metal, or DirectX 11+

**Clone and run:**
```bash
git clone https://github.com/<org>/bevy-experimental-editor.git
cd bevy-experimental-editor
cargo run -p bevy_editor
```

**Faster dev builds:**
```bash
# Use dynamic linking for faster iteration
cargo run -p bevy_editor --features bevy/dynamic_linking
```

---

## Using the Editor

Check [`docs/USER_GUIDE.md`](docs/USER_GUIDE.md) for a walkthrough. Key shortcuts:

| Shortcut | Action |
| -------- | ------ |
| Ctrl+S | Save scene |
| Ctrl+Z / Ctrl+Shift+Z | Undo / Redo |
| Q / W / E | Move / Rotate / Scale gizmo |
| G | Toggle grid snap |
| Middle mouse + drag | Pan viewport |
| Mouse wheel | Zoom |

---

## Modular Design

The editor is split into independent crates so you can embed just the parts you need:

- `bevy_editor_scene` - Scene management and persistence
- `bevy_editor_tilemap` - Tile painting and layer system
- `bevy_editor_project` - Project workspace and CLI integration
- `bevy_editor_assets` - Asset scanning and browser state
- `bevy_editor_ui_egui` - egui-based frontend (swappable)

Example of using just the scene backend:

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

See [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) for details on the crate structure.

---

## Contributing

Pull requests welcome! Please:
- Run `cargo fmt` and `cargo clippy` before submitting
- Include tests for new features
- Update docs if you add public APIs

Open an issue first for larger changes so we can discuss the approach.

---

## License

Dual-licensed under MIT or Apache 2.0, your choice.

- [MIT](LICENSE-MIT)
- [Apache 2.0](LICENSE-APACHE)

---

## Credits

Built with:
- [Bevy](https://bevyengine.org/) - Game engine
- [egui](https://github.com/emilk/egui) - UI framework (via bevy_egui)
- [bevy-inspector-egui](https://github.com/jakobhellermann/bevy-inspector-egui) - Component inspector
- [bevy_ecs_tilemap](https://github.com/StarArawn/bevy_ecs_tilemap) - Tilemap rendering

Thanks to the Bevy community for feedback and testing!
