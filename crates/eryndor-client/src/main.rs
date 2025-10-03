use bevy::prelude::*;
use clap::Parser;
use eryndor_common::ProjectMetadata;
use std::path::PathBuf;

mod level_loader;
mod tilemap_renderer;

use level_loader::*;

/// Eryndor MMO Game Client
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
                    level_path: "world/world.json".to_string(),
                    asset_path: "../eryndor-editor/assets".to_string(),
                }
            }
        }
    } else {
        // No project path provided, use defaults
        warn!("No project path provided, using default paths");
        ClientConfig {
            project_metadata: None,
            level_path: args.level.unwrap_or_else(|| "world/world.json".to_string()),
            asset_path: "../eryndor-editor/assets".to_string(),
        }
    };

    let window_title = client_config.project_metadata.as_ref()
        .map(|m| m.config.client_config.window_title.clone())
        .unwrap_or_else(|| "Eryndor MMO Client".to_string());

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
            bevy_ecs_tilemap::TilemapPlugin,
        ))
        .insert_resource(client_config)
        .init_asset::<LevelAsset>()
        .init_asset_loader::<LevelAssetLoader>()
        .add_event::<LevelLoadedEvent>()
        .add_systems(Startup, setup_client)
        .add_systems(Update, (
            watch_level_asset,
            tilemap_renderer::handle_level_loaded,
        ))
        .run();
}

fn setup_client(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<ClientConfig>,
) {
    // Spawn camera
    commands.spawn(Camera2d);

    info!("Eryndor Client initialized!");

    // Load level asset - Bevy will watch for changes automatically
    let level_handle: Handle<LevelAsset> = asset_server.load(&config.level_path);

    commands.insert_resource(CurrentLevel {
        handle: level_handle,
    });
}
