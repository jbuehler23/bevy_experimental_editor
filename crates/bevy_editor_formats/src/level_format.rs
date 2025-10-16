use serde::{Deserialize, Serialize};

use super::entities::EntitySpawnConfig;
use super::math::Vector2;

/// Level metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelMetadata {
    pub name: String,
    pub version: String,
    pub author: Option<String>,
    pub description: Option<String>,
}

/// Platform/collision data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformData {
    pub position: Vector2,
    pub size: Vector2,
    pub is_one_way: bool,
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

impl Default for LevelTilemapData {
    fn default() -> Self {
        Self {
            grid_size: 32.0,
            map_width: 64,
            map_height: 64,
            tilesets: Vec::new(),
            selected_tileset_id: None,
            layers: vec![LevelLayerData {
                id: 0,
                name: "Layer 0".to_string(),
                visible: true,
                tiles: Vec::new(),
            }],
        }
    }
}

/// Level data that can be exported from the editor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelData {
    pub metadata: LevelMetadata,
    /// Static collision platforms/geometry
    pub platforms: Vec<PlatformData>,
    /// Entity spawn configurations
    pub entities: Vec<EntitySpawnConfig>,
    /// World bounds
    pub world_bounds: WorldBounds,
    /// Background layers for visual rendering
    pub background_layers: Vec<BackgroundLayer>,
    /// Tilemap data (optional for backward compatibility)
    #[serde(default)]
    pub tilemap: Option<LevelTilemapData>,
}

/// Background layer for visual decoration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundLayer {
    pub name: String,
    pub texture_path: String,
    pub position: Vector2,
    pub parallax_factor: f32,
    pub z_order: i32,
}

/// World boundaries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldBounds {
    pub min: Vector2,
    pub max: Vector2,
}

impl WorldBounds {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            min: Vector2::zero(),
            max: Vector2::new(width, height),
        }
    }

    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }
}

impl LevelData {
    /// Create a new empty level
    pub fn new(name: String, world_width: f32, world_height: f32) -> Self {
        Self {
            metadata: LevelMetadata {
                name,
                version: "1.0".to_string(),
                author: None,
                description: None,
            },
            platforms: Vec::new(),
            entities: Vec::new(),
            world_bounds: WorldBounds::new(world_width, world_height),
            background_layers: Vec::new(),
            tilemap: Some(LevelTilemapData::default()),
        }
    }

    /// Add a platform to the level
    pub fn add_platform(&mut self, platform: PlatformData) {
        self.platforms.push(platform);
    }

    /// Add a background layer
    pub fn add_background_layer(&mut self, layer: BackgroundLayer) {
        self.background_layers.push(layer);
    }

    /// Add an entity spawn to the level
    pub fn add_entity(&mut self, entity: EntitySpawnConfig) {
        self.entities.push(entity);
    }

    /// Save to JSON file
    pub fn save_to_json(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load from JSON file
    pub fn load_from_json(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(path)?;
        let level: LevelData = serde_json::from_str(&json)?;
        Ok(level)
    }
}
