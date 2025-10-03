use bevy::prelude::*;
use spacetimedb_sdk::Identity;

// ============ PLAYER COMPONENTS ============

#[derive(Component)]
pub struct Player {
    pub identity: Identity,
    pub name: String,
    pub health: i32,
    pub max_health: i32,
    pub level: u32,
    pub facing_right: bool,
}

#[derive(Component)]
pub struct LocalPlayer;

#[derive(Component)]
pub struct RemotePlayer;

// ============ RESOURCES ============

#[derive(Resource)]
pub struct LocalIdentity(pub Option<Identity>);

#[derive(Resource)]
pub struct Username(pub String);

#[derive(Resource, Default)]
pub struct EntityMap {
    pub players: std::collections::HashMap<Identity, Entity>,
    pub monsters: std::collections::HashMap<u32, Entity>,
    pub platforms: std::collections::HashMap<u32, Entity>,
}

#[derive(Resource, Default)]
pub struct GameState {
    pub connected: bool,
    pub in_game: bool,
}