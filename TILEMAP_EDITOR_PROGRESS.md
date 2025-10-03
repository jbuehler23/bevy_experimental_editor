# Eryndor Tilemap Editor - Implementation Progress

## Overview
Transforming the Eryndor editor into a feature-rich tilemap editor with:
- Tileset painting using bevy_ecs_tilemap
- Custom entity classes with user-defined fields
- Collision layer painting (IntGrid-style)
- SpacetimeDB integration for MMO functionality
- Admin client workflow for world uploads

---

## ✅ Completed Phases

### Phase 1: SpacetimeDB Schema Design

**Status**: ✅ Complete

Created comprehensive database schema with 7 new tables optimized for bevy_ecs_tilemap compatibility:

#### Tables Created

1. **`tileset`** - [tileset.rs](crates/eryndor-server-spacetimedb/src/tables/tileset.rs)
   - Stores tileset metadata (texture path, dimensions, grid layout)
   - Fields: id, identifier, texture_path, tile_width, tile_height, columns, rows, spacing, padding

2. **`tilemap_layer`** - [tilemap_layer.rs](crates/eryndor-server-spacetimedb/src/tables/tilemap_layer.rs)
   - Layer metadata compatible with bevy_ecs_tilemap
   - Fields: id, level_id, identifier, layer_type, tileset_id, grid_size, width, height, z_index, opacity, parallax_x, parallax_y

3. **`tilemap_tile`** - [tilemap_tile.rs](crates/eryndor-server-spacetimedb/src/tables/tilemap_tile.rs)
   - Individual tile placements
   - Fields: id (auto-inc), layer_id, x, y, tile_id, flip_x, flip_y

4. **`intgrid_cell`** - [intgrid_cell.rs](crates/eryndor-server-spacetimedb/src/tables/intgrid_cell.rs)
   - Collision/physics layer data (LDTk-style)
   - Fields: id (auto-inc), layer_id, x, y, value

5. **`entity_definition`** - [entity_definition.rs](crates/eryndor-server-spacetimedb/src/tables/entity_definition.rs)
   - Custom entity classes with user-defined fields
   - Fields: id, identifier, width, height, color, field_definitions (JSON)

6. **`entity_instance`** - [entity_definition.rs](crates/eryndor-server-spacetimedb/src/tables/entity_definition.rs)
   - Entity placements with custom field values
   - Fields: id (auto-inc), level_id, entity_def_id, x, y, field_values (JSON)

7. **`enum_definition`** - [enum_definition.rs](crates/eryndor-server-spacetimedb/src/tables/enum_definition.rs)
   - Custom enums for entity fields
   - Fields: id, identifier, values (JSON array)

8. **`world_metadata`** - [world_metadata.rs](crates/eryndor-server-spacetimedb/src/tables/world_metadata.rs)
   - World configuration and full editor export
   - Fields: id, version, last_updated, editor_data (full JSON export)

#### Reducers Created

- **`upload_world_data`** - [upload_world_data.rs](crates/eryndor-server-spacetimedb/src/reducers/upload_world_data.rs)
  - Parses WorldExport JSON
  - Clears existing tilemap data
  - Inserts tilesets, layers, tiles, intgrid cells, entity definitions, entity instances, enums
  - Stores raw export in world_metadata for re-importing

---

### Phase 2: Common Types for Tilemap System

**Status**: ✅ Complete

Created shared data structures in `eryndor-common` crate for use across editor, server, and client:

#### Modules Created

1. **`tilemap.rs`** - [tilemap.rs](crates/eryndor-common/src/tilemap.rs)
   - `TilesetData` - Tileset configuration
   - `LayerType` - Enum: Tiles, IntGrid, Entities, AutoLayer
   - `LayerMetadata` - Layer configuration
   - `TileData` - Individual tile with position and flip flags
   - `IntGridCellData` - Collision cell data
   - `IntGridValue` - IntGrid value definitions (e.g., 1=solid, 2=ladder)
   - `LayerData` - Complete layer with tiles and metadata

2. **`entity_definition.rs`** - [entity_definition.rs](crates/eryndor-common/src/entity_definition.rs)
   - `FieldType` - Enum for custom field types (Int, Float, String, Bool, Enum, Color, Point, Array)
   - `CustomField` - Field definition with type and default value
   - `FieldValue` - Runtime value for custom fields
   - `EntityDefinitionData` - Complete entity class definition
   - `EntityInstanceData` - Entity placement with field values
   - `EnumDefinitionData` - Custom enum definition
   - Helper methods for JSON serialization (SpacetimeDB storage)

3. **`world_export.rs`** - [world_export.rs](crates/eryndor-common/src/world_export.rs)
   - `WorldExport` - Top-level export format
   - `LayerExportData` - Layer with tiles for export
   - `TileExportData` - Tile export format
   - `IntGridCellExportData` - IntGrid cell export format
   - `WorldMetadataExport` - World metadata
   - Methods: `save_to_file()`, `load_from_file()`, `to_json()`, `from_json()`

#### Features

- **Type-safe**: Strongly typed field system with validation
- **Serializable**: Full serde support for JSON import/export
- **Builder pattern**: Fluent API for constructing world data
- **SpacetimeDB compatible**: JSON serialization for storing complex data in string fields

---

## 🔄 In Progress

### Compilation Verification

Currently verifying that:
- ✅ eryndor-common compiles with new types
- ✅ eryndor-server-spacetimedb compiles with new tables and reducers
- ⏳ No dependency conflicts

---

## 📋 Remaining Phases

### Phase 3: Tileset Manager & UI

**Goal**: Add tileset loading and tile painting to editor

**Tasks**:
- Create `TilesetManager` resource
- Implement tileset loading from PNG files
- Add tileset palette UI panel
- Implement tile selection/brush tool
- Add tile painting system
- Support tile flipping (X/Y)

**Files to Create**:
- `crates/eryndor-editor/src/tileset_manager.rs`
- `crates/eryndor-editor/src/tile_painter.rs`
- Update `crates/eryndor-editor/src/ui.rs` (add Tilesets tab)

---

### Phase 4: Layer System

**Goal**: Multi-layer rendering with z-ordering

**Tasks**:
- Create `LayerManager` resource
- Implement layer list UI
- Add layer visibility toggles
- Implement layer opacity controls
- Support layer reordering (z-index)
- Render multiple layers simultaneously

**Files to Create**:
- `crates/eryndor-editor/src/layer_manager.rs`
- Update `crates/eryndor-editor/src/ui.rs` (add Layers tab)

---

### Phase 5: IntGrid/Collision System

**Goal**: Paint collision layers like LDTk

**Tasks**:
- Create `IntGridEditor` module
- Define IntGrid value palette (solid, ladder, etc.)
- Implement collision painting tool
- Add visual color-coding for IntGrid values
- Export IntGrid data to world format

**Files to Create**:
- `crates/eryndor-editor/src/intgrid_editor.rs`
- Update `crates/eryndor-editor/src/ui.rs` (collision tools)

---

### Phase 6: Custom Entity Classes

**Goal**: LDTk-style entity definitions with custom fields

**Tasks**:
- Create `EntityClassEditor` UI
- Implement custom field system (add/remove/edit)
- Add field type selector (Int, Float, String, Bool, Enum, Color, Point, Array)
- Implement field constraints (min/max, default values)
- Create enum editor
- Add entity inspector for editing field values

**Files to Create**:
- `crates/eryndor-editor/src/entity_class_editor.rs`
- Update `crates/eryndor-editor/src/ui.rs` (Entities tab)

---

### Phase 7: Export/Import System

**Goal**: Save and load world data in JSON format

**Tasks**:
- Implement `WorldExport` generation from editor state
- Add "Export World" button
- Implement "Import World" functionality
- Add file dialogs for export/import
- Validate imported data
- Support versioning

**Files to Create**:
- `crates/eryndor-editor/src/export_import.rs`
- Update `crates/eryndor-editor/src/ui.rs` (File menu, Project tab)

---

### Phase 8: Admin Client

**Goal**: Upload world data to SpacetimeDB

**Tasks**:
- Create new binary crate `eryndor-admin`
- Add SpacetimeDB connection
- Implement world upload via `upload_world_data` reducer
- Add progress monitoring
- Handle upload errors gracefully

**Files to Create**:
- `crates/eryndor-admin/` (new crate)
- `crates/eryndor-admin/src/main.rs`
- `crates/eryndor-admin/Cargo.toml`

---

### Phase 9: Client Integration

**Goal**: Load tilemaps from SpacetimeDB using bevy_ecs_tilemap

**Tasks**:
- Add bevy_ecs_tilemap dependency to client
- Subscribe to tilemap tables
- Implement tilemap loading from DB
- Spawn bevy_ecs_tilemap entities
- Generate colliders from IntGrid layer
- Support layer parallax and opacity

**Files to Create**:
- `crates/eryndor-client/src/tilemap_loader.rs`
- `crates/eryndor-client/src/collision_generator.rs`
- Update `crates/eryndor-client/Cargo.toml`

---

### Phase 10: Testing & Polish

**Goal**: End-to-end workflow validation

**Tasks**:
- Test editor → export → admin upload → client load workflow
- Optimize tile batching for performance
- Add keyboard shortcuts
- Improve UI/UX
- Write documentation
- Create example world

---

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                     Eryndor Editor                          │
│  - Tileset Manager                                          │
│  - Layer System (Tiles, IntGrid, Entities)                  │
│  - Entity Class Editor                                      │
│  - Collision Painter                                        │
│                                                             │
│         ↓ Export WorldExport JSON                           │
└─────────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────────┐
│                    Eryndor Admin Client                     │
│  - Load WorldExport JSON                                    │
│  - Connect to SpacetimeDB                                   │
│  - Call upload_world_data reducer                           │
└─────────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────────┐
│                     SpacetimeDB Tables                      │
│  - tileset, tilemap_layer, tilemap_tile                    │
│  - intgrid_cell, entity_definition, entity_instance        │
│  - enum_definition, world_metadata                          │
└─────────────────────────────────────────────────────────────┘
                           ↓ Subscribe
┌─────────────────────────────────────────────────────────────┐
│                     Game Clients                            │
│  - Subscribe to tilemap tables                              │
│  - Spawn bevy_ecs_tilemap entities                          │
│  - Generate colliders from IntGrid                          │
│  - Real-time updates                                        │
└─────────────────────────────────────────────────────────────┘
```

---

## Dependencies to Add

### Editor
```toml
bevy_ecs_tilemap = "0.15"  # For tilemap preview rendering
image = "0.25"  # For tileset image loading
```

### Client
```toml
bevy_ecs_tilemap = "0.15"  # For tilemap rendering
bevy_rapier2d = "0.28"     # Or bevy_xpbd_2d for physics
```

### Admin
```toml
bevy = { workspace = true, default-features = false, features = ["minimal"] }
bevy-spacetimedb = "0.1"
serde_json = "1.0"
```

---

## File Structure

```
eryndor-mmo/
├── crates/
│   ├── eryndor-common/
│   │   └── src/
│   │       ├── tilemap.rs ✅
│   │       ├── entity_definition.rs ✅
│   │       └── world_export.rs ✅
│   ├── eryndor-editor/
│   │   └── src/
│   │       ├── tileset_manager.rs (TODO)
│   │       ├── layer_manager.rs (TODO)
│   │       ├── tile_painter.rs (TODO)
│   │       ├── intgrid_editor.rs (TODO)
│   │       ├── entity_class_editor.rs (TODO)
│   │       └── export_import.rs (TODO)
│   ├── eryndor-admin/ (TODO - new crate)
│   ├── eryndor-client/
│   │   └── src/
│   │       ├── tilemap_loader.rs (TODO)
│   │       └── collision_generator.rs (TODO)
│   └── eryndor-server-spacetimedb/
│       └── src/
│           ├── tables/ ✅
│           │   ├── tileset.rs ✅
│           │   ├── tilemap_layer.rs ✅
│           │   ├── tilemap_tile.rs ✅
│           │   ├── intgrid_cell.rs ✅
│           │   ├── entity_definition.rs ✅
│           │   ├── enum_definition.rs ✅
│           │   └── world_metadata.rs ✅
│           └── reducers/ ✅
│               └── upload_world_data.rs ✅
```

---

## Key Design Decisions

### 1. JSON-based Storage
- Custom fields stored as JSON strings in SpacetimeDB
- Allows flexible schema without database migrations
- Easy to version and validate

### 2. bevy_ecs_tilemap Compatibility
- Table structure matches bevy_ecs_tilemap requirements
- Direct mapping from DB to tilemap entities
- Efficient batch loading

### 3. LDTk-inspired Architecture
- IntGrid for collision layers
- Custom entity classes with fields
- Layer types (Tiles, IntGrid, Entities, AutoLayer)
- Familiar workflow for level designers

### 4. Separation of Concerns
- **Editor**: Create and edit worlds (offline)
- **Admin Client**: Upload worlds to server
- **Game Clients**: Load and render worlds from server
- **Server**: Store and distribute world data

### 5. Type Safety
- Strongly typed Rust structs for all data
- Serde validation on import/export
- Compile-time guarantees where possible

---

## Next Steps

1. ✅ Verify compilation of server and common crates
2. Add bevy_ecs_tilemap dependency to editor
3. Implement TilesetManager resource
4. Create tileset UI panel
5. Implement tile painting tool

---

## Success Criteria

- [x] SpacetimeDB schema complete and compiling
- [x] Common types implemented and usable
- [ ] Editor can load tilesets
- [ ] Editor can paint tiles on multiple layers
- [ ] Editor can export world to JSON
- [ ] Admin client can upload to SpacetimeDB
- [ ] Game client can load and render tilemap
- [ ] IntGrid collision generation works
- [ ] End-to-end workflow tested
