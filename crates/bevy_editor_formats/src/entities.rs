use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::components::*;
use super::math::Vector2;

/// Entity type identifier for spawning
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntityType {
    Player,
    Npc(NpcType),
    Resource(ResourceType),
    Interactive(InteractiveType),
    SpawnPoint(SpawnType),
}

/// Entity spawn configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitySpawnConfig {
    pub entity_type: EntityType,
    pub position: Vector2,
    pub properties: EntityProperties,
}

/// Entity-specific properties
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum EntityProperties {
    Player {
        name: String,
        max_health: i32,
    },
    Npc {
        name: String,
        npc_type: NpcType,
        max_health: i32,
        patrol_points: Vec<Vector2>,
    },
    Resource {
        resource_type: ResourceType,
        max_health: i32,
        respawn_time: f32,
    },
    Interactive {
        object_type: InteractiveType,
    },
    SpawnPoint {
        spawn_type: SpawnType,
        level_id: String,
    },
}

impl EntitySpawnConfig {
    /// Create a player spawn config
    pub fn player(position: Vector2, name: String, max_health: i32) -> Self {
        Self {
            entity_type: EntityType::Player,
            position,
            properties: EntityProperties::Player { name, max_health },
        }
    }

    /// Create an NPC spawn config
    pub fn npc(
        position: Vector2,
        name: String,
        npc_type: NpcType,
        max_health: i32,
        patrol_points: Vec<Vector2>,
    ) -> Self {
        Self {
            entity_type: EntityType::Npc(npc_type),
            position,
            properties: EntityProperties::Npc {
                name,
                npc_type,
                max_health,
                patrol_points,
            },
        }
    }

    /// Create a resource node spawn config
    pub fn resource(
        position: Vector2,
        resource_type: ResourceType,
        max_health: i32,
        respawn_time: f32,
    ) -> Self {
        Self {
            entity_type: EntityType::Resource(resource_type),
            position,
            properties: EntityProperties::Resource {
                resource_type,
                max_health,
                respawn_time,
            },
        }
    }

    /// Create an interactive object spawn config
    pub fn interactive(position: Vector2, object_type: InteractiveType) -> Self {
        Self {
            entity_type: EntityType::Interactive(object_type),
            position,
            properties: EntityProperties::Interactive { object_type },
        }
    }

    /// Create a spawn point config
    pub fn spawn_point(position: Vector2, spawn_type: SpawnType, level_id: String) -> Self {
        Self {
            entity_type: EntityType::SpawnPoint(spawn_type),
            position,
            properties: EntityProperties::SpawnPoint {
                spawn_type,
                level_id,
            },
        }
    }
}
