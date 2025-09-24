use bevy::{log::LogPlugin, prelude::*};
use bevy_spacetimedb::{
    ReadDeleteEvent, ReadInsertEvent, ReadInsertUpdateEvent, ReadReducerEvent,
    ReadStdbConnectedEvent, ReadUpdateEvent, ReducerResultEvent, RegisterReducerEvent,
    StdbConnection, StdbPlugin, TableEvents,
};
use spacetimedb_sdk::ReducerEvent;
use stdb::{DbConnection, Reducer};

use crate::stdb::{
    Player, Circle, Entity, Food, Config,
    RemoteModule, RemoteReducers, RemoteTables,
};
use crate::stdb::connect_reducer::connect;

mod stdb;

#[derive(Debug, RegisterReducerEvent)]
#[allow(dead_code)]
pub struct Connect {
    event: ReducerEvent<Reducer>,
}

pub type SpacetimeDB<'a> = Res<'a, StdbConnection<DbConnection>>;

pub fn main() {
    App::new()
        .add_plugins((MinimalPlugins, LogPlugin::default()))
        .add_plugins(
            StdbPlugin::default()
                .with_uri("http://localhost:3000")
                .with_module_name("spacetime-module")
                .with_run_fn(DbConnection::run_threaded)
                .add_table::<Player>()
                .add_table::<Entity>()
                .add_table::<Circle>()
                .add_table::<Food>()
                .add_table::<Config>()
                .add_reducer::<Connect>(),
        )
        .add_systems(Update, on_connected)
        .add_systems(Update, on_player_inserted)
        .add_systems(Update, on_player_deleted)
        .add_systems(Update, on_entity_inserted)
        .add_systems(Update, on_connect_reducer)
        .run();
}

// SpacetimeDB is defined as an alias for the StdbConnection with DbConnection.
fn on_connected(mut events: ReadStdbConnectedEvent, stdb: SpacetimeDB) {
    for _ev in events.read() {
        info!("Connected to SpacetimeDB");

        // Subscribe to all players
        stdb.subscription_builder()
            .on_applied(|_| info!("Subscription to players applied"))
            .on_error(|_, err| error!("Subscription to players failed for: {}", err))
            .subscribe("SELECT * FROM player");

        // Subscribe to all entities (includes circles and food)
        stdb.subscription_builder()
            .on_applied(|_| info!("Subscription to entities applied"))
            .on_error(|_, err| error!("Subscription to entities failed for: {}", err))
            .subscribe("SELECT * FROM entity");

        // Subscribe to circles
        stdb.subscription_builder()
            .on_applied(|_| info!("Subscription to circles applied"))
            .on_error(|_, err| error!("Subscription to circles failed for: {}", err))
            .subscribe("SELECT * FROM circle");

        // Subscribe to food
        stdb.subscription_builder()
            .on_applied(|_| info!("Subscription to food applied"))
            .on_error(|_, err| error!("Subscription to food failed for: {}", err))
            .subscribe("SELECT * FROM food");

        // Subscribe to config
        stdb.subscription_builder()
            .on_applied(|_| info!("Subscription to config applied"))
            .on_error(|_, err| error!("Subscription to config failed for: {}", err))
            .subscribe("SELECT * FROM config");
    }
}

fn on_player_inserted(mut events: ReadInsertEvent<Player>) {
    for event in events.read() {
        // Row below is just an example, does not actually compile.
        // commands.spawn(Player { id: event.row.id });
        info!("Player inserted: {:?}", event.row);
    }
}


fn on_player_deleted(mut events: ReadDeleteEvent<Player>) {
    for event in events.read() {
        info!("Player deleted: {:?}", event.row);
    }
}

fn on_entity_inserted(mut events: ReadInsertEvent<Entity>) {
    for event in events.read() {
        info!("Entity inserted: {:?}", event.row);
    }
}

fn on_connect_reducer(mut events: ReadReducerEvent<Connect>) {
    for event in events.read() {
        info!("Connect reducer called: {:?}", event.result);
    }
}
