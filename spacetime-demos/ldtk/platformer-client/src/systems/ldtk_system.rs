use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

// Component to mark collision tiles from LDTk
#[derive(Component)]
pub struct CollisionTile;

// Component to mark wall entities
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;

// Bundle for wall tiles from LDTk IntGrid
#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

pub fn setup_ldtk_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load the LDTk project
    let ldtk_handle = asset_server.load("Typical_2D_platformer_example.ldtk");

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: LdtkProjectHandle { handle: ldtk_handle },
        ..Default::default()
    });

    info!("LDTk level loading started!");
}

// REMOVED: process_collision_tiles
//
// This system was removed because clients no longer need to process collision tiles
// for server synchronization. The server handles all collision logic directly from
// the LDTk file. Clients only use LDTk for visual rendering.

// REMOVED: extract_and_send_platforms
//
// This function was removed to fix the architecture flaw where every client
// would send collision data to the server. The server now loads level collision
// data directly from the LDTk file on initialization.
//
// The client should only use LDTk for rendering/visuals, not for collision logic.