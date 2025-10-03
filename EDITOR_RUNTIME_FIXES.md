# Eryndor Editor - Runtime Fixes

## Issue: Editor Crashes at Runtime

### Initial Error
When running the editor with `bevy_egui = "0.36"`, it crashed with:
```
Called `available_rect()` before `Context::run()`
```

This was an egui 0.32 API incompatibility issue.

### Root Cause
bevy_egui 0.36 uses egui 0.32, which has breaking API changes compared to egui 0.31. The API changed how egui contexts are managed, causing panels to fail when calling `.show()`.

### Solution
Downgraded to `bevy_egui = "0.34"` which uses egui 0.31, avoiding the breaking changes.

## API Differences Between bevy_egui 0.34 and 0.36

### 1. EguiContexts::ctx_mut() Return Type

**bevy_egui 0.34** (egui 0.31):
```rust
let ctx = contexts.ctx_mut();  // Returns &mut Context directly
```

**bevy_egui 0.36** (egui 0.32):
```rust
let Ok(ctx) = contexts.ctx_mut() else { return; };  // Returns Result<&mut Context, _>
```

### 2. EguiPlugin Construction

**bevy_egui 0.34**:
```rust
EguiPlugin {
    enable_multipass_for_primary_context: false,
}
```

**bevy_egui 0.36**:
```rust
EguiPlugin::default()  // Has Default trait implementation
```

## Files Modified

### [crates/eryndor-editor/Cargo.toml](crates/eryndor-editor/Cargo.toml)
```toml
# Changed from:
bevy_egui = "0.36"

# To:
bevy_egui = "0.34"  # Version 0.34 uses egui 0.31, avoiding the 0.32 API breaking changes
```

### [crates/eryndor-editor/src/main.rs](crates/eryndor-editor/src/main.rs:32-34)
```rust
EguiPlugin {
    enable_multipass_for_primary_context: false,
},
```

### [crates/eryndor-editor/src/ui.rs](crates/eryndor-editor/src/ui.rs:14)
```rust
let ctx = contexts.ctx_mut();  // Direct assignment, not Result pattern
```

### [crates/eryndor-editor/src/selection.rs](crates/eryndor-editor/src/selection.rs:70-73)
```rust
let ctx = contexts.ctx_mut();
if ctx.is_pointer_over_area() {
    return;
}
```

### [crates/eryndor-editor/src/systems.rs](crates/eryndor-editor/src/systems.rs)
Two locations (entity placement and platform editing):
```rust
let ctx = contexts.ctx_mut();
if ctx.is_pointer_over_area() {
    return;
}
```

## Final Result

✅ **Editor compiles successfully**
✅ **Editor runs without crashes**
✅ **Window opens with title: "Eryndor Level Editor"**
✅ **All UI panels render correctly (egui 0.31)**

## Running the Editor

```bash
cd crates/eryndor-editor
cargo run
```

The editor window should open with:
- Top menu bar (File, Edit, View)
- Left toolbar (Select, Platform, Entity, Erase tools)
- Right panel (Entity palette with NPCs, Resources, Interactive Objects, Spawn Points)
- Bottom status bar (Level stats, modification indicator)
- Central viewport with grid (pan with right mouse, zoom with scroll wheel)

## Version Compatibility Matrix

| Crate | Version | egui Version | Status |
|-------|---------|--------------|--------|
| bevy | 0.16.0 | N/A | ✅ Working |
| bevy_egui | 0.34 | 0.31 | ✅ Working |
| bevy_egui | 0.36 | 0.32 | ❌ API Breaking Changes |
| bevy-inspector-egui | 0.31 | 0.31 | ✅ Working |
| bevy_pancam | 0.18 | N/A | ✅ Working |

## Future Considerations

When upgrading to bevy_egui 0.36+ in the future:
1. Update all `ctx_mut()` calls to use `Result` pattern:
   ```rust
   let Ok(ctx) = contexts.ctx_mut() else { return; };
   ```
2. Change `EguiPlugin` construction to:
   ```rust
   EguiPlugin::default()
   ```
3. Review egui 0.32 migration guide for other potential breaking changes
4. Test panel rendering to ensure no `available_rect()` issues

## Notes

- The editor is fully functional with bevy_egui 0.34
- All features work as expected (selection, placement, save/load, UI)
- The downgrade is temporary until egui 0.32 becomes more stable in the Bevy ecosystem
- bevy-inspector-egui 0.31 is compatible with both versions as it depends on egui 0.31
