# User Guide

This document walks through the day-to-day editor workflow after the modular refactor.

_Last updated: 2025-10-16_

---

## Launching the Editor

```bash
cargo run -p bevy_editor
```

At first launch you will be prompted to either **Create Project** or **Open Project**.  
The workspace file is stored under `~/.bevy_experimental_editor/workspace.json`.

---

## Workspace Overview

| Panel | Purpose | Tips |
| ----- | ------- | ---- |
| **Viewport** | Interactive scene view with gizmos | Q/W/E to toggle Move/Rotate/Scale, hold middle mouse to pan, mouse wheel to zoom |
| **Scene Tabs** | Each tab represents an `OpenScene` | Unsaved changes are snapshotted when switching tabs; save with Ctrl+S |
| **Scene Tree (left)** | Hierarchical list of entities | Double-click renames in the inspector; drag-and-drop reparenting coming soon |
| **Inspector (right)** | Component editing | Sprite textures can be picked via file dialog (project-relative paths) |
| **Asset Browser** | Project assets grouped by type | Double-click an image to assign it to the selected sprite |
| **Project Browser** | File system view of the workspace | Use toolbar to refresh, add folders, or open in Explorer/Finder |
| **CLI Output** | Captures `cargo`/`bevy` commands | Toolbar buttons run Build/Run/Web/Lint; Stop terminates the active command |
| **Tilemap Panels** | Layer list, tileset preview, collision tools | Load a tileset, select a layer, then pick a brush mode |

---

## Core Workflows

### Creating & Saving Scenes

1. Use the **Scene Tabs** bar to add a new scene or switch between existing ones.
2. The current tab’s state lives in `OpenScenes`. When you switch tabs the editor:
   - Snapshots the current world into an in-memory `DynamicScene`
   - Restores the target tab’s runtime snapshot (or loads from disk)
3. Save with **Ctrl+S** to write the scene to disk (`.scn.ron`).
4. The tab label displays `* SceneName` while unsaved.

### Editing Entities

1. **Add** entities via the Scene Tree “Add Entity” menu (Empty, Sprite, Camera2D, Button, Text).
2. Use gizmos to move, rotate, or scale (Q/W/E).
3. The Inspector lets you tweak:
   - `Transform`, `Visibility`, `Sprite`, UI `Node`, `Text`, etc.
   - Sprite textures (`Assign Texture` button opens a picker rooted at the project assets folder).
4. Double-click any property field to edit the value, press Enter to commit.

### Asset Management

- The **Asset Browser** scans textures beneath the project’s `assets/` directory.
- Click **Refresh** to rescan; double-click an image to assign it to the currently selected sprite.
- The **Project Browser** shows the raw file hierarchy, allowing new folders and deletion.

### Tilemap Tooling

1. Load a tileset (`Load Tileset` button) and select a layer in the **Layer** panel.
2. Choose a brush mode (single, rectangle, line, bucket, stamp).
3. Paint directly onto the viewport – the cursor snaps to the grid.
4. Use the collision editor to define per-tile collision shapes.

### CLI Integration

- The toolbar provides `Run`, `Web`, `Build`, `Lint`, and `Stop` buttons.
- Output appears in the CLI panel; the panel auto-opens when a command starts.
- Commands are sent through `EditorAction::RunProjectCommand` and surfaced back via `EditorEvent`.

---

## Keyboard Shortcuts

| Shortcut | Action |
| -------- | ------ |
| Ctrl + S | Save current scene |
| Ctrl + O | Open scene (file dialog) |
| Ctrl + Shift + S | Save As |
| Ctrl + Z / Ctrl + Shift + Z | Undo / Redo |
| Ctrl + Y | Redo (alternative) |
| Q / W / E | Move / Rotate / Scale gizmo |
| V / I / E | Select / Eyedropper / Erase tool |
| G | Toggle grid snap |
| Alt + drag | Temporary eyedropper |

---

## Extending the Editor

- Implement `EditorFrontend` to create a new UI. Feed actions/events through `bevy_editor_frontend_api`.
- Embed only the backends you need by depending on the relevant crates (scene/project/assets/tilemap).
- Consult [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) for crate responsibilities and layering guidelines.

---

## Troubleshooting

| Issue | Resolution |
| ----- | ---------- |
| Scene disappears after tab switch | Ensure you built with the latest `cache_runtime_scene_on_scene_switch` logic (`bevy_editor_app`). |
| Sprites load blank textures | Inspector assigns project-relative paths; confirm the file exists under the project’s `assets/` directory. |
| CLI commands fail | Check that the project has a valid Cargo workspace; ensure `bevy` CLI is installed if using template commands. |

---

Happy editing! Contributions are welcome – see the root `README.md` for setup and testing instructions.
