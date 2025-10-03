use spacetimedb::Table;

/// Entity definition - custom entity classes with user-defined fields
/// Similar to LDTk's entity definitions
#[spacetimedb::table(name = entity_definition)]
pub struct EntityDefinition {
    #[primary_key]
    #[auto_inc]
    pub id: u32,
    pub identifier: String,
    pub width: u32,
    pub height: u32,
    pub color: String,  // hex color for editor visualization (e.g., "#FF0000")
    pub field_definitions: String,  // JSON: [{"name":"health","type":"Int","default":100}]
}

/// Entity instance - actual entity placements in the world with custom field values
#[spacetimedb::table(name = entity_instance)]
pub struct EntityInstance {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub level_id: u32,
    pub entity_def_id: u32,
    pub x: f32,
    pub y: f32,
    pub field_values: String,  // JSON: {"health":100,"patrol_points":[...]}
}
