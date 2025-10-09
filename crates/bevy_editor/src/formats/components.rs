use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Marker component for the local player
#[derive(Component, Debug, Clone, Copy)]
pub struct LocalPlayer;

/// Player component with stats and state
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub health: i32,
    pub max_health: i32,
    pub level: u32,
}

/// NPC component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Npc {
    pub npc_type: NpcType,
    pub name: String,
    pub health: i32,
    pub max_health: i32,
    pub patrol_points: Vec<crate::math::Vector2>,
    pub current_patrol_index: usize,
}

/// NPC types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NpcType {
    Friendly,
    Hostile,
    Neutral,
    Vendor,
}

/// Resource node component (trees, ore, etc.)
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct ResourceNode {
    pub resource_type: ResourceType,
    pub health: i32,
    pub max_health: i32,
    pub respawn_time: f32,
}

/// Resource types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceType {
    Tree,
    Rock,
    IronOre,
    GoldOre,
    Bush,
}

/// Interactive object component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Interactive {
    pub object_type: InteractiveType,
    pub is_active: bool,
}

/// Interactive object types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InteractiveType {
    Door,
    Chest,
    Lever,
    Button,
    Portal,
}

/// Spawn point component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct SpawnPoint {
    pub spawn_type: SpawnType,
    pub level_id: String,
}

/// Spawn point types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpawnType {
    PlayerStart,
    EnemySpawn,
    ItemSpawn,
}

/// Movement component
#[derive(Component, Debug, Clone, Copy)]
pub struct Movement {
    pub velocity: Vec2,
    pub speed: f32,
}

/// Health component
#[derive(Component, Debug, Clone, Copy)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

impl Health {
    pub fn new(max: i32) -> Self {
        Self { current: max, max }
    }

    pub fn is_alive(&self) -> bool {
        self.current > 0
    }

    pub fn heal(&mut self, amount: i32) {
        self.current = (self.current + amount).min(self.max);
    }

    pub fn damage(&mut self, amount: i32) {
        self.current = (self.current - amount).max(0);
    }
}
