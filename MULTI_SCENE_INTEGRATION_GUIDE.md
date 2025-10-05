# Multi-Scene Tab Integration Guide

This guide provides step-by-step instructions for completing the integration of the multi-scene tab system into the Eryndor editor.

## Overview

The multi-scene tab system foundation is complete ([scene_tabs.rs](crates/eryndor-editor/src/scene_tabs.rs)). The remaining work is to integrate it by replacing `CurrentLevel` with `OpenScenes` throughout the codebase.

---

## Step 1: Initialize OpenScenes Resource

**File:** `crates/eryndor-editor/src/main.rs`

**Location:** After line 80 (after `.init_resource::<CurrentLevel>()`)

**Change:**
```rust
// BEFORE:
.init_resource::<CurrentLevel>()

// AFTER:
.init_resource::<CurrentLevel>()  // Keep for backward compatibility temporarily
.init_resource::<OpenScenes>()     // Add new multi-scene resource
```

---

## Step 2: Add Scene Tabs UI System

**File:** `crates/eryndor-editor/src/main.rs`

**Location:** In the first `.add_systems(Update, (...))` block, after `toolbar_ui`

**Change:**
```rust
// BEFORE:
toolbar_ui,
tileset_panel_ui,

// AFTER:
toolbar_ui,
scene_tabs_ui,  // Add scene tabs UI
tileset_panel_ui,
```

---

## Step 3: Update Status Bar UI

**File:** `crates/eryndor-editor/src/ui.rs`

**Function:** `ui_system` (lines 8-16)

**Add parameter:**
```rust
pub fn ui_system(
    mut contexts: EguiContexts,
    mut editor_state: ResMut<EditorState>,
    mut current_level: ResMut<CurrentLevel>,
    mut entity_palette: ResMut<EntityPalette>,
    mut collision_editor: ResMut<CollisionEditor>,
    workspace: Option<Res<crate::workspace::EditorWorkspace>>,
    mut project_selection: Option<ResMut<crate::project_manager::ProjectSelection>>,
    open_scenes: Res<OpenScenes>,  // ADD THIS LINE
) {
```

**Update status bar** (lines 21-35):
```rust
// BEFORE:
egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
    ui.horizontal(|ui| {
        ui.label(format!("Level: {}", current_level.level_data.metadata.name));
        ui.separator();
        ui.label(format!("Platforms: {}", current_level.level_data.platforms.len()));
        ui.separator();
        ui.label(format!("Entities: {}", current_level.level_data.entities.len()));
        ui.separator();
        if current_level.is_modified {
            ui.label("● Modified");
        } else {
            ui.label("○ Saved");
        }
    });
});

// AFTER:
egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
    ui.horizontal(|ui| {
        if let Some(scene) = open_scenes.active_scene() {
            ui.label(format!("Scene: {}", scene.name));
            ui.separator();
            ui.label(format!("Platforms: {}", scene.level_data.platforms.len()));
            ui.separator();
            ui.label(format!("Entities: {}", scene.level_data.entities.len()));
            ui.separator();
            if scene.is_modified {
                ui.label("● Modified");
            } else {
                ui.label("○ Saved");
            }
        } else {
            ui.label("No scene loaded");
        }
    });
});
```

---

## Step 4: Update Save/Load Systems

**File:** `crates/eryndor-editor/src/systems.rs`

### 4a. Update `handle_save_load` function

**Find:** `pub fn handle_save_load(` (around line 129)

**Change parameters:**
```rust
// BEFORE:
pub fn handle_save_load(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut current_level: ResMut<CurrentLevel>,
    editor_state: Res<EditorState>,
    ...
) {

// AFTER:
pub fn handle_save_load(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut open_scenes: ResMut<OpenScenes>,  // CHANGE THIS
    editor_state: Res<EditorState>,
    ...
) {
```

### 4b. Update save logic

**Find:** Lines 145-174 (Ctrl+S handler)

**Change:**
```rust
// BEFORE:
if let Some(path) = current_level.file_path.clone() {
    save_level_with_tilemap(
        &mut current_level,
        &path,
        ...
    );

// AFTER:
if let Some(scene) = open_scenes.active_scene_mut() {
    if let Some(path) = scene.file_path.clone() {
        save_level_with_tilemap(
            scene,
            &path,
            ...
        );
        scene.is_modified = false;
    } else {
        // No path set, show Save As dialog
        save_as_dialog_with_tilemap(
            scene,
            ...
        );
    }
}
```

### 4c. Update `save_level_with_tilemap` function

**Find:** `fn save_level_with_tilemap(` (around line 196)

**Change signature:**
```rust
// BEFORE:
fn save_level_with_tilemap(
    current_level: &mut ResMut<CurrentLevel>,
    path: &str,
    ...
) {

// AFTER:
fn save_level_with_tilemap(
    scene: &mut OpenScene,  // CHANGE THIS
    path: &str,
    ...
) {
```

**Update usages inside function:**
```rust
// BEFORE:
current_level.level_data.tilemap = Some(tilemap_data);
current_level.file_path = Some(path.to_string());
current_level.is_modified = false;

// AFTER:
scene.level_data.tilemap = Some(tilemap_data);
scene.file_path = Some(path.to_string());
scene.is_modified = false;
```

### 4d. Update `save_as_dialog_with_tilemap` function

Similar changes to `save_level_with_tilemap`.

### 4e. Update `open_dialog` function

**Add scene to tabs instead of replacing current_level:**
```rust
// Load scene
match BevyScene::load_from_file(&path) {
    Ok(scene_data) => {
        let new_scene = OpenScene {
            name: path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Untitled")
                .to_string(),
            file_path: Some(path.to_string_lossy().to_string()),
            level_data: scene_data.data,
            is_modified: false,
        };
        open_scenes.add_scene(new_scene);
    }
    Err(e) => error!("Failed to load scene: {}", e),
}
```

---

## Step 5: Update Scene Auto-Loader

**File:** `crates/eryndor-editor/src/scene_loader.rs`

### Update `auto_load_scene_system`

**Change parameters:**
```rust
// BEFORE:
pub fn auto_load_scene_system(
    mut commands: Commands,
    project: Option<Res<CurrentProject>>,
    mut current_level: ResMut<CurrentLevel>,
    mut auto_loader: ResMut<SceneAutoLoader>,
) {

// AFTER:
pub fn auto_load_scene_system(
    mut commands: Commands,
    project: Option<Res<CurrentProject>>,
    mut open_scenes: ResMut<OpenScenes>,  // CHANGE THIS
    mut auto_loader: ResMut<SceneAutoLoader>,
) {
```

**Update scene loading logic:**
```rust
// BEFORE:
current_level.level_data = scene.data;
current_level.file_path = Some(scene_path.to_string_lossy().to_string());
current_level.is_modified = false;

// AFTER:
let new_scene = OpenScene {
    name: scene_to_load.clone(),
    file_path: Some(scene_path.to_string_lossy().to_string()),
    level_data: scene.data,
    is_modified: false,
};
// Replace the default untitled scene with the loaded scene
if open_scenes.scenes.len() == 1 &&
   open_scenes.scenes[0].name.starts_with("Untitled") &&
   !open_scenes.scenes[0].is_modified {
    open_scenes.scenes[0] = new_scene;
} else {
    open_scenes.add_scene(new_scene);
}
```

---

## Step 6: Update Other Systems Using CurrentLevel

Search for all usages of `CurrentLevel` in the codebase and update them to use `OpenScenes`:

```bash
cd crates/eryndor-editor
grep -r "CurrentLevel" src/ --include="*.rs"
```

**Files that may need updates:**
- `tile_painter.rs` - If it accesses level data
- `collision_editor.rs` - If it modifies level data
- `map_canvas.rs` - If it reads level data
- Any other systems that read/write level data

**Pattern for updates:**
```rust
// BEFORE:
fn my_system(mut current_level: ResMut<CurrentLevel>) {
    current_level.level_data.something = value;
    current_level.is_modified = true;
}

// AFTER:
fn my_system(mut open_scenes: ResMut<OpenScenes>) {
    if let Some(scene) = open_scenes.active_scene_mut() {
        scene.level_data.something = value;
        scene.is_modified = true;
    }
}
```

---

## Step 7: Test the Integration

### Test Cases:

1. **Basic Tab Functionality:**
   - [ ] Open editor
   - [ ] Default "Untitled" tab appears
   - [ ] Click ➕ to create new tab
   - [ ] Multiple tabs visible
   - [ ] Click tabs to switch between them
   - [ ] Click ✖ to close tabs (last tab resets instead of closing)

2. **Save/Load:**
   - [ ] Create a scene, paint some tiles
   - [ ] Ctrl+S to save (modified indicator ● disappears)
   - [ ] Close tab, reopen file
   - [ ] Scene loads in new tab
   - [ ] Multiple scenes can be open simultaneously

3. **Project Integration:**
   - [ ] Create new project
   - [ ] `main.bscene` loads automatically
   - [ ] Make changes, save
   - [ ] Close editor
   - [ ] Reopen project from Recent Projects
   - [ ] `main.bscene` auto-loads

4. **Modified State:**
   - [ ] Make changes to a scene
   - [ ] Tab shows ● indicator
   - [ ] Save the scene
   - [ ] ● indicator disappears
   - [ ] Switch tabs, ● persists on modified tabs

---

## Step 8: Optional - Remove CurrentLevel (Final Cleanup)

Once all systems are migrated to `OpenScenes`:

1. Remove `.init_resource::<CurrentLevel>()` from main.rs
2. Remove the `CurrentLevel` struct definition
3. Ensure all imports are updated
4. Run `cargo check` to catch any remaining usages

---

## Troubleshooting

### Build Errors

**Error:** `cannot find type CurrentLevel`
- **Solution:** You've removed it too early, add it back temporarily and find remaining usages

**Error:** `no method named active_scene on OpenScenes`
- **Solution:** Make sure you imported `use scene_tabs::*;` in the file

### Runtime Errors

**Error:** Panic "index out of bounds"
- **Solution:** Always check `open_scenes.active_scene()` returns `Some` before accessing

**Error:** Scene not saving
- **Solution:** Make sure you're calling `scene.is_modified = false` after save

---

## Benefits After Integration

✅ **Multiple scenes open** - Work on different levels simultaneously
✅ **Quick switching** - Click tabs to switch instantly
✅ **Visual state** - See which scenes are modified at a glance
✅ **Better workflow** - Copy/paste between scenes becomes possible
✅ **Professional UX** - Matches industry-standard editors

---

## Estimated Time

- **Steps 1-2:** 15 minutes (initialization)
- **Steps 3-5:** 1-2 hours (main system updates)
- **Step 6:** 30-60 minutes (remaining systems)
- **Step 7:** 30 minutes (testing)
- **Step 8:** 15 minutes (cleanup)

**Total: 3-4 hours**

---

*Integration guide created: 2025-10-05*
*For use with: bevy_experimental_editor v0.1.0*
