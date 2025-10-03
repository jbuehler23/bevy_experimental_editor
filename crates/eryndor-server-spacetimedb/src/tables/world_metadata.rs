use spacetimedb::Table;

/// World metadata - stores world-level configuration and full editor export
#[spacetimedb::table(name = world_metadata)]
pub struct WorldMetadata {
    #[primary_key]
    pub id: u32,
    pub version: String,
    pub last_updated: u64,  // timestamp in milliseconds
    pub editor_data: String,  // JSON: full editor export for re-importing
}
