use bevy::{log::LogPlugin, prelude::*};
use bevy_spacetimedb::{
    ReadDeleteEvent, ReadInsertEvent, ReadStdbConnectedEvent, ReadUpdateEvent, StdbConnection,
    StdbPlugin,
};
use spacetimedb_sdk::Identity;
use stdb::DbConnection;

use crate::stdb::circle_table::CircleTableAccess;
use crate::stdb::config_table::ConfigTableAccess;
use crate::stdb::enter_game_reducer::enter_game;
use crate::stdb::entity_table::EntityTableAccess;
use crate::stdb::food_table::FoodTableAccess;
use crate::stdb::player_table::PlayerTableAccess;
use crate::stdb::{Circle, Config, Entity as DbEntity, Food, Player, RemoteTables};

mod components;
mod stdb;
mod systems;
mod utils;

use components::*;
use systems::*;

pub type SpacetimeDB<'a> = Res<'a, StdbConnection<DbConnection>>;

#[derive(Resource)]
pub struct LocalIdentity(pub Option<Identity>);

// Simplified subscription state - just tracking if we've entered the game
#[derive(Resource, Default)]
pub struct SubscriptionState {
    pub entered_game: bool,
}

pub fn main() {
    App::new()
        // Core plugins - using DefaultPlugins for rendering
        .add_plugins(DefaultPlugins.set(LogPlugin {
            level: bevy::log::Level::INFO,
            ..default()
        }))
        // SpacetimeDB plugin
        .add_plugins(
            StdbPlugin::default()
                .with_uri("http://localhost:3000")
                .with_module_name("blackholio")
                .with_run_fn(DbConnection::run_threaded)
                .add_table(RemoteTables::player)
                .add_table(RemoteTables::entity)
                .add_table(RemoteTables::circle)
                .add_table(RemoteTables::food)
                .add_table(RemoteTables::config),
        )
        // Resources
        .init_resource::<ArenaConfig>()
        .init_resource::<EntityMap>()
        .init_resource::<PlayerMap>()
        .insert_resource(LocalPlayerEntity(None))
        .insert_resource(LocalIdentity(None))
        .init_resource::<SubscriptionState>()
        // Setup systems
        .add_systems(Startup, setup_camera)
        // Connection and subscription systems
        .add_systems(Update, on_connected)
        // Table event handlers
        .add_systems(
            Update,
            (
                on_config_received,
                on_player_inserted,
                on_player_deleted,
                on_entity_inserted,
                on_entity_updated,
                on_entity_deleted,
                on_circle_inserted,
                on_food_inserted,
            ),
        )
        // Game systems
        .add_systems(
            Update,
            (
                update_entity_positions,
                camera_follow_player,
                handle_player_death,
            ),
        )
        .run();
}

fn on_connected(
    mut events: ReadStdbConnectedEvent,
    stdb: SpacetimeDB,
    mut local_identity: ResMut<LocalIdentity>,
    mut subscription_state: ResMut<SubscriptionState>,
) {
    for ev in events.read() {
        info!("Connected to SpacetimeDB with identity: {:?}", ev.identity);
        local_identity.0 = Some(ev.identity.clone());

        // Subscribe to config first to get world size - this is the critical one
        stdb.subscription_builder()
            .on_applied(|ctx| {
                info!("Subscription to config applied - calling enter_game!");
                // Call enter_game as soon as config is available, like Unity does
                if let Err(err) = ctx.reducers.enter_game("Joe".to_string()) {
                    error!("Failed to call enter_game reducer: {}", err);
                } else {
                    info!("Successfully called enter_game reducer");
                }
            })
            .on_error(|_, err| error!("Subscription to config failed: {}", err))
            .subscribe("SELECT * FROM config");

        // Subscribe to all other tables
        stdb.subscription_builder()
            .on_applied(|_| info!("Subscription to players applied"))
            .on_error(|_, err| error!("Subscription to players failed: {}", err))
            .subscribe("SELECT * FROM player");

        stdb.subscription_builder()
            .on_applied(|_| info!("Subscription to entities applied"))
            .on_error(|_, err| error!("Subscription to entities failed: {}", err))
            .subscribe("SELECT * FROM entity");

        stdb.subscription_builder()
            .on_applied(|_| info!("Subscription to circles applied"))
            .on_error(|_, err| error!("Subscription to circles failed: {}", err))
            .subscribe("SELECT * FROM circle");

        stdb.subscription_builder()
            .on_applied(|_| info!("Subscription to food applied"))
            .on_error(|_, err| error!("Subscription to food failed: {}", err))
            .subscribe("SELECT * FROM food");

        // Mark that we've initiated the connection process
        subscription_state.entered_game = true;
    }
}

fn on_config_received(
    mut events: ReadInsertEvent<Config>,
    mut commands: Commands,
    mut arena_config: ResMut<ArenaConfig>,
) {
    for event in events.read() {
        info!("Config received with world size: {}", event.row.world_size);
        arena_config.world_size = event.row.world_size as f32;

        // Setup arena borders when we receive config
        setup_arena(&mut commands, arena_config.as_ref());
    }
}

fn on_player_inserted(
    mut events: ReadInsertEvent<Player>,
    mut commands: Commands,
    mut player_map: ResMut<PlayerMap>,
    local_identity: Res<LocalIdentity>,
    mut local_player_entity: ResMut<LocalPlayerEntity>,
) {
    for event in events.read() {
        info!("Player inserted: {}", event.row.name);

        let player_entity = spawn_player(&mut commands, &event.row, local_identity.0.as_ref());

        player_map
            .players
            .insert(event.row.player_id, player_entity);

        // Track local player entity
        if let Some(identity) = &local_identity.0 {
            if event.row.identity == *identity {
                local_player_entity.0 = Some(player_entity);
            }
        }
    }
}

fn on_player_deleted(
    mut events: ReadDeleteEvent<Player>,
    mut commands: Commands,
    mut player_map: ResMut<PlayerMap>,
) {
    for event in events.read() {
        info!("Player deleted: {}", event.row.name);

        if let Some(entity) = player_map.players.remove(&event.row.player_id) {
            commands.entity(entity).despawn();
        }
    }
}

fn on_circle_inserted(
    mut events: ReadInsertEvent<Circle>,
    mut commands: Commands,
    stdb: SpacetimeDB,
    player_map: Res<PlayerMap>,
    mut entity_map: ResMut<EntityMap>,
) {
    for event in events.read() {
        // Get the corresponding entity data
        let db = stdb.db();
        if let Some(entity) = db.entity().entity_id().find(&event.row.entity_id) {
            // Get the player data
            if let Some(player) = db.player().player_id().find(&event.row.player_id) {
                // Get player entity
                if let Some(&player_entity) = player_map.players.get(&event.row.player_id) {
                    let circle_entity = spawn_circle(
                        &mut commands,
                        &event.row,
                        &entity,
                        player_entity,
                        event.row.player_id,
                        &player.name,
                    );

                    entity_map
                        .entities
                        .insert(event.row.entity_id, circle_entity);

                    info!("Circle spawned for player {}", player.name);
                }
            }
        }
    }
}

fn on_food_inserted(
    mut events: ReadInsertEvent<Food>,
    mut commands: Commands,
    stdb: SpacetimeDB,
    mut entity_map: ResMut<EntityMap>,
) {
    for event in events.read() {
        // Get the corresponding entity data
        let db = stdb.db();
        if let Some(entity) = db.entity().entity_id().find(&event.row.entity_id) {
            let food_entity = spawn_food(&mut commands, &event.row, &entity);
            entity_map.entities.insert(event.row.entity_id, food_entity);

            info!("Food spawned with id {}", event.row.entity_id);
        }
    }
}

fn on_entity_inserted(mut events: ReadInsertEvent<DbEntity>) {
    for event in events.read() {
        // Entity insertions are handled by circle_inserted and food_inserted
        // This is just for logging
        info!(
            "Entity inserted: id={}, mass={}",
            event.row.entity_id, event.row.mass
        );
    }
}

fn on_entity_updated(
    mut events: ReadUpdateEvent<DbEntity>,
    entity_map: Res<EntityMap>,
    mut entity_query: Query<(&mut EntityController, &mut Sprite)>,
) {
    for event in events.read() {
        let new_position: Vec2 = event.new.position.clone().into();

        if let Some(&entity) = entity_map.entities.get(&event.new.entity_id) {
            if let Ok((mut controller, mut sprite)) = entity_query.get_mut(entity) {
                // Update lerp targets
                controller.lerp_time = 0.0;
                controller.lerp_start = controller.lerp_target;
                controller.lerp_target = new_position.extend(0.0);
                controller.target_scale = utils::mass_to_scale(event.new.mass);

                // Update sprite size
                sprite.custom_size = Some(Vec2::splat(utils::mass_to_diameter(event.new.mass)));
            }
        }
    }
}

fn on_entity_deleted(
    mut events: ReadDeleteEvent<DbEntity>,
    mut commands: Commands,
    mut entity_map: ResMut<EntityMap>,
) {
    for event in events.read() {
        info!("Entity deleted: id={}", event.row.entity_id);

        if let Some(entity) = entity_map.entities.remove(&event.row.entity_id) {
            commands.entity(entity).despawn();
        }
    }
}

// Remove the problematic mark_subscriptions_applied function - we'll use a simpler approach

// Removed check_enter_game system - now handled directly in subscription callback
