use bevy::prelude::*;
use bevy::asset::{AssetLoader, LoadContext};
use eryndor_common::{LevelData, BevyScene};

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

/// System to watch for level asset changes and fire events
pub fn watch_level_asset(
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

