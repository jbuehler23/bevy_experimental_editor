use spacetimedb::Table;

/// Individual tile in a tilemap layer
/// Stores the actual tile data for rendering with bevy_ecs_tilemap
#[spacetimedb::table(name = tilemap_tile)]
pub struct TilemapTile {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub layer_id: u32,
    pub x: u32,
    pub y: u32,
    pub tile_id: u32,  // Index in tileset
    pub flip_x: bool,
    pub flip_y: bool,
}

#[spacetimedb::table(name = tilemap_tile, public)]
pub fn tilemap_tile_by_layer(layer_id: u32) -> impl Iterator<Item = TilemapTile> {
    TilemapTile::filter_by_layer_id(&layer_id)
}
