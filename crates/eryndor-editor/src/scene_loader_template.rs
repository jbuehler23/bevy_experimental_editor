// Template content for scene_loader.rs
pub const SCENE_LOADER_TEMPLATE: &str = r#"// Scene loading data structures and utilities
// This module contains minimal data structures needed to load .bscene files

use serde::{Deserialize, Serialize};

/// Bevy Scene Format (.bscene) - JSON-based scene storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BevyScene {
    pub format_version: String,
    pub metadata: SceneMetadata,
    pub data: LevelData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneMetadata {
    pub last_modified: Option<String>,
    pub editor_version: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Level data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelData {
    pub metadata: LevelMetadata,
    #[serde(default)]
    pub tilemap: Option<LevelTilemapData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelMetadata {
    pub name: String,
    pub version: String,
    pub author: Option<String>,
    pub description: Option<String>,
}

/// Tilemap data for the level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelTilemapData {
    pub grid_size: f32,
    pub map_width: u32,
    pub map_height: u32,
    pub tilesets: Vec<LevelTilesetData>,
    pub selected_tileset_id: Option<u32>,
    pub layers: Vec<LevelLayerData>,
}

/// Tileset entry for tilemap rendering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelTilesetData {
    pub id: u32,
    pub identifier: String,
    pub texture_path: String,
    pub tile_width: u32,
    pub tile_height: u32,
}

/// Layer data with tile placements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelLayerData {
    pub id: u32,
    pub name: String,
    pub visible: bool,
    pub tiles: Vec<LevelTileInstance>,
}

/// Individual tile instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelTileInstance {
    pub x: u32,
    pub y: u32,
    pub tile_id: u32,
}
"#;

pub const PROJECT_FORMAT_TEMPLATE: &str = r#"// Project configuration format
// This module contains structures for loading project.bvy files

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Client/game configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfig {
    pub window_title: String,
    pub window_width: u32,
    pub window_height: u32,
    pub asset_watch: bool,
    pub default_level: String,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            window_title: "Bevy Game".to_string(),
            window_width: 1280,
            window_height: 720,
            asset_watch: true,
            default_level: "world/world.bscene".to_string(),
        }
    }
}

/// Project configuration file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub version: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub client_config: ClientConfig,
}

impl ProjectConfig {
    /// Load project config from a file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = std::fs::read_to_string(path)?;
        let config: ProjectConfig = serde_json::from_str(&contents)?;
        Ok(config)
    }
}

/// Project metadata tracked at runtime
#[derive(Debug, Clone)]
pub struct ProjectMetadata {
    pub config: ProjectConfig,
    pub root_path: PathBuf,
    pub assets_path: PathBuf,
    pub levels_path: PathBuf,
}

impl ProjectMetadata {
    /// Create project metadata from a project root directory
    pub fn from_project_path<P: AsRef<Path>>(project_path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let root_path = project_path.as_ref().to_path_buf();
        let config_path = root_path.join("project.bvy");

        let config = ProjectConfig::load_from_file(&config_path)?;

        let assets_path = root_path.join("assets");
        let levels_path = assets_path.join("world");

        Ok(Self {
            config,
            root_path,
            assets_path,
            levels_path,
        })
    }
}
"#;
