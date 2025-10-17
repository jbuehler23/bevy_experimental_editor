//! {{PROJECT_NAME}} - A Bevy game
//!
//! This is a minimal Bevy project created with the editor.
//! Add your game systems, components, and resources below.

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
    // When enabled, this loads .scn.ron files from assets/world/ based on
    // the BEVY_EDITOR_SCENE environment variable
    #[cfg(feature = "editor-runtime")]
    {
        app.add_plugins(bevy_editor_runtime::EditorSceneLoaderPlugin);
        info!("Editor scene loader enabled - set BEVY_EDITOR_SCENE to load a scene");
    }

    // Add your game systems here
    app.add_systems(Startup, setup);
    app.add_systems(Update, game_update);

    app.run();
}

/// Initial setup - spawns camera and other persistent entities
fn setup(mut commands: Commands) {
    // Spawn a 2D camera
    commands.spawn((
        Camera2d,
        Name::new("Main Camera"),
    ));

    info!("{{PROJECT_NAME}} initialized!");
}

/// Main game update loop - runs every frame
fn game_update() {
    // Add your game logic here:
    // - Player input handling
    // - Game state updates
    // - Physics simulation
    // - AI logic
    // etc.
}

// Example: Add more systems as needed
//
// fn player_movement(
//     keyboard: Res<ButtonInput<KeyCode>>,
//     mut player_query: Query<&mut Transform, With<Player>>,
//     time: Res<Time>,
// ) {
//     for mut transform in &mut player_query {
//         let mut direction = Vec3::ZERO;
//
//         if keyboard.pressed(KeyCode::KeyW) {
//             direction.y += 1.0;
//         }
//         if keyboard.pressed(KeyCode::KeyS) {
//             direction.y -= 1.0;
//         }
//         if keyboard.pressed(KeyCode::KeyA) {
//             direction.x -= 1.0;
//         }
//         if keyboard.pressed(KeyCode::KeyD) {
//             direction.x += 1.0;
//         }
//
//         if direction.length() > 0.0 {
//             direction = direction.normalize();
//             transform.translation += direction * 200.0 * time.delta_secs();
//         }
//     }
// }
