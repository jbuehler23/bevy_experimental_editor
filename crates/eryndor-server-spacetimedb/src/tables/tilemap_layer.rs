use spacetimedb::Table;

/// Tilemap layer metadata - describes a single layer in the world
/// Compatible with bevy_ecs_tilemap structure
#[spacetimedb::table(name = tilemap_layer)]
pub struct TilemapLayer {
    #[primary_key]
    #[auto_inc]
    pub id: u32,
    pub level_id: u32,
    pub identifier: String,
    pub layer_type: String, // "Tiles", "IntGrid", "Entities", "AutoLayer"
    pub tileset_id: Option<u32>,
    pub grid_size: u32,
    pub width: u32,  // in tiles
    pub height: u32, // in tiles
    pub z_index: i32,
    pub opacity: f32,
    pub parallax_x: f32,
    pub parallax_y: f32,
}
