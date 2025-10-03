use bevy::prelude::*;

mod level_loader;
mod tilemap_renderer;

use level_loader::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    // Point to editor's assets folder for shared assets
                    file_path: "../eryndor-editor/assets".to_string(),
                    watch_for_changes_override: Some(true),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Eryndor MMO Client".to_string(),
                        resolution: (1280.0, 720.0).into(),
                        ..default()
                    }),
                    ..default()
                }),
            bevy_ecs_tilemap::TilemapPlugin,
        ))
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
) {
    // Spawn camera
    commands.spawn(Camera2d);

    info!("Eryndor Client initialized!");

    // Load level asset - Bevy will watch for changes automatically
    let level_handle: Handle<LevelAsset> = asset_server.load("world/world.json");

    commands.insert_resource(CurrentLevel {
        handle: level_handle,
    });
}
