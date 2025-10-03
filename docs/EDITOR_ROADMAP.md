# Eryndor Editor Enhancement Roadmap
## Tiled/Godot-Inspired Tilemap Editor Features

**Version:** 1.0
**Last Updated:** 2025-10-03
**Status:** Planning Phase

---

## Current State Analysis

### ✅ Implemented Features
- Single tile painting with left-click
- Tileset loading and management
- Multi-layer support with visibility toggles
- Collision shape editor
- Scene save/load with `.bscene` JSON format
- Hot-reload integration with client
- Project management (`.bvy` format)
- Async build system for game compilation

### 🟡 Partially Implemented
- **Rectangle/BucketFill/Line tools**: Enum definitions exist in `tile_painter.rs` but no implementation
- **Multi-tile selection**: Data structures (`selected_tiles`, `selection_start/end`) exist in `TilesetManager` but no UI

### ❌ Missing Critical Features
- Undo/redo system
- Keyboard shortcuts
- Multi-tile stamp brushes
- Copy/paste functionality
- Eyedropper tool
- Grid customization

---

## Phase 1: Advanced Tile Painting Tools
**Priority:** 🔴 HIGH
**Target Sprint:** Sprint 1 (Week 1-2)

### 1.1 Rectangle Fill Tool
**Inspired by:** Tiled's rectangle selection tool
**Complexity:** ⭐⭐ Medium (8-12 hours)

**Features:**
- Click and drag to define rectangular area
- Fill entire selection with currently selected tile
- Visual preview rectangle while dragging (semi-transparent overlay)
- Respects layer boundaries
- Keyboard modifier: Hold `Shift` while dragging to constrain to square

**Implementation Notes:**
```rust
// In tile_painter.rs
struct RectangleTool {
    start_pos: Option<(u32, u32)>,
    current_pos: Option<(u32, u32)>,
    preview_enabled: bool,
}

fn handle_rectangle_tool(
    mouse_button: Res<ButtonInput<MouseButton>>,
    tile_painter: ResMut<TilePainter>,
    // ... draw preview rect, fill on release
)
```

**Files to Modify:**
- `tile_painter.rs` - Add rectangle tool logic
- `gizmos.rs` - Add rectangle preview rendering
- `map_canvas.rs` - Add batch tile placement

**Dependencies:** None

---

### 1.2 Multi-Tile Stamp Brush
**Inspired by:** Tiled's stamp brush, Godot's TileMapLayer patterns
**Complexity:** ⭐⭐⭐ High (16-20 hours)

**Features:**
- Select rectangular region in tileset panel (click-drag)
- Selected tiles highlighted with colored border
- Paint entire NxM pattern as a "stamp" onto canvas
- Stamp repeats on drag-painting
- Rotate stamp: `R` key (90° CW), `Shift+R` (90° CCW)
- Flip stamp: `X` key (horizontal), `Y` key (vertical)
- Preview stamp ghost at cursor position

**Implementation Notes:**
```rust
// In tileset_manager.rs - already has structure!
pub struct TilesetManager {
    pub selected_tiles: Vec<u32>,           // ✅ exists
    pub selection_start: Option<(u32, u32)>, // ✅ exists
    pub selection_end: Option<(u32, u32)>,   // ✅ exists
}

// New: Stamp pattern structure
pub struct TileStamp {
    tiles: Vec<Vec<u32>>,  // 2D array of tile IDs
    width: u32,
    height: u32,
    rotation: Rotation,     // 0°, 90°, 180°, 270°
    flip_x: bool,
    flip_y: bool,
}
```

**Files to Modify:**
- `tileset_panel.rs` - Implement rectangle selection UI
- `tileset_manager.rs` - Add stamp creation/transformation methods
- `tile_painter.rs` - Stamp painting logic
- `gizmos.rs` - Stamp preview ghost rendering

**Dependencies:** None

**UI/UX:**
- Display stamp size in status bar: "Stamp: 3x2 tiles"
- Keyboard shortcuts shown in tooltip
- Visual feedback for rotations/flips

---

### 1.3 Bucket Fill Tool
**Inspired by:** Tiled's bucket fill, Photoshop's paint bucket
**Complexity:** ⭐⭐⭐ High (12-16 hours)

**Features:**
- Click to flood-fill connected tiles of same type
- Two modes:
  1. **Replace matching**: Only replaces tiles matching clicked tile
  2. **Fill empty**: Only fills empty (null) tiles
- Configurable max fill area (default: 1000 tiles, prevents accidents)
- Visual "spreading" animation (optional, cool factor)
- Undo support critical for this tool

**Implementation Notes:**
```rust
// Flood fill algorithm
fn flood_fill(
    start: (u32, u32),
    target_tile_id: Option<u32>,
    replacement_tile_id: u32,
    max_tiles: usize,
) -> Vec<(u32, u32)> {
    // BFS or DFS with 4-directional connectivity
    // Return list of affected positions
}
```

**Files to Modify:**
- `tile_painter.rs` - Flood fill algorithm & mode selection
- `ui.rs` - Fill mode selector UI
- `map_canvas.rs` - Batch tile updates

**Dependencies:**
- **Strongly recommended:** Undo/redo system (Phase 4.2)
- Max fill limit prevents disasters without undo

**Performance Considerations:**
- Large fills (>500 tiles) may lag - needs batch rendering
- Consider async fill for huge areas

---

### 1.4 Line Tool
**Inspired by:** Tiled's line drawing tool
**Complexity:** ⭐⭐ Medium (6-8 hours)

**Features:**
- Click-drag to draw straight line of tiles
- Bresenham's line algorithm for smooth lines
- Preview line while dragging
- Configurable thickness: 1-5 tiles (default 1)
- Keyboard modifier: Hold `Shift` to constrain to 45° angles

**Implementation Notes:**
```rust
// Bresenham's algorithm
fn draw_line(start: (u32, u32), end: (u32, u32)) -> Vec<(u32, u32)> {
    // Classic line rasterization
}
```

**Files to Modify:**
- `tile_painter.rs` - Line drawing logic
- `gizmos.rs` - Line preview
- `ui.rs` - Thickness selector

**Dependencies:** None

---

### 1.5 Eyedropper Tool
**Inspired by:** Photoshop eyedropper, Godot tile picker
**Complexity:** ⭐ Low (4-6 hours)

**Features:**
- Pick tile from canvas to set as active brush
- Keyboard shortcut: Hold `Alt` while hovering (temporary tool)
- Click to pick single tile
- Shift+Click to pick multi-tile stamp pattern (if possible)
- Visual indicator: cursor changes to eyedropper icon

**Implementation Notes:**
```rust
fn eyedropper_pick(pos: (u32, u32), layer: &Layer) -> Option<u32> {
    layer.get_tile_at(pos)
}

// For pattern picking (advanced)
fn pick_pattern(pos: (u32, u32), size: (u32, u32)) -> TileStamp {
    // Extract NxN region around click
}
```

**Files to Modify:**
- `tile_painter.rs` - Eyedropper logic
- `main.rs` - Add `EditorTool::Eyedropper` enum variant
- `ui.rs` - Eyedropper button/shortcut

**Dependencies:** None

---

## Phase 2: Enhanced Tileset Management
**Priority:** 🟡 MEDIUM
**Target Sprint:** Sprint 2-3 (Week 3-4)

### 2.1 Tileset Panel Improvements
**Complexity:** ⭐⭐ Medium (10-14 hours)

**Features:**
- **Rectangle selection:** Click-drag to select multiple tiles
- **Additive selection:** Ctrl+Click to add/remove from selection
- **Selection visual:** Highlight with colored border (blue default)
- **Quick actions toolbar:**
  - Rotate selection 90° CW/CCW
  - Flip horizontal/vertical
  - Clear selection
- **Tileset zoom:** Mouse wheel to zoom in/out (1x, 2x, 4x)
- **Grid overlay toggle**

**Implementation Notes:**
```rust
// In tileset_panel.rs
struct TilesetPanelState {
    zoom_level: f32,  // 1.0, 2.0, 4.0
    show_grid: bool,
    selection_color: Color,
}
```

**Files to Modify:**
- `tileset_panel.rs` - Selection UI, zoom controls
- `tileset_manager.rs` - Selection management methods

---

### 2.2 Tile Animation Support
**Complexity:** ⭐⭐⭐⭐ Very High (24-32 hours)
**Note:** Advanced feature, may defer to later sprint

**Features:**
- Define animation sequences per tile
- Configure frame duration (milliseconds)
- Preview animations in tileset panel
- Export to `.bscene` format with animation data
- Runtime playback in client

**Implementation Notes:**
```rust
pub struct TileAnimation {
    frames: Vec<u32>,        // Tile IDs
    frame_duration_ms: u32,   // Per-frame duration
    loop_animation: bool,
}

// In TilesetData
pub animations: HashMap<u32, TileAnimation>,
```

**Files to Modify:**
- `eryndor-common/src/tilemap.rs` - Add animation data structures
- `tileset_panel.rs` - Animation editor UI
- `tilemap_renderer.rs` (client) - Animation playback system

**Dependencies:**
- Custom properties (2.3) for metadata
- Significant client-side changes

---

### 2.3 Tile Properties & Metadata
**Complexity:** ⭐⭐⭐ High (14-18 hours)

**Features:**
- Per-tile custom properties (key-value pairs)
- Common property templates:
  - `walkable: bool`
  - `damage: i32`
  - `trigger_id: string`
  - `friction: f32`
- Inspector panel for editing properties
- Properties saved in scene format
- Copy properties between tiles

**Implementation Notes:**
```rust
pub struct TileProperties {
    properties: HashMap<String, PropertyValue>,
}

pub enum PropertyValue {
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
    Color(Color),
}
```

**Files to Modify:**
- `eryndor-common/src/tilemap.rs` - Property data structures
- New file: `tile_inspector.rs` - Property editor UI
- `scene_format.rs` - Serialize properties

---

## Phase 3: Advanced Layer Features
**Priority:** 🟡 MEDIUM
**Target Sprint:** Sprint 4-5 (Week 5-7)

### 3.1 Layer Blending & Opacity
**Complexity:** ⭐⭐ Medium (8-12 hours)

**Features:**
- Per-layer opacity slider (0-100%)
- Blend modes: Normal, Add, Multiply, Screen
- Layer tint color (modulate)
- Real-time preview in editor

**Files to Modify:**
- `layer_manager.rs` - Add opacity/blend fields
- `layer_panel.rs` - UI controls
- Rendering system - Apply blending (shader changes)

---

### 3.2 Layer Groups/Folders
**Complexity:** ⭐⭐⭐ High (16-20 hours)

**Features:**
- Hierarchical layer organization
- Collapse/expand groups in layer panel
- Group operations: lock/hide entire group
- Drag-drop layers between groups
- Nested groups supported

**Files to Modify:**
- `layer_manager.rs` - Tree structure instead of flat list
- `layer_panel.rs` - Tree UI with egui::CollapsingHeader

---

### 3.3 Auto-Tiling System
**Complexity:** ⭐⭐⭐⭐⭐ Very High (40-60 hours)
**Note:** This is a major feature - separate epic

**Features:**
- Define autotile rules (bitmask patterns)
- 3x3 or 2x2 autotile templates
- Rule-based tile placement (e.g., grass edges auto-connect)
- Real-time updates when neighbors change
- Support for Tiled-style Wang tiles

**Implementation:** Separate from this roadmap - needs dedicated design doc

---

## Phase 4: Workflow & UX Improvements
**Priority:** 🔴 HIGH
**Target Sprint:** Sprint 1-3 (parallel with Phase 1)

### 4.1 Keyboard Shortcuts System
**Complexity:** ⭐⭐ Medium (10-12 hours)

**Essential Shortcuts:**

| Shortcut | Action | Status |
|----------|--------|--------|
| `B` | Brush tool | ❌ |
| `R` | Rectangle tool | ❌ |
| `F` | Fill (bucket) | ❌ |
| `L` | Line tool | ❌ |
| `E` | Erase tool | ✅ Exists |
| `I` | Eyedropper | ❌ |
| `Alt + Hover` | Temporary eyedropper | ❌ |
| `G` | Toggle grid | ❌ |
| `Space + Drag` | Pan camera | ❌ |
| `Ctrl+Z` | Undo | ❌ Critical! |
| `Ctrl+Y` / `Ctrl+Shift+Z` | Redo | ❌ Critical! |
| `[` / `]` | Previous/next layer | ❌ |
| `Ctrl+D` | Duplicate layer | ❌ |
| `Ctrl+S` | Save scene | ✅ Exists |
| `Ctrl+O` | Open scene | ❌ |
| `Ctrl+N` | New scene | ❌ |
| `Ctrl+C` / `Ctrl+V` | Copy/paste | ❌ |
| `Delete` | Delete selection | ❌ |
| `Tab` | Toggle UI panels | ❌ |

**Implementation Notes:**
```rust
// Global keyboard handler
fn handle_global_shortcuts(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut editor_state: ResMut<EditorState>,
) {
    if keyboard.just_pressed(KeyCode::KeyB) {
        editor_state.current_tool = EditorTool::Brush;
    }
    // ... etc
}
```

**Files to Modify:**
- New file: `shortcuts.rs` - Centralized shortcut handling
- `main.rs` - Register shortcut system
- `ui.rs` - Show shortcuts in tooltips

---

### 4.2 Undo/Redo System
**Complexity:** ⭐⭐⭐⭐ Very High (20-28 hours)
**Priority:** 🔴 **CRITICAL** - Required for most tools

**Features:**
- Command pattern for all edit operations
- History size limit (default 50 actions, configurable)
- Visual history panel (optional advanced feature)
- Per-layer undo support (stretch goal)
- Memory-efficient serialization

**Implementation Notes:**
```rust
// Command pattern
trait EditorCommand {
    fn execute(&self, state: &mut EditorState);
    fn undo(&self, state: &mut EditorState);
    fn description(&self) -> String;
}

struct PaintTileCommand {
    layer_id: u32,
    position: (u32, u32),
    old_tile: Option<u32>,
    new_tile: u32,
}

struct BatchCommand {
    commands: Vec<Box<dyn EditorCommand>>,
}

#[derive(Resource)]
struct UndoHistory {
    commands: Vec<Box<dyn EditorCommand>>,
    current_index: usize,
    max_history: usize,
}
```

**Commands to Implement:**
- `PaintTileCommand` - Single tile change
- `BatchPaintCommand` - Multiple tiles (rect fill, etc.)
- `LayerCreateCommand`
- `LayerDeleteCommand`
- `LayerReorderCommand`
- `TilesetLoadCommand`

**Files to Create:**
- `undo_system.rs` - Core undo/redo logic
- `commands.rs` - Individual command implementations

**Files to Modify:**
- All editing systems to use commands instead of direct mutation
- `ui.rs` - Undo/redo buttons, history panel

**Performance Considerations:**
- Large batch operations (1000+ tiles) need efficient storage
- Consider delta compression for large commands

---

### 4.3 Copy/Paste Support
**Complexity:** ⭐⭐⭐ High (12-16 hours)

**Features:**
- Copy rectangular region from canvas (Ctrl+C after selection)
- Paste with preview ghost (Ctrl+V)
- Paste modes:
  - **Merge:** Keep existing tiles, only fill empty
  - **Replace:** Overwrite everything
  - **Overlay:** Blend with existing
- Cross-layer paste support
- Clipboard persists between scenes

**Implementation Notes:**
```rust
#[derive(Resource)]
struct EditorClipboard {
    tiles: Vec<Vec<Option<u32>>>,  // 2D array
    width: u32,
    height: u32,
    source_layer: Option<u32>,
}
```

**Files to Modify:**
- New file: `clipboard.rs` - Clipboard management
- `selection.rs` - Copy from selection
- `tile_painter.rs` - Paste logic with preview

**Dependencies:**
- Selection tool (currently exists for entities, needs tile support)

---

### 4.4 Grid & Snap Options
**Complexity:** ⭐⭐ Medium (6-10 hours)

**Features:**
- Configurable grid size (16x16, 32x32, 64x64, custom)
- Snap to grid toggle (on by default)
- Grid color customization (RGBA)
- Grid opacity slider
- Ruler overlay with measurements
- Multiple grid layers (major/minor lines)

**Implementation Notes:**
```rust
#[derive(Resource)]
struct GridSettings {
    size: u32,               // Tile size in pixels
    enabled: bool,
    snap_enabled: bool,
    color: Color,
    opacity: f32,
    show_ruler: bool,
}
```

**Files to Modify:**
- `gizmos.rs` - Grid rendering
- New UI panel: `grid_settings.rs`
- `EditorState` - Add grid settings

---

## Phase 5: Object & Entity Placement
**Priority:** 🟢 LOW
**Target Sprint:** Sprint 6+ (Week 8+)

### 5.1 Object Templates
**Complexity:** ⭐⭐⭐ High (16-20 hours)

**Features:**
- Predefined entity templates (spawn points, items, triggers)
- Visual gizmos for each entity type
- Property inspector for entities
- Entity library panel

---

### 5.2 Prefab System
**Complexity:** ⭐⭐⭐⭐ Very High (24-32 hours)

**Features:**
- Save tile + entity combinations as prefabs
- Reusable across scenes
- Nested prefab support
- Prefab variants (override properties)

---

## Phase 6: Scene Management
**Priority:** 🟡 MEDIUM
**Target Sprint:** Sprint 5-6

### 6.1 Multi-Scene Editing
**Complexity:** ⭐⭐⭐ High (14-18 hours)

**Features:**
- Open multiple scenes in tabs
- Switch between scenes quickly
- Copy/paste between scenes
- Dirty state tracking per scene

---

### 6.2 Scene Templates
**Complexity:** ⭐⭐ Medium (8-12 hours)

**Features:**
- New scene wizard with templates
- Template categories: Platformer, Top-Down, Dungeon, etc.
- Custom template creation
- Template preview thumbnails

---

### 6.3 Scene Inheritance/Variants
**Complexity:** ⭐⭐⭐⭐ Very High (20-28 hours)

**Features:**
- Create variant of base scene
- Override specific tiles/objects
- Propagate changes from base scene
- Visual diff showing changes

---

## Phase 7: Export & Integration
**Priority:** 🟢 LOW
**Target Sprint:** Sprint 7+ (Post-MVP)

### 7.1 Advanced Export Options
**Complexity:** ⭐⭐⭐ High

**Features:**
- Export to PNG (rendered tilemap image)
- Export collision data separately (JSON/RON)
- Export metadata (CSV, JSON, TOML)
- Optimized `.bscene.ron` binary format

---

### 7.2 External Tool Integration
**Complexity:** ⭐⭐⭐⭐⭐ Very High (40+ hours)

**Features:**
- **Import from Tiled:** `.tmx` / `.tmj` support
- **Import from LDtk:** `.ldtk` level editor
- **Export to Godot:** `.tscn` scene format
- Automated asset path conversion
- Property mapping configuration

---

## Implementation Priority Order

### 🚀 Sprint 1: Core Tile Tools (Weeks 1-2)
**Goal:** Dramatically improve tile painting workflow

- [ ] Rectangle fill tool (1.1) - 8-12h
- [ ] Multi-tile stamp brush (1.2) - 16-20h
- [ ] Tileset rectangle selection UI (2.1 partial) - 6-8h
- [ ] Basic keyboard shortcuts (4.1 partial) - 4-6h
  - Tool selection shortcuts (B, R, F, L, E)
  - Grid toggle (G)

**Estimated Total:** 34-46 hours (~2 weeks)

---

### 🛠️ Sprint 2: Essential Workflow (Weeks 3-4)
**Goal:** Complete core editing experience

- [ ] Bucket fill tool (1.3) - 12-16h
- [ ] Eyedropper tool (1.5) - 4-6h
- [ ] **Undo/redo system** (4.2) - 20-28h ⚠️ Critical!
- [ ] Line tool (1.4) - 6-8h
- [ ] Remaining keyboard shortcuts (4.1) - 6-8h

**Estimated Total:** 48-66 hours (~2 weeks)

---

### 🎨 Sprint 3: Polish & Productivity (Weeks 5-6)
**Goal:** Professional editor feel

- [ ] Copy/paste support (4.3) - 12-16h
- [ ] Layer opacity (3.1 partial) - 4-6h
- [ ] Grid customization (4.4) - 6-10h
- [ ] Tileset panel improvements (2.1 complete) - 4-6h
- [ ] Keyboard shortcut refinement (4.1) - 4-6h
- [ ] Bug fixes & polish - 8-12h

**Estimated Total:** 38-56 hours (~2 weeks)

---

### 🏗️ Sprint 4+: Advanced Features (Week 7+)
**Post-MVP features - prioritize based on user feedback**

- Auto-tiling system (3.3)
- Tile animations (2.2)
- Tile properties (2.3)
- Layer groups (3.2)
- Object templates (5.1)
- Multi-scene editing (6.1)

---

## Success Metrics

### MVP Success Criteria (End of Sprint 3)
- ✅ All Sprint 1-3 features implemented
- ✅ Undo/redo working reliably for all operations
- ✅ Keyboard shortcuts documented and functional
- ✅ Editor doesn't crash during normal workflow
- ✅ Scenes save/load without data loss
- ✅ Performance: 60fps with 10,000+ tiles on-screen

### Quality Metrics
- **Code Coverage:** >70% for core systems
- **Performance:** <16ms per frame for all tools
- **Memory:** <500MB RAM for typical project
- **Stability:** Zero crashes in 1-hour test session

---

## Technical Debt & Refactoring

### High Priority Refactors
1. **Separate tile painting from editor state** - Currently tightly coupled
2. **Extract rendering into dedicated system** - Improve testability
3. **Centralize event handling** - Too many scattered event readers
4. **Resource management** - Some resources should be local state

### Code Quality Improvements
- Add doc comments to all public APIs
- Write unit tests for algorithms (flood fill, line drawing)
- Add integration tests for undo/redo
- Performance benchmarks for large tilemaps

---

## Questions & Decisions Needed

### Design Decisions
- **Undo history limit:** 50 actions good default? Configurable?
- **Stamp rotation:** 90° increments only or free rotation?
- **Tile animations:** Frame-based or time-based?
- **Auto-tiling:** Which algorithm? Wang tiles? Blob patterns?

### Technical Decisions
- **Command serialization:** For save/replay of editing sessions?
- **Tileset format:** Support external .tsx files like Tiled?
- **Layer limit:** Hard limit or unlimited layers?
- **File format versioning:** How to handle `.bscene` migrations?

---

## Future Considerations

### Post-1.0 Features
- Scripting support (Lua/WASM)
- Collaborative editing (multiplayer editor)
- Version control integration (Git diff for scenes)
- Asset pipeline (texture packing, atlas generation)
- Mobile/touch support
- VR/AR editing mode (aspirational!)

### Platform Support
- Windows: ✅ Primary target
- Linux: 🟡 Should work (needs testing)
- macOS: 🟡 Should work (needs testing)
- Web (WASM): 🔴 Not priority but possible

---

## Resources & References

### Inspirational Tools
- **Tiled Map Editor:** https://www.mapeditor.org/
- **Godot TileMap:** https://docs.godotengine.org/en/stable/tutorials/2d/using_tilemaps.html
- **LDtk:** https://ldtk.io/
- **Aseprite:** (for general editor UX patterns)

### Technical References
- Bevy ECS patterns
- egui best practices
- Flood fill algorithms
- Wang tile systems

---

## Maintenance & Updates

This roadmap should be reviewed and updated:
- ✅ After each sprint (adjust priorities)
- ✅ When new features are requested
- ✅ When technical blockers are discovered
- ✅ Quarterly for long-term planning

**Document Owner:** Development Team
**Review Cycle:** Bi-weekly during active development

---

*Last updated: 2025-10-03*
*Next review: End of Sprint 1*
