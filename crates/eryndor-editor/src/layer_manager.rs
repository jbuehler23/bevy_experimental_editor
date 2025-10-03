use bevy::prelude::*;
use eryndor_common::{LayerData, LayerMetadata, LayerType, TileData};
use std::collections::HashMap;

/// Manages layers in the editor
#[derive(Resource)]
pub struct LayerManager {
    /// All layers in the current level
    pub layers: Vec<LayerData>,
    /// Currently active layer index
    pub active_layer: Option<usize>,
    /// Layer visibility states
    pub layer_visibility: HashMap<u32, bool>,
    /// Next available layer ID
    next_id: u32,
}

impl Default for LayerManager {
    fn default() -> Self {
        Self::new()
    }
}

impl LayerManager {
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            active_layer: None,
            layer_visibility: HashMap::new(),
            next_id: 0,
        }
    }

    /// Add a new layer
    pub fn add_layer(&mut self, mut metadata: LayerMetadata) -> usize {
        metadata.id = self.next_id;
        self.next_id += 1;

        let layer = LayerData::new(metadata.clone());
        let index = self.layers.len();

        self.layers.push(layer);
        self.layer_visibility.insert(metadata.id, true);

        // Auto-select first layer
        if self.active_layer.is_none() {
            self.active_layer = Some(index);
        }

        index
    }

    /// Remove a layer by index
    pub fn remove_layer(&mut self, index: usize) {
        if index < self.layers.len() {
            let layer_id = self.layers[index].metadata.id;
            self.layers.remove(index);
            self.layer_visibility.remove(&layer_id);

            // Update active layer
            if self.active_layer == Some(index) {
                self.active_layer = if self.layers.is_empty() {
                    None
                } else {
                    Some(index.min(self.layers.len() - 1))
                };
            }
        }
    }

    /// Get layer by index
    pub fn get_layer(&self, index: usize) -> Option<&LayerData> {
        self.layers.get(index)
    }

    /// Get mutable layer by index
    pub fn get_layer_mut(&mut self, index: usize) -> Option<&mut LayerData> {
        self.layers.get_mut(index)
    }

    /// Get active layer
    pub fn get_active_layer(&self) -> Option<&LayerData> {
        self.active_layer.and_then(|idx| self.layers.get(idx))
    }

    /// Get mutable active layer
    pub fn get_active_layer_mut(&mut self) -> Option<&mut LayerData> {
        self.active_layer.and_then(|idx| self.layers.get_mut(idx))
    }

    /// Set active layer by index
    pub fn set_active_layer(&mut self, index: usize) {
        if index < self.layers.len() {
            self.active_layer = Some(index);
        }
    }

    /// Set layer visibility
    pub fn set_layer_visibility(&mut self, layer_id: u32, visible: bool) {
        self.layer_visibility.insert(layer_id, visible);
    }

    /// Get layer visibility
    pub fn is_layer_visible(&self, layer_id: u32) -> bool {
        self.layer_visibility.get(&layer_id).copied().unwrap_or(true)
    }

    /// Move layer up in z-order (increases z_index)
    pub fn move_layer_up(&mut self, index: usize) {
        if index > 0 && index < self.layers.len() {
            self.layers.swap(index, index - 1);
            if self.active_layer == Some(index) {
                self.active_layer = Some(index - 1);
            }
        }
    }

    /// Move layer down in z-order (decreases z_index)
    pub fn move_layer_down(&mut self, index: usize) {
        if index < self.layers.len().saturating_sub(1) {
            self.layers.swap(index, index + 1);
            if self.active_layer == Some(index) {
                self.active_layer = Some(index + 1);
            }
        }
    }

    /// Add a tile to the active layer
    pub fn add_tile(&mut self, tile: TileData) {
        if let Some(layer) = self.get_active_layer_mut() {
            // Remove existing tile at this position
            layer.tiles.retain(|t| t.x != tile.x || t.y != tile.y);
            layer.tiles.push(tile);
        }
    }

    /// Remove tile at position from active layer
    pub fn remove_tile(&mut self, x: u32, y: u32) {
        if let Some(layer) = self.get_active_layer_mut() {
            layer.tiles.retain(|t| t.x != x || t.y != y);
        }
    }

    /// Get tile at position in active layer
    pub fn get_tile_at(&self, x: u32, y: u32) -> Option<&TileData> {
        self.get_active_layer()?
            .tiles
            .iter()
            .find(|t| t.x == x && t.y == y)
    }


    /// Clear all layers
    pub fn clear(&mut self) {
        self.layers.clear();
        self.active_layer = None;
        self.layer_visibility.clear();
        self.next_id = 0;
    }

    /// Get layers sorted by z-index for rendering
    pub fn get_sorted_layers(&self) -> Vec<&LayerData> {
        let mut sorted: Vec<&LayerData> = self.layers.iter().collect();
        sorted.sort_by_key(|layer| layer.metadata.z_index);
        sorted
    }
}

/// Create a default layer
pub fn create_default_layer(layer_type: LayerType, name: &str, z_index: i32, tileset_id: Option<u32>) -> LayerMetadata {
    LayerMetadata {
        id: 0, // Will be set by LayerManager
        level_id: 0,
        identifier: name.to_string(),
        layer_type,
        tileset_id,
        grid_size: 16,
        width: 64,
        height: 64,
        z_index,
        opacity: 1.0,
        parallax_x: 1.0,
        parallax_y: 1.0,
    }
}
