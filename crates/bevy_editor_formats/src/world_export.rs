use super::{EntityDefinitionData, EntityInstanceData, EnumDefinitionData, LayerData, TilesetData};
use serde::{Deserialize, Serialize};

/// Complete world export format
/// This is the primary format for editor exports and imports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldExport {
    pub version: String,
    pub tilesets: Vec<TilesetData>,
    pub layers: Vec<LayerExportData>,
    pub entity_definitions: Vec<EntityDefinitionData>,
    pub entity_instances: Vec<EntityInstanceData>,
    pub enum_definitions: Vec<EnumDefinitionData>,
    pub metadata: WorldMetadataExport,
}

impl WorldExport {
    pub fn new(version: &str) -> Self {
        Self {
            version: version.to_string(),
            tilesets: Vec::new(),
            layers: Vec::new(),
            entity_definitions: Vec::new(),
            entity_instances: Vec::new(),
            enum_definitions: Vec::new(),
            metadata: WorldMetadataExport::default(),
        }
    }

    pub fn with_tileset(mut self, tileset: TilesetData) -> Self {
        self.tilesets.push(tileset);
        self
    }

    pub fn with_layer(mut self, layer: LayerExportData) -> Self {
        self.layers.push(layer);
        self
    }

    pub fn with_entity_definition(mut self, entity_def: EntityDefinitionData) -> Self {
        self.entity_definitions.push(entity_def);
        self
    }

    pub fn with_entity_instance(mut self, entity_inst: EntityInstanceData) -> Self {
        self.entity_instances.push(entity_inst);
        self
    }

    pub fn with_enum_definition(mut self, enum_def: EnumDefinitionData) -> Self {
        self.enum_definitions.push(enum_def);
        self
    }

    /// Save to JSON file
    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load from JSON file
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(path)?;
        let world = serde_json::from_str(&json)?;
        Ok(world)
    }

    /// Serialize to JSON string
    pub fn to_json(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    /// Deserialize from JSON string
    pub fn from_json(json: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str(json)?)
    }
}

impl Default for WorldExport {
    fn default() -> Self {
        Self::new("1.0.0")
    }
}

/// Layer export data - includes both metadata and tile/intgrid data
/// This matches the structure expected by the upload_world_data reducer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerExportData {
    pub id: u32,
    pub level_id: u32,
    pub identifier: String,
    pub layer_type: String, // Serialized as string for JSON
    pub tileset_id: Option<u32>,
    pub grid_size: u32,
    pub width: u32,
    pub height: u32,
    pub z_index: i32,
    pub opacity: f32,
    pub parallax_x: f32,
    pub parallax_y: f32,
    pub tiles: Vec<TileExportData>,
}

impl LayerExportData {
    pub fn from_layer_data(layer: &LayerData) -> Self {
        Self {
            id: layer.metadata.id,
            level_id: layer.metadata.level_id,
            identifier: layer.metadata.identifier.clone(),
            layer_type: layer.metadata.layer_type.as_str().to_string(),
            tileset_id: layer.metadata.tileset_id,
            grid_size: layer.metadata.grid_size,
            width: layer.metadata.width,
            height: layer.metadata.height,
            z_index: layer.metadata.z_index,
            opacity: layer.metadata.opacity,
            parallax_x: layer.metadata.parallax_x,
            parallax_y: layer.metadata.parallax_y,
            tiles: layer
                .tiles
                .iter()
                .map(TileExportData::from_tile_data)
                .collect(),
        }
    }
}

/// Tile export data - matches the structure expected by SpacetimeDB
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileExportData {
    pub x: u32,
    pub y: u32,
    pub tile_id: u32,
    pub flip_x: bool,
    pub flip_y: bool,
}

impl TileExportData {
    pub fn from_tile_data(tile: &super::tilemap::TileData) -> Self {
        Self {
            x: tile.x,
            y: tile.y,
            tile_id: tile.tile_id,
            flip_x: tile.flip_x,
            flip_y: tile.flip_y,
        }
    }
}

/// World metadata export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldMetadataExport {
    pub name: String,
    pub description: String,
    pub author: String,
    pub created_at: u64,
    pub modified_at: u64,
}

impl Default for WorldMetadataExport {
    fn default() -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        Self {
            name: "Untitled World".to_string(),
            description: String::new(),
            author: String::new(),
            created_at: now,
            modified_at: now,
        }
    }
}
