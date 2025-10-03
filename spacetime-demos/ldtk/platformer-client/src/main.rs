use bevy::prelude::*;
use bevy_spacetimedb::{
    ReadInsertEvent, ReadStdbConnectedEvent, StdbConnection, StdbPlugin,
};
use bevy_ecs_ldtk::prelude::*;
use crate::stdb::enter_game_reducer::enter_game;
use crate::stdb::player_table::PlayerTableAccess;
use crate::stdb::update_physics_reducer::update_physics;

mod components;
mod stdb;
mod systems;

use components::*;
use systems::{setup_camera, camera_follow_player, spawn_player_sprite, setup_ldtk_level, WallBundle};
use systems::player_input::{InputState, handle_player_input, handle_attack_input};

// Type alias for SpacetimeDB resource access
pub type SpacetimeDB<'a> = Res<'a, StdbConnection<stdb::DbConnection>>;

fn main() {
    // Parse command-line arguments for username
    let args: Vec<String> = std::env::args().collect();
    let username = if args.len() > 1 {
        args[1].clone()
    } else {
        "Player".to_string()
    };

    println!("Starting platformer client with username: {}", username);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LdtkPlugin)
        .add_plugins(
            StdbPlugin::default()
                .with_uri("http://localhost:3000")
                .with_module_name("ldtk-platformer")
                .with_run_fn(stdb::DbConnection::run_threaded)
                .add_table(|db| db.player()),
        )
        .insert_resource(LevelSelection::index(0))
        // Resources
        .init_resource::<EntityMap>()
        .init_resource::<GameState>()
        .init_resource::<InputState>()
        .insert_resource(LocalIdentity(None))
        .insert_resource(Username(username))
        // Setup systems
        .add_systems(Startup, (setup_camera, setup_ldtk_level))
        // Connection systems
        .add_systems(Update, on_connected)
        // SpacetimeDB event handlers
        .add_systems(Update, on_player_inserted)
        // LDTk systems (client-side rendering only)
        .register_ldtk_int_cell::<WallBundle>(1)  // Register IntGrid value 1 as walls for visual rendering
        // Input systems
        .add_systems(Update, (handle_player_input, handle_attack_input))
        // Physics system
        .add_systems(Update, update_server_physics)
        // Game systems
        .add_systems(Update, camera_follow_player)
        .run();
}

// SpacetimeDB connection handler
fn on_connected(
    mut events: ReadStdbConnectedEvent,
    stdb: SpacetimeDB,
    mut local_identity: ResMut<LocalIdentity>,
    mut game_state: ResMut<GameState>,
    username: Res<Username>,
) {
    for ev in events.read() {
        println!("Connected to SpacetimeDB with identity: {:?}", ev.identity);
        local_identity.0 = Some(ev.identity.clone());
        game_state.connected = true;

        // Subscribe to all tables
        let username_clone = username.0.clone();
        stdb.subscription_builder()
            .on_applied(move |ctx| {
                println!("Subscriptions applied - entering game as {}", username_clone);
                // Call enter_game reducer
                if let Err(err) = ctx.reducers.enter_game(username_clone.clone()) {
                    println!("Failed to enter game: {}", err);
                } else {
                    println!("Successfully entered game!");
                }
            })
            .on_error(|_, err| println!("Subscription failed: {}", err))
            .subscribe("SELECT * FROM *");
    }
}

// SpacetimeDB Event Handlers
fn on_player_inserted(
    mut events: ReadInsertEvent<stdb::Player>,
    mut commands: Commands,
    mut entity_map: ResMut<EntityMap>,
    local_identity: Res<LocalIdentity>,
    mut game_state: ResMut<GameState>,
) {
    for event in events.read() {
        let player = &event.row;
        let is_local = local_identity.0.as_ref().map_or(false, |id| id == &player.identity);

        println!("Player inserted: {} (local: {})", player.name, is_local);

        let player_entity = spawn_player_sprite(
            &mut commands,
            Vec3::new(player.x, player.y, 0.0),
            is_local
        );

        commands.entity(player_entity).insert(Player {
            identity: player.identity,
            name: player.name.clone(),
            health: player.health,
            max_health: player.max_health,
            level: player.level,
            facing_right: player.facing_right,
        });

        if is_local {
            commands.entity(player_entity).insert(LocalPlayer);
            game_state.in_game = true;
            println!("Local player entered game!");
        }

        entity_map.players.insert(player.identity, player_entity);
    }
}

// Physics Update System
#[derive(Resource)]
struct PhysicsTimer {
    timer: Timer,
}

impl Default for PhysicsTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.05, TimerMode::Repeating), // 20 Hz
        }
    }
}

fn update_server_physics(
    time: Res<Time>,
    mut physics_timer: Local<Option<PhysicsTimer>>,
    game_state: Res<GameState>,
    stdb: SpacetimeDB,
) {
    if !game_state.connected || !game_state.in_game {
        return;
    }

    // Initialize timer if not already done
    if physics_timer.is_none() {
        *physics_timer = Some(PhysicsTimer::default());
    }

    let timer = physics_timer.as_mut().unwrap();
    timer.timer.tick(time.delta());

    if timer.timer.just_finished() {
        // Call the server physics update
        if let Err(err) = stdb.reducers().update_physics() {
            error!("Failed to update server physics: {}", err);
        }
    }
}