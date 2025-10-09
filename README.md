# Bevy Experimental Editor

An experimental, in-development game editor for the Bevy Engine built with Bevy itself. Uses bevy CLI and templates for building/running/creating new projects.

> **WARNING - Early Development**: This editor is in active development and many features are incomplete or experimental.

## Features

### Currently Implemented

- **Scene Editor**
  - Visual scene editing with viewport
  - Entity selection and manipulation
  - Transform gizmos (Move/Rotate/Scale modes with Q/W/E shortcuts)
  - Scene tree panel showing entity hierarchy
  - Inspector panel for component editing

- **Undo/Redo System**
  - Full history tracking for all editor operations
  - Command pattern implementation
  - Keyboard shortcuts (Ctrl+Z, Ctrl+Shift+Z, Ctrl+Y)

- **Multi-Panel Interface**
  - Asset browser
  - Project browser
  - Scene hierarchy
  - Inspector
  - CLI output panel
  - Tileset manager
  - Layer management

- **Project Management**
  - Project creation wizard
  - Scene loading/saving
  - Asset management

- **Tilemap Support**
  - Tilemap editing with brush tools
  - Tileset management
  - Layer system
  - Collision editor

### Planned Features

See [docs/EDITOR_ROADMAP.md](docs/EDITOR_ROADMAP.md) for the full development roadmap covering:
- Asset pipeline
- Physics integration
- Prefabs & scenes
- Visual scripting
- Particle editor
- UI builder
- Play mode testing

## Getting Started

### Prerequisites

- Rust 1.90+ (stable toolchain)
- Windows/Linux/macOS

### Installation

```bash
# Clone the repository
git clone <repository-url>
cd bevy_experimental_editor

# Run the editor
bevy run
```

### Quick Start

1. Launch the editor
2. Create a new project or open an existing one
3. Use the viewport to edit your scene
4. Select entities with left-click
5. Transform entities using gizmos:
   - **Q**: Move mode
   - **W**: Rotate mode
   - **E**: Scale mode
6. Save your scene with Ctrl+S

## Architecture

The editor is built as a Cargo workspace with the following structure:

```
crates/
└── bevy_editor/          # Main editor application
    ├── src/
    │   ├── formats/      # File format definitions
    │   ├── main.rs       # Entry point
    │   └── ...           # Editor modules
    └── Cargo.toml
```

### Key Modules

- **formats/**: Serialization formats for scenes, levels, and projects
- **gizmos.rs**: Visual manipulation tools (move/rotate/scale)
- **viewport_selection.rs**: Entity selection and gizmo interaction
- **editor_history.rs**: Undo/redo system using command pattern
- **editor_commands.rs**: All undoable editor commands
- **scene_editor.rs**: Main scene editing logic
- **inspector_panel.rs**: Component inspector UI

## Development

### Running Tests

```bash
cargo test --workspace
```

### Building for Release

```bash
cargo build --release --package bevy_editor
```

### Code Style

The project follows standard Rust formatting:

```bash
cargo fmt --all
cargo clippy --workspace -- -D warnings
```

## Contributing

This is an experimental project and contributions are welcome! Areas that need help:

- Asset pipeline implementation
- Physics integration
- Performance optimization
- Documentation
- Bug fixes

Please check the [roadmap](docs/EDITOR_ROADMAP.md) to see planned features and current progress.

## License

Licensed under either:

- MIT License
- Apache License 2.0

at your option.

## Acknowledgments

Built with:
- [Bevy](https://bevyengine.org/) - Game engine and ECS framework
- [egui](https://github.com/emilk/egui) - Immediate mode GUI (via bevy_egui)
- [bevy-inspector-egui](https://github.com/jakobhellermann/bevy-inspector-egui) - Inspector widgets
- [bevy_ecs_tilemap](https://github.com/StarArawn/bevy_ecs_tilemap) - Tilemap rendering

