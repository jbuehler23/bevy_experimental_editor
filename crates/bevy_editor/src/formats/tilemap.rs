use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::math::Vector2;

/// Collision shape types - Tiled-style per-tile collision shapes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CollisionShape {
    Rectangle { x: f32, y: f32, width: f32, height: f32 },
    Ellipse { x: f32, y: f32, rx: f32, ry: f32 },
    Polygon { points: Vec<Vector2> },
    Polyline { points: Vec<Vector2> },
    Point { x: f32, y: f32 },
}

/// Per-tile collision data - stores all collision shapes for a single tile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileCollisionData {
    pub tile_id: u32,
    pub shapes: Vec<CollisionShape>,
}

/// Tileset data - shared between editor and runtime
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TilesetData {
    pub id: u32,
    pub identifier: String,
    pub texture_path: String,
    pub tile_width: u32,
    pub tile_height: u32,
    pub columns: u32,
    pub rows: u32,
    pub spacing: u32,
    pub padding: u32,
    pub collision_data: HashMap<u32, TileCollisionData>,
}

impl Default for TilesetData {
    fn default() -> Self {
        Self {
            id: 0,
            identifier: "default_tileset".to_string(),
            texture_path: "tilesets/default.png".to_string(),
            tile_width: 16,
            tile_height: 16,
            columns: 16,
            rows: 16,
            spacing: 0,
            padding: 0,
            collision_data: HashMap::new(),
        }
    }
}

/// Layer type - matches LDTk layer types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LayerType {
    Tiles,      // Regular tile layer
    IntGrid,    // Collision/physics layer
    Entities,   // Entity placement layer
    AutoLayer,  // Auto-tiled layer based on IntGrid
}

impl LayerType {
    pub fn as_str(&self) -> &'static str {
        match self {
            LayerType::Tiles => "Tiles",
            LayerType::IntGrid => "IntGrid",
            LayerType::Entities => "Entities",
            LayerType::AutoLayer => "AutoLayer",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Tiles" => Some(LayerType::Tiles),
            "IntGrid" => Some(LayerType::IntGrid),
            "Entities" => Some(LayerType::Entities),
            "AutoLayer" => Some(LayerType::AutoLayer),
            _ => None,
        }
    }
}

/// Layer metadata - describes a single layer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerMetadata {
    pub id: u32,
    pub level_id: u32,
    pub identifier: String,
    pub layer_type: LayerType,
    pub tileset_id: Option<u32>,
    pub grid_size: u32,
    pub width: u32,  // in tiles
    pub height: u32, // in tiles
    pub z_index: i32,
    pub opacity: f32,
    pub parallax_x: f32,
    pub parallax_y: f32,
}

impl Default for LayerMetadata {
    fn default() -> Self {
        Self {
            id: 0,
            level_id: 0,
            identifier: "new_layer".to_string(),
            layer_type: LayerType::Tiles,
            tileset_id: Some(0),
            grid_size: 16,
            width: 32,
            height: 32,
            z_index: 0,
            opacity: 1.0,
            parallax_x: 1.0,
            parallax_y: 1.0,
        }
    }
}

/// Tile data - individual tile placement
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TileData {
    pub x: u32,
    pub y: u32,
    pub tile_id: u32,
    pub flip_x: bool,
    pub flip_y: bool,
}


/// Complete layer data including tiles and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerData {
    #[serde(flatten)]
    pub metadata: LayerMetadata,
    pub tiles: Vec<TileData>,
}

impl LayerData {
    pub fn new(metadata: LayerMetadata) -> Self {
        Self {
            metadata,
            tiles: Vec::new(),
        }
    }

    pub fn with_tiles(mut self, tiles: Vec<TileData>) -> Self {
        self.tiles = tiles;
        self
    }
}
