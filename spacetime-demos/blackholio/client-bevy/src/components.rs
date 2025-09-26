use bevy::prelude::*;
use std::collections::HashSet;

#[derive(Component)]
pub struct EntityController {
    pub entity_id: u32,
    pub lerp_time: f32,
    pub lerp_start: Vec3,
    pub lerp_target: Vec3,
}

impl EntityController {
    pub fn new(entity_id: u32, position: Vec2) -> Self {
        let pos3 = position.extend(0.0);
        Self {
            entity_id,
            lerp_time: 0.0,
            lerp_start: pos3,
            lerp_target: pos3,
        }
    }
}

#[derive(Component)]
pub struct CircleController {
    pub player_id: u32,
}

#[derive(Component)]
pub struct FoodController;

#[derive(Component)]
pub struct PlayerController {
    pub player_id: u32,
    pub owned_circles: HashSet<Entity>,
    pub is_local: bool,
    pub username: String,
}

impl PlayerController {
    pub fn new(player_id: u32, username: String, is_local: bool) -> Self {
        Self {
            player_id,
            owned_circles: HashSet::new(),
            is_local,
            username,
        }
    }

    pub fn total_mass(&self) -> u32 {
        // This will be calculated from entity queries
        0
    }

    pub fn center_of_mass(&self) -> Option<Vec2> {
        // This will be calculated from entity queries
        None
    }
}

#[derive(Component)]
pub struct Border;

#[derive(Resource)]
pub struct ArenaConfig {
    pub world_size: f32,
    pub border_thickness: f32,
}

impl Default for ArenaConfig {
    fn default() -> Self {
        Self {
            world_size: 100.0,
            border_thickness: 10.0,
        }
    }
}

#[derive(Resource)]
pub struct LocalPlayerEntity(pub Option<Entity>);

#[derive(Resource)]
pub struct EntityMap {
    pub entities: std::collections::HashMap<u32, Entity>,
}

impl Default for EntityMap {
    fn default() -> Self {
        Self {
            entities: std::collections::HashMap::new(),
        }
    }
}

#[derive(Resource)]
pub struct PlayerMap {
    pub players: std::collections::HashMap<u32, Entity>,
}

impl Default for PlayerMap {
    fn default() -> Self {
        Self {
            players: std::collections::HashMap::new(),
        }
    }
}

#[derive(Component)]
pub struct PlayerInputState {
    pub last_movement_send_timestamp: f32,
    pub lock_input_position: Option<Vec2>,
    pub current_mouse_position: Vec2,
    pub send_updates_frequency: f32, // 1/20 = 0.05 seconds (50ms)
}

impl Default for PlayerInputState {
    fn default() -> Self {
        Self {
            last_movement_send_timestamp: 0.0,
            lock_input_position: None,
            current_mouse_position: Vec2::new(400.0, 300.0), // Default to reasonable center position
            send_updates_frequency: 1.0 / 20.0, // 20 FPS like Unity
        }
    }
}