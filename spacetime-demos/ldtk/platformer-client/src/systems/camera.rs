use bevy::prelude::*;
use crate::components::LocalPlayer;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 1000.0),
    ));
}

pub fn camera_follow_player(
    player_query: Query<&Transform, (With<LocalPlayer>, Without<Camera2d>)>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
) {
    if let Ok(player_transform) = player_query.single() {
        if let Ok(mut camera_transform) = camera_query.single_mut() {
            // Smooth camera follow with lerp
            let target_x = player_transform.translation.x;
            let target_y = player_transform.translation.y;

            camera_transform.translation.x = camera_transform.translation.x.lerp(target_x, 0.1);
            camera_transform.translation.y = camera_transform.translation.y.lerp(target_y, 0.1);
        }
    }
}