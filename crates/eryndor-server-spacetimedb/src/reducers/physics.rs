use spacetimedb::ReducerContext;
use eryndor_common::CollisionShape;
use crate::collision::*;
use crate::math::DbVector2;

/// Physics constants
const PHYSICS_TICK_RATE: f32 = 1.0 / 20.0; // 20 Hz (50ms per tick)
const GRAVITY: f32 = 980.0; // pixels per second squared
const MAX_VELOCITY_Y: f32 = 1000.0;
const MAX_VELOCITY_X: f32 = 500.0;

/// Update physics for all entities
#[spacetimedb::reducer]
pub fn update_physics(ctx: &ReducerContext) {
    let delta_time = PHYSICS_TICK_RATE;

    // Update all players
    for mut player in ctx.db.player().iter() {
        // Apply gravity
        player.velocity_y += GRAVITY * delta_time;
        player.velocity_y = player.velocity_y.clamp(-MAX_VELOCITY_Y, MAX_VELOCITY_Y);

        // Apply horizontal damping (friction)
        player.velocity_x *= 0.9;
        player.velocity_x = player.velocity_x.clamp(-MAX_VELOCITY_X, MAX_VELOCITY_X);

        // Get player input (would be from player_input table in full implementation)
        // For now, using placeholder values
        let input_x = 0.0; // Would come from input table
        let move_speed = 200.0;
        player.velocity_x += input_x * move_speed * delta_time;

        // Store old position for collision resolution
        let old_pos = DbVector2 {
            x: player.position_x,
            y: player.position_y,
        };

        // Update position
        player.position_x += player.velocity_x * delta_time;
        player.position_y += player.velocity_y * delta_time;

        // Player AABB size (should be from player properties)
        let player_size = DbVector2 { x: 32.0, y: 64.0 };
        let mut player_pos = DbVector2 {
            x: player.position_x,
            y: player.position_y,
        };

        // Check collisions with tiles
        let mut on_ground = false;

        for tile in ctx.db.tilemap_tile().iter() {
            // Get collision shapes for this tile
            for collision_shape in ctx.db.tile_collision_shape().iter() {
                if collision_shape.tileset_id != tile.tileset_id
                    || collision_shape.tile_id != tile.tile_id
                {
                    continue;
                }

                // Deserialize collision shape
                let shape: Result<CollisionShape, _> =
                    serde_json::from_str(&collision_shape.shape_data);

                if let Ok(shape) = shape {
                    let tile_pos = DbVector2 {
                        x: tile.x as f32,
                        y: tile.y as f32,
                    };

                    if check_shape_collision(&player_pos, &player_size, &shape, &tile_pos) {
                        // Resolve collision
                        resolve_collision(&mut player_pos, &player_size, &shape, &tile_pos);

                        // Check if landed on top (simplified)
                        if player.velocity_y > 0.0 {
                            player.velocity_y = 0.0;
                            on_ground = true;
                        }
                    }
                }
            }
        }

        // Update player position after collision resolution
        player.position_x = player_pos.x;
        player.position_y = player_pos.y;

        // Update player state
        player.is_grounded = on_ground;

        // Update in database
        ctx.db.player().identity().update(player);
    }
}

/// Update player input (called from client)
#[spacetimedb::reducer]
pub fn update_player_input(
    ctx: &ReducerContext,
    move_x: f32,
    move_y: f32,
    jump: bool,
) {
    // Find player for this identity
    for mut player in ctx.db.player().iter() {
        if player.identity == ctx.sender {
            // Store input for next physics update
            // In a full implementation, this would go into a player_input table
            // For now, we can apply it directly

            // Horizontal movement
            player.velocity_x = move_x * 200.0;

            // Jump
            if jump && player.is_grounded {
                player.velocity_y = -400.0; // Jump velocity
            }

            ctx.db.player().identity().update(player);
            break;
        }
    }
}

/// Player input table (for storing inputs between physics ticks)
#[spacetimedb::table(name = player_input)]
pub struct PlayerInput {
    #[primary_key]
    pub player_identity: spacetimedb::Identity,
    pub move_x: f32,
    pub move_y: f32,
    pub jump: bool,
    pub attack: bool,
}
