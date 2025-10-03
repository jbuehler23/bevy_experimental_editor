use bevy::prelude::*;
use crate::components::GameState;
use crate::SpacetimeDB;
use crate::stdb::update_player_input_reducer::update_player_input;
use crate::stdb::attack_reducer::attack;

#[derive(Resource, Default)]
pub struct InputState {
    pub move_x: f32,
    pub jump: bool,
    pub attack: bool,
    pub last_sent: f32,
}

pub fn handle_player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut input_state: ResMut<InputState>,
    time: Res<Time>,
    game_state: Res<GameState>,
    stdb: SpacetimeDB,
) {
    if !game_state.connected || !game_state.in_game {
        return;
    }

    // Get movement input
    let mut move_x = 0.0;
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        move_x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        move_x += 1.0;
    }

    // Get jump input
    let jump = keyboard.pressed(KeyCode::Space);

    // Get attack input
    let attack = keyboard.just_pressed(KeyCode::KeyZ) || keyboard.just_pressed(KeyCode::KeyX);

    // Update input state
    input_state.move_x = move_x;
    input_state.jump = jump;
    input_state.attack = attack;

    // Send input to server at 20Hz
    let current_time = time.elapsed_secs();
    if current_time - input_state.last_sent >= 0.05 {  // 20 Hz
        input_state.last_sent = current_time;

        // Call SpacetimeDB reducer to send input
        if let Err(err) = stdb.reducers().update_player_input(move_x, jump, attack) {
            error!("Failed to send player input: {}", err);
        }
    }
}

pub fn handle_attack_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    game_state: Res<GameState>,
    stdb: SpacetimeDB,
) {
    if !game_state.connected || !game_state.in_game {
        return;
    }

    if keyboard.just_pressed(KeyCode::KeyZ) || keyboard.just_pressed(KeyCode::KeyX) {
        // Call SpacetimeDB attack reducer
        if let Err(err) = stdb.reducers().attack() {
            error!("Failed to attack: {}", err);
        } else {
            info!("Attack sent to server!");
        }
    }
}