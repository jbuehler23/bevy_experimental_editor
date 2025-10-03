use bevy::prelude::*;

// This system would sync entities from SpacetimeDB tables to Bevy entities
// For now, it's a placeholder that will be filled when we have the generated bindings

pub fn spawn_player_sprite(
    commands: &mut Commands,
    position: Vec3,
    is_local: bool,
) -> Entity {
    let color = if is_local {
        Color::srgb(0.0, 1.0, 0.0)
    } else {
        Color::srgb(0.0, 0.0, 1.0)
    };

    commands.spawn((
        Sprite {
            color,
            custom_size: Some(Vec2::new(32.0, 48.0)),
            ..default()
        },
        Transform::from_translation(position),
    )).id()
}