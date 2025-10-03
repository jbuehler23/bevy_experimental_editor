# Testing the Networked LDTk Platformer

## Prerequisites

1. **SpacetimeDB running**: Make sure SpacetimeDB is running locally on port 3000
2. **Server deployed**: The platformer server should be built and published
3. **Client bindings generated**: SpacetimeDB client bindings should be generated

## Quick Start

### 1. Start SpacetimeDB (if not already running)
```bash
spacetime start
```

### 2. Deploy the Server
```bash
cd platformer-server
spacetime build
spacetime publish --clear-database ldtk-platformer
```

### 3. Generate Client Bindings
```bash
spacetime generate --lang rust --out-dir ../platformer-client/src/stdb --project-path .
```

### 4. Run Multiple Clients
```bash
cd ../platformer-client

# Terminal 1 - Alice
cargo run -- "Alice"

# Terminal 2 - Bob
cargo run -- "Bob"

# Terminal 3 - Charlie
cargo run -- "Charlie"
```

## Expected Behavior

### Connection and Spawning
1. **Client connects**: Should see "Connected to SpacetimeDB" message
2. **Player spawns**: Should see "Player inserted" and "Local player entered game!"
3. **Multiple players**: Each new client should appear on all connected clients

### Movement and Physics
1. **Movement**: Use A/D or Arrow keys to move left/right
2. **Jumping**: Use Space to jump
3. **Gravity**: Players fall and land on platforms
4. **Real-time sync**: All players see each other's movement in real-time

### Combat
1. **Attack**: Press Z or X to attack
2. **Range-based**: Only hits monsters in front of player within range
3. **Monster health**: Monsters take damage and die when health reaches 0
4. **Logging**: Attack messages appear in console

### World Elements
1. **Platforms**: Brown rectangular platforms for collision
2. **Monsters**: Green slime monsters that patrol back and forth
3. **Camera**: Follows local player smoothly

## Controls

- **Movement**: A/D or Arrow Left/Right
- **Jump**: Space
- **Attack**: Z or X

## Debugging

### Connection Issues
- Check SpacetimeDB is running: `spacetime status`
- Check server is deployed: `spacetime list`
- Check console for "Connected to SpacetimeDB" message

### No Player Movement
- Check "Local player entered game!" message appears
- Verify input is being sent: should see physics update calls
- Check player position updates in server logs

### No Other Players Visible
- Make sure multiple clients are running
- Check "Player inserted" messages for remote players
- Verify subscription is working

### Performance Issues
- Physics updates run at 20Hz from client
- Input is sent at 20Hz
- SpacetimeDB handles all game logic server-side

## Architecture Notes

### Client-Server Split
- **Server (SpacetimeDB)**: All game logic, physics, collision detection
- **Client (Bevy)**: Rendering, input handling, interpolation

### Physics Updates
- Client sends input at 20Hz
- Client calls physics update at 20Hz
- Server processes physics and sends position updates
- Client interpolates between server updates for smoothness

### Multiplayer Sync
- All players and monsters synchronized via SpacetimeDB tables
- Real-time updates via SpacetimeDB subscriptions
- Authoritative server prevents cheating

## Known Limitations

1. **No LDTk Integration Yet**: Currently uses hardcoded platforms
2. **Simple Physics**: Basic collision detection only
3. **No Animations**: Static sprites only
4. **Basic AI**: Monsters just patrol back and forth

## Next Steps

1. **LDTk Integration**: Load collision data from LDTk files
2. **Sprite Animations**: Add movement and attack animations
3. **Skills System**: Implement RuneScape-style skill leveling
4. **Equipment**: Add weapons and armor affecting stats
5. **Advanced AI**: More complex monster behaviors
6. **Persistence**: Save player progress to database

## Troubleshooting

### "Failed to connect" errors
```bash
# Check SpacetimeDB status
spacetime status

# Restart if needed
spacetime stop
spacetime start
```

### "Module not found" errors
```bash
# Redeploy the server
cd platformer-server
spacetime publish --clear-database ldtk-platformer
```

### "Reducer not found" errors
```bash
# Regenerate client bindings
spacetime generate --lang rust --out-dir ../platformer-client/src/stdb --project-path .
```

### Build errors
```bash
# Clean and rebuild
cargo clean
cargo build
```