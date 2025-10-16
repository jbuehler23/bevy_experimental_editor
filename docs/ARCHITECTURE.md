# Modular Editor Architecture

Shared reference for the crate layout, dependencies, and runtime flow of the modular editor workspace.

_Last updated: 2025-10-16_

---

## Principles

- **Layered crates:** shared data → domain backends → frontend contracts → UI implementations → application shells.
- **Opt-in dependencies:** egui, tilemap, CLI, and other heavy stacks live behind dedicated crates or features.
- **ECS boundaries:** communication happens via Bevy resources/events/commands rather than direct module access.
- **Frontend agnostic:** UIs render view-models from `bevy_editor_frontend_api` and emit `EditorAction`s.
- **Headless-friendly:** every backend runs without egui for automated tests or custom shells.

---

## Layer Map

```
foundation ───────► core primitives
      │                │
      ├───────────────┘
      ▼                ▼
domain backends    frontend api
      │                │
      ▼                ▼
 frontend impls    shared tooling
      │
      ▼
application shells
```

### Foundation (`bevy_editor_foundation`)
- `EditorState`, `EditorTool`, action/event enums, shared resource types.
- No optional dependencies; depends only on `bevy`.

### Core (`bevy_editor_core`)
- Editor camera systems, selection helpers, gizmo state, keyboard shortcuts.
- Provides plugin bundles (`EditorCameraPlugin`, `SelectionPlugin`, `GizmoPlugin`).

### Commands (`bevy_editor_commands`)
- Undo/redo history resource and command trait.
- Consumed by frontends and domain-specific command implementations.

### Domain Backends

| Crate | Responsibilities |
| ----- | ---------------- |
| `bevy_editor_scene` | Multi-scene state (`OpenScenes`), runtime snapshots, scene persistence, tab syncing |
| `bevy_editor_assets` | Asset browser state, texture helpers, rescan events |
| `bevy_editor_project` | Workspace metadata, Bevy CLI integration, template generation |
| `bevy_editor_tilemap` | Tile layer manager, painting tools, tileset palette, collision editor |

### Frontend API (`bevy_editor_frontend_api`)
- View models for scene tree, inspector, asset/project panels, CLI output, tilemap panels.
- `EditorFrontend` trait, `EditorAction` / `EditorEvent` enums, reusable panel state resources.

### Frontend Implementation (`bevy_editor_ui_egui`)
- egui-based UI that consumes the frontend API, renders view models, and emits actions/events.
- Additional frontends (native Bevy UI, iced, web) can follow the same contract.

### Application Shell (`bevy_editor_app`)
- `EditorAppPlugin` wires the foundation + domain plugins and exposes them for reuse.
- The binary crate (`bevy_editor`) merely composes `EditorAppPlugin::new(EguiFrontend::default())` and debug plugins.

---

## Scene Tab Flow

1. **Tab change initiated** – UI layer emits `SceneTabChanged`.
2. **Runtime snapshot** – `cache_runtime_scene_on_scene_switch` serialises the active scene into a `DynamicScene` asset, stores the handle on the corresponding `OpenScene`, and records `is_modified`.
3. **Scene swap** – `sync_editor_scene_on_tab_change` gathers existing editor entities (entity + optional `ChildOf` parent) and calls `bevy_editor_scene::sync_active_scene`.
4. **`sync_active_scene`**:
   - Clears only root entities of the previous scene.
   - Loads the new tab from disk if `file_path` is set; otherwise spawns the cached runtime scene; otherwise spawns a fresh `"Scene Root"` entity.
5. **Mark & clean up** – `mark_loaded_scene_entities` tags newly spawned entities with `EditorSceneEntity`, updates the active root, and removes the temporary `LoadingSceneRoot`.

Outcome: unsaved tabs retain hierarchy, names, and component state when you switch away and back.

---

## Refactor Milestones (Completed)

1. Extract shared foundation crate (`bevy_editor_foundation`).
2. Split undo/redo infrastructure (`bevy_editor_commands`).
3. Migrate scene/assets/project/tilemap systems into dedicated backends.
4. Introduce frontend contract crate (`bevy_editor_frontend_api`) and adapt egui UI.
5. Reduce application shell to `bevy_editor_app` (root binary is a thin wrapper).
6. Add runtime snapshotting and tab restoration (`cache_runtime_scene_on_scene_switch`, `sync_active_scene`).
7. Document the architecture, usage, and refactor progress.

Future work (tracked in `docs/EDITOR_ROADMAP.md`) covers additional backends, alternative frontends, and pipeline automation.

---

## Documentation Expectations

- Each crate ships a concise `README.md` covering purpose, features, and sample usage.
- Architectural decisions are logged under `docs/adr/` when needed.
- Stage-by-stage updates belong in `docs/REFACTOR_PROGRESS.md`.
- Root documentation (`README.md`, `docs/USER_GUIDE.md`, this file) stays synchronised with the codebase.

Please update this document whenever responsibilities shift or new layers are introduced.
