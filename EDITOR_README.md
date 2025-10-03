# Eryndor Level Editor

A custom Bevy-based level editor for the Eryndor 2D side-scrolling MMORPG.

## Architecture

### Project Structure

```
crates/
├── eryndor-common/          # Shared types and components
│   ├── components.rs        # Game components (Player, NPC, Resource, etc.)
│   ├── entities.rs          # Entity type definitions and spawn configs
│   ├── level_format.rs      # Level data serialization format
│   └── math.rs              # Math types compatible with Bevy and SpacetimeDB
│
├── eryndor-editor/          # Level editor application
│   ├── main.rs              # Editor entry point
│   ├── systems.rs           # Editor systems (placement, editing)
│   └── ui.rs                # Editor UI (entity palette, tools)
│
├── eryndor-server-spacetimedb/  # Authoritative game server
│   └── lib.rs               # SpacetimeDB tables and reducers
│
└── eryndor-client/          # Game client (Bevy)
    └── main.rs              # Client application

assets/
└── levels/
    └── *.json               # Level data files
```

## Data Flow

### 1. Level Editing (Editor → JSON)

The editor allows you to:
- Place platforms/collision geometry
- Spawn entities (NPCs, resources, interactive objects)
- Configure entity properties
- Save levels as JSON files

**Level JSON Format:**
```json
{
  "metadata": { "name": "...", "version": "..." },
  "platforms": [
    { "position": {"x": 0, "y": 0}, "size": {"x": 100, "y": 20}, "is_one_way": false }
  ],
  "entities": [
    {
      "entity_type": {"Npc": "Hostile"},
      "position": {"x": 100, "y": 50},
      "properties": {
        "type": "Npc",
        "name": "Goblin",
        "npc_type": "Hostile",
        "max_health": 50,
        "patrol_points": [...]
      }
    }
  ],
  "world_bounds": { "min": {...}, "max": {...} },
  "background_layers": []
}
```

### 2. Server Initialization (JSON → SpacetimeDB)

On server startup (`init()` reducer):
1. Server loads level JSON from `assets/levels/`
2. Parses JSON into `LevelData` struct (from `eryndor-common`)
3. Populates SpacetimeDB tables:
   - `platform` - Static collision geometry
   - `npc` - NPC instances with patrol points
   - `resource_node` - Resource nodes (trees, rocks, ore)
   - `interactive_object` - Doors, chests, levers
   - `spawn_point` - Player and enemy spawn locations

### 3. Runtime State (SpacetimeDB)

**Static Data (loaded once):**
- Platforms
- NPC spawn configs
- Resource node locations
- Interactive object placements

**Dynamic Data (changes during gameplay):**
- Player positions, health, inventory
- NPC current position, AI state
- Resource node health, depletion state
- Interactive object activation state
- Loot drops, projectiles

### 4. Client Rendering (SpacetimeDB → Bevy)

Clients subscribe to SpacetimeDB tables and receive real-time updates:
- Render platforms as collision bodies
- Spawn Bevy entities for NPCs, resources, players
- Update entity positions via interpolation
- Visualize interactive objects

## Usage

### Running the Editor

```bash
cd crates/eryndor-editor
cargo run
```

**Controls:**
- **Tools Panel (Left):** Select between Select, Platform, Entity, and Erase tools
- **Entity Palette (Right):** Choose entities to place (NPCs, resources, spawn points)
- **Grid Snap:** Enable/disable in View menu
- **Save:** Ctrl+S (saves to current file)
- **Load:** Ctrl+O (opens file dialog - TODO)

### Workflow

1. **Create a New Level:**
   - Launch editor
   - File → New Level
   - Set world bounds in properties

2. **Build the World:**
   - Select Platform tool
   - Click to place platforms (hold Shift to drag)
   - Adjust size in properties panel

3. **Place Entities:**
   - Select Entity tool
   - Choose entity from palette (right panel)
   - Click to place in world
   - Edit properties in inspector

4. **Save Level:**
   - File → Save Level As...
   - Save to `assets/levels/your_level.json`

5. **Test in Game:**
   - Update server to load your level file
   - Publish server: `spacetime publish`
   - Run client and test gameplay

## SpacetimeDB Tables

### Platform
```rust
#[spacetimedb::table(name = platform)]
pub struct Platform {
    id: u32,
    position: DbVector2,
    size: DbVector2,
    is_one_way: bool,
}
```

### NPC
```rust
#[spacetimedb::table(name = npc)]
pub struct Npc {
    id: u32,
    name: String,
    npc_type: String,        // "Friendly", "Hostile", "Neutral", "Vendor"
    position: DbVector2,
    health: i32,
    max_health: i32,
    patrol_points: String,   // JSON-serialized Vec<Vector2>
    current_patrol_index: u32,
}
```

### ResourceNode
```rust
#[spacetimedb::table(name = resource_node)]
pub struct ResourceNode {
    id: u32,
    resource_type: String,   // "Tree", "Rock", "IronOre", etc.
    position: DbVector2,
    health: i32,
    max_health: i32,
    respawn_time: f32,
    is_depleted: bool,
}
```

### InteractiveObject
```rust
#[spacetimedb::table(name = interactive_object)]
pub struct InteractiveObject {
    id: u32,
    object_type: String,     // "Door", "Chest", "Lever", "Portal"
    position: DbVector2,
    is_active: bool,
}
```

### SpawnPoint
```rust
#[spacetimedb::table(name = spawn_point)]
pub struct SpawnPoint {
    id: u32,
    spawn_type: String,      // "PlayerStart", "EnemySpawn", "ItemSpawn"
    position: DbVector2,
    level_id: String,
}
```

## Future Enhancements

### Phase 2: Enhanced Editor Features
- [ ] Multi-select and transform tools
- [ ] Copy/paste entities
- [ ] Undo/redo system
- [ ] File browser dialog for open/save
- [ ] Drag to create platforms
- [ ] Visual grid rendering

### Phase 3: Advanced Features
- [ ] Multi-level/zone management
- [ ] Prefab system for reusable entity groups
- [ ] NPC behavior tree editor
- [ ] Path visualization for patrol routes
- [ ] Terrain painting/tilemap support
- [ ] Lighting and atmosphere editing
- [ ] Live editing (connect to running server)

### Phase 4: Bevy Editor Integration
- [ ] Extract editor as standalone crate
- [ ] Contribute to `bevy_editor_prototypes`
- [ ] Add plugin system for custom entity types
- [ ] Asset pipeline integration
- [ ] Scene hot-reloading

## Why Not LDTk/Godot?

**Advantages of Custom Bevy Editor:**
- ✅ **Code Sharing:** Uses exact game components in editor
- ✅ **Instant Testing:** Play Mode without recompilation
- ✅ **Type Safety:** Rust types ensure data consistency
- ✅ **SpacetimeDB Integration:** Direct connection to server
- ✅ **Extensibility:** Add game-specific tools easily
- ✅ **Future-Proof:** Can become official Bevy editor

**LDTk/Godot Limitations:**
- ❌ Requires custom export pipeline
- ❌ Different language/tooling
- ❌ Can't test with actual game code
- ❌ Manual sync of entity types

## Contributing to Bevy Ecosystem

This editor is designed to be extractable as a standalone tool. Key goals:

1. **Generic Entity System:** Works with any Bevy component
2. **Plugin Architecture:** Easy to add custom tools
3. **Format Agnostic:** Can export to any format
4. **Documentation:** Well-documented for contributors

Once mature, we plan to:
- Open-source as `bevy_level_editor` crate
- Contribute to `bevy_editor_prototypes`
- Collaborate with Bevy community

## License

MIT (planned - pending open-source release)
