# Refactor Progress Log

This log tracks what we have finished and what still needs attention for each
phase of the modular editor refactor. Update it as phases advance so future
sessions can resume quickly without rereading large diffs.

## Phase 1 - Tilemap Module Extraction
- **Completed**
  - Extracted all tilemap code into `bevy_editor_tilemap`.
  - Introduced `TilemapEditorPlugin` and `TilemapCorePlugin` behind feature
    flags.
  - Updated the app crate to depend on the new crate and removed inlined
    tilemap modules.
  - Authored a crate README with a quick-start and integration tips.
  - Audited downstream crates to ensure they consume re-exported tilemap APIs.
- **Remaining**
  - None. Phase 1 is fully wrapped.

## Phase 2 - Project Management Extraction
- **Completed**
  - Wired `bevy_editor_project` into the workspace and enabled the `cli` UI
    feature set for the app.
  - Replaced `bevy_editor`'s in-tree project manager, wizard, CLI runner, and
    workspace modules with the new crate.
  - Registered `ProjectManagerPlugin` in `bevy_editor` and funneled project
    systems through `ProjectManagerSet` for ordered chaining.
  - Added a bridge system so the app's `ProjectBrowser` tracks the active
    project resources exposed by `bevy_editor_project`.
  - Confirmed `cargo check -p bevy_editor` succeeds after the migration.
  - Authored a crate README and smoke test covering the `ProjectCorePlugin`
    headless flow.
  - Documented that the project-browser sync helper remains in the app crate
    (depends on app-specific types) and referenced it from the crate README.
- **Remaining**
  - None. Phase 2 is complete; prepare to kick off Phase 3 (asset backend).

## Phase 3 - Asset Backend Extraction
- **Completed**
  - Created the `bevy_editor_assets` crate with core and full plugins plus an
    `AssetBrowserSet` for scheduling.
  - Migrated the asset browser resource and scan system out of `bevy_editor`
    into the new crate.
  - Added an integration system in the app to keep the asset browser root in
    sync with the active project before scans run.
  - Documented the current rescan/file-watcher strategy in the crate README.
  - Added smoke tests that scan a temporary assets directory via the
    `TextureHandleProvider` abstraction.
- **Remaining**
  - None. Egui texture helper work has been folded into Phase 4's frontend
    abstraction decisions.

## Phase 4 - Frontend API Extraction *(current phase)*
- **Completed**
  - Introduced the `bevy_editor_frontend_api` crate to host UI-agnostic
    contracts.
  - Migrated scene tree view-models and command events from
    `bevy_editor_ui_egui` to the new crate, updating the egui layer to depend
    on the shared types.
  - Extracted the inspector component snapshot and panel state so other
    frontends can reuse the same data contract.
  - Added a shared asset-browser panel state so UI layers can coordinate
    thumbnail sizing and visibility.
  - Migrated the project browser resource and panel state into the shared crate
    so every frontend shares the same file tree view model.
  - Removed the legacy `bevy_editor::tileset_manager` module now that
    `bevy_editor_tilemap` owns tileset state.
  - Updated event writers, single-entity queries, and egui frame helpers to the
    modern APIs so the workspace builds without deprecation errors.
  - Defined the `EditorFrontend` trait with shared `EditorAction`/`EditorEvent`
    channels and upgraded the egui frontend to implement it.
  - Extracted scene-tree and CLI output panel resources into the frontend API so
    other UIs can reuse visibility/width/auto-scroll defaults.
  - Documented frontend contracts and action/event routing in the updated
    `README.md`, `docs/USER_GUIDE.md`, and `docs/ARCHITECTURE.md`.
- **Remaining**
  - None. Phase 4 closes with frontend contracts documented and reused.

## Phase 6 - Editor Shell Extraction
- **Completed**
  - Added the `bevy_editor_app` crate with an `EditorAppPlugin` that wires the
    backend crates, registers shared events, and accepts any `EditorFrontend`.
  - Converted the top-level `bevy_editor` binary into a thin launcher that
    delegates to `EditorAppPlugin` with the egui frontend.
  - Centralized CLI command handling through shared `EditorAction` events and
    emitted status notifications for the UI.
  - Added headless smoke coverage (`examples/headless.rs`) and integration tests
    that exercise the action bridge when no project is loaded.
- **Remaining**
  - Backfill automation/tests for successful CLI completion and cancel flows
    (action bridge currently tests only error cases).

## Upcoming Phases
- Phase 5: Extract `bevy_editor_ui_egui` (egui UI code). ✔
- Phase 7: Add more shell examples demonstrating different crate combinations.

> When closing a phase, move any unresolved bullets into the next phase or an
> explicit "Backlog" section so the open work remains visible.
