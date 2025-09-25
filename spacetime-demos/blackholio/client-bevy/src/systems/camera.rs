use bevy::prelude::*;
use crate::components::{ArenaConfig, PlayerController, CircleController, EntityController};
use crate::systems::player::calculate_player_center_of_mass;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 999.0),
        Name::new("Main Camera"),
    ));
}

pub fn camera_follow_player(
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera2d>>,
    player_query: Query<&PlayerController>,
    entity_query: Query<(&Transform, &CircleController, &EntityController), Without<Camera2d>>,
    arena_config: Res<ArenaConfig>,
    time: Res<Time>,
) {
    let Ok((mut camera_transform, mut projection)) = camera_query.get_single_mut() else {
        return;
    };

    // Find local player
    let local_player = player_query.iter().find(|p| p.is_local);

    let target_position = if let Some(player) = local_player {
        // Calculate center of mass for local player's circles
        if let Some(center) = calculate_center_of_mass(player, &entity_query) {
            center.extend(camera_transform.translation.z)
        } else {
            // No circles, center on arena
            Vec3::new(
                arena_config.world_size / 2.0,
                arena_config.world_size / 2.0,
                camera_transform.translation.z,
            )
        }
    } else {
        // No local player, center on arena
        Vec3::new(
            arena_config.world_size / 2.0,
            arena_config.world_size / 2.0,
            camera_transform.translation.z,
        )
    };

    // Lerp camera position for smooth movement
    camera_transform.translation = camera_transform.translation.lerp(
        target_position,
        time.delta_secs() * 5.0,
    );

    // Adjust camera zoom based on player mass and split count
    if let Some(player) = local_player {
        let (total_mass, circle_count) = calculate_player_stats(player, &entity_query);
        let target_scale = calculate_camera_scale(total_mass, circle_count);

        // Lerp camera scale for smooth zoom
        projection.scale = projection.scale.lerp(target_scale, time.delta_secs() * 2.0);
    }
}

fn calculate_center_of_mass(
    player: &PlayerController,
    entity_query: &Query<(&Transform, &CircleController, &EntityController), Without<Camera2d>>,
) -> Option<Vec2> {
    let mut total_position = Vec2::ZERO;
    let mut total_mass = 0.0;
    let mut circle_count = 0;

    for (transform, circle_controller, _entity_controller) in entity_query.iter() {
        if circle_controller.player_id == player.player_id {
            // Using uniform mass for now - in real implementation, get from entity data
            let mass = 1.0;
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

fn calculate_player_stats(
    player: &PlayerController,
    entity_query: &Query<(&Transform, &CircleController, &EntityController), Without<Camera2d>>,
) -> (u32, usize) {
    let mut total_mass = 0u32;
    let mut circle_count = 0usize;

    for (_transform, circle_controller, _entity_controller) in entity_query.iter() {
        if circle_controller.player_id == player.player_id {
            // In real implementation, get mass from entity data
            total_mass += 10; // Placeholder
            circle_count += 1;
        }
    }

    (total_mass, circle_count)
}

fn calculate_camera_scale(total_mass: u32, circle_count: usize) -> f32 {
    // Base size
    let base_scale = 1.0;

    // Increase camera scale with mass
    let mass_scale = (total_mass as f32 / 500.0).min(0.5);

    // Zoom out when player splits
    let split_scale = if circle_count > 1 { 0.3 } else { 0.0 };

    base_scale + mass_scale + split_scale
}