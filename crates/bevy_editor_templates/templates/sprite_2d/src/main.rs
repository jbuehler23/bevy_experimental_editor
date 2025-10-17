//! {{PROJECT_NAME}} - A 2D Sprite-based Bevy game
//!
//! This template includes:
//! - Sprite rendering setup
//! - 2D camera with pan and zoom controls
//! - Example player movement system
//! - Editor scene loading support

use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    // Add default Bevy plugins with custom window settings
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "{{PROJECT_NAME}}".to_string(),
            resolution: (1280.0, 720.0).into(),
            ..default()
        }),
        ..default()
    }));

    // Add the editor scene loader plugin (optional, controlled by feature flag)
    #[cfg(feature = "editor-runtime")]
    {
        app.add_plugins(bevy_editor_runtime::EditorSceneLoaderPlugin);
        info!("Editor scene loader enabled - set BEVY_EDITOR_SCENE to load a scene");
    }

    // Add game systems
    app.add_systems(Startup, setup);
    app.add_systems(Update, (
        player_movement,
        camera_controls,
    ));

    app.run();
}

/// Marker component for the player entity
#[derive(Component)]
struct Player;

/// Component for camera movement
#[derive(Component)]
struct GameCamera;

/// Initial setup - spawns camera and example player
fn setup(mut commands: Commands) {
    // Spawn a 2D camera
    commands.spawn((
        Camera2d,
        GameCamera,
        Name::new("Main Camera"),
    ));

    // Spawn an example player sprite
    // Note: You'll need to add an actual sprite asset to assets/sprites/
    // For now, this creates a colored square as a placeholder
    commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.6, 0.8),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        Player,
        Name::new("Player"),
    ));

    info!("{{PROJECT_NAME}} initialized!");
    info!("Controls:");
    info!("  WASD - Move player");
    info!("  Arrow Keys - Pan camera");
    info!("  Q/E - Zoom camera in/out");
}

/// Player movement system - WASD controls
fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    for mut transform in &mut player_query {
        let mut direction = Vec3::ZERO;
        let speed = 200.0;

        if keyboard.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
            transform.translation += direction * speed * time.delta_secs();
        }
    }
}

/// Camera controls - Arrow keys for panning, Q/E for zoom
fn camera_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<GameCamera>>,
    mut projection_query: Query<&mut OrthographicProjection, With<GameCamera>>,
    time: Res<Time>,
) {
    let mut camera_transform = camera_query.single_mut();
    let mut projection = projection_query.single_mut();

    // Camera panning
    let pan_speed = 300.0;
    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    if direction.length() > 0.0 {
        direction = direction.normalize();
        camera_transform.translation += direction * pan_speed * time.delta_secs();
    }

    // Camera zoom
    let zoom_speed = 1.0;
    if keyboard.pressed(KeyCode::KeyQ) {
        projection.scale = (projection.scale - zoom_speed * time.delta_secs()).max(0.1);
    }
    if keyboard.pressed(KeyCode::KeyE) {
        projection.scale = (projection.scale + zoom_speed * time.delta_secs()).min(10.0);
    }
}

// ============================================================================
// EXAMPLES: Add more systems as needed
// ============================================================================

// Example: Sprite animation system
// #[derive(Component)]
// struct AnimationIndices {
//     first: usize,
//     last: usize,
// }
//
// #[derive(Component, Deref, DerefMut)]
// struct AnimationTimer(Timer);
//
// fn animate_sprite(
//     time: Res<Time>,
//     mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
// ) {
//     for (indices, mut timer, mut sprite) = &mut query {
//         timer.tick(time.delta());
//         if timer.just_finished() {
//             if let Some(atlas) = &mut sprite.texture_atlas {
//                 atlas.index = if atlas.index == indices.last {
//                     indices.first
//                 } else {
//                     atlas.index + 1
//                 };
//             }
//         }
//     }
// }

// Example: Collision detection
// #[derive(Component)]
// struct Collider {
//     size: Vec2,
// }
//
// fn check_collisions(
//     query: Query<(Entity, &Transform, &Collider)>,
// ) {
//     let entities: Vec<_> = query.iter().collect();
//     for (i, (entity_a, transform_a, collider_a)) in entities.iter().enumerate() {
//         for (entity_b, transform_b, collider_b) in entities.iter().skip(i + 1) {
//             let distance = transform_a.translation.truncate()
//                 .distance(transform_b.translation.truncate());
//             let min_distance = (collider_a.size.x + collider_b.size.x) / 2.0;
//
//             if distance < min_distance {
//                 info!("Collision detected between {:?} and {:?}", entity_a, entity_b);
//             }
//         }
//     }
// }
