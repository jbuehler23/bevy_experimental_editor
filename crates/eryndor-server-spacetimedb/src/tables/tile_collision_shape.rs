use spacetimedb::Table;

/// Tile collision shape - stores collision shapes defined for individual tiles in a tileset
#[spacetimedb::table(name = tile_collision_shape)]
pub struct TileCollisionShape {
    #[primary_key]
    #[auto_inc]
    pub id: u32,
    pub tileset_id: u32,
    pub tile_id: u32,
    pub shape_type: String, // "Rectangle", "Ellipse", "Polygon", "Polyline", "Point"
    pub shape_data: String, // JSON-serialized shape data
}
