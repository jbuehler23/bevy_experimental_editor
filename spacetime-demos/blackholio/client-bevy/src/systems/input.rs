use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::components::*;
use crate::stdb::update_player_input_reducer::update_player_input;
use crate::stdb::DbVector2;
use crate::SpacetimeDB;

pub fn handle_keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&PlayerController, &mut PlayerInputState)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut cursor_moved_events: EventReader<CursorMoved>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyQ) {
        let Ok(window) = window_query.get_single() else {
            return;
        };

        // Get current mouse position
        let mouse_position = if let Some(cursor_event) = cursor_moved_events.read().last() {
            cursor_event.position
        } else {
            Vec2::new(window.width() / 2.0, window.height() / 2.0)
        };

        // Find the local player and toggle input lock
        for (player_controller, mut input_state) in player_query.iter_mut() {
            if player_controller.is_local {
                if input_state.lock_input_position.is_some() {
                    input_state.lock_input_position = None;
                    info!("Input unlocked - following mouse");
                } else {
                    input_state.lock_input_position = Some(mouse_position);
                    info!("Input locked at position: {:?}", mouse_position);
                }
                break;
            }
        }
    }
}

pub fn handle_mouse_input_and_send_updates(
    time: Res<Time>,
    stdb: SpacetimeDB,
    mut player_query: Query<(&PlayerController, &mut PlayerInputState), (With<PlayerController>, With<PlayerInputState>)>,
    circle_query: Query<&CircleController>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut cursor_moved_events: EventReader<CursorMoved>,
) {
    // Get the primary window
    let Ok(window) = window_query.single() else {
        return;
    };

    // Get current mouse position - use the latest cursor moved event or default to center
    let mouse_position = if let Some(cursor_event) = cursor_moved_events.read().last() {
        cursor_event.position
    } else {
        // Default to center if no mouse movement yet
        Vec2::new(window.width() / 2.0, window.height() / 2.0)
    };

    // Find the local player
    for (player_controller, mut input_state) in player_query.iter_mut() {
        if !player_controller.is_local {
            continue;
        }

        // Check if player has any circles (like Unity's NumberOfOwnedCircles == 0 check)
        let has_circles = circle_query.iter()
            .any(|c| c.player_id == player_controller.player_id);

        if !has_circles {
            continue;
        }

        // Throttled input requests (like Unity's SEND_UPDATES_FREQUENCY)
        let current_time = time.elapsed_secs();
        if current_time - input_state.last_movement_send_timestamp >= input_state.send_updates_frequency {
            input_state.last_movement_send_timestamp = current_time;

            // Use locked position or current mouse position
            let effective_mouse_position = input_state.lock_input_position.unwrap_or(mouse_position);

            // Convert to direction like Unity does
            let screen_size = Vec2::new(window.width(), window.height());
            let center_of_screen = screen_size / 2.0;

            // Unity math: direction = (mousePosition - centerOfScreen) / (screenSize.y / 3)
            let direction = (effective_mouse_position - center_of_screen) / (screen_size.y / 3.0);

            // Convert to DbVector2 for the reducer
            let db_direction = DbVector2 {
                x: direction.x,
                y: -direction.y, // Flip Y because screen coordinates are inverted vs world coordinates
            };

            // Call the reducer (equivalent to Unity's GameManager.Conn.Reducers.UpdatePlayerInput(direction))
            if let Err(err) = stdb.reducers().update_player_input(db_direction) {
                error!("Failed to send player input: {}", err);
            } else {
                // Only log occasionally to avoid spam
                if input_state.last_movement_send_timestamp as u32 % 2 == 0 {
                    debug!("Sent player input: direction={:?}", direction);
                }
            }
        }

        break; // Only handle one local player
    }
}