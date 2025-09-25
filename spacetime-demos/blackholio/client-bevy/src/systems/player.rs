use bevy::prelude::*;
use crate::components::*;
use crate::stdb::Player;
use spacetimedb_sdk::Identity;

pub fn spawn_player(
    commands: &mut Commands,
    player: &Player,
    local_identity: Option<&Identity>,
) -> Entity {
    let is_local = if let Some(identity) = local_identity {
        player.identity == *identity
    } else {
        false
    };

    let player_entity = commands.spawn((
        PlayerController::new(player.player_id, player.name.clone(), is_local),
        Name::new(format!("Player-{}", player.name)),
        // Players don't have a visual representation themselves, just their circles
        Transform::default(),
    )).id();

    player_entity
}

pub fn update_player_mass_ui(
    query: Query<&PlayerController>,
    entity_query: Query<(&EntityController, &CircleController)>,
    entity_map: Res<EntityMap>,
) {
    // This would be used to update UI elements showing player mass
    // For now, we'll just calculate the mass for logging purposes
    for player in query.iter() {
        if player.is_local {
            let mut total_mass = 0u32;

            for (entity_controller, circle_controller) in entity_query.iter() {
                if circle_controller.player_id == player.player_id {
                    // In a real implementation, we'd need to look up the mass from the entity data
                    // For now, this is a placeholder
                }
            }

            // In Unity version, this would be displayed as GUI text
            // In Bevy, we'd spawn UI elements or use egui
            info!("Player {} total mass: {}", player.username, total_mass);
        }
    }
}

pub fn calculate_player_center_of_mass(
    player_query: Query<&PlayerController>,
    entity_query: Query<(&Transform, &CircleController, &EntityController)>,
) -> Option<Vec2> {
    // Find local player
    let local_player = player_query.iter().find(|p| p.is_local)?;

    let mut total_position = Vec2::ZERO;
    let mut total_mass = 0.0;
    let mut circle_count = 0;

    for (transform, circle_controller, _entity_controller) in entity_query.iter() {
        if circle_controller.player_id == local_player.player_id {
            // Weight position by mass (we'd need actual mass data here)
            let mass = 1.0; // Placeholder - should get from entity data
            total_position += transform.translation.truncate() * mass;
            total_mass += mass;
            circle_count += 1;
        }
    }

    if circle_count > 0 && total_mass > 0.0 {
        Some(total_position / total_mass)
    } else {
        None
    }
}

pub fn handle_player_death(
    mut commands: Commands,
    player_query: Query<(Entity, &PlayerController)>,
    circle_query: Query<&CircleController>,
) {
    for (player_entity, player_controller) in player_query.iter() {
        if player_controller.is_local {
            // Check if player has any circles left
            let has_circles = circle_query.iter()
                .any(|c| c.player_id == player_controller.player_id);

            if !has_circles {
                // Player is dead - in Unity version this shows death screen
                info!("Player {} has been eliminated!", player_controller.username);
                // Could trigger death UI here
            }
        }
    }
}