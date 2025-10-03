use std::fs;
use std::path::{Path, PathBuf};
use eryndor_common::ProjectConfig;

/// Template for a new Bevy project
#[derive(Debug, Clone, PartialEq)]
pub enum ProjectTemplate {
    /// Empty Bevy project with minimal setup
    Empty,
    /// 2D platformer with tilemap support
    Tilemap2D,
}

impl ProjectTemplate {
    pub fn name(&self) -> &str {
        match self {
            ProjectTemplate::Empty => "Empty Bevy Project",
            ProjectTemplate::Tilemap2D => "2D Tilemap Game",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            ProjectTemplate::Empty => "A minimal Bevy project with basic setup",
            ProjectTemplate::Tilemap2D => "A 2D game with tilemap rendering and level loading support",
        }
    }
}

/// Generate a new Bevy project from a template
pub fn generate_project(
    project_path: &Path,
    project_name: &str,
    template: ProjectTemplate,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create project directory
    fs::create_dir_all(project_path)?;

    // Create directory structure
    create_directory_structure(project_path)?;

    // Generate Cargo.toml
    generate_cargo_toml(project_path, project_name, &template)?;

    // Generate project.bvy (editor config)
    generate_project_config(project_path, project_name)?;

    // Generate main.rs
    generate_main_rs(project_path, &template)?;

    // Generate tilemap_renderer.rs if using tilemap template
    if matches!(template, ProjectTemplate::Tilemap2D) {
        generate_tilemap_renderer(project_path)?;
    }

    // Generate .gitignore
    generate_gitignore(project_path)?;

    Ok(())
}

/// Create the standard directory structure
fn create_directory_structure(project_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let dirs = vec![
        "src",
        "assets",
        "assets/tilesets",
        "assets/world",
        "assets/sprites",
        "assets/audio",
        ".cargo",
    ];

    for dir in dirs {
        fs::create_dir_all(project_path.join(dir))?;
    }

    // Create .cargo/config.toml to optimize builds for memory usage
    generate_cargo_config(project_path)?;

    Ok(())
}

/// Generate Cargo.toml for the project
fn generate_cargo_toml(
    project_path: &Path,
    project_name: &str,
    template: &ProjectTemplate,
) -> Result<(), Box<dyn std::error::Error>> {
    // Convert project name to valid Rust package name (lowercase, replace spaces with dashes)
    let package_name = project_name
        .to_lowercase()
        .replace(' ', "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
        .collect::<String>();

    let dependencies = match template {
        ProjectTemplate::Empty => {
            r#"bevy = "0.16""#
        }
        ProjectTemplate::Tilemap2D => {
            r#"bevy = { version = "0.16", features = ["file_watcher"] }
bevy_ecs_tilemap = "0.16"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
clap = { version = "4.5", features = ["derive"] }
eryndor-common = { path = "../eryndor-common" }"#
        }
    };

    let cargo_toml = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "{}"
path = "src/main.rs"

[dependencies]
{}

[profile.dev]
opt-level = 1  # Improve development performance

[profile.dev.package."*"]
opt-level = 3  # Optimize dependencies for better performance
"#,
        package_name, package_name, dependencies
    );

    fs::write(project_path.join("Cargo.toml"), cargo_toml)?;

    Ok(())
}

/// Generate project.bvy (editor configuration)
fn generate_project_config(
    project_path: &Path,
    project_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = ProjectConfig::new(project_name.to_string());
    config.client_config.window_title = project_name.to_string();

    let config_path = project_path.join("project.bvy");
    config.save_to_file(config_path)?;

    Ok(())
}

/// Generate main.rs based on template
fn generate_main_rs(
    project_path: &Path,
    template: &ProjectTemplate,
) -> Result<(), Box<dyn std::error::Error>> {
    let main_rs = match template {
        ProjectTemplate::Empty => generate_empty_main(),
        ProjectTemplate::Tilemap2D => generate_tilemap_main(),
    };

    fs::write(project_path.join("src/main.rs"), main_rs)?;

    Ok(())
}

/// Generate a minimal Bevy main.rs
fn generate_empty_main() -> String {
    r#"use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "My Bevy Game".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, hello_world)
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn camera
    commands.spawn(Camera2d);

    info!("Game initialized!");
}

fn hello_world() {
    // Your game logic here
}
"#
    .to_string()
}

/// Generate a tilemap-enabled main.rs with full scene loading support
fn generate_tilemap_main() -> String {
    r#"use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy::asset::{AssetLoader, LoadContext};
use eryndor_common::{LevelData, BevyScene, ProjectMetadata};
use clap::Parser;
use std::path::PathBuf;

mod tilemap_renderer;
use tilemap_renderer::*;

/// Game command line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the project directory
    #[arg(short, long)]
    project_path: Option<String>,

    /// Specific level file to load (overrides project default)
    #[arg(short, long)]
    level: Option<String>,
}

/// Custom asset type for level data
#[derive(Asset, TypePath, Clone)]
pub struct LevelAsset {
    pub data: LevelData,
}

/// Asset loader for .bscene files
#[derive(Default)]
pub struct LevelAssetLoader;

impl AssetLoader for LevelAssetLoader {
    type Asset = LevelAsset;
    type Settings = ();
    type Error = std::io::Error;

    async fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        // Parse as BevyScene and extract the level data
        let scene: BevyScene = serde_json::from_slice(&bytes)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        Ok(LevelAsset { data: scene.data })
    }

    fn extensions(&self) -> &[&str] {
        &["bscene"]
    }
}

/// Resource to track the currently loaded level
#[derive(Resource)]
pub struct CurrentLevel {
    pub handle: Handle<LevelAsset>,
}

/// Event fired when a level has been loaded
#[derive(Event)]
pub struct LevelLoadedEvent {
    pub level_data: LevelData,
}

/// Resource holding client configuration
#[derive(Resource)]
struct ClientConfig {
    project_metadata: Option<ProjectMetadata>,
    level_path: String,
    asset_path: String,
}

fn main() {
    let args = Args::parse();

    // Load project config if provided
    let client_config = if let Some(project_path) = args.project_path {
        match ProjectMetadata::from_project_path(&project_path) {
            Ok(metadata) => {
                let level_path = if let Some(level) = args.level {
                    level
                } else {
                    metadata.config.client_config.default_level.clone()
                };

                let asset_path = metadata.assets_path.to_string_lossy().to_string();

                info!("Loaded project: {}", metadata.config.name);
                info!("Assets path: {}", asset_path);
                info!("Level path: {}", level_path);

                ClientConfig {
                    project_metadata: Some(metadata),
                    level_path,
                    asset_path,
                }
            }
            Err(e) => {
                eprintln!("Failed to load project: {}", e);
                eprintln!("Falling back to defaults");
                ClientConfig {
                    project_metadata: None,
                    level_path: "world/world.bscene".to_string(),
                    asset_path: "assets".to_string(),
                }
            }
        }
    } else {
        // No project path provided, use defaults
        warn!("No project path provided, using default paths");
        ClientConfig {
            project_metadata: None,
            level_path: args.level.unwrap_or_else(|| "world/world.bscene".to_string()),
            asset_path: "assets".to_string(),
        }
    };

    let window_title = client_config.project_metadata.as_ref()
        .map(|m| m.config.client_config.window_title.clone())
        .unwrap_or_else(|| "Bevy Game".to_string());

    let (window_width, window_height) = client_config.project_metadata.as_ref()
        .map(|m| (m.config.client_config.window_width, m.config.client_config.window_height))
        .unwrap_or((1280, 720));

    let asset_path = client_config.asset_path.clone();
    let level_path = client_config.level_path.clone();

    App::new()
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: asset_path,
                    watch_for_changes_override: Some(true),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: window_title,
                        resolution: (window_width as f32, window_height as f32).into(),
                        ..default()
                    }),
                    ..default()
                }),
            TilemapPlugin,
        ))
        .insert_resource(client_config)
        .init_asset::<LevelAsset>()
        .init_asset_loader::<LevelAssetLoader>()
        .add_event::<LevelLoadedEvent>()
        .add_systems(Startup, setup_game)
        .add_systems(Update, (
            watch_level_asset,
            handle_level_loaded,
        ))
        .run();
}

fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<ClientConfig>,
) {
    // Spawn camera
    commands.spawn(Camera2d);

    info!("Game initialized!");

    // Load level asset - Bevy will watch for changes automatically
    let level_handle: Handle<LevelAsset> = asset_server.load(&config.level_path);

    commands.insert_resource(CurrentLevel {
        handle: level_handle,
    });
}

/// System to watch for level asset changes and fire events
fn watch_level_asset(
    current_level: Res<CurrentLevel>,
    level_assets: Res<Assets<LevelAsset>>,
    mut level_loaded_events: EventWriter<LevelLoadedEvent>,
    mut asset_events: EventReader<AssetEvent<LevelAsset>>,
) {
    for event in asset_events.read() {
        match event {
            AssetEvent::Added { id } | AssetEvent::Modified { id } => {
                if current_level.handle.id() == *id {
                    if let Some(level_asset) = level_assets.get(*id) {
                        info!("Level asset loaded/modified: {}", level_asset.data.metadata.name);

                        level_loaded_events.write(LevelLoadedEvent {
                            level_data: level_asset.data.clone(),
                        });
                    }
                }
            }
            _ => {}
        }
    }
}
"#
    .to_string()
}

/// Generate tilemap_renderer.rs module
fn generate_tilemap_renderer(project_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let tilemap_renderer = r#"use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use crate::LevelLoadedEvent;

/// Component to mark entities as part of the current level (for despawning on reload)
#[derive(Component)]
pub struct LevelEntity;

/// System to render tilemap when level is loaded
pub fn handle_level_loaded(
    mut events: EventReader<LevelLoadedEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // Query existing level entities to despawn them
    existing_entities: Query<Entity, With<LevelEntity>>,
) {
    for event in events.read() {
        // Despawn existing level entities
        for entity in &existing_entities {
            commands.entity(entity).despawn();
        }

        // Check if level has tilemap data
        let Some(tilemap_data) = &event.level_data.tilemap else {
            info!("Level has no tilemap data, skipping tilemap rendering");
            continue;
        };

        info!("Rendering tilemap: {} layers, {} tilesets",
            tilemap_data.layers.len(),
            tilemap_data.tilesets.len()
        );

        // Load all tilesets and create tilemaps
        for (layer_idx, layer) in tilemap_data.layers.iter().enumerate() {
            if layer.tiles.is_empty() {
                info!("Layer {} '{}' has no tiles, skipping", layer_idx, layer.name);
                continue;
            }

            // Find the first tileset (in a real implementation, you'd match by tileset ID)
            let Some(tileset) = tilemap_data.tilesets.first() else {
                warn!("No tilesets found for layer {}", layer_idx);
                continue;
            };

            // Load tileset texture
            let texture_handle: Handle<Image> = asset_server.load(&tileset.texture_path);

            // Create tilemap
            let map_size = TilemapSize {
                x: tilemap_data.map_width,
                y: tilemap_data.map_height,
            };

            let tile_size = TilemapTileSize {
                x: tileset.tile_width as f32,
                y: tileset.tile_height as f32,
            };

            let grid_size = TilemapGridSize {
                x: tileset.tile_width as f32,
                y: tileset.tile_height as f32,
            };

            let mut tile_storage = TileStorage::empty(map_size);

            let tilemap_entity = commands.spawn((
                LevelEntity,
                Name::new(format!("Tilemap Layer: {}", layer.name)),
            )).id();

            // Spawn tiles
            for tile_instance in &layer.tiles {
                let tile_pos = TilePos {
                    x: tile_instance.x,
                    y: tile_instance.y,
                };

                let tile_entity = commands.spawn((
                    TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture_index: TileTextureIndex(tile_instance.tile_id),
                        visible: TileVisible(true),
                        ..default()
                    },
                    LevelEntity,
                )).id();

                tile_storage.set(&tile_pos, tile_entity);
            }

            // Half-tile offset for proper grid alignment (same as editor)
            let half_tile_x = tileset.tile_width as f32 / 2.0;
            let half_tile_y = tileset.tile_height as f32 / 2.0;

            // Add tilemap bundle to the tilemap entity
            commands.entity(tilemap_entity).insert(TilemapBundle {
                grid_size,
                size: map_size,
                storage: tile_storage,
                texture: TilemapTexture::Single(texture_handle),
                tile_size,
                transform: Transform::from_xyz(half_tile_x, half_tile_y, layer_idx as f32),
                map_type: TilemapType::Square,
                ..default()
            });

            info!("Spawned {} tiles for layer '{}'", layer.tiles.len(), layer.name);
        }
    }
}
"#;

    fs::write(project_path.join("src/tilemap_renderer.rs"), tilemap_renderer)?;

    Ok(())
}

/// Generate .gitignore
fn generate_gitignore(project_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let gitignore = r#"/target
/.vscode
/.idea
*.log
.DS_Store
Cargo.lock
"#;

    fs::write(project_path.join(".gitignore"), gitignore)?;

    Ok(())
}

/// Generate .cargo/config.toml to optimize build performance and reduce memory usage
fn generate_cargo_config(project_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let cargo_config = r#"# Cargo configuration for optimized Bevy builds
# This reduces memory usage during compilation

[build]
# Use a single codegen unit to reduce memory usage (slower but uses less RAM)
# Increase this number if you have more RAM available
# codegen-units = 1

[profile.dev]
# Optimize dependencies even in dev mode to reduce memory usage
opt-level = 1

[profile.dev.package."*"]
# Optimize all dependencies in dev mode
opt-level = 3

# Use LLD linker for faster builds (Windows/Linux)
[target.x86_64-pc-windows-msvc]
linker = "rust-lld"

[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

# Reduce parallel jobs to prevent OOM on machines with limited RAM
# Uncomment and adjust if you're still running out of memory
# [build]
# jobs = 4
"#;

    fs::write(project_path.join(".cargo/config.toml"), cargo_config)?;

    Ok(())
}

/// Get the package name from a project's Cargo.toml
pub fn get_package_name_from_cargo_toml(project_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let cargo_toml_path = project_path.join("Cargo.toml");
    let contents = fs::read_to_string(cargo_toml_path)?;

    let cargo_toml: toml::Value = toml::from_str(&contents)?;

    if let Some(package) = cargo_toml.get("package") {
        if let Some(name) = package.get("name") {
            if let Some(name_str) = name.as_str() {
                return Ok(name_str.to_string());
            }
        }
    }

    Err("Could not find package name in Cargo.toml".into())
}