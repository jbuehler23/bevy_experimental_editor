use spacetimedb::{Identity, ReducerContext, Table, SpacetimeType};
use std::collections::HashSet;

// Constants
const GRAVITY: f32 = 1000.0; // pixels per second^2
const JUMP_VELOCITY: f32 = -400.0; // pixels per second
const MOVE_SPEED: f32 = 200.0; // pixels per second
const PHYSICS_TICK_RATE: f32 = 1.0 / 20.0; // 20 Hz

// ============ TYPES ============

#[derive(SpacetimeType)]
pub struct PlatformData {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

// ============ TABLES ============

#[spacetimedb::table(name = player)]
pub struct Player {
    #[primary_key]
    pub identity: Identity,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub facing_right: bool,
    pub health: i32,
    pub max_health: i32,
    pub level: u32,
    pub grounded: bool,
}

#[spacetimedb::table(name = platform)]
pub struct Platform {
    #[primary_key]
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[spacetimedb::table(name = monster)]
pub struct Monster {
    #[primary_key]
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub health: i32,
    pub max_health: i32,
    pub monster_type: String,
    pub grounded: bool,
}

#[spacetimedb::table(name = player_input)]
pub struct PlayerInput {
    #[primary_key]
    pub identity: Identity,
    pub move_x: f32, // -1.0 (left), 0.0 (none), 1.0 (right)
    pub jump: bool,
    pub attack: bool,
}

#[spacetimedb::table(name = game_config)]
pub struct GameConfig {
    #[primary_key]
    pub id: u32,
    pub world_width: f32,
    pub world_height: f32,
    pub spawn_x: f32,
    pub spawn_y: f32,
}

// ============ LEVEL LOADING ============

fn load_ldtk_level_collision() -> Vec<PlatformData> {
    let ldtk_content = include_str!("../assets/Typical_2D_platformer_example.ldtk");

    match parse_ldtk_collision(ldtk_content) {
        Ok(platforms) => {
            log::info!("Successfully loaded {} platforms from LDTk", platforms.len());
            platforms
        }
        Err(e) => {
            log::error!("Failed to load LDTk level: {}", e);
            // Return default platforms as fallback
            vec![
                PlatformData { x: 0.0, y: 600.0, width: 2000.0, height: 20.0 },
                PlatformData { x: 300.0, y: 450.0, width: 200.0, height: 20.0 },
                PlatformData { x: 600.0, y: 350.0, width: 200.0, height: 20.0 },
            ]
        }
    }
}

fn parse_ldtk_collision(ldtk_content: &str) -> Result<Vec<PlatformData>, Box<dyn std::error::Error>> {
    use serde_json::Value;

    let project: Value = serde_json::from_str(ldtk_content)?;
    let mut platforms = Vec::new();

    // Get the first world and level
    if let Some(worlds) = project["worlds"].as_array() {
        if let Some(world) = worlds.first() {
            if let Some(levels) = world["levels"].as_array() {
                if let Some(level) = levels.first() {
                    let level_width = level["pxWid"].as_f64().unwrap_or(0.0) as f32;
                    let level_height = level["pxHei"].as_f64().unwrap_or(0.0) as f32;

                    // Process each layer
                    if let Some(layer_instances) = level["layerInstances"].as_array() {
                        for layer in layer_instances {
                            if let Some(layer_type) = layer["__type"].as_str() {
                                if layer_type == "IntGrid" {
                                    // This is a collision layer - extract platforms
                                    let grid_size = layer["__gridSize"].as_f64().unwrap_or(16.0) as f32;

                                    if let Some(int_grid) = layer["intGridCsv"].as_array() {
                                        let layer_width = layer["__cWid"].as_f64().unwrap_or(0.0) as i32;

                                        // Convert grid values to platform rectangles
                                        let collision_platforms = extract_platforms_from_intgrid(
                                            int_grid,
                                            layer_width,
                                            grid_size,
                                            level_height
                                        );
                                        platforms.extend(collision_platforms);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(platforms)
}

fn extract_platforms_from_intgrid(
    int_grid: &[Value],
    layer_width: i32,
    grid_size: f32,
    level_height: f32
) -> Vec<PlatformData> {
    let mut platforms = Vec::new();
    let mut visited = vec![false; int_grid.len()];

    for (index, cell) in int_grid.iter().enumerate() {
        if visited[index] || cell.as_i64().unwrap_or(0) != 1 {
            continue; // Skip non-collision tiles or already processed
        }

        let x = (index as i32 % layer_width) as f32 * grid_size;
        let y = level_height - ((index as i32 / layer_width) as f32 * grid_size) - grid_size; // Flip Y coordinate

        // For simplicity, create individual tile platforms
        // TODO: Optimize by combining adjacent tiles
        platforms.push(PlatformData {
            x,
            y,
            width: grid_size,
            height: grid_size,
        });

        visited[index] = true;
    }

    platforms
}

// ============ INIT REDUCER ============

#[spacetimedb::reducer(init)]
pub fn init(ctx: &ReducerContext) {
    // Initialize default game config
    ctx.db.game_config().insert(GameConfig {
        id: 0,
        world_width: 2000.0,
        world_height: 1000.0,
        spawn_x: 100.0,
        spawn_y: 500.0,
    });

    // Load platforms from LDTk level file (server-side)
    let platforms = load_ldtk_level_collision();

    // Insert platforms into database
    let mut platform_id = 0;
    for platform_data in platforms {
        ctx.db.platform().insert(Platform {
            id: platform_id,
            x: platform_data.x,
            y: platform_data.y,
            width: platform_data.width,
            height: platform_data.height,
        });
        platform_id += 1;
    }

    // Spawn a test monster
    ctx.db.monster().insert(Monster {
        id: 0,
        x: 500.0,
        y: 400.0,
        velocity_x: 0.0,
        velocity_y: 0.0,
        health: 50,
        max_health: 50,
        monster_type: "slime".to_string(),
        grounded: false,
    });

    // Note: SpacetimeDB doesn't support scheduled reducers in the same way
    // We'll need to handle physics updates differently, possibly via client triggers
    // or using a different approach

    log::info!("Platformer server initialized!");
}

// ============ CONNECTION HANDLERS ============

#[spacetimedb::reducer(client_connected)]
pub fn client_connected(ctx: &ReducerContext) {
    log::info!("Client connected: {:?}", ctx.sender);
}

#[spacetimedb::reducer(client_disconnected)]
pub fn client_disconnected(ctx: &ReducerContext) {
    let identity = ctx.sender;

    // Remove player and their input when they disconnect
    if let Some(player) = ctx.db.player().identity().find(&identity) {
        log::info!("Player {} disconnected", player.name);
        ctx.db.player().identity().delete(&identity);
    }

    ctx.db.player_input().identity().delete(&identity);
}

// ============ GAMEPLAY REDUCERS ============

#[spacetimedb::reducer]
pub fn enter_game(ctx: &ReducerContext, name: String) {
    let identity = ctx.sender;

    // Check if player already exists
    if ctx.db.player().identity().find(&identity).is_some() {
        log::warn!("Player already in game: {:?}", identity);
        return;
    }

    // Get spawn position from config
    let config = ctx.db.game_config().id().find(&0).unwrap();

    // Create new player
    ctx.db.player().insert(Player {
        identity,
        name: name.clone(),
        x: config.spawn_x,
        y: config.spawn_y,
        velocity_x: 0.0,
        velocity_y: 0.0,
        facing_right: true,
        health: 100,
        max_health: 100,
        level: 1,
        grounded: false,
    });

    // Initialize player input
    ctx.db.player_input().insert(PlayerInput {
        identity,
        move_x: 0.0,
        jump: false,
        attack: false,
    });

    log::info!("Player {} entered the game", name);
}

#[spacetimedb::reducer]
pub fn update_player_input(ctx: &ReducerContext, move_x: f32, jump: bool, attack: bool) {
    let identity = ctx.sender;

    if let Some(mut input) = ctx.db.player_input().identity().find(&identity) {
        input.move_x = move_x.max(-1.0).min(1.0); // Clamp to [-1, 1]
        input.jump = jump;
        input.attack = attack;
        ctx.db.player_input().identity().update(input);
    }
}

// DEPRECATED: This reducer is no longer needed as server loads levels automatically on init.
// Keeping it for backward compatibility but restricting access.
#[spacetimedb::reducer]
pub fn load_platforms(ctx: &ReducerContext, platforms: Vec<PlatformData>) {
    log::warn!(
        "load_platforms reducer called by {:?} - this is deprecated. Server loads levels automatically on init.",
        ctx.sender
    );

    // For security: Only allow if no platforms exist (fresh database)
    let platform_count = ctx.db.platform().iter().count();
    if platform_count > 0 {
        log::error!(
            "load_platforms blocked: Platforms already exist. Server loads levels on init, clients should not send collision data."
        );
        return;
    }

    log::info!("Emergency load_platforms: Loading {} platforms from client", platforms.len());

    let mut id = 0;
    for platform_data in &platforms {
        ctx.db.platform().insert(Platform {
            id,
            x: platform_data.x,
            y: platform_data.y,
            width: platform_data.width,
            height: platform_data.height,
        });
        id += 1;
    }
}

#[spacetimedb::reducer]
pub fn attack(ctx: &ReducerContext) {
    let identity = ctx.sender;

    if let Some(player) = ctx.db.player().identity().find(&identity) {
        let attack_range = 50.0;
        let attack_damage = 10;

        // Check all monsters in range
        for mut monster in ctx.db.monster().iter() {
            let dx = monster.x - player.x;
            let dy = monster.y - player.y;
            let distance = (dx * dx + dy * dy).sqrt();

            // Check if monster is in range and in front of player
            if distance <= attack_range {
                let is_in_front = (player.facing_right && dx > 0.0) || (!player.facing_right && dx < 0.0);
                if is_in_front {
                    let monster_id = monster.id; // Store ID before modification
                    monster.health = (monster.health - attack_damage).max(0);

                    if monster.health <= 0 {
                        log::info!("Player {} killed monster {}", player.name, monster_id);
                        ctx.db.monster().id().delete(&monster_id);
                    } else {
                        log::info!("Player {} hit monster {} for {} damage", player.name, monster_id, attack_damage);
                        ctx.db.monster().id().update(monster);
                    }
                }
            }
        }
    }
}

// ============ PHYSICS UPDATE REDUCER ============
// Note: In SpacetimeDB, we handle physics updates through input events
// rather than a scheduled tick. This simplifies the architecture.

#[spacetimedb::reducer]
pub fn update_physics(ctx: &ReducerContext) {
    let delta_time = PHYSICS_TICK_RATE;

    // Update all players
    for mut player in ctx.db.player().iter() {
        // Get player input
        if let Some(input) = ctx.db.player_input().identity().find(&player.identity) {
            // Apply horizontal movement
            player.velocity_x = input.move_x * MOVE_SPEED;

            // Update facing direction
            if input.move_x > 0.0 {
                player.facing_right = true;
            } else if input.move_x < 0.0 {
                player.facing_right = false;
            }

            // Apply jump if grounded
            if input.jump && player.grounded {
                player.velocity_y = JUMP_VELOCITY;
            }
        }

        // Apply gravity
        if !player.grounded {
            player.velocity_y += GRAVITY * delta_time;
            player.velocity_y = player.velocity_y.min(1000.0); // Terminal velocity
        }

        // Update position
        let new_x = player.x + player.velocity_x * delta_time;
        let new_y = player.y + player.velocity_y * delta_time;

        // Check platform collisions
        let mut grounded = false;
        let final_x = new_x;
        let mut final_y = new_y;

        let player_width = 32.0;
        let player_height = 48.0;

        for platform in ctx.db.platform().iter() {
            if check_collision(
                final_x, final_y, player_width, player_height,
                platform.x, platform.y, platform.width, platform.height
            ) {
                // Simple collision resolution - just place on top of platform
                if player.velocity_y > 0.0 && player.y < platform.y {
                    final_y = platform.y - player_height;
                    player.velocity_y = 0.0;
                    grounded = true;
                }
            }
        }

        // Apply world bounds
        let final_x = final_x.max(0.0).min(2000.0 - player_width);

        player.x = final_x;
        player.y = final_y;
        player.grounded = grounded;

        ctx.db.player().identity().update(player);
    }

    // Update all monsters (simple AI)
    for mut monster in ctx.db.monster().iter() {
        // Simple AI - pace back and forth
        if monster.grounded {
            if monster.velocity_x == 0.0 {
                monster.velocity_x = 50.0; // Start moving right
            }

            // Reverse at edges (simple patrol)
            if monster.x > 700.0 || monster.x < 300.0 {
                monster.velocity_x = -monster.velocity_x;
            }
        }

        // Apply gravity
        if !monster.grounded {
            monster.velocity_y += GRAVITY * delta_time;
            monster.velocity_y = monster.velocity_y.min(1000.0);
        }

        // Update position
        let new_x = monster.x + monster.velocity_x * delta_time;
        let new_y = monster.y + monster.velocity_y * delta_time;

        // Check platform collisions
        let mut grounded = false;
        let final_x = new_x;
        let mut final_y = new_y;

        let monster_width = 32.0;
        let monster_height = 32.0;

        for platform in ctx.db.platform().iter() {
            if check_collision(
                final_x, final_y, monster_width, monster_height,
                platform.x, platform.y, platform.width, platform.height
            ) {
                if monster.velocity_y > 0.0 && monster.y < platform.y {
                    final_y = platform.y - monster_height;
                    monster.velocity_y = 0.0;
                    grounded = true;
                }
            }
        }

        monster.x = final_x;
        monster.y = final_y;
        monster.grounded = grounded;

        ctx.db.monster().id().update(monster);
    }
}

// ============ HELPER FUNCTIONS ============

fn check_collision(
    x1: f32, y1: f32, w1: f32, h1: f32,
    x2: f32, y2: f32, w2: f32, h2: f32
) -> bool {
    x1 < x2 + w2 &&
    x1 + w1 > x2 &&
    y1 < y2 + h2 &&
    y1 + h1 > y2
}