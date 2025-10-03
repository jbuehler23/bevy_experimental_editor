use spacetimedb::{ReducerContext, Table, Identity};
use eryndor_common::*;

pub mod math;
pub mod player;
pub mod tables;
pub mod reducers;
pub mod collision;

// ============ TABLES ============

#[spacetimedb::table(name = person)]
pub struct Person {
    name: String,
}

#[spacetimedb::table(name = npc)]
pub struct Npc {
    #[primary_key]
    pub id: u32,
    pub name: String,
    pub npc_type: String, // Stored as string for SpacetimeDB compatibility
    pub position: math::DbVector2,
    pub health: i32,
    pub max_health: i32,
    pub patrol_points: String, // JSON-serialized Vec<Vector2>
    pub current_patrol_index: u32,
}

#[spacetimedb::table(name = resource_node)]
pub struct ResourceNode {
    #[primary_key]
    pub id: u32,
    pub resource_type: String,
    pub position: math::DbVector2,
    pub health: i32,
    pub max_health: i32,
    pub respawn_time: f32,
    pub is_depleted: bool,
}

#[spacetimedb::table(name = interactive_object)]
pub struct InteractiveObject {
    #[primary_key]
    pub id: u32,
    pub object_type: String,
    pub position: math::DbVector2,
    pub is_active: bool,
}

#[spacetimedb::table(name = spawn_point)]
pub struct SpawnPoint {
    #[primary_key]
    pub id: u32,
    pub spawn_type: String,
    pub position: math::DbVector2,
    pub level_id: String,
}

#[spacetimedb::table(name = platform)]
pub struct Platform {
    #[primary_key]
    pub id: u32,
    pub position: math::DbVector2,
    pub size: math::DbVector2,
    pub is_one_way: bool,
}

#[spacetimedb::reducer(init)]
pub fn init(ctx: &ReducerContext) {
    log::info!("Initializing Eryndor server...");

    // Load level data from embedded JSON
    let level_json = include_str!("../../../assets/levels/test_level.json");

    match load_level_data(ctx, level_json) {
        Ok(_) => log::info!("Level data loaded successfully!"),
        Err(e) => log::error!("Failed to load level data: {}", e),
    }
}

/// Load level data and populate database tables
fn load_level_data(ctx: &ReducerContext, json: &str) -> Result<(), Box<dyn std::error::Error>> {
    let level: LevelData = serde_json::from_str(json)?;

    log::info!("Loading level: {}", level.metadata.name);

    // Load platforms
    for (idx, platform_data) in level.platforms.iter().enumerate() {
        ctx.db.platform().insert(Platform {
            id: idx as u32,
            position: math::DbVector2 {
                x: platform_data.position.x,
                y: platform_data.position.y,
            },
            size: math::DbVector2 {
                x: platform_data.size.x,
                y: platform_data.size.y,
            },
            is_one_way: platform_data.is_one_way,
        });
    }
    log::info!("Loaded {} platforms", level.platforms.len());

    // Load entities
    let mut npc_id = 0;
    let mut resource_id = 0;
    let mut interactive_id = 0;
    let mut spawn_id = 0;

    for entity in level.entities.iter() {
        match &entity.properties {
            EntityProperties::Npc { name, npc_type, max_health, patrol_points } => {
                let patrol_json = serde_json::to_string(patrol_points)?;
                ctx.db.npc().insert(Npc {
                    id: npc_id,
                    name: name.clone(),
                    npc_type: format!("{:?}", npc_type),
                    position: math::DbVector2 {
                        x: entity.position.x,
                        y: entity.position.y,
                    },
                    health: *max_health,
                    max_health: *max_health,
                    patrol_points: patrol_json,
                    current_patrol_index: 0,
                });
                npc_id += 1;
            }
            EntityProperties::Resource { resource_type, max_health, respawn_time } => {
                ctx.db.resource_node().insert(ResourceNode {
                    id: resource_id,
                    resource_type: format!("{:?}", resource_type),
                    position: math::DbVector2 {
                        x: entity.position.x,
                        y: entity.position.y,
                    },
                    health: *max_health,
                    max_health: *max_health,
                    respawn_time: *respawn_time,
                    is_depleted: false,
                });
                resource_id += 1;
            }
            EntityProperties::Interactive { object_type } => {
                ctx.db.interactive_object().insert(InteractiveObject {
                    id: interactive_id,
                    object_type: format!("{:?}", object_type),
                    position: math::DbVector2 {
                        x: entity.position.x,
                        y: entity.position.y,
                    },
                    is_active: true,
                });
                interactive_id += 1;
            }
            EntityProperties::SpawnPoint { spawn_type, level_id } => {
                ctx.db.spawn_point().insert(SpawnPoint {
                    id: spawn_id,
                    spawn_type: format!("{:?}", spawn_type),
                    position: math::DbVector2 {
                        x: entity.position.x,
                        y: entity.position.y,
                    },
                    level_id: level_id.clone(),
                });
                spawn_id += 1;
            }
            EntityProperties::Player { .. } => {
                // Players are spawned dynamically when clients connect
            }
        }
    }

    log::info!("Loaded {} NPCs, {} resources, {} interactive objects, {} spawn points",
        npc_id, resource_id, interactive_id, spawn_id);

    Ok(())
}

#[spacetimedb::reducer(client_connected)]
pub fn identity_connected(_ctx: &ReducerContext) {
    // Called everytime a new client connects
}

#[spacetimedb::reducer(client_disconnected)]
pub fn identity_disconnected(_ctx: &ReducerContext) {
    // Called everytime a client disconnects
}

#[spacetimedb::reducer]
pub fn add(ctx: &ReducerContext, name: String) {
    ctx.db.person().insert(Person { name });
}

#[spacetimedb::reducer]
pub fn say_hello(ctx: &ReducerContext) {
    for person in ctx.db.person().iter() {
        log::info!("Hello, {}!", person.name);
    }
    log::info!("Hello, World!");
}
