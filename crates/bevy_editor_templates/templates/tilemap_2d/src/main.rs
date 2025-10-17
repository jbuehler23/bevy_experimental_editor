//! {{PROJECT_NAME}} - A 2D Tilemap-based Bevy game
//!
//! This template includes:
//! - Tilemap rendering with bevy_ecs_tilemap
//! - Scene loading from .bscene files created in the editor
//! - Command-line arguments for project/level selection
//! - Hot-reloading support for assets

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use clap::Parser;

/// Game command line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the project directory
    #[arg(short, long)]
    project_path: Option<String>,

    /// Specific level file to load
    #[arg(short, long)]
    level: Option<String>,
}

fn main() {
    let args = Args::parse();

    let mut app = App::new();

    // Configure asset path if project path is provided
    let asset_plugin = if let Some(ref project_path) = args.project_path {
        let assets_path = std::path::Path::new(project_path).join("assets");
        AssetPlugin {
            file_path: assets_path.to_string_lossy().to_string(),
            watch_for_changes_override: Some(true),
            ..default()
        }
    } else {
        AssetPlugin {
            watch_for_changes_override: Some(true),
            ..default()
        }
    };

    // Add default Bevy plugins with custom window and asset settings
    app.add_plugins((
        DefaultPlugins
            .set(asset_plugin)
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "{{PROJECT_NAME}}".to_string(),
                    resolution: (1280.0, 720.0).into(),
                    ..default()
                }),
                ..default()
            }),
        TilemapPlugin,
    ));

    // Add the editor scene loader plugin (optional, controlled by feature flag)
    #[cfg(feature = "editor-runtime")]
    {
        app.add_plugins(bevy_editor_runtime::EditorSceneLoaderPlugin);
        info!("Editor scene loader enabled - set BEVY_EDITOR_SCENE to load a scene");
    }

    // Store level to load
    if let Some(level) = args.level {
        app.insert_resource(LevelToLoad(level));
    }

    // Add game systems
    app.add_systems(Startup, setup);

    app.run();
}

/// Resource to track which level should be loaded
#[derive(Resource)]
struct LevelToLoad(String);

/// Initial setup - spawns camera
fn setup(mut commands: Commands) {
    // Spawn a 2D camera
    commands.spawn((
        Camera2d,
        Name::new("Main Camera"),
    ));

    info!("{{PROJECT_NAME}} initialized!");
    info!("Create tilemaps in the editor and they will be rendered here.");
    info!("");
    info!("Usage:");
    info!("  --project-path <path>  : Path to project directory");
    info!("  --level <name>         : Level file to load");
}

// ============================================================================
// TILEMAP RENDERING
// ============================================================================
//
// The editor saves tilemap data in .bscene files. To render these in your game:
//
// 1. Load the .bscene file using Bevy's asset system
// 2. Parse the tilemap data from the scene
// 3. Use bevy_ecs_tilemap to create the tilemap entities
//
// Example tilemap rendering system:
//
// fn render_tilemap(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     // Your tilemap data resource here
// ) {
//     // Load tileset texture
//     let texture_handle: Handle<Image> = asset_server.load("tilesets/tileset.png");
//
//     // Create tilemap
//     let map_size = TilemapSize { x: 32, y: 32 };
//     let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
//     let grid_size = TilemapGridSize { x: 16.0, y: 16.0 };
//
//     let mut tile_storage = TileStorage::empty(map_size);
//     let tilemap_entity = commands.spawn_empty().id();
//
//     // Spawn tiles
//     for x in 0..map_size.x {
//         for y in 0..map_size.y {
//             let tile_pos = TilePos { x, y };
//             let tile_entity = commands.spawn(TileBundle {
//                 position: tile_pos,
//                 tilemap_id: TilemapId(tilemap_entity),
//                 texture_index: TileTextureIndex(0), // Your tile index here
//                 ..default()
//             }).id();
//             tile_storage.set(&tile_pos, tile_entity);
//         }
//     }
//
//     // Add tilemap bundle
//     commands.entity(tilemap_entity).insert(TilemapBundle {
//         grid_size,
//         size: map_size,
//         storage: tile_storage,
//         texture: TilemapTexture::Single(texture_handle),
//         tile_size,
//         transform: Transform::from_xyz(0.0, 0.0, 0.0),
//         ..default()
//     });
// }

// ============================================================================
// GAME LOGIC
// ============================================================================
//
// Add your game systems here:
// - Player movement
// - Enemy AI
// - Collision detection
// - Score tracking
// - Level progression
// etc.
