# Development Guide

## Getting Started

This project was created with the Bevy Editor. You can use the editor to create and edit scenes, or develop the game standalone.

## Build Performance

### Initial Build Times
- **First build**: 5-15 minutes (compiling Bevy and all dependencies)
- **Subsequent builds**: 10-60 seconds (incremental compilation)

### Optimization Tips

#### 1. Fast Linker (Already Configured ✅)
- **Windows**: Using `rust-lld.exe` linker
- **Linux**: Can use `mold` linker (uncomment in `.cargo/config.toml`)
- **macOS**: Default linker is already optimal

#### 2. Build Caching with sccache (Recommended)
Install: `cargo install sccache`

Then uncomment in `.cargo/config.toml`:
```toml
[build]
rustc-wrapper = "sccache"
```

**Performance**: Reduces rebuild times by 50-80%!

#### 3. Dynamic Linking (Already Enabled ✅)
The `dev` feature enables `bevy/dynamic_linking`, which dramatically speeds up iterative builds.

#### 4. Incremental Compilation
Rust automatically uses incremental compilation. Clean your build with `cargo clean` only when necessary.

## Running the Game

```bash
# Dev build with hot-reloading (faster compilation)
cargo run

# Release build (slower compilation, better performance)
cargo run --release
```

## Editor Integration

### Opening in Editor
From the editor, use **File → Open Project** and select this directory.

### Creating Scenes
1. **File → New Scene** or click the ➕ tab button
2. Add entities and components using the editor UI
3. Save with **Ctrl+S**

### Running Scenes
- Click **"Play Scene"** in the editor to test the current scene with your game logic
- Click **"Run"** to run the full game normally

### Loading Scenes from Code
The `bevy_editor_runtime` plugin is enabled by default (via the `editor-runtime` feature).
It automatically loads scenes when you set the `BEVY_EDITOR_SCENE` environment variable:

```bash
BEVY_EDITOR_SCENE=level1 cargo run
```

This will load `assets/world/level1.scn.ron` when the game starts.

## Troubleshooting

### Out of Memory During Compilation
Reduce parallel jobs in `.cargo/config.toml`:
```toml
[build]
jobs = 4  # Reduce from default (number of CPU cores)
```

### Slow Incremental Builds
Try: `cargo clean && cargo build`

### Windows Linker Issues
Ensure LLVM tools are installed:
```bash
cargo install -f cargo-binutils
rustup component add llvm-tools
```

## Project Structure

```
{{PROJECT_NAME}}/
├── src/
│   └── main.rs          # Main game code
├── assets/
│   └── world/           # Editor scene files (.scn.ron)
├── .cargo/
│   └── config.toml      # Build configuration
├── Cargo.toml           # Project dependencies
├── project.bvy          # Editor configuration
└── DEVELOPMENT.md       # This file
```

## Adding Game Logic

Edit `src/main.rs` to add your game systems:

```rust
// Add to main():
app.add_systems(Update, my_game_system);

// Define your system:
fn my_game_system(
    // Add queries, resources, etc.
) {
    // Game logic here
}
```

## Performance Profiling

To profile your game with Tracy:
1. Remove `max_level_debug` and `release_max_level_warn` from `tracing` dependency
2. Add `tracy` feature to Bevy
3. Run with `cargo run --release --features bevy/trace_tracy`

See: https://github.com/bevyengine/bevy/blob/main/docs/profiling.md

## Resources

- [Bevy Documentation](https://bevyengine.org/learn/)
- [Bevy Assets](https://bevyengine.org/assets/)
- [Bevy Discord](https://discord.gg/bevy)
- [Bevy Examples](https://github.com/bevyengine/bevy/tree/main/examples)
