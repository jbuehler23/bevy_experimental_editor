use spacetimedb::Table;

/// Enum definition - custom enums for entity field types
/// Similar to LDTk's enum definitions
#[spacetimedb::table(name = enum_definition)]
pub struct EnumDefinition {
    #[primary_key]
    #[auto_inc]
    pub id: u32,
    pub identifier: String,
    pub values: String,  // JSON: ["Friendly","Hostile","Neutral","Vendor"]
}
