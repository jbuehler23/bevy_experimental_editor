use spacetimedb::Table;

/// Tileset definition - stores metadata about a tileset texture
#[spacetimedb::table(name = tileset)]
pub struct Tileset {
    #[primary_key]
    #[auto_inc]
    pub id: u32,
    pub identifier: String,
    pub texture_path: String,
    pub tile_width: u32,
    pub tile_height: u32,
    pub columns: u32,
    pub rows: u32,
    pub spacing: u32,
    pub padding: u32,
}
