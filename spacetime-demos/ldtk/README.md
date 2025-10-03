# LDTk Platformer with SpacetimeDB

A simple networked 2D platformer example demonstrating how to build a MapleStory-like MMORPG using:
- **SpacetimeDB** for authoritative server and networking
- **Bevy** for the game client
- **LDTk** for level design
- **bevy_ecs_ldtk** for loading LDTk levels in Bevy

## Architecture Overview

This example demonstrates the recommended architecture for a MapleStory-style game:

### Server (SpacetimeDB)
- Authoritative game state
- Physics simulation at 20Hz
- Player management and authentication
- Collision detection
- Combat and damage calculations
- Skill and equipment systems (ready to extend)

### Client (Bevy)
- Renders the game world using LDTk levels
- Sends player input to server
- Interpolates entity positions for smooth movement
- Displays health bars and player names
- Camera follows local player

## Setup Instructions

### 1. Build and Deploy the Server

```bash
cd platformer-server

# Build the server module (must use spacetime build, not cargo build)
spacetime build

# Deploy to local SpacetimeDB (make sure spacetime is running)
spacetime publish --clear-database platformer-server

# Generate client bindings
spacetime generate --lang rust --out-dir ../platformer-client/src/stdb --project-path .
```

### 2. Run the Client

```bash
cd platformer-client

# Run with default username
cargo run

# Or run with a custom username
cargo run -- "PlayerName"

# Run multiple clients to test multiplayer
cargo run -- "Alice" &
cargo run -- "Bob"
```

## Game Features

### Current Implementation
- **Multiplayer**: Multiple players can connect and see each other
- **Platforming**: Gravity, jumping, and platform collisions
- **Movement**: A/D or Arrow keys to move, Space to jump
- **Combat**: Z or X to attack (simple melee)
- **Monsters**: Basic AI monsters that patrol platforms
- **Health System**: Players and monsters have health bars
- **Physics**: Server-authoritative physics at 20Hz

### Ready to Extend
- **Skills**: Skill table ready for RuneScape-like leveling
- **Equipment**: Equipment table for weapons and armor
- **Crafting**: Easy to add with SpacetimeDB tables
- **LDTk Integration**: Collision extraction from LDTk levels

## Project Structure

```
platformer-server/
├── src/
│   └── lib.rs          # SpacetimeDB server implementation
└── Cargo.toml

platformer-client/
├── src/
│   ├── main.rs         # Bevy app setup
│   ├── components.rs   # Game components
│   ├── stdb/           # Generated SpacetimeDB bindings
│   └── systems/        # Game systems
│       ├── camera.rs       # Camera following
│       ├── ldtk_system.rs  # LDTk level loading
│       ├── player_input.rs # Input handling
│       └── sync_system.rs  # Entity synchronization
├── assets/
│   └── *.ldtk          # LDTk level files
└── Cargo.toml
```

## Key Design Decisions

### Why SpacetimeDB over Lightyear?
- **Perfect for RPG mechanics**: Built-in database for inventory, skills, quests
- **Automatic state sync**: No manual networking code needed
- **Authentication**: Built-in identity management
- **MapleStory-style latency**: ~100ms latency is acceptable for this genre

### LDTk Loading Strategy
- Client loads and renders the full LDTk level
- Collision geometry is extracted and sent to server
- Server uses collision data for authoritative physics
- Visual-only elements stay client-side

## Extending the Game

### Adding Skills (RuneScape-style)
```rust
// Add to server lib.rs
#[spacetimedb::table(name = skill)]
pub struct Skill {
    #[primary_key]
    pub id: u64,
    pub player_identity: Identity,
    pub skill_type: String,  // "mining", "crafting", etc.
    pub level: u32,
    pub experience: u64,
}
```

### Adding Equipment
```rust
#[spacetimedb::table(name = equipment)]
pub struct Equipment {
    #[primary_key]
    pub id: u64,
    pub player_identity: Identity,
    pub slot: String,  // "weapon", "armor", etc.
    pub item_id: u32,
    pub damage_bonus: i32,
    pub defense_bonus: i32,
}
```

### Adding Crafting
```rust
#[spacetimedb::table(name = recipe)]
pub struct Recipe {
    #[primary_key]
    pub id: u32,
    pub required_skill: String,
    pub required_level: u32,
    pub ingredients: Vec<u32>,
    pub output_item: u32,
}
```

## Performance Considerations

- **Server tick rate**: 20Hz is sufficient for MapleStory-style gameplay
- **Client interpolation**: Smooths movement between server updates
- **LDTk optimization**: Only collision tiles sent to server
- **Entity culling**: Only sync entities near players (not implemented yet)

## Known Limitations

- Physics is simplified (basic AABB collision)
- No sprite animations yet (easy to add with Bevy)
- LDTk collision extraction is basic (improve for complex levels)
- No persistent storage yet (add with SpacetimeDB persistence)

## Next Steps

1. **Complete SpacetimeDB Integration**: Uncomment and test the SpacetimeDB connection code
2. **Add Animations**: Use Bevy's sprite animation system
3. **Implement Skills**: Add mining, crafting, combat skills
4. **Add Items**: Implement inventory and equipment system
5. **World Events**: Add boss spawns, events using SpacetimeDB schedulers
6. **Polish**: Add particle effects, sound, and UI