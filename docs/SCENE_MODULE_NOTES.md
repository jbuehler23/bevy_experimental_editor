# Scene Module Notes

## Current Structure

- `bevy_editor_scene` owns the reusable scene backend:
  - `EditorScene` resource and `EditorSceneEntity` marker component.
  - Undoable scene edit events (`TransformEditEvent`, `NameEditEvent`, `SpriteTextureEvent`).
  - Multi-scene primitives: `OpenScene`, `OpenScenes`, `SceneTabChanged`, `SceneAutoLoader`, and `LoadingSceneRoot`.
  - Runtime snapshot helpers (`capture_editor_scene_runtime`, `sync_active_scene`, `mark_loaded_scene_entities`).
- `bevy_editor_app` wires in `cache_runtime_scene_on_scene_switch` so unsaved tabs are preserved when switching.
- `bevy_editor_ui_egui` collects entity/parent pairs and delegates scene transitions to `bevy_editor_scene`; the UI simply clears its name buffer after calling `apply_scene_tab_change`.

## Integration Summary

- Tab switching now snapshots the active scene to an in-memory `DynamicScene` and restores it when the tab becomes active again.
- `sync_active_scene` only despawns root entities to avoid double-despawn warnings and handles three cases:
  1. Load from disk (`file_path` present)
  2. Spawn cached runtime snapshot (`runtime_scene`)
  3. Create a fresh `"Scene Root"` entity
- `mark_loaded_scene_entities` discovers the true root spawned by the dynamic scene and re-tags editor entities.
- Inspector, project browser, and project loader interact exclusively through the shared scene APIs.

## Follow-up Ideas

1. **Scene command crate** – consider extracting scene-specific undoable commands into `bevy_editor_scene_commands` so other frontends can reuse them.
2. **Headless example** – add a minimal sample showing how to drive `SceneEditorPlugin` and `OpenScenes` without egui.
3. **View-model helpers** – optionally expose a tab descriptor iterator so non-egui frontends can render tab UI without touching internal state.

These notes are now mostly historical; for the authoritative overview see [`docs/ARCHITECTURE.md`](ARCHITECTURE.md).

